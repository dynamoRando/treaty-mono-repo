use treaty_types::enums::UpdatesFromHostBehavior;
use treaty_http_endpoints::client::GET_UPDATES_FROM_HOST_BEHAVIOR;
use yew::{function_component, html, Html};

use treaty_types::types::treaty_proto::{GetUpdatesFromHostBehaviorReply, GetUpdatesFromHostBehaviorRequest};
use yew::{use_state_eq, AttrValue, Callback};

mod change_behavior;
mod view_pending_updates;

use crate::{
    log::log_to_console,
    pages::{
        behaviors::updates_from_host::{
            change_behavior::ChangeBehavior, view_pending_updates::ViewPendingUpdates,
        },
        common::{select_database::SelectDatabase, select_table::SelectTable},
    },
    request::{
        self, clear_status, get_databases, get_token, set_status, update_token_login_status,
    },
};

#[function_component]
pub fn UpdatesFromHost() -> Html {
    let active_database = use_state_eq(move || String::from(""));
    let active_table_database = active_database.clone();
    let active_table = use_state_eq(move || String::from(""));
    let database = active_database.clone();
    let db = database.clone();
    let table = active_table.clone();
    let tbl = table.clone();

    let behavior_type_state = use_state_eq(move || String::from(""));

    let table_names = use_state_eq(move || {
        let x: Vec<String> = Vec::new();
        x
    });

    let onclick_db = {
        let table_names = table_names;
        Callback::from(move |db_name: String| {
            let databases = get_databases();

            let database = databases
                .iter()
                .find(|x| x.database_name.as_str() == db_name)
                .unwrap()
                .clone();

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

                let token = get_token();

                let request = GetUpdatesFromHostBehaviorRequest {
                    authentication: Some(token.auth()),
                    database_name: active_database.to_string(),
                    table_name,
                };

                let body = serde_json::to_string(&request).unwrap();
                let url = format!("{}{}", token.addr, GET_UPDATES_FROM_HOST_BEHAVIOR);

                let cb = Callback::from(move |response: Result<AttrValue, String>| {
                    if let Ok(ref x) = response {
                        log_to_console(x.to_string());
                        clear_status();

                        let reply: GetUpdatesFromHostBehaviorReply =
                            serde_json::from_str(x).unwrap();

                        let is_authenticated = reply
                            .authentication_result
                            .as_ref()
                            .unwrap()
                            .is_authenticated;
                        update_token_login_status(is_authenticated);

                        if is_authenticated {
                            let behavior = reply.behavior.unwrap();
                            let behavior_value =
                                UpdatesFromHostBehavior::from_u32(behavior).as_string();
                            behavior_type_state.set(behavior_value);
                        }
                    } else {
                        set_status(response.err().unwrap());
                    }
                });

                request::post(url, body, cb);
            }
        })
    };

    html! {
        <div>
            <div class="box">
                <p><h1 class="subtitle">{"Updates From Host"}</h1></p>
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
                < ViewPendingUpdates active_database={db} active_table={tbl}/>
                < ChangeBehavior active_database={database} active_table={table}/>
            </div>
        </div>
    }
}
