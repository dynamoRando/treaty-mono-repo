use crate::{
    setup::{DemoClientBuilder, TreatyInstance, DB_NAME},
    DEBUG,
};
use stdext::function_name;
use treaty_client::client_actions::ClientActions;
use treaty_types::enums::DatabaseType;

/// A user (participant) of our application
#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: u32,
    pub alias: String,
}

impl User {
    pub fn new(alias: &str) -> Self {
        Self {
            id: 0,
            alias: alias.to_string(),
        }
    }

    pub fn new_with_id(alias: &str, id: u32) -> Self {
        Self {
            id,
            alias: alias.to_string(),
        }
    }

    /// add the User to Treaty
    pub async fn add(&self) {
        let mut client = DemoClientBuilder::build(TreatyInstance::Host).await;
        let db_type = DatabaseType::to_u32(DatabaseType::Sqlite);
        let result = client
            .execute_read_at_host(DB_NAME, "SELECT MAX(id) max_id FROM USERS;", db_type)
            .await
            .unwrap();

        let max_id_string = result
            .rows
            .first()
            .unwrap()
            .values
            .first()
            .unwrap()
            .string_value
            .clone();

        let max_id: u32 = if max_id_string.is_empty() {
            0
        } else {
            max_id_string.parse().unwrap()
        };

        let id = max_id + 1;

        let sql_statement = r#"
        INSERT INTO USERS
        (
            id,
            alias
        )
        VALUES
        (
            :id,
            ':alias'
        );
        "#;

        let sql_statement = sql_statement.replace(":id", &id.to_string());
        let sql_statement = sql_statement.replace(":alias", &self.alias);

        client
            .execute_write_at_host(DB_NAME, &sql_statement, db_type, "")
            .await
            .unwrap();
    }

    pub async fn get_all() -> Vec<User> {
        let mut client = DemoClientBuilder::build(TreatyInstance::Host).await;
        let db_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let sql_statement = r#"
        SELECT 
            id, 
            alias
        FROM
            USERS
        ;
        "#;

        let result = client
            .execute_read_at_host(DB_NAME, sql_statement, db_type)
            .await
            .unwrap();

        if DEBUG {
            println!("[{}]: {result:?}", function_name!());
        }

        let mut users: Vec<User> = Vec::new();

        for row in result.rows {
            let user = User::from_row(&row);
            if DEBUG {
                println!("[{}]: user_from_row: {user:?}", function_name!());
            }
            users.push(user);
        }

        users
    }

    pub async fn get(&self) -> User {
        let users = User::get_all().await;

        if !users.is_empty() {
            let result = users.iter().find(|x| x.alias.as_str() == self.alias);
            match result {
                Some(user) => {
                    if DEBUG {
                        println!("[{}]: found user by alias: {user:?}", function_name!());
                    }
                    return user.clone();
                }
                None => {
                    let result = users.iter().find(|x: &&User| x.id == self.id);
                    match result {
                        Some(user) => {
                            if DEBUG {
                                println!("[{}]: found user by id: {user:?}", function_name!());
                            }
                            return user.clone();
                        }
                        None => return User::default(),
                    }
                }
            }
        }

        User::default()
    }

    fn from_row(row: &treaty::treaty_proto::Row) -> User {
        let mut id: u32 = 0;
        let mut alias = String::from("");

        for value in row.values.iter() {
            if value.column.as_ref().unwrap().column_name == "id" {
                id = value.string_value.parse().unwrap();
            }

            if value.column.as_ref().unwrap().column_name == "alias" {
                alias = value.string_value.clone();
            }
        }

        User::new_with_id(&alias, id)
    }
}
