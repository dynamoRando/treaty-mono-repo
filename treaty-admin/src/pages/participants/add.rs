use js_sys::Date;
use treaty_http_endpoints::client::ADD_PARTICIPANT;
use treaty_types::types::treaty_proto::{AddParticipantReply, AddParticipantRequest};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, use_state_eq, AttrValue, Callback, Html};

use crate::{
    log::log_to_console,
    pages::participants::ActiveDbProps,
    request::{self, clear_status, set_status, get_client},
};

#[function_component]
pub fn AddParticipant(ActiveDbProps { active_db }: &ActiveDbProps) -> Html {
    let active_db = active_db.clone();

    let add_participant_result = use_state_eq(move || String::from(""));

    let ui_alias = use_node_ref();
    let ui_addr = use_node_ref();
    let ui_port = use_node_ref();
    let ui_http = use_node_ref();
    let ui_http_port = use_node_ref();

    let onclick = {
        let add_participant_result = add_participant_result.clone();

        let ui_alias = ui_alias.clone();
        let ui_addr = ui_addr.clone();
        let ui_port = ui_port.clone();
        let ui_http = ui_http.clone();
        let ui_http_port = ui_http_port.clone();

        Callback::from(move |_| {
            let add_participant_result = add_participant_result.clone();
            let active_db = active_db.clone();
            let db_name = (*active_db).clone();

            let alias = ui_alias.cast::<HtmlInputElement>().unwrap().value();
            let addr = ui_addr.cast::<HtmlInputElement>().unwrap().value();
            let port = ui_port.cast::<HtmlInputElement>().unwrap().value();
            let http = ui_http.cast::<HtmlInputElement>().unwrap().value();
            let http_port = ui_http_port.cast::<HtmlInputElement>().unwrap().value();

            let client = get_client();
            let url = format!("{}{}", client.user_addr_port(), ADD_PARTICIPANT);

            let request = AddParticipantRequest {
                database_name: db_name,
                alias: alias.clone(),
                ip4_address: addr,
                db_port: Some(port.parse().unwrap()),
                info_port: port.parse().unwrap(),
                http_addr: http,
                http_port: http_port.parse().unwrap(),
                id: None,
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let callback = Callback::from(move |response: Result<AttrValue, String>| {
                if let Ok(ref x) = response {
                    log_to_console(x.to_string());
                    clear_status();

                    let reply: AddParticipantReply = serde_json::from_str(x).unwrap();

                    let now = Date::new(&Date::new_0());
                    let now = Date::to_iso_string(&now);

                    let message = format!(
                        "Alias: {} Is Successful: {} At Time: {}",
                        alias, reply.is_successful, now
                    );
                    add_participant_result.set(message);
                } else {
                    set_status(response.err().unwrap());
                }
            });

            request::post(url, request_json, callback);
        })
    };

    html!(
        <div>
            <div class="container">
                <p><h1 class="subtitle">{"Add Particpant"}</h1></p>

                <p><label for="participant_alias">{ "Participant Alias" }</label>
                <input type="text" class="input" id ="participant_alias" placeholder="Alias" ref={&ui_alias}/></p>

                <p><label for="participant_ip_address">{ "Participant IP Address" }</label>
                <input type="text" class="input" id="participant_ip_address" placeholder="127.0.0.1" ref={&ui_addr} /></p>

                <p><label for="participant_db_port">{ "Participant Data Port Number" }</label>
                <input type="text" class="input" id="participant_db_port" placeholder="50052" ref={&ui_port} /></p>

                <p><label for="participant_http_addr">{ "Participant HTTP Addr" }</label>
                <input type="text" class="input" id="participant_http_addr" placeholder="localhost" ref={&ui_http} /></p>

                <p><label for="participant_http_port">{ "Participant HTTP Port Number" }</label>
                <input type="text" class="input" id="participant_http_port" placeholder="50055" ref={&ui_http_port} /></p>

                <button type="button" class="button is-primary" id="add_participant" {onclick}><span class="mdi mdi-account-plus-outline">{" Add Participant"}</span></button>

                <p><label for="last_add_result">{ "Last Result: "}</label>{(*add_participant_result).to_string()}</p>

            </div>
        </div>
    )
}
