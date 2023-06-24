use crate::{
    log::log_to_console,
    request::{
        self,
        treaty::{get_treaty_token, set_status, update_token_login_status},
    },
};
use treaty_types::types::treaty_proto::{
    GetLogsByLastNumberReply, GetLogsByLastNumberRequest, TreatyLogEntry,
};
use treaty_types::proxy::request_type::RequestType;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, use_state_eq, AttrValue, Callback, Html};

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
            let token = get_treaty_token();
            let request = GetLogsByLastNumberRequest {
                authentication: Some(token.auth()),
                number_of_logs: num_logs,
            };

            let body = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if let Ok(ref x) = response {
                    log_to_console(x);

                    let reply: GetLogsByLastNumberReply = serde_json::from_str(x).unwrap();

                    let is_authenticated = reply.authentication_result.unwrap().is_authenticated;
                    update_token_login_status(is_authenticated);

                    if is_authenticated {
                        logs.set(reply.logs);
                    }
                } else {
                    set_status(response.err().unwrap());
                }
            });

            request::post(RequestType::GetLogsByLastNumber, &body, cb);
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
