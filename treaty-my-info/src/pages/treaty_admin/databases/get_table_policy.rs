use crate::{log::log_to_console, pages::treaty_admin::databases::columns::ColumnProps};

use treaty_types::types::treaty_proto::{GetLogicalStoragePolicyReply, GetLogicalStoragePolicyRequest};
use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html};

use crate::request::{
    self,
    treaty::{clear_status, get_treaty_token, set_status, update_token_login_status},
};
use treaty_types::proxy::request_type::RequestType;

#[function_component]
pub fn GetTablePolicy(ColumnProps { table }: &ColumnProps) -> Html {
    let message = format!("{}{}", "entered table policy for: ", table.table_name);
    log_to_console(&message);

    let database_name = table.database_name.clone();
    let table_name = table.table_name.clone();

    let table_policy = use_state_eq(|| None);
    let policy = table_policy.clone();

    let get_policy_response_cb = Callback::from(move |response: Result<AttrValue, String>| {
        if let Ok(ref x) = response {
            log_to_console(x);
            clear_status();

            let table_policy = table_policy.clone();

            let reply: GetLogicalStoragePolicyReply = serde_json::from_str(x).unwrap();

            let is_authenticated = reply
                .authentication_result
                .as_ref()
                .unwrap()
                .is_authenticated;
            update_token_login_status(is_authenticated);

            if is_authenticated {
                let policy_value = reply.policy_mode;

                let policy_name = match policy_value {
                    0 => "None",
                    1 => "Host Only",
                    2 => "Participant Owned",
                    3 => "Shared",
                    4 => "Mirror",
                    _ => "Unknown",
                };

                table_policy.set(Some(policy_name));
            }
        } else {
            set_status(response.err().unwrap());
        }
    });

    let token = get_treaty_token();

    let request = GetLogicalStoragePolicyRequest {
        authentication: Some(token.auth()),
        database_name,
        table_name: table_name.clone(),
    };

    let request_json = serde_json::to_string(&request).unwrap();

    request::post(
        RequestType::GetLogicalStoragePolicy,
        &request_json,
        get_policy_response_cb,
    );

    html!(
        <div class="container">
            <h2 class="subtitle">{"Table Policy"}</h2>
            <h3 class="subtitle">{"Get Policy"}</h3>
            <p>
                <p>{"Click on the table name FIRST before viewing/updating table policy."}</p>
                <p><label for="selected_table_policy">{"Current Policy For Table: "}{table_name}</label></p>
                <p><input class="input" type="text" id ="selected_table_policy" placeholder="Logical Storage Policy" value={*(policy)} readonly=true/></p>
            </p>
        </div>
    )
}
