use treaty_types::types::treaty_proto::{GenerateHostInfoReply, GenerateHostInfoRequest};
use treaty_types::proxy::request_type::RequestType;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, use_state_eq, AttrValue, Callback, Html};

use crate::request;
use crate::{
    log::log_to_console,
    request::treaty::{clear_status, get_treaty_token, set_status, update_token_login_status},
};

#[function_component]
pub fn GenerateInfo() -> Html {
    let last_gen_result = use_state_eq(move || String::from(""));

    let ui_host_name = use_node_ref();

    let generate_onclick = {
        let ui_host_name = ui_host_name.clone();
        let last_gen_result = last_gen_result.clone();

        Callback::from(move |_| {
            let last_gen_result = last_gen_result.clone();
            let host_name = ui_host_name.cast::<HtmlInputElement>().unwrap().value();

            let token = get_treaty_token();

            let request = GenerateHostInfoRequest {
                authentication: Some(token.auth()),
                host_name,
            };

            let json_request = serde_json::to_string(&request).unwrap();

            let cb = {
                let last_gen_result = last_gen_result;
                Callback::from(move |response: Result<AttrValue, String>| {
                    if let Ok(ref x) = response {
                        log_to_console(x);
                        clear_status();

                        let reply: GenerateHostInfoReply = serde_json::from_str(x).unwrap();

                        let is_authenticated = reply
                            .authentication_result
                            .as_ref()
                            .unwrap()
                            .is_authenticated;
                        update_token_login_status(is_authenticated);

                        if is_authenticated {
                            let message =
                                format!("{}{}", "Last gen result was: ", reply.is_successful);
                            last_gen_result.set(message);
                        }
                    } else {
                        set_status(response.err().unwrap());
                    }
                })
            };

            request::post(RequestType::GenerateHostInfo, &json_request, cb);
        })
    };

    html!(
        <div>
          <p><h1 class="subtitle"> {"Generate Host Info"} </h1></p>

                    <p> <label for="host_name">{ "Enter Host Name" }</label>
                    <input type="text" class="input"  id="host_name" placeholder="Enter Host Name" ref={&ui_host_name} /></p>

                    <p>
                    <button class="button is-primary" onclick={generate_onclick}>
                        <span class="mdi mdi-autorenew">{" Generate Host Info"}</span>
                    </button>
                    </p>

                    <p><h3 class="subtitle"> {"Last Generate Result"} </h3></p>
                    <p>{(*last_gen_result).clone()}</p>
        </div>
    )
}
