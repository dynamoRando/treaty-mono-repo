use treaty_http_endpoints::client::GET_LAST_LOGS;
use treaty_types::types::treaty_proto::{
    GetLogsByLastNumberReply, GetLogsByLastNumberRequest, TreatyLogEntry,
};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, use_state_eq, AttrValue, Callback, Html};

use crate::{
    log::log_to_console,
    request::{self, set_status, get_client},
};

#[function_component]
pub fn Logs() -> Html {
    let ui_last_num = use_node_ref();
    let logs = use_state_eq(move || {
        let x: Vec<TreatyLogEntry> = Vec::new();
        x
    });

    let onclick = {
        let logs = logs.clone();
        let ui_last_num = ui_last_num.clone();
        Callback::from(move |_| {
            let logs = logs.clone();
            let num_logs: u32 = ui_last_num
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .trim()
                .parse()
                .unwrap();
            let client = get_client();
            let request = GetLogsByLastNumberRequest {
                number_of_logs: num_logs,
            };

            let url = format!("{}{}", client.user_addr_port(), GET_LAST_LOGS);
            let body = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if let Ok(ref x) = response {
                    log_to_console(x.to_string());

                    let reply: GetLogsByLastNumberReply = serde_json::from_str(x).unwrap();

                    logs.set(reply.logs);
                } else {
                    set_status(response.err().unwrap());
                }
            });

            request::post(url, body, cb);
        })
    };

    html!(
        <div>
            <div class="container">
                <div class="box">
                    <h1 class="subtitle">{"View Logs"}</h1>
                    <label for="last_num_logs">{ "Last Number Of Logs" }</label>
                    <input type="text" class="input"  id="last_num_logs" placeholder="50" ref={&ui_last_num} />
                    <button type="button" class="button is-primary" id="view_logs" value="View" onclick={onclick}>
                        <span class="mdi mdi-magnify">{" View Logs"}</span>
                    </button>

                    <div class="table-container">
                    <table class="table is-narrow">
                        <thead>
                            <tr>
                                <th>{"DT"}</th>
                                <th>{"DT UTC"}</th>
                                <th>{"Level"}</th>
                                <th>{"Message"}</th>
                            </tr>
                        </thead>
                        {
                            (*logs).clone().into_iter().map(|l|
                                {
                                    let dt = l.dt.clone();
                                    let utc = l.dt_utc.clone();
                                    let level = l.level.clone();
                                    let msg = l.message.clone();

                                    html!{
                                        <tr>
                                            <td>{dt}</td>
                                            <td>{utc}</td>
                                            <td>{level}</td>
                                            <td>{msg}</td>
                                        </tr>
                                    }
                                }).collect::<Html>()
                        }
                    </table>
                    </div>
                </div>
            </div>
        </div>
    )
}
