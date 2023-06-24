use gloo::{
    console::debug,
    storage::{SessionStorage, Storage},
};
use serde::{de, Deserialize, Serialize};
use treaty_client_wasm::token::Token;
use treaty_types::proxy::{
    request_type::RequestType,
    server_messages::{
        http::{EXECUTE, REGISTER_URL, REVOKE_TOKEN_URL, TOKEN_URL},
        AuthForTokenReply, AuthForTokenRequest, ExecuteReply, ExecuteRequest, RegisterLoginReply,
        RegisterLoginRequest,
    },
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;
use web_sys::{Request, RequestInit, RequestMode};

use crate::log::log_to_console;

const TREATYPROXY: &str = "treatymyinfo.key.proxy";
const KEY: &str = "treatymyinfo.key.treatyproxy.instance";
const INFO_UN: &str = "treatymyinfo.current.un";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TreatyProxy {
    addr: String,
}

impl TreatyProxy {
    pub fn new(addr: &str) -> Self {
        
        if !addr.contains("http") {
            let addr = format!("{}{}", "http://", addr);
            return Self { addr }
        }

        Self { addr: addr.to_string() }
    }

    pub async fn register_account(
        &mut self,
        un: &str,
        pw: &str,
    ) -> Result<RegisterLoginReply, String> {
        let request = RegisterLoginRequest {
            login: un.to_string(),
            pw: pw.to_string(),
        };

        let url = self.get_http_url(REGISTER_URL);
        let result: Result<RegisterLoginReply, String> =
            self.get_http_result_error(url, request).await;

        match result {
            Ok(registration) => Ok(registration),
            Err(e) => Err(e),
        }
    }

    pub async fn logout(&mut self, un: &str) {
        let request = AuthForTokenRequest {
            login: un.to_string(),
            pw: "".to_string(),
        };

        let url = self.get_http_url(REVOKE_TOKEN_URL);
        let result: Result<AuthForTokenReply, String> =
            self.get_http_result_error(url, request).await;
        let debug = format!("{result:?}");
        log_to_console(&debug);
    }

    pub async fn execute_request(
        &mut self,
        request_json: &str,
        request_type: RequestType,
    ) -> Result<String, String> {
        let token = get_proxy_token();

        if token.id.is_some() {
            let request = ExecuteRequest {
                login: None,
                pw: None,
                jwt: Some(token.jwt.clone()),
                request_type: request_type.into(),
                request_json: request_json.to_string(),
            };

            let url = self.get_http_url(EXECUTE);

            let result: Result<ExecuteReply, String> =
                self.get_http_result_error(url, request).await;

            match result {
                Ok(result) => {
                    if result.execute_success {
                        Ok(result.reply.unwrap())
                    } else {
                        let message = format!("could not execute: {result:?}");
                        Err(message)
                    }
                }
                Err(e) => Err(e),
            }
        } else {
            Err("Host Id not in token".to_string())
        }
    }

    pub async fn execute_request_as<T: de::DeserializeOwned + std::clone::Clone>(
        &mut self,
        request_json: &str,
        request_type: RequestType,
    ) -> Result<T, String> {
        let token = get_proxy_token();

        if token.id.is_some() {
            let request = ExecuteRequest {
                login: None,
                pw: None,
                jwt: Some(token.jwt.clone()),
                request_type: request_type.into(),
                request_json: request_json.to_string(),
            };

            let url = self.get_http_url(EXECUTE);

            let result: Result<ExecuteReply, String> =
                self.get_http_result_error(url, request).await;

            match result {
                Ok(result) => {
                    if result.execute_success {
                        Ok(serde_json::from_str::<T>(&result.reply.unwrap()).unwrap())
                    } else {
                        let message = format!("could not execute: {result:?}");
                        Err(message)
                    }
                }
                Err(e) => Err(e),
            }
        } else {
            Err("Host Id not in token".to_string())
        }
    }

    pub async fn auth_for_token(&mut self, un: &str, pw: &str) -> Result<Token, String> {
        let request = AuthForTokenRequest {
            login: un.to_string(),
            pw: pw.to_string(),
        };

        let url = self.get_http_url(TOKEN_URL);
        let result: Result<AuthForTokenReply, String> =
            self.get_http_result_error(url, request).await;
        let debug = format!("{result:?}");
        log_to_console(&debug);
        match result {
            Ok(r) => Ok(Token {
                jwt: r.jwt.unwrap(),
                jwt_exp: r.expiration_utc.unwrap(),
                addr: self.addr.clone(),
                is_logged_in: true,
                id: r.id,
            }),
            Err(e) => Err(e),
        }
    }

    async fn get_http_result_error<
        'a,
        'b,
        T: de::DeserializeOwned + std::clone::Clone,
        U: de::DeserializeOwned + serde::Serialize + std::clone::Clone,
    >(
        &mut self,
        url: String,
        request: U,
    ) -> Result<T, String> {
        let body = serde_json::to_string(&request).unwrap();
        let result = post_result(&url, &body).await;

        match result {
            Ok(r) => {
                let value: T = serde_json::from_str(&r).unwrap();
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    fn get_http_url(&self, action_url: &str) -> String {
        let address = &self.addr;
        let url = format!("{address}/{action_url}");
        debug!(url.clone());
        url
    }
}

pub fn set_proxy(client: &TreatyProxy) {
    let client_json = serde_json::to_string(&client).unwrap();
    SessionStorage::set(TREATYPROXY, client_json).expect("failed to set");
}

pub fn set_un(un: &str) {
    SessionStorage::set(INFO_UN, un).expect("failed to set");
}

pub fn get_un() -> String {
    SessionStorage::get(INFO_UN).unwrap_or_else(|_| String::from(""))
}

pub fn get_proxy() -> TreatyProxy {
    let client = SessionStorage::get(TREATYPROXY).unwrap_or_else(|_| String::from(""));
    if client.is_empty() {
        TreatyProxy::new("localhost:0")
    } else {
        let client: TreatyProxy = serde_json::from_str(&client).unwrap();
        client
    }
}

pub fn clear_proxy_token() {
    SessionStorage::set(KEY, "").expect("failed to set");
}

/// Saves the JWT to Session Storage
pub fn set_proxy_token(token: Token) {
    let token = serde_json::to_string(&token).unwrap();
    SessionStorage::set(KEY, token).expect("failed to set");
}

pub fn has_proxy_token() -> bool {
    let token = SessionStorage::get(KEY).unwrap_or_else(|_| String::from(""));
    if token.is_empty() {
        return false;
    }
    true
}

/// Gets the JWT from Session Storage
pub fn get_proxy_token() -> Token {
    let token = SessionStorage::get(KEY).unwrap_or_else(|_| String::from(""));
    if token.is_empty() {
        Token::new()
    } else {
        let token: Token = serde_json::from_str(&token).unwrap();
        token
    }
}

pub async fn post_result(url: &str, body: &str) -> Result<String, String> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from_str(body)));

    let request = Request::new_with_str_and_init(url, &opts);

    match request {
        Ok(r) => {
            r.headers().set("Content-Type", "application/json").unwrap();

            let window = web_sys::window().unwrap();
            let resp_value_result = JsFuture::from(window.fetch_with_request(&r)).await;
            match resp_value_result {
                Ok(result) => {
                    assert!(result.is_instance_of::<Response>());
                    let resp: Response = result.dyn_into().unwrap();

                    let json = JsFuture::from(resp.text().unwrap()).await.unwrap();

                    Ok(JsValue::as_string(&json).unwrap())
                }
                Err(e) => {
                    // let m = format!("{:?}", e);
                    // log_to_console(m);

                    if JsValue::is_string(&e) {
                        Err(JsValue::as_string(&e).unwrap())
                    } else {
                        Err("Unable to connect".to_string())
                    }
                }
            }
        }
        Err(e) => {
            if JsValue::is_string(&e) {
                Err(JsValue::as_string(&e).unwrap())
            } else {
                Err("Unable to connect".to_string())
            }
        }
    }
}
