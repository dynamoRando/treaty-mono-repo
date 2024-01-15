use treaty_http_endpoints::client::GET_DELETES_TO_HOST_BEHAVIOR;
use treaty_types::enums::DeletesToHostBehavior;
use treaty_types::types::treaty_proto::{
    GetDeletesToHostBehaviorReply, GetDeletesToHostBehaviorRequest,
};
use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html};

use crate::request::get_client;
use crate::{
    log::log_to_console,
    pages::{
        behaviors::deletes_to_host::change_behavior::ChangeBehavior,
        common::{select_database::SelectDatabase, select_table::SelectTable},
    },
    request::{self, clear_status, get_database, set_status},
};
use num::FromPrimitive;
mod change_behavior;

#[function_component]
pub fn DeletesToHost() -> Html {
    let active_database = use_state_eq(move || String::from(""));
    let active_table_database = active_database.clone();
    let database = active_database.clone();
    let active_table = use_state_eq(move || String::from(""));
    let table = active_table.clone();

    let behavior_type_state = use_state_eq(move || String::from(""));

    let table_names = use_state_eq(move || {
        let x: Vec<String> = Vec::new();
        x
    });

    let onclick_db = {
        let table_names = table_names;
        Callback::from(move |db_name: String| {
            let database = get_database(&db_name);
            let mut names: Vec<String> = Vec::new();

            for table in &database.tables {
                names.push(table.table_name.clone());
            }

            table_names.set(names);
        })
    };

    let onclick_table = {
        let active_database = active_database.clone();
        let behavior_type_state = behavior_type_state.clone();
        Callback::from(move |table_name: String| {
            let behavior_type_state = behavior_type_state.clone();
            if !table_name.is_empty() {
                log_to_console(table_name.clone());

                let client = get_client();

                let request = GetDeletesToHostBehaviorRequest {
                    database_name: active_database.to_string(),
                    table_name,
                };

                let body = serde_json::to_string(&request).unwrap();
                let url = format!("{}{}", client.user_addr_port(), GET_DELETES_TO_HOST_BEHAVIOR);

                let cb =
                    Callback::from(move |response: Result<AttrValue, String>| match response {
                        Ok(response) => {
                            log_to_console(response.to_string());
                            clear_status();

                            let reply: GetDeletesToHostBehaviorReply =
                                serde_json::from_str(&response).unwrap();

                            let behavior = reply.behavior.unwrap();
                            let behavior_val = DeletesToHostBehavior::from_u32(behavior)
                                .unwrap()
                                .to_string();
                            behavior_type_state.set(behavior_val);
                        }
                        Err(error_message) => {
                            set_status(error_message);
                        }
                    });

                request::post(url, body, cb);
            }
        })
    };

    html! {
        <div>
            <div class="box">
                <p><h1 class="subtitle">{"Deletes To Host"}</h1></p>
                <p><label for="databases">{ "Select Database " }</label></p>
                <p>< SelectDatabase active_db_name={active_database} onclick_db={onclick_db}/></p>
                <p><label for="tables">{ "Select Table " }</label></p>
                <p>< SelectTable
                    active_database_name={active_table_database}
                    active_table_name = {active_table}
                    onclick_table={onclick_table}/>
                </p>
                <p>{"Current Behavior: "}</p>
                <p>{(*behavior_type_state).clone()}</p>
                < ChangeBehavior active_database={database} active_table={table}/>
            </div>
        </div>
    }
}
