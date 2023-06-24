use treaty_client_wasm::{client::TreatyClient, token::Token};
use treaty_http_endpoints::client::{GET_DATABASES, REVOKE_TOKEN};
use treaty_types::types::treaty_proto::{
    DatabaseSchema, GetDatabasesReply, GetDatabasesRequest, RevokeReply,
};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    log::log_to_console,
    request::{
        self, clear_status, get_token, set_client, set_databases, set_participants, set_status,
        set_token, update_token_login_status,
    },
};

#[function_component]
pub fn Home() -> Html {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <div class="has-text-centered">
                        <p><h1 class="title is-1">{ "Treaty Admin" }</h1></p>
                        <p><h3 class="subtitle">{"version "}{VERSION}</h3></p>
                    </div>
                </div>
            </div>

            <div class="tile is-parent container">
                <Connect />
            </div>
        </div>
    }
}

#[function_component]
pub fn Connect() -> Html {
    /*
        https://docs.rs/yew/latest/yew/functional/fn.use_reducer.html
        https://docs.rs/yew/latest/yew/functional/fn.use_node_ref.html
    */

    let ui_addr = use_node_ref();
    let ui_port = use_node_ref();
    let ui_un = use_node_ref();
    let ui_pw = use_node_ref();

    let database_names = use_state(|| {
        let databases: Vec<String> = Vec::new();
        databases
    });

    let onclick_logout = {
        Callback::from(move |_| {
            let token = get_token();
            let request = serde_json::to_string(&token.auth()).unwrap();
            let url = format!("{}{}", token.addr, REVOKE_TOKEN);

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if let Ok(ref x) = response {
                    log_to_console(x.to_string());
                    clear_status();
                    let reply: RevokeReply = serde_json::from_str(x).unwrap();
                    if reply.is_successful {
                        let token = Token::new();
                        set_token(token);
                        set_databases(Vec::new());
                        set_participants(Vec::new());
                    }
                } else {
                    set_status(response.err().unwrap());
                }
            });

            request::post(url, request, cb)
        })
    };

    let onclick = {
        let ui_addr = ui_addr.clone();
        let ui_port = ui_port.clone();
        let ui_un = ui_un.clone();
        let ui_pw = ui_pw.clone();

        let database_names = database_names.clone();

        Callback::from(move |_| {
            let database_names = database_names.clone();

            let ui_addr = ui_addr.clone();
            let ui_port = ui_port.clone();
            let ui_un = ui_un.clone();
            let ui_pw = ui_pw.clone();

            let un = &ui_un;
            let pw = &ui_pw;
            let ip = &ui_addr;
            let port = &ui_port;

            let un_val = un.cast::<HtmlInputElement>().unwrap().value();
            let pw_val = pw.cast::<HtmlInputElement>().unwrap().value();
            let ip_val = ip.cast::<HtmlInputElement>().unwrap().value();
            let port_val = port.cast::<HtmlInputElement>().unwrap().value();

            let mut client = TreatyClient::new(&ip_val, port_val.parse::<u32>().unwrap());
            set_client(&client);

            let u = un_val;
            let p = pw_val;

            spawn_local(async move {
                let result = client.auth_for_token(&u, &p).await;
                // let result = client.auth_for_token_x(&u, &p).await;
                match result {
                    Ok(token) => {
                        save_token(token, database_names);
                    }
                    Err(e) => log_to_console(e),
                };
            })
        })
    };

    html! {
        <div>
            <div class="container">
                <div class="box">
                    <h2 class="subtitle">{"Connect To Treaty"}</h2>

                    <label for="ip_address">{ "IP Address" }</label>
                    <input type="text" class="input" id ="ip_address" placeholder="localhost" ref={&ui_addr}/>

                    <label for="port">{ "Port Number" }</label>
                    <input type="text" class="input"  id="port" placeholder="50055" ref={&ui_port} />

                    <label for="un">{ "User Name" }</label>
                    <input type="text" class="input"  id="un" placeholder="tester" ref={&ui_un} />

                    <label for="pw">{ "Pw" }</label>
                    <input type="text" class="input"  id="pw" placeholder="123456" ref={&ui_pw} />

                    <div class="buttons">
                        <button type="button" class="button is-primary" id="submit" value="Connect" {onclick}>
                            <span class="mdi mdi-lan-connect">{" Connect"}</span>
                        </button>
                        <button type="button" class="button is-danger" id="disconnect" value="Disconnect" onclick={onclick_logout}>
                            <span class="mdi mdi-lan-disconnect">{" Disconnect"}</span>
                        </button>
                    </div>
                </div>
            </div>

            <div class="container">
                <div class="box">
                    <h1 class="subtitle"> {"Databases"} </h1>

                    <p>{"After connecting, the list of databases on the treaty instance will appear here."}</p>
                    <p>{"Note that connecting gives a token authorization for 20 minutes by default. If there are errors
                    or data is outdated, you may need to re-auth against the instance."}</p>
                    <div class="content">
                        <ul>
                            {
                                (*database_names).clone().into_iter().map(|name| {
                                    let db_name = name.clone();
                                    html!{<div key={db_name.clone()}>
                                    <li>{db_name.clone()}</li></div>
                            }
                                }).collect::<Html>()
                            }
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Takes the http address of an Treaty instance and a AuthRequest seralized to JSON
/// and attempts to get a JWT from the Treaty instance. If successfully authenticated,
/// it will save the JWT to Session Storage
fn save_token(token: Token, database_names: UseStateHandle<Vec<String>>) {
    set_token(token);
    databases(database_names);
}

pub fn databases(database_names: UseStateHandle<Vec<String>>) {
    let token = get_token();
    let auth_request = token.auth();

    let db_request = GetDatabasesRequest {
        authentication: Some(auth_request),
    };

    let db_request_json = serde_json::to_string(&db_request).unwrap();

    let db_callback = Callback::from(move |response: Result<AttrValue, String>| {
        if let Ok(ref x) = response {
            log_to_console(x.to_string());
            clear_status();

            let database_names = database_names.clone();

            let db_response: GetDatabasesReply = serde_json::from_str(x).unwrap();

            let is_authenticated = db_response
                .authentication_result
                .as_ref()
                .unwrap()
                .is_authenticated;
            update_token_login_status(is_authenticated);

            if is_authenticated {
                let databases = db_response.databases;
                set_databases(databases.clone());

                let mut db_names: Vec<String> = Vec::new();

                for db in &databases {
                    db_names.push(db.database_name.clone());
                }
                database_names.set(db_names);
            }
        } else {
            set_status(response.err().unwrap());
        }
    });

    let url = format!("{}{}", token.addr, GET_DATABASES);
    request::post(url, db_request_json, db_callback);
}

#[derive(Properties, PartialEq, Clone)]
pub struct DatabaseDetails {
    pub dbs: Vec<DatabaseSchema>,
}

impl<'a> Default for DatabaseDetails {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseDetails {
    pub fn new() -> DatabaseDetails {
        DatabaseDetails { dbs: Vec::new() }
    }
}
