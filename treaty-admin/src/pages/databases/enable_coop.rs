use treaty_http_endpoints::client::ENABLE_COOPERATIVE_FEATURES;
use treaty_types::types::treaty_proto::{EnableCoooperativeFeaturesReply, EnableCoooperativeFeaturesRequest};
use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html};

use crate::{
    log::log_to_console,
    pages::common::select_database::SelectDatabase,
    request::{self, clear_status, get_token, set_status, update_token_login_status},
};

#[function_component]
pub fn EnableCoop() -> Html {
    let active_database = use_state_eq(move || String::from(""));
    let onclick_db: Option<Callback<String>> = None;
    let enable_result = use_state_eq(move || String::from(""));

    let onclick = {
        let active_database = active_database.clone();
        let enable_result = enable_result.clone();
        Callback::from(move |_| {
            let active_database = active_database.clone();
            let enable_result = enable_result.clone();
            let token = get_token();

            let request = EnableCoooperativeFeaturesRequest {
                authentication: Some(token.auth()),
                database_name: (*active_database).clone(),
            };

            let json_request = serde_json::to_string(&request).unwrap();
            let url = format!("{}{}", token.addr, ENABLE_COOPERATIVE_FEATURES);

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if let Ok(ref x) = response {
                    log_to_console(x.to_string());
                    clear_status();

                    let reply: EnableCoooperativeFeaturesReply = serde_json::from_str(x).unwrap();
                    let is_authenticated = reply.authentication_result.unwrap().is_authenticated;
                    update_token_login_status(is_authenticated);

                    if is_authenticated {
                        let message = format!(
                            "{}{}",
                            "Last cooperation enable request for database was: ",
                            reply.is_successful
                        );
                        enable_result.set(message);
                    }
                } else {
                    set_status(response.err().unwrap());
                }
            });

            request::post(url, json_request, cb);
        })
    };

    html!(
        <div>
            <div class="container">
                <div class="box">
                    <h1 class="subtitle"> {"Enable Cooperative Features"} </h1>
                    <p>{"Enabling cooperative features on a database creates additional schema objects in that database and is tracked by Treaty
                    for cooperation purposes."}</p>
                    <p><label for="execute_sql_dbs">{ "Select Database " }</label></p>
                    <p>< SelectDatabase active_db_name={active_database} onclick_db={onclick_db}/></p>
                    <p><button class="button is-primary" {onclick}><span class="mdi mdi-handshake">{" Enable Cooperation"}</span></button></p>
                    <p><h2 class="subtitle"> {"Last Enable Result"} </h2></p>
                    <p>{(*enable_result).clone()}</p>
                </div>
            </div>
        </div>
    )
}
