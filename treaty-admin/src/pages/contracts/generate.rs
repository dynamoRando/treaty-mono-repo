use treaty_http_endpoints::client::GENERATE_CONTRACT;
use treaty_types::types::treaty_proto::{GenerateContractReply, GenerateContractRequest};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, use_state_eq, AttrValue, Callback, Html};

use crate::{
    log::log_to_console,
    pages::common::select_database::SelectDatabase,
    request::{self, clear_status, set_status, get_client},
};

#[function_component]
pub fn Generate() -> Html {
    let last_generate_result = use_state_eq(move || String::from(""));
    let active_db = use_state_eq(move || String::from(""));
    let onclick_db: Option<Callback<String>> = None;

    let ui_host_name = use_node_ref();
    let ui_desc = use_node_ref();
    let ui_delete_behavior = use_node_ref();

    let onclick_generate = {
        let ui_host_name = ui_host_name.clone();
        let ui_desc = ui_desc.clone();
        let ui_delete_behavior = ui_delete_behavior.clone();
        let last_generate_result = last_generate_result.clone();
        let active_db = active_db.clone();

        Callback::from(move |_| {
            let last_generate_result = last_generate_result.clone();
            let active_db = active_db.clone();
            let host_name = ui_host_name.cast::<HtmlInputElement>().unwrap().value();
            let desc = ui_desc.cast::<HtmlInputElement>().unwrap().value();
            let delete_behavior = ui_delete_behavior
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();

            let client = get_client();

            let request = GenerateContractRequest {
                database_name: (*active_db).clone().into(),
                description: desc.into(),
                host_name: host_name.into(),
                remote_delete_behavior: delete_behavior.parse().unwrap(),
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let url = format!("{}{}", client.user_addr_port(), GENERATE_CONTRACT);

            let cb = {
                Callback::from(move |response: Result<AttrValue, String>| {
                    if let Ok(ref x) = response {
                        log_to_console(x.to_string());
                        clear_status();

                        let reply: GenerateContractReply = serde_json::from_str(x).unwrap();

                        let is_successful = reply.is_successful;
                        last_generate_result.set(is_successful.to_string());
                    } else {
                        set_status(response.err().unwrap());
                    }
                })
            };

            request::post(url, request_json, cb);
        })
    };

    html! {
        <div>
            <h1 class="subtitle"> {"Generate Contract"} </h1>
            <p>{"Note: Before you can generate a contract, you must ensure that every user table in your target
            database has a Logical Storage Policy applied for it." }</p>
            <p>< SelectDatabase active_db_name={active_db} onclick_db={onclick_db} /></p>

            <label for="host_name">{ "Host Name" }</label>
            <input type="text" class="input" id ="host_name"
            placeholder="Name you wish to identify yourself to participants" ref={&ui_host_name}/>

            <label for="description">{ "Description" }</label>
            <textarea class="textarea" rows="5" cols="60" id ="sql_Result"
            placeholder="A description of the purpose of this database"
            ref={&ui_desc} />

            <label for="set_remote_delete_behavior">{ "Set Remote Delete Behavior" }</label>
            <p>
            <div class ="select is-multiple">
            <select name="set_remote_delete_behavior" id="set_remote_delete_behavior" ref={ui_delete_behavior} >
                <option value="0">{"SELECT BEHAVIOR"}</option>
                <option value="1">{"Ignore"}</option>
                <option value="2">{"AutoDelete"}</option>
                <option value="3">{"UpdateStatusOnly"}</option>
            </select>
            </div>
            </p>

            <div class="content">
                <p>{"Explanation: The Remote Delete Behavior determines how reference rows in the host database will be updated.
                The options are: "}
                <ul>
                    <li>{"Ignore: If the participant has deleted the row, the host will take no action."}</li>
                    <li>{"AutoDelete: If the participant has deleted the row, the host will also delete the reference on it's side."}</li>
                    <li>{"UpdateStatusOnly: If the participant has deleted the row, the host will mark the reference as deleted, but keep the reference to the row."}</li>
                </ul>
                    <p>{"Note that a reference row in the host database, while having it's target marked as deleted, can itself be deleted at any time."}</p>
                </p>
            </div>
            <p>
                <button class="button is-primary" type="button" id="generate_new_contract" value="Generate Contract" onclick={onclick_generate}>
                    <span class="mdi mdi-file-document-plus">{" Generate"}</span>
                </button>
            </p>
            <p><label for="last_generate_result">{ "Last Result: "}</label>{(*last_generate_result).to_string()}</p>

        </div>
    }
}
