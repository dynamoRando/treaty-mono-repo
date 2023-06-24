use gloo::{
    net::http::{Method, Request},
    storage::{SessionStorage, Storage},
};
use treaty_client_wasm::{client::TreatyClient, token::Token};
use treaty_types::types::treaty_proto::{DatabaseSchema, ParticipantStatus};
use yew::{platform::spawn_local, AttrValue, Callback};

use crate::log::log_to_console;

const KEY: &str = "treatyadmin.key.instance";
const DATABASES: &str = "treatyadmin.key.databases";
const PARTICIPANTS: &str = "treatyadmin.key.participants";
const STATUS: &str = "treatyadmin.key.status";
const CLIENT: &str = "treatyadmin.key.client";

/// sends an HTTP POST to the specified URL with the treaty-message as JSON, returning JSON if successful,
/// otherwise a string describing the error that occurred
pub fn post(url: String, body: String, callback: Callback<Result<AttrValue, String>>) {
    let message = format!("{}{}", "outgoing message: ", body);
    log_to_console(message);
    if !body.is_empty() {
        spawn_local(async move {
            let result = Request::new(&url)
                .method(Method::POST)
                .header("Content-Type", "application/json")
                .body(body)
                .send()
                .await;

            if result.is_ok() {
                let response = result.as_ref().unwrap().text().await;

                if let Ok(data) = response {
                    callback.emit(Ok(AttrValue::from(data)));
                } else {
                    let err = result.err().unwrap().to_string();
                    callback.emit(Err(err))
                }
            }
        });
    }
}

pub fn set_client(client: &TreatyClient) {
    let client_json = serde_json::to_string(&client).unwrap();
    SessionStorage::set(CLIENT, client_json).expect("failed to set");
}

pub fn get_client() -> TreatyClient {
    let client = SessionStorage::get(CLIENT).unwrap_or_else(|_| String::from(""));
    if client.is_empty() {
        TreatyClient::new("", 0)
    } else {
        let client: TreatyClient = serde_json::from_str(&client).unwrap();
        client
    }
}

/// Saves the JWT to Session Storage
pub fn set_token(token: Token) {
    let token = serde_json::to_string(&token).unwrap();
    SessionStorage::set(KEY, token).expect("failed to set");
}

/// Gets the JWT from Session Storage
pub fn get_token() -> Token {
    let token = SessionStorage::get(KEY).unwrap_or_else(|_| String::from(""));
    if token.is_empty() {
        Token::new()
    } else {
        let token: Token = serde_json::from_str(&token).unwrap();
        token
    }
}

/// Saves the Treaty instance's Database Schemas to Session Storage
pub fn set_databases(dbs: Vec<DatabaseSchema>) {
    let dbs_json = serde_json::to_string(&dbs).unwrap();
    SessionStorage::set(DATABASES, dbs_json).expect("failed to set");
}

pub fn get_database(db_name: &str) -> DatabaseSchema {
    let databases = get_databases();
    databases
        .iter()
        .find(|x| x.database_name.as_str() == db_name)
        .unwrap()
        .clone()
}

/// Gets the Treaty instance's Database Schemas from Session Storage
pub fn get_databases() -> Vec<DatabaseSchema> {
    let databases = SessionStorage::get(DATABASES).unwrap_or_else(|_| String::from(""));
    let databases: Vec<DatabaseSchema> = serde_json::from_str(&databases).unwrap();
    databases
}

/// Saves the Treaty databases Participants to Session Storage
pub fn set_participants(participants: Vec<ParticipantStatus>) {
    let participants_json = serde_json::to_string(&participants).unwrap();
    SessionStorage::set(PARTICIPANTS, participants_json).expect("failed to set");
}

/// Gets the Treaty databases Participants to Session Storage
pub fn get_participants() -> Vec<ParticipantStatus> {
    let participants = SessionStorage::get(PARTICIPANTS).unwrap_or_else(|_| String::from(""));
    let participants: Vec<ParticipantStatus> = serde_json::from_str(&participants).unwrap();
    participants
}

/// updates our status on if we're logged in or not
pub fn update_token_login_status(is_logged_in: bool) {
    let mut token = get_token();
    token.is_logged_in = is_logged_in;
    set_token(token);
}

pub fn set_status(status: String) {
    let date = js_sys::Date::new_0();
    let now = String::from(date.to_locale_time_string("en-US"));
    let message = format!("{}: {}", now, &status);
    SessionStorage::set(STATUS, message).expect("failed to set");
}

/// Gets the JWT from Session Storage
pub fn get_status() -> String {
    SessionStorage::get(STATUS).unwrap_or_else(|_| String::from(""))
}

pub fn clear_status() {
    SessionStorage::set(STATUS, "".to_string()).expect("failed to set");
}
