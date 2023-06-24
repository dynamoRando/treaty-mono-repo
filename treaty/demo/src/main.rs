use std::env;
use models::{item::Item, user::User, list::ListItem};
pub mod setup;
pub mod models;

pub const DEBUG: bool = false;
pub const DEBUG_LIST: bool = true;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    process_cmd_args(args).await;
}

async fn process_cmd_args(args: Vec<String>) {
    if args.len() >= 2 {
        let cmd = args[1].as_str();

        match cmd {
            "setup" => {
                setup::init_host().await;
                setup::init_part1().await;
                setup::init_part2().await;
            }
            "signup-users" => {
                setup::add_participant1().await;
                setup::add_participant2().await;
                setup::send_contract_to_participant1().await;
                setup::send_contract_to_participant2().await;
            },
            "part1-accept" => {
                setup::part1_accept_contract().await;
            },
            "part2-accept" => {
                setup::part2_accept_contract().await;
            },
            "add-item" => {
                // args would be "add-item <name>"
                // example: "add-item tomatoes" or "add-item eggs"
                let item_name = args[2].as_str();
                let item = Item::new(item_name);
                item.add().await;
                println!("finished adding {item:?}")
            },
            "add-list" => {
                // args would be "add-list <user-alias> <item-name>"
                // example: "add-list participant1 eggs"
                let user_name = args[2].as_str();
                let item_name = args[3].as_str();

                let user = User::new(user_name);
                let user = user.get().await;

                let item = Item::new(item_name);
                let item = item.get().await;

                let list_item = ListItem::new(user, item);
                list_item.add().await;
                println!("finished adding {list_item:?}")
            },
            "view-items" => {
                let items = Item::get_all().await;
                println!("items in database: {items:?}");
            },
            "view-users" => {
                let users = User::get_all().await;
                println!("users in database: {users:?}");
            },
            "view-lists" => {
                let lists = ListItem::get_all().await;
                println!("lists in database: {lists:?}");
            },
            "part1-data-update" => {
                setup::part1_update_manual().await;
                println!("part1-data-update finished");
            }
            _ => {}
        }
    }
}