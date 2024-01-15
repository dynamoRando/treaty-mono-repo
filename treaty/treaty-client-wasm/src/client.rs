use serde::{de, Deserialize, Serialize};
use treaty_http_endpoints::client::{
    COOPERATIVE_WRITE_SQL_AT_HOST, GET_PARTICIPANTS, READ_SQL_AT_HOST, WRITE_SQL_AT_HOST,
    WRITE_SQL_AT_PARTICIPANT,
};
use treaty_http_endpoints::headers::{
    TREATY_AUTH_HEADER_AUTHOR, TREATY_AUTH_HEADER_BASIC, TREATY_AUTH_HEADER_METADATA,
    TREATY_AUTH_HEADER_WEB_TOKEN,
};
use treaty_http_endpoints::info::{INFO_AUTH_FOR_TOKEN, INFO_TRY_AUTH};
use treaty_types::enums::DatabaseType;

use treaty_types::types::treaty_proto::{
    AuthRequestAuthor, AuthRequestBasic, AuthRequestMetadata, AuthRequestWebToken,
    ExecuteCooperativeWriteReply, ExecuteCooperativeWriteRequest, ExecuteReadReply,
    ExecuteReadRequest, ExecuteWriteReply, ExecuteWriteRequest, GetParticipantsReply,
    GetParticipantsRequest, StatementResultset, TokenReply, TryAuthResult,
};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Serialize, Deserialize, Clone)]
pub struct TreatyClient {
    user_addr: String,
    user_port: u32,
    info_addr: String,
    info_port: u32,
    basic_auth: AuthRequestBasic,
    token_auth: Option<AuthRequestWebToken>,
    host_id: Option<String>,
}

impl TreatyClient {
    pub fn new(
        user_addr: &str,
        user_port: u32,
        info_addr: &str,
        info_port: u32,
        un: &str,
        pw: &str,
        host_id: Option<String>,
    ) -> Self {
        let basic = AuthRequestBasic {
            user_name: un.to_string(),
            pw: pw.to_string(),
        };

        Self {
            user_addr: user_addr.to_string(),
            user_port,
            info_addr: info_addr.to_string(),
            info_port,
            basic_auth: basic,
            token_auth: None,
            host_id,
        }
    }

    pub fn is_logged_in(&self) -> bool {
        self.jwt().is_some()
    }

    pub fn user_addr(&self) -> String {
        self.user_addr.clone()
    }

    pub fn info_addr(&self) -> String {
        self.info_addr.clone()
    }

    pub fn user_port(&self) -> u32 {
        self.user_port
    }

    pub fn info_port(&self) -> u32 {
        self.info_port
    }

    pub fn user_addr_port(&self) -> String {
        format!("http://{}:{}", self.user_addr, self.user_port)
    }

    pub fn info_addr_port(&self) -> String {
        format!("http://{}:{}", self.info_addr, self.info_port)
    }

    pub fn jwt(&self) -> Option<AuthRequestWebToken> {
        self.token_auth.clone()
    }

    pub async fn execute_cooperative_write_at_host<'a>(
        &mut self,
        db_name: &str,
        cmd: &str,
        participant_alias: &str,
        where_clause: &str,
    ) -> Result<bool, String> {
        let request = ExecuteCooperativeWriteRequest {
            database_name: db_name.to_string().into(),
            sql_statement: cmd.to_string().into(),
            database_type: DatabaseType::to_u32(DatabaseType::Sqlite),
            alias: participant_alias.to_string().into(),
            participant_id: String::from("").into(),
            where_clause: where_clause.to_string().into(),
        };

        let url = self.get_http_url(COOPERATIVE_WRITE_SQL_AT_HOST);

        let result: Result<ExecuteCooperativeWriteReply, String> =
            self.get_http_result_error(url, request).await;

        match result {
            Ok(r) => Ok(r.is_successful),
            Err(e) => Err(e),
        }
    }

    pub async fn execute_write_at_participant<'a>(
        &mut self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
        where_clause: &str,
    ) -> Result<bool, String> {
        let request = ExecuteWriteRequest {
            database_name: db_name.to_string().into(),
            sql_statement: sql_statement.to_string().into(),
            database_type: db_type,
            where_clause: where_clause.to_string().into(),
        };

        let url = self.get_http_url(WRITE_SQL_AT_PARTICIPANT);

        let result: Result<ExecuteWriteReply, String> =
            self.get_http_result_error(url, request).await;

        match result {
            Ok(r) => Ok(r.is_successful),
            Err(e) => Err(e),
        }
    }

    pub async fn execute_write_at_host<'a>(
        &'a mut self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
        where_clause: &str,
    ) -> Result<bool, String> {
        let request = ExecuteWriteRequest {
            database_name: db_name.to_string().into(),
            sql_statement: sql_statement.to_string().into(),
            database_type: db_type,
            where_clause: where_clause.to_string().into(),
        };

        let url = self.get_http_url(WRITE_SQL_AT_HOST);

        let result: Result<ExecuteWriteReply, String> =
            self.get_http_result_error(url, request).await;

        match result {
            Ok(r) => Ok(r.is_successful),
            Err(e) => Err(e),
        }
    }

    pub async fn execute_read_at_host<'a>(
        &mut self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
    ) -> Result<StatementResultset, String> {
        let request = ExecuteReadRequest {
            database_name: db_name.to_string().into(),
            sql_statement: sql_statement.to_string().into(),
            database_type: db_type,
        };

        let url = self.get_http_url(READ_SQL_AT_HOST);
        let result: Result<ExecuteReadReply, String> =
            self.get_http_result_error(url, request).await;

        match result {
            Ok(r) => Ok(r.results[0].clone()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_participants(
        &mut self,
        database_name: &str,
    ) -> Result<GetParticipantsReply, String> {
        let request = GetParticipantsRequest {
            database_name: database_name.to_string().into(),
        };

        let url = self.get_http_url(GET_PARTICIPANTS);
        let result: Result<GetParticipantsReply, String> =
            self.get_http_result_error(url, request).await;

        match result {
            Ok(r) => Ok(r),
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
        let result = self.send_http_message(&url, &body).await;

        match result {
            Ok(r) => {
                let value: T = serde_json::from_str(&r).unwrap();
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    fn get_http_url(&self, action_url: &str) -> String {
        let address = format!("{}{}{}{}", "http://", self.user_addr, ":", self.user_port);
        let url = format!("{address}{action_url}");
        url
    }

    async fn validate_web_token(&mut self) -> bool {
        if let Some(web_token) = self.token_auth.clone() {
            let json_message = serde_json::to_string(&web_token).unwrap();
            let author = AuthRequestAuthor { author_type: 1 };

            let metadata = AuthRequestMetadata {
                id: self.host_id.clone(),
                db_name: None,
            };

            let http_base = format!("{}{}:{}", "http://", self.info_addr, self.info_port);
            let action_url = INFO_TRY_AUTH;
            let url: String = format!("{http_base}{action_url}");

            let mut opts = RequestInit::new();
            opts.method("POST");
            opts.mode(RequestMode::Cors);
            opts.body(Some(&JsValue::from_str(&json_message)));

            let web_token = self.token_auth.as_ref().unwrap();

            let request = Request::new_with_str_and_init(&url, &opts);
            match request {
                Ok(r) => {
                    r.headers().set("Content-Type", "application/json").unwrap();
                    r.headers()
                        .set(
                            TREATY_AUTH_HEADER_WEB_TOKEN,
                            &serde_json::to_string(web_token).unwrap(),
                        )
                        .unwrap();
                    r.headers()
                        .set(
                            TREATY_AUTH_HEADER_AUTHOR,
                            &serde_json::to_string(&author).unwrap(),
                        )
                        .unwrap();
                    r.headers()
                        .set(
                            TREATY_AUTH_HEADER_METADATA,
                            &serde_json::to_string(&metadata).unwrap(),
                        )
                        .unwrap();

                    let window = web_sys::window().unwrap();
                    let resp_value_result = JsFuture::from(window.fetch_with_request(&r)).await;
                    match resp_value_result {
                        Ok(result) => {
                            assert!(result.is_instance_of::<Response>());
                            let resp: Response = result.dyn_into().unwrap();

                            let json = JsFuture::from(resp.text().unwrap()).await.unwrap();

                            let auth_result_string = JsValue::as_string(&json).unwrap();
                            let auth_result: TryAuthResult =
                                serde_json::from_str(&auth_result_string).unwrap();
                            return auth_result.is_authenticated;
                        }
                        Err(_) => {
                            return false;
                        }
                    }
                }
                Err(_) => return false,
            }
        }

        false
    }

    async fn update_token(&mut self) {
        if !self.validate_web_token().await {
            let metadata = AuthRequestMetadata {
                id: self.host_id.clone(),
                db_name: None,
            };
            let author = AuthRequestAuthor { author_type: 1 };
            let json_message = serde_json::to_string(&self.basic_auth).unwrap();
            let http_base = format!("{}{}:{}", "http://", self.info_addr, self.info_port);
            let action_url = INFO_AUTH_FOR_TOKEN;
            let url = format!("{http_base}{action_url}");

            let mut opts = RequestInit::new();
            opts.method("POST");
            opts.mode(RequestMode::Cors);
            opts.body(Some(&JsValue::from_str(&json_message)));

            let basic_token = &self.basic_auth;

            let request = Request::new_with_str_and_init(&url, &opts);
            match request {
                Ok(r) => {
                    r.headers().set("Content-Type", "application/json").unwrap();
                    r.headers()
                        .set(
                            TREATY_AUTH_HEADER_BASIC,
                            &serde_json::to_string(&basic_token).unwrap(),
                        )
                        .unwrap();
                    r.headers()
                        .set(
                            TREATY_AUTH_HEADER_AUTHOR,
                            &serde_json::to_string(&author).unwrap(),
                        )
                        .unwrap();
                    r.headers()
                        .set(
                            TREATY_AUTH_HEADER_METADATA,
                            &serde_json::to_string(&metadata).unwrap(),
                        )
                        .unwrap();

                    let window = web_sys::window().unwrap();
                    let resp_value_result = JsFuture::from(window.fetch_with_request(&r)).await;
                    match resp_value_result {
                        Ok(result) => {
                            assert!(result.is_instance_of::<Response>());
                            let resp: Response = result.dyn_into().unwrap();

                            let json = JsFuture::from(resp.text().unwrap()).await.unwrap();

                            let token_reply_string = JsValue::as_string(&json).unwrap();
                            let token_reply: TokenReply =
                                serde_json::from_str(&token_reply_string).unwrap();
                            if token_reply.is_successful {
                                self.token_auth = Some(AuthRequestWebToken {
                                    jwt: token_reply.jwt,
                                });
                            }
                        }
                        Err(_) => {
                            self.token_auth = None;
                        }
                    }
                }
                Err(_) => self.token_auth = None,
            }
        }
    }

    pub async fn send_http_message(
        &mut self,
        json_message: &str,
        url: &str,
    ) -> Result<String, String> {
        self.update_token().await;
        let is_web_token_valid = self.validate_web_token().await;
        let author = AuthRequestAuthor { author_type: 1 };
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);
        opts.body(Some(&JsValue::from_str(&json_message)));

        let metadata = AuthRequestMetadata {
            id: self.host_id.clone(),
            db_name: None,
        };
        if is_web_token_valid {
            let web_token = self.token_auth.as_ref().unwrap();
            let request = Request::new_with_str_and_init(&url, &opts);
            match request {
                Ok(r) => {
                    r.headers().set("Content-Type", "application/json").unwrap();
                    r.headers()
                        .set(
                            TREATY_AUTH_HEADER_WEB_TOKEN,
                            &serde_json::to_string(web_token).unwrap(),
                        )
                        .unwrap();
                    r.headers()
                        .set(
                            TREATY_AUTH_HEADER_AUTHOR,
                            &serde_json::to_string(&author).unwrap(),
                        )
                        .unwrap();
                    r.headers()
                        .set(
                            TREATY_AUTH_HEADER_METADATA,
                            &serde_json::to_string(&metadata).unwrap(),
                        )
                        .unwrap();

                    let window = web_sys::window().unwrap();
                    let resp_value_result = JsFuture::from(window.fetch_with_request(&r)).await;
                    match resp_value_result {
                        Ok(result) => {
                            assert!(result.is_instance_of::<Response>());
                            let resp: Response = result.dyn_into().unwrap();

                            let json = JsFuture::from(resp.text().unwrap()).await.unwrap();

                            let result_string = JsValue::as_string(&json).unwrap();
                            return Ok(result_string);
                        }
                        Err(_) => {
                            return Err("Error sending message".to_string());
                        }
                    }
                }
                Err(_) => return Err("Error sending message".to_string()),
            }
        } else {
            let request = Request::new_with_str_and_init(&url, &opts);
            match request {
                Ok(r) => {
                    r.headers().set("Content-Type", "application/json").unwrap();
                    r.headers()
                        .set(
                            TREATY_AUTH_HEADER_BASIC,
                            &serde_json::to_string(&self.basic_auth).unwrap(),
                        )
                        .unwrap();
                    r.headers()
                        .set(
                            TREATY_AUTH_HEADER_AUTHOR,
                            &serde_json::to_string(&author).unwrap(),
                        )
                        .unwrap();
                    r.headers()
                        .set(
                            TREATY_AUTH_HEADER_METADATA,
                            &serde_json::to_string(&metadata).unwrap(),
                        )
                        .unwrap();

                    let window = web_sys::window().unwrap();
                    let resp_value_result = JsFuture::from(window.fetch_with_request(&r)).await;
                    match resp_value_result {
                        Ok(result) => {
                            assert!(result.is_instance_of::<Response>());
                            let resp: Response = result.dyn_into().unwrap();

                            let json = JsFuture::from(resp.text().unwrap()).await.unwrap();

                            let result_string = JsValue::as_string(&json).unwrap();
                            return Ok(result_string);
                        }
                        Err(_) => {
                            return Err("Error sending message".to_string());
                        }
                    }
                }
                Err(_) => return Err("Error sending message".to_string()),
            }
        }
    }
}
