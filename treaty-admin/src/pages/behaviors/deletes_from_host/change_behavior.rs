use num::FromPrimitive;
use treaty_http_endpoints::client::CHANGE_DELETES_FROM_HOST_BEHAVIOR;
use treaty_types::enums::{DeletesFromHostBehavior, UpdatesFromHostBehavior};
use treaty_types::types::treaty_proto::{
    ChangeDeletesFromHostBehaviorReply, ChangeDeletesFromHostBehaviorRequest,
};
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_node_ref, AttrValue, Callback, Html, Properties, UseStateHandle,
};

use crate::request::get_client;
use crate::{
    log::log_to_console,
    request::{self, clear_status, set_status},
};

#[derive(Properties, PartialEq)]
pub struct ChangeBehaviorProps {
    pub active_database: UseStateHandle<String>,
    pub active_table: UseStateHandle<String>,
}

#[function_component]
pub fn ChangeBehavior(
    ChangeBehaviorProps {
        active_database,
        active_table,
    }: &ChangeBehaviorProps,
) -> Html {
    let ui_behavior = use_node_ref();
    let database = active_database.clone();
    let table = active_table.clone();

    let onclick = {
        let ui_behavior = ui_behavior.clone();
        let database = database;
        let table = table;
        Callback::from(move |_| {
            let behavior = ui_behavior.cast::<HtmlInputElement>().unwrap().value();
            let database = database.clone();
            let table = table.clone();

            let behavior_value =
                UpdatesFromHostBehavior::from_u32(behavior.parse::<u32>().unwrap());
            let behavior_value = UpdatesFromHostBehavior::to_u32(behavior_value);

            let client = get_client();
            let url = format!("{}{}", client.user_addr_port(), CHANGE_DELETES_FROM_HOST_BEHAVIOR);
            let request = ChangeDeletesFromHostBehaviorRequest {
                database_name: (*database).clone(),
                table_name: (*table).clone(),
                behavior: behavior_value,
            };

            let body = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if response.is_ok() {
                    let database = database.clone();
                    let table = table.clone();
                    clear_status();
                    let response = response.unwrap();
                    log_to_console(response.to_string());

                    let reply: ChangeDeletesFromHostBehaviorReply =
                        serde_json::from_str(&response).unwrap();

                    if reply.is_successful {
                        let behavior: DeletesFromHostBehavior =
                            DeletesFromHostBehavior::from_u32(behavior_value).unwrap();
                        let behavior = behavior.to_string();

                        let message = format!(
                            "{}{}{}{}{}{}",
                            "Behavior Updated For: ",
                            (*database),
                            " table: ",
                            (*table),
                            " behavior to: ",
                            behavior
                        );
                        set_status(message);
                    } else {
                        let behavior: DeletesFromHostBehavior =
                            DeletesFromHostBehavior::from_u32(behavior_value).unwrap();
                        let behavior = behavior.to_string();

                        let message = format!(
                            "{}{}{}{}{}{}",
                            "Behavior Updated FAILED For: ",
                            (*database),
                            " table: ",
                            (*table),
                            " behavior to: ",
                            behavior
                        );
                        set_status(message);
                    }
                }
            });

            request::post(url, body, cb);
        })
    };

    html!(
        <div>
            <p><h1 class="subtitle">{"Change Behavior"}</h1></p>
                <div class ="select is-multiple">
                <select name="set_deletes_from_host_behavior" id="set_deletes_from_host_behavior" ref={ui_behavior} >
                    <option value="0">{"SELECT BEHAVIOR"}</option>
                    <option value="1">{"AllowRemoval"}</option>
                    <option value="2">{"QueueForReview"}</option>
                    <option value="3">{"DeleteWithLog"}</option>
                    <option value="4">{"Ignore"}</option>
                    <option value="5">{"QueueForReviewAndLog"}</option>
                </select>
                </div>
                <button
                    class="button"
                    type="button"
                    id="updae_behavior"
                    value="Update Behavior"
                    onclick={onclick}>
                        <span class="mdi mdi-eject-circle">{" Update Behavior"}</span>
                </button>
        </div>
    )
}
