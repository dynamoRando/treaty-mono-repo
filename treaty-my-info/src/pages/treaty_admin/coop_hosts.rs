use crate::request;
use crate::{
    log::log_to_console,
    request::treaty::{clear_status, get_treaty_token, set_status},
};
use treaty_types::enums::HostStatus;
use treaty_types::proxy::request_type::RequestType;
use treaty_types::types::treaty_proto::{
    ChangeHostStatusReply, ChangeHostStatusRequest, GetCooperativeHostsReply, HostInfoStatus,
};

use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html};

#[function_component]
pub fn CooperativeHosts() -> Html {
    let hosts_state = use_state_eq(move || {
        let x: Vec<HostInfoStatus> = Vec::new();
        x
    });

    let onclick = {
        let hosts_state = hosts_state.clone();
        Callback::from(move |_| {
            let token = get_treaty_token();

            let hosts_state = hosts_state.clone();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if let Ok(ref x) = response {
                    log_to_console(x);
                    clear_status();

                    let coop_response: GetCooperativeHostsReply = serde_json::from_str(x).unwrap();

                    let hosts = coop_response.hosts;
                    hosts_state.set(hosts);
                } else {
                    set_status(response.err().unwrap());
                }
            });

            request::post(RequestType::GetCooperativeHosts, "", cb)
        })
    };

    html! {
        <div>
            <div class="container">
                <div class="box">
                    <p><h1 class="subtitle"> {"View Cooperating Hosts"} </h1></p>
                    <p>{"This would normally show us a list of hosts that we're cooperating with, but
                    we don't have a service call for that yet. We can change the host status for 
                    hosts that we're already cooperating with though, to deny or authorize them."}</p>
                    <p><button class="button is-primary" {onclick}><span class="mdi mdi-handshake">{" View Hosts"}</span></button></p>
                </div>
                <div class="table-container">
                    <table class="table is-narrow">
                        <thead>
                            <tr>
                                <th>{"Host Id"}</th>
                                <th>{"Host Name"}</th>
                                <th>{"IP4"}</th>
                                <th>{"IP6"}</th>
                                <th>{"DB Port"}</th>
                                <th>{"Last Communication UTC"}</th>
                                <th>{"Status"}</th>
                                <th>{"HTTP Addr"}</th>
                                <th>{"HTTP Port"}</th>
                                <th>{"Change Status"}</th>
                            </tr>
                        </thead>
                        {
                            (*hosts_state).clone().into_iter().map(|h|{
                                let id = h.host.as_ref().unwrap().host_guid.clone();
                                let name = h.host.as_ref().unwrap().host_name.clone();

                                let network = h.host.as_ref().unwrap().network.as_ref().unwrap();

                                let ip4 = network.ip4_address.clone();
                                let ip6 = network.ip6_address.clone();
                                let db_port = network.database_port_number.as_ref().unwrap().to_string();
                                let http_addr = network.http_addr.clone();
                                let http_port = network.http_port.as_ref().unwrap().to_string();
                                let last_comm = h.last_communcation_utc.clone();
                                let status = HostStatus::from_u32(h.status).as_string();
                                html!{
                                    <tr>
                                        <td>{id.clone()}</td>
                                        <td>{name.clone()}</td>
                                        <td>{ip4}</td>
                                        <td>{ip6}</td>
                                        <td>{db_port}</td>
                                        <td>{last_comm}</td>
                                        <td>{status}</td>
                                        <td>{http_addr}</td>
                                        <td>{http_port}</td>
                                        <td>
                                            <div class="buttons">
                                                <button type="button" class="button is-primary" id="allow_host" value="Allow"
                                                onclick={
                                                    let id = id.clone();
                                                    let name = name.clone();
                                                    Callback::from(move |_|{
                                                        let name = name.clone();
                                                        let token = get_treaty_token();

                                                        let request = ChangeHostStatusRequest {
                                                            host_id: id.clone(),
                                                            host_alias: name.clone(),
                                                            status: HostStatus::to_u32(HostStatus::Allow)
                                                        };

                                                        let body = serde_json::to_string(&request).unwrap();

                                                        let cb = Callback::from(move |response: Result<AttrValue, String>| {

                                                            if let Ok(ref x) = response {
                                                                let name = name.clone();
                                                                clear_status();
                                                                log_to_console(x);

                                                                let reply: ChangeHostStatusReply = serde_json::from_str(x).unwrap();

                                                                if  reply.is_successful {
                                                                    let message = format!("{}{}{}", "Host status for ", name.clone(), " changed to allow.");
                                                                    set_status(message);
                                                                }
                                                            }  else {
                                                                let error_message = response.err().unwrap();
                                                                set_status(error_message);
                                                            }

                                                            });

                                                        request::post(RequestType::ChangeHostStatus, &body, cb);
                                                    })
                                                }>
                                                    <span class="mdi mdi-account-check">{" Allow"}</span>
                                                </button>
                                                <button type="button" class="button is-danger" id="deny_host" value="Deny"
                                                onclick={
                                                    let id = id.clone();
                                                    let name = name.clone();
                                                    Callback::from(move |_|{
                                                        let name = name.clone();
                                                        let token = get_treaty_token();

                                                        let request = ChangeHostStatusRequest {

                                                            host_id: id.clone(),
                                                            host_alias: name.clone(),
                                                            status: HostStatus::to_u32(HostStatus::Deny)
                                                        };

                                                        let body = serde_json::to_string(&request).unwrap();

                                                        let cb = Callback::from(move |response: Result<AttrValue, String>| {

                                                            if let Ok(ref x) = response {
                                                                let name = name.clone();
                                                                clear_status();
                                                                log_to_console(x);

                                                                let reply: ChangeHostStatusReply = serde_json::from_str(x).unwrap();


                                                                if  reply.is_successful {
                                                                    let message = format!("{}{}{}", "Host status for ", name.clone(), " changed to deny.");
                                                                    set_status(message);
                                                                }
                                                            } else {
                                                                let error_message = response.err().unwrap();
                                                                set_status(error_message);
                                                            }
                                                            });

                                                        request::post(RequestType::ChangeHostStatus, &body, cb);
                                                    })
                                                }>
                                                    <span class="mdi mdi-account-cancel">{" Deny"}</span>
                                                </button>
                                            </div>
                                        </td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    </table>
                    </div>
            </div>
        </div>
    }
}
