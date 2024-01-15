use crate::request;
use crate::{
    log::log_to_console,
    pages::treaty_admin::common::pending_actions::PendingActions,
    request::treaty::{clear_status, get_treaty_token, set_status},
};
use treaty_types::proxy::request_type::RequestType;
use treaty_types::types::treaty_proto::{
    AcceptPendingActionReply, AcceptPendingActionRequest, GetPendingActionsReply,
    GetPendingActionsRequest, PendingStatement,
};
use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Html, Properties, UseStateHandle,
};

#[derive(Properties, PartialEq)]
pub struct ViewPendingDeleteProps {
    pub active_database: UseStateHandle<String>,
    pub active_table: UseStateHandle<String>,
}

#[function_component]
pub fn ViewPendingDeletes(
    ViewPendingDeleteProps {
        active_database,
        active_table,
    }: &ViewPendingDeleteProps,
) -> Html {
    let pending_actions = use_state_eq(move || {
        let x: Vec<PendingStatement> = Vec::new();
        x
    });

    let active_database = active_database.clone();
    let active_table = active_table.clone();

    let callback_accept = {
        let active_database = active_database.clone();
        let active_table = active_table.clone();

        Callback::from(move |accepted_row_id: u32| {
            let active_database = active_database.clone();
            let active_table = active_table.clone();

            let token = get_treaty_token();

            let request = AcceptPendingActionRequest {
                database_name: (*active_database).clone(),
                table_name: (*active_table).clone(),
                row_id: accepted_row_id,
            };

            let body = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| match response {
                Ok(response) => {
                    clear_status();

                    log_to_console(&response);

                    let reply: AcceptPendingActionReply = serde_json::from_str(&response).unwrap();
                }
                Err(error_message) => {
                    set_status(error_message);
                }
            });

            request::post(RequestType::AcceptPendingContract, &body, cb);
        })
    };

    let callback_reject = {
        let active_database = active_database.clone();
        let active_table = active_table.clone();

        Callback::from(move |rejected_row_id: u32| {
            // LOL: We never wrote a reject message
            let active_database = active_database.clone();
            let active_table = active_table.clone();

            let token = get_treaty_token();

            let request = AcceptPendingActionRequest {
                database_name: (*active_database).clone(),
                table_name: (*active_table).clone(),
                row_id: rejected_row_id,
            };

            let body = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| match response {
                Ok(response) => {
                    clear_status();
                    log_to_console(&response);

                    let reply: AcceptPendingActionReply = serde_json::from_str(&response).unwrap();
                }
                Err(error_message) => {
                    set_status(error_message);
                }
            });

            request::post(RequestType::AcceptPendingAction, &body, cb);
            todo!("LOL: We never wrote a reject message");
        })
    };

    let onclick_view = {
        let active_database = active_database;
        let active_table = active_table;
        let pending_actions = pending_actions.clone();

        Callback::from(move |_| {
            let pending_actions = pending_actions.clone();
            let token = get_treaty_token();

            let request = GetPendingActionsRequest {
                database_name: (*active_database).clone(),
                table_name: (*active_table).clone(),
                action: "DELETE".to_string(),
            };

            let body = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| match response {
                Ok(response) => {
                    clear_status();

                    log_to_console(&response);

                    let reply: GetPendingActionsReply = serde_json::from_str(&response).unwrap();

                    let actions = reply.pending_statements;
                    pending_actions.set(actions);
                }
                Err(error_message) => {
                    set_status(error_message);
                }
            });

            request::post(RequestType::GetPendingActions, &body, cb);
        })
    };

    html!(
        <div>
            <p><h1 class="subtitle">{"View Pending Deletes From Host"}</h1></p>
                <button class="button" type="button" id="view_pending_deletes" value="View Pending Deletes" onclick={onclick_view}>
                    <span class="mdi mdi-magnify"></span>{" View Pending Deletes"}
                </button>
            <p><h1 class="subtitle">{"Pending Deletes From Host"}</h1></p>
            < PendingActions pending_actions={pending_actions} onclick_accept={callback_accept} onclick_reject={callback_reject} />
        </div>
    )
}
