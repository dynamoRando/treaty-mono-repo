use treaty_http_endpoints::client::GET_HOST_INFO;
use treaty_types::types::treaty_proto::{Host, HostInfoReply};

use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html};

use crate::{
    log::log_to_console,
    request::{self, clear_status, get_token, set_status, update_token_login_status},
};

#[function_component]
pub fn GetInfo() -> Html {
    let host_info = use_state_eq(move || Host {
        host_guid: "".to_string(),
        host_name: "".to_string(),
        network: None,
        token: Vec::new(),
    });

    let get_host_info_onclick = {
        let host_info = host_info.clone();
        Callback::from(move |_| {
            let host_info = host_info.clone();
            let token = get_token();
            let url = format!("{}{}", token.addr, GET_HOST_INFO);

            let request_json = token.auth_json();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if let Ok(ref x) = response {
                    clear_status();
                    log_to_console(x.to_string());
                    let host_info = host_info.clone();
                    let reply: HostInfoReply = serde_json::from_str(x).unwrap();

                    let is_authenticated = reply
                        .authentication_result
                        .as_ref()
                        .unwrap()
                        .is_authenticated;
                    update_token_login_status(is_authenticated);

                    if is_authenticated {
                        host_info.set(reply.host_info.unwrap());
                    }
                } else {
                    let error_message = response.err().unwrap();
                    set_status(error_message);
                }
            });

            request::post(url, request_json, cb);
        })
    };

    html!(
        <div>
            <p><h1 class="subtitle"> {"View Host Info"} </h1></p>
            <p>
                <button class="button is-primary" onclick={get_host_info_onclick}>
                    <span class="mdi mdi-eye">{" Get Info"}</span>
                </button>
            </p>
            <p>
                <div class="table-container">
                    <table class="table is-narrow">
                        <thead>
                            <th>{"Id"}</th>
                            <th>{"Name"}</th>
                        </thead>
                        <tr>
                            <td>{host_info.host_guid.clone()}</td>
                            <td>{host_info.host_name.clone()}</td>
                        </tr>
                    </table>
                </div>
            </p>
        </div>
    )
}
