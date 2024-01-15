use treaty_types::types::treaty_proto::{
    EnableCoooperativeFeaturesReply, EnableCoooperativeFeaturesRequest,
};
use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html};

use crate::{log::log_to_console, pages::treaty_admin::common::select_database::SelectDatabase};

use crate::request::{
    self,
    treaty::{clear_status, get_treaty_token, set_status},
};
use treaty_types::proxy::request_type::RequestType;

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
            let token = get_treaty_token();

            let request = EnableCoooperativeFeaturesRequest {
                database_name: (*active_database).clone(),
            };

            let json_request = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if let Ok(ref x) = response {
                    log_to_console(x);
                    clear_status();

                    let reply: EnableCoooperativeFeaturesReply = serde_json::from_str(x).unwrap();

                    let message = format!(
                        "{}{}",
                        "Last cooperation enable request for database was: ", reply.is_successful
                    );
                    enable_result.set(message);
                } else {
                    set_status(response.err().unwrap());
                }
            });

            request::post(RequestType::EnableCooperativeFeatures, &json_request, cb);
        })
    };

    html!(
        <div>
            <div class="container">
                <div class="box">
                    <h1 class="subtitle"> {"Enable Cooperative Features"} </h1>
                    <p>{"Enabling cooperative features on a database creates additional schema objects in that database and is tracked by treaty
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
