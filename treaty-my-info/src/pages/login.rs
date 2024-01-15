use treaty_client_wasm::token::Token;
use treaty_types::{
    proxy::{request_type::RequestType, server_messages::AuthForTokenReply},
    types::treaty_proto::{AuthRequestBasic, GetDatabasesReply},
};
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};

use crate::{
    log::log_to_console,
    request::{
        proxy::{
            clear_proxy_token, get_proxy, get_proxy_token, set_proxy, set_proxy_token, set_un,
            TreatyProxy,
        },
        treaty::{clear_status, get_treaty_token, set_databases, set_status, set_treaty_token},
    },
};

#[function_component]
pub fn Login() -> Html {
    let ui_un = use_node_ref();
    let ui_pw = use_node_ref();
    let ui_addr_port = use_node_ref();
    let login_result = use_state_eq(move || String::from(""));

    let onclick = {
        clear_status();
        let ui_un = ui_un.clone();
        let ui_pw = ui_pw.clone();
        let ui_addr_port = ui_addr_port.clone();
        let login_result = login_result.clone();

        Callback::from(move |_| {
            let ui_un = ui_un.clone();
            let ui_pw = ui_pw.clone();
            let login_result = login_result.clone();
            let ui_addr_port = ui_addr_port.clone();

            let un = &ui_un;
            let pw = &ui_pw;

            let un_val = un.cast::<HtmlInputElement>().unwrap().value();
            let pw_val = pw.cast::<HtmlInputElement>().unwrap().value();
            let api_val = ui_addr_port.cast::<HtmlInputElement>().unwrap().value();

            let mut proxy = TreatyProxy::new(&api_val);
            proxy.set_basic(&un_val, &pw_val);
            set_proxy(&proxy);

            let u = un_val;
            let p = pw_val;

            spawn_local(async move {
                let result = proxy.auth_for_token(&u, &p).await;
                let message = format!("{result:?}");
                log_to_console(&message);
                match result {
                    Ok(token) => {
                        if token.is_logged_in {
                            set_un(&u);
                            set_proxy_token(token);
                            login_to_treaty_instance(&u, &p).await;
                            databases().await;
                            login_result
                                .set("Login success! You can now admin your instance.".to_string());
                        } else {
                            login_result.set("Login failed.".to_string());
                        }
                    }
                    Err(e) => log_to_console(&e),
                };
            })
        })
    };

    let onclick_logout = {
        let ui_un = ui_un.clone();
        let token = get_proxy_token();
        Callback::from(move |_| {
            let ui_un = ui_un.clone();
            let un = &ui_un;
            let un_val = un.cast::<HtmlInputElement>().unwrap().value();
            let mut proxy = TreatyProxy::new(&token.addr);
            set_proxy(&proxy);
            clear_proxy_token();
            spawn_local(async move {
                proxy.logout(&un_val).await;
            })
        })
    };

    html! {
        <div>
            <div class="container">
                <div class="box">
                    <div class="has-text-centered">
                        <h1 class="subtitle"> {"Login"} </h1>

                        <p>
                            <label for="ip_address">{ "API " }</label>
                            <input type="text" class="input" id ="addr_port" placeholder="localhost:50040" ref={&ui_addr_port}/>
                        </p>

                        <label for="ip_address">{ "User Name" }</label>
                        <input type="text" class="input" id ="username" placeholder="username" ref={&ui_un}/>

                        <label for="port">{ "Password" }</label>
                        <input type="text" class="input"  id="pw" placeholder="pw" ref={&ui_pw} />

                        <div class="buttons">
                        <button type="button" class="button is-primary" id="login" value="Login" {onclick}>
                            <span class="mdi mdi-login">{" Login"}</span>
                        </button>
                        <button type="button" class="button is-info" id="logout" value="Logout" onclick={onclick_logout}>
                        <span class="mdi mdi-logout">{" Logout"}</span>
                    </button>
                        </div>
                        <h2 class="subtitle"> {"Login Result"} </h2>
                        <p>{(*login_result).clone()}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

async fn login_to_treaty_instance(un: &str, pw: &str) {
    let mut proxy = get_proxy();
    let proxy_token = get_proxy_token();
    let token = get_treaty_token();

    let request = AuthRequestBasic {
        user_name: un.to_string(),
        pw: pw.to_string(),
    };

    let request_json = serde_json::to_string(&request).unwrap();
    let request_type = RequestType::Auth;

    let r = proxy
        .execute_request_as::<AuthForTokenReply>(&request_json, request_type)
        .await;

    if let Ok(r) = r {
        if r.is_successful {
            let token = Token {
                jwt: r.jwt.unwrap(),
                jwt_exp: r.expiration_utc.unwrap(),
                addr: proxy_token.addr.clone(),
                is_logged_in: true,
                id: r.id,
            };

            set_treaty_token(token);
        }
    };
}

pub async fn databases() {
    let mut proxy = get_proxy();

    let request_type = RequestType::GetDatabases;

    let r = proxy
        .execute_request_as::<GetDatabasesReply>("", request_type)
        .await;

    match r {
        Ok(r) => {
            let databases = r.databases;
            set_databases(databases);
        }
        Err(e) => set_status(e),
    };
}
