use stdext::function_name;
use treaty_client::client_actions::ClientActions;
use treaty_types::enums::DatabaseType;

use crate::{
    setup::{DemoClientBuilder, TreatyInstance, DB_NAME},
    DEBUG_LIST
};

use super::{item::Item, user::User};

/// A grocery list item from our user
#[derive(Debug, Clone, Default)]
pub struct ListItem {
    user: User,
    item: Item,
}

impl ListItem {
    pub fn new(user: User, item: Item) -> Self {
        Self { user, item }
    }

    /// Save the grocery list item to Treaty
    pub async fn add(&self) {
        let mut client = DemoClientBuilder::build(TreatyInstance::Host).await;

        let sql_statement = r#"
        INSERT INTO LISTS
        (
            user_id,
            item_id,
            item_name
        )
        VALUES
        (
            :user_id,
            :item_id,
            ':item_name'
        );
        "#;

        let sql_statement = sql_statement.replace(":user_id", &self.user.id.to_string());
        let sql_statement = sql_statement.replace(":item_id", &self.item.id.to_string());
        let sql_statement = sql_statement.replace(":item_name", &self.item.name.to_string());

        client
            .execute_cooperative_write_at_host(DB_NAME, &sql_statement, &self.user.alias, "")
            .await
            .unwrap();
    }

    pub async fn get_all() -> Vec<ListItem> {
        let mut client = DemoClientBuilder::build(TreatyInstance::Host).await;
        let db_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let sql_statement = r#"
        SELECT
            user_id,
            item_id,
            item_name
        FROM
            LISTS
        ;
        "#;

        let result = client
            .execute_read_at_host(DB_NAME, sql_statement, db_type)
            .await
            .unwrap();

        if DEBUG_LIST {
            println!("[{}]: {result:?}", function_name!());
        }
        let mut items: Vec<ListItem> = Vec::new();

        for row in result.rows {
            let item = ListItem::from_row(&row).await;
            items.push(item);
        }

        items
    }

    async fn from_row(row: &treaty::treaty_proto::Row) -> ListItem {
        let mut user_id: u32 = 0;
        let mut item_id: u32 = 0;
        let mut item_name = String::from("");

        for value in row.values.iter() {
            if value.column.as_ref().unwrap().column_name == "user_id" {
                user_id = value.string_value.parse().unwrap();
            }

            if value.column.as_ref().unwrap().column_name == "item_id" {
                item_id = value.string_value.parse().unwrap();
            }

            if value.column.as_ref().unwrap().column_name == "item_name" {
                item_name = value.string_value.clone();
            }
        }

        let user = User::new_with_id("", user_id);
        let user = user.get().await;

        let item = Item::new_with_id(&item_name, item_id);
        let item = item.get().await;

        ListItem::new(user, item)
    }
}
