use crate::{log::log_to_console, pages::treaty_admin::databases::columns::ColumnProps};

use crate::request::{
    self,
    treaty::{clear_status, get_treaty_token, set_status, update_token_login_status},
};
use treaty_types::types::treaty_proto::{SetLogicalStoragePolicyReply, SetLogicalStoragePolicyRequest};
use treaty_types::proxy::request_type::RequestType;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::{function_component, html, use_node_ref, use_state_eq, AttrValue, Callback, Html};

#[function_component]
pub fn SetTablePolicy(ColumnProps { table }: &ColumnProps) -> Html {
    let set_policy_result = use_state_eq(|| None);
    let set_new_policy = use_state_eq(|| None);

    let ui_new_policy = use_node_ref();

    let database_name = table.database_name.clone();
    let table_name = table.table_name.clone();

    let onchange = {
        let set_new_policy = set_new_policy.clone();
        let ui_new_policy = ui_new_policy.clone();

        Callback::from(move |_| {
            log_to_console("SetTablePolicy - Onchange");
            let policy = ui_new_policy.cast::<HtmlInputElement>();

            if policy.is_some() {
                let policy_val = ui_new_policy.cast::<HtmlInputElement>().unwrap().value();
                set_new_policy.set(Some(policy_val));
            }
        })
    };

    let onclick: Callback<MouseEvent> = {
        log_to_console("SetTablePolicy - Clicked");
        let set_new_policy = set_new_policy;

        if set_new_policy.is_some() {
            let policy_val = set_new_policy.as_ref().unwrap();

            let message = format!("{}{}", "SetTablePolicy Value: ", policy_val.clone());
            log_to_console(&message);

            let policy_num: u32 = policy_val.parse().unwrap();

            let database_name = database_name;
            let table_name = table_name.clone();

            let set_policy_result = set_policy_result.clone();

            Callback::from(move |_| {
                let token = get_treaty_token();
                let set_policy_result = set_policy_result.clone();

                let cb = Callback::from(move |response: Result<AttrValue, String>| {
                    if let Ok(ref x) = response {
                        log_to_console(x);
                        clear_status();

                        let reply: SetLogicalStoragePolicyReply = serde_json::from_str(x).unwrap();

                        let is_authenticated = reply
                            .authentication_result
                            .as_ref()
                            .unwrap()
                            .is_authenticated;
                        update_token_login_status(is_authenticated);

                        if is_authenticated {
                            set_policy_result.set(Some(reply.is_successful));
                        }
                    } else {
                        set_status(response.err().unwrap());
                    }
                });

                let database_name = database_name.clone();
                let table_name = table_name.clone();

                let request = SetLogicalStoragePolicyRequest {
                    authentication: Some(token.auth()),
                    database_name,
                    table_name,
                    policy_mode: policy_num,
                };

                let request_json = serde_json::to_string(&request).unwrap();

                let message = format!("{}{}", "SENDING REQUEST FOR NEW POLICY: ", request_json);
                log_to_console(&message);

                request::post(RequestType::SetLogicalStoragePolicy, &request_json, cb);
            })
        } else {
            Callback::from(move |_| {})
        }
    };

    html!(
        <div class="container">
            <p>
                <h3 class="subtitle">{"Set Policy"}</h3>
                <p>{"Click on the table name FIRST before setting new policy."}</p>
                <label for="table_policy_value">{ "Set Policy For Table: " }{&table_name}</label>
                <p>
                    <div class="select is-multiple">
                        <select name="set_policy_for_table" id="set_policy_for_table" ref={&ui_new_policy} {onchange}>
                            /*
                                None = 0,
                                HostOnly = 1,
                                ParticpantOwned = 2,
                                Shared = 3,
                                Mirror = 4,
                            */
                            <option value={"0"}>{"None"}</option>
                            <option value={"1"}>{"Host Only"}</option>
                            <option value={"2"}>{"Participant Owned"}</option>
                            <option value={"3"}>{"Shared"}</option>
                            <option value={"4"}>{"Mirror"}</option>
                        </select>
                    </div>
                </p>
                <p><input class="button is-primary" type="button" id="submit_new_table_policy" value="Update Policy" {onclick}/></p>
                <p>{"Last result for table: "}{&table_name}{" was: "}{*(set_policy_result)}</p>
            </p>
        </div>
    )
}
