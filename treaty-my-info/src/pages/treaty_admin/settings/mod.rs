use crate::{
    log::log_to_console,
    request::{
        self,
        treaty::{get_treaty_token, set_status, update_token_login_status},
    },
};
use treaty_types::types::treaty_proto::{GetSettingsReply, GetSettingsRequest};
use treaty_types::proxy::request_type::RequestType;
use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html};

#[function_component]
pub fn Settings() -> Html {
    let settings = use_state_eq(move || String::from(""));

    let onclick = {
        let settings = settings.clone();
        Callback::from(move |_| {
            let settings = settings.clone();
            let token = get_treaty_token();
            let request = GetSettingsRequest {
                authentication: Some(token.auth()),
            };

            let body = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if let Ok(ref x) = response {
                    log_to_console(x);

                    let reply: GetSettingsReply = serde_json::from_str(x).unwrap();

                    let is_authenticated = reply.authentication_result.unwrap().is_authenticated;
                    update_token_login_status(is_authenticated);

                    if is_authenticated {
                        settings.set(reply.settings_json.unwrap());
                    }
                } else {
                    set_status(response.err().unwrap());
                }
            });

            request::post(RequestType::GetSettings, &body, cb);
        })
    };

    html!(
        <div>
            <div class="container">
                <div class="box">
                    <h1 class="subtitle">{"Settings"}</h1>
                    <button type="button" class="button is-primary" id="view_settings" value="View" onclick={onclick}>
                        <span class="mdi mdi-magnify">{" View Settings"}</span>
                    </button>
                    <p>
                    <textarea class="textarea" rows="5" cols="60"
                    id ="settings" placeholder="Settings Will Appear Here As Json" value={(*settings).clone()}/>
                    </p>
                </div>
            </div>
        </div>
    )
}
