use crate::{
    log::log_to_console,
    request::{
        self,
        treaty::{get_treaty_token, set_status},
    },
};
use treaty_types::proxy::request_type::RequestType;
use treaty_types::types::treaty_proto::GetSettingsReply;
use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html};

#[function_component]
pub fn Settings() -> Html {
    let settings = use_state_eq(move || String::from(""));

    let onclick = {
        let settings = settings.clone();
        Callback::from(move |_| {
            let settings = settings.clone();
            let token = get_treaty_token();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if let Ok(ref x) = response {
                    log_to_console(x);

                    let reply: GetSettingsReply = serde_json::from_str(x).unwrap();

                    settings.set(reply.settings_json.unwrap());
                } else {
                    set_status(response.err().unwrap());
                }
            });

            request::post(RequestType::GetSettings, "", cb);
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
