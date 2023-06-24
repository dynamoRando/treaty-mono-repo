use treaty_http_endpoints::client::{ACCEPT_PENDING_CONTRACT, VIEW_PENDING_CONTRACTS};
use treaty_types::{types::treaty_proto::{
    AcceptPendingContractReply, AcceptPendingContractRequest, Contract, ViewPendingContractsReply,
    ViewPendingContractsRequest,
}, formatter};
use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html};

use crate::{
    log::log_to_console,
    request::{self, clear_status, get_token, set_status, update_token_login_status},
};

#[function_component]
pub fn Pending() -> Html {
    let pending_contracts = use_state_eq(move || {
        let x: Vec<Contract> = Vec::new();
        x
    });

    let view_pending_contract_details = use_state_eq(move || String::from(""));
    let last_accept_reject_result = use_state_eq(move || String::from(""));

    let onclick = {
        let pending_contracts = pending_contracts.clone();
        Callback::from(move |_| {
            let token = get_token();
            let pending_contracts = pending_contracts.clone();

            let request = ViewPendingContractsRequest {
                authentication: Some(token.auth()),
            };

            let request_json = serde_json::to_string(&request).unwrap();
            let url = format!("{}{}", token.addr, VIEW_PENDING_CONTRACTS);
            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if let Ok(ref x) = response {
                    log_to_console(x.to_string());
                    clear_status();

                    let reply: ViewPendingContractsReply = serde_json::from_str(x).unwrap();
                    let is_authenticated = reply
                        .authentication_result
                        .as_ref()
                        .unwrap()
                        .is_authenticated;
                    update_token_login_status(is_authenticated);

                    if is_authenticated {
                        let contracts = reply.contracts;
                        pending_contracts.set(contracts);
                    }
                } else {
                    set_status(response.err().unwrap());
                }
            });

            request::post(url, request_json, cb);
        })
    };

    html! {
        <div>
            <h1 class="subtitle">{"View Pending Contracts"}</h1>
            <p>
                <button class="button is-primary" {onclick}>
                    <span class="mdi mdi-timer">{" View Pending Contracts"}</span>
                </button>
            </p>
            <p><h2 class="subtitle">{"Pending Contracts"}</h2></p>
            <div class="table-container">
            <table class="table is-narrow">
                <thead>
                    <tr>
                        <th>{"Host Name"}</th>
                        <th>{"Database Name"}</th>
                        <th>{"Description"}</th>
                        <th>{"View/Accept/Reject?"}</th>
                    </tr>
                </thead>
                {
                    (*pending_contracts).clone().into_iter().map(|c|{
                        let contract = c.clone();
                        let host_name = c.host_info.unwrap().host_name.clone();
                        let database_name = c.schema.unwrap().database_name.clone();
                        let description = c.description.clone();
                        let last_accept_reject_result = last_accept_reject_result.clone();

                        html!{
                            <tr>
                                <td>{host_name.clone()}</td>
                                <td>{database_name}</td>
                                <td>{description}</td>
                                <td>
                                    <div class="buttons">
                                    <button type="button" class="button is-primary" id="get_databases" value="View Contract"
                                    onclick=
                                    {
                                        let view_pending_contract_details = view_pending_contract_details.clone();
                                        Callback::from(move |_| {
                                            let text = formatter::markdown::contract::contract_to_markdown_table(&contract);
                                            view_pending_contract_details.set(text);
                                        })
                                    }>
                                    <span class="mdi mdi-file-eye">{" View"}</span>
                                    </button>
                                    <button type="button" class="button is-success" id="get_databases" value="View Contract"
                                    onclick=
                                    {
                                        let host_name = host_name.clone();
                                        let last_accept_reject_result = last_accept_reject_result.clone();
                                        Callback::from(move |_| {
                                            let token = get_token().clone();
                                            let request = AcceptPendingContractRequest{
                                                authentication: Some(token.auth()),
                                                host_alias: host_name.clone(),
                                            };

                                            let json_request = serde_json::to_string(&request).unwrap();
                                            let url = format!("{}{}", token.addr, ACCEPT_PENDING_CONTRACT);
                                            let last_accept_reject_result = last_accept_reject_result.clone();

                                            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                                                if let Ok(ref x) = response {
                                                    clear_status();
                                                    log_to_console(x.to_string());

                                                    let reply: AcceptPendingContractReply = serde_json::from_str(x).unwrap();
                                                    let is_authenticated = reply.authentication_result.as_ref().unwrap().is_authenticated;
                                                    update_token_login_status(is_authenticated);

                                                    if is_authenticated {
                                                        let message = format!("{}{}", "Last accept/reject status was: ", reply.is_successful);
                                                        last_accept_reject_result.set(message);
                                                    }
                                                } else {
                                                    let error_message = response.err().unwrap();
                                                    set_status(error_message);
                                                }
                                            });

                                            request::post(url, json_request, cb)
                                        }
                                    )}>
                                    <span class="mdi mdi-file-document-check">{" Accept"}</span>
                                    </button>
                                    <button type="button" class="button is-danger" id="get_databases" value="View Contract"
                                    onclick={Callback::from(move |_|{})}>
                                    <span class="mdi mdi-file-document-remove">{" Reject"}</span>
                                    </button>
                                    </div>
                                </td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </table>
            </div>
            <p><h2 class="subittle">{"Contract Details"}</h2></p>
            <p><textarea class="textarea" rows="5" cols="60" id ="sql_Result"
            placeholder="Contract Details Will Be Displayed Here"
            value={(*view_pending_contract_details).clone()} readonly=true/></p>
            <p><h2 class="subittle">{"Last Accept/Reject Result"}</h2></p>
            <p>{last_accept_reject_result.to_string()}</p>
        </div>
    }
}
