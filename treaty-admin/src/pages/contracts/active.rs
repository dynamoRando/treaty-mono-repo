use treaty_http_endpoints::client::GET_ACTIVE_CONTRACT;
use treaty_types::{types::treaty_proto::{GetActiveContractReply, GetActiveContractRequest}, formatter};

use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html};

use crate::{
    log::log_to_console,
    pages::common::select_database::SelectDatabase,
    request::{self, clear_status, get_database, get_token, set_status, update_token_login_status},
};

#[function_component]
pub fn Active() -> Html {
    let active_db = use_state_eq(move || String::from(""));
    let active_contract_text = use_state_eq(move || String::from(""));

    let onclick_db = {
        let active_contract_text = active_contract_text.clone();
        Callback::from(move |db_name: String| {
            let active_contract_text = active_contract_text.clone();

            if db_name.is_empty() || db_name == "SELECT DATABASE" {
            } else {
                active_contract_text.set(String::from(""));

                let database = get_database(&db_name);
                let cooperation_enabled = database.cooperation_enabled;

                if cooperation_enabled {
                    let token = get_token();
                    let auth = token.auth();

                    let get_active_contract_request = GetActiveContractRequest {
                        authentication: Some(auth),
                        database_name: db_name.clone().into(),
                    };

                    let request_json = serde_json::to_string(&get_active_contract_request).unwrap();
                    let url = format!("{}{}", token.addr, GET_ACTIVE_CONTRACT);

                    let cb = Callback::from(move |response: Result<AttrValue, String>| {
                        if let Ok(ref x) = response {
                            log_to_console(x.to_string());
                            clear_status();

                            let reply: GetActiveContractReply = serde_json::from_str(x).unwrap();

                            let is_authenticated = reply
                                .authentication_result
                                .as_ref()
                                .unwrap()
                                .is_authenticated;
                            update_token_login_status(is_authenticated);

                            if is_authenticated {
                                let contract = reply.contract.unwrap();
                                let contract_text =
                                    formatter::markdown::contract::contract_to_markdown_table(
                                        &contract,
                                    );
                                active_contract_text.set(contract_text);
                            }
                        } else {
                            set_status(response.err().unwrap());
                        }
                    });

                    let message = format!("{}{}", "sending active contract request for: ", db_name);
                    log_to_console(message);

                    request::post(url, request_json, cb);
                }
            }
        })
    };

    html! {
        <div>
            <p><h1 class="subtitle">{"View Active Contract"}</h1></p>
            <p>< SelectDatabase active_db_name={active_db} onclick_db={onclick_db} /></p>
            <p><textarea class="textarea" rows="5" cols="60" id ="active_contract"
            placeholder="Active Contract Details Will Be Displayed Here" value={(*active_contract_text).clone()} readonly=true/></p>
        </div>
    }
}
