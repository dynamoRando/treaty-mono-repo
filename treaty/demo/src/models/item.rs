use treaty_client::client_actions::ClientActions;
use treaty_types::enums::DatabaseType;
use stdext::function_name;
use crate::{setup::{DemoClientBuilder, TreatyInstance, DB_NAME}, DEBUG};

/// A grocery list item
#[derive(Debug, Clone, Default)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

impl Item {
    /// create a new grocery list item with the following name and default id of 0
    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
        }
    }

    /// create a new grocery list item with the specified name and id
    pub fn new_with_id(name: &str, id: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    /// add the grocery list item to Treaty
    pub async fn add(&self) {
        let mut client = DemoClientBuilder::build(TreatyInstance::Host).await;
        let db_type = DatabaseType::to_u32(DatabaseType::Sqlite);
        let result = client
            .execute_read_at_host(DB_NAME, "SELECT MAX(id) max_id FROM ITEMS;", db_type)
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
        INSERT INTO ITEMS
        (
            id,
            name
        )
        VALUES
        (
            :id,
            ':name'
        );
        "#;

        let sql_statement = sql_statement.replace(":id", &id.to_string());
        let sql_statement = sql_statement.replace(":name", &self.name);

        client
            .execute_write_at_host(DB_NAME, &sql_statement, db_type, "")
            .await
            .unwrap();
    }

    pub async fn get_all() -> Vec<Item> {
        let mut client = DemoClientBuilder::build(TreatyInstance::Host).await;
        let db_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let sql_statement = r#"
        SELECT 
            id, 
            name
        FROM
            ITEMS
        ;
        "#;

        let result = client
            .execute_read_at_host(DB_NAME, sql_statement, db_type)
            .await
            .unwrap();

        if DEBUG {
        println!("[{}]: {result:?}", function_name!());
        }
        let mut items: Vec<Item> = Vec::new();

        for row in result.rows {
            let item = Item::from_row(&row);
            items.push(item);
        }

        items
    }

    pub async fn get(&self) -> Item {
        let items = Item::get_all().await;

        if !items.is_empty() {
            let result = items.iter().find(|x| x.name.as_str() == self.name);
            match result {
                Some(item) => return item.clone(),
                None => {
                    let result = items.iter().find(|x| x.id == self.id);
                    match result {
                        Some(item) => return item.clone(),
                        None => return Item::default(),
                    }
                }
            }
        }

        Item::default()
    }

    fn from_row(row: &treaty::treaty_proto::Row) -> Item {
        let mut id: u32 = 0;
        let mut name = String::from("");

        for value in row.values.iter() {
            if value.column.as_ref().unwrap().column_name == "id" {
                id = value.string_value.parse().unwrap();
            }

            if value.column.as_ref().unwrap().column_name == "name" {
                name = value.string_value.clone();
            }
        }

        Item::new_with_id(&name, id)
    }
}
