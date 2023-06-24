use gloo::storage::{SessionStorage, Storage};
use treaty_client_wasm::{client::TreatyClient, token::Token};
use treaty_types::types::treaty_proto::{DatabaseSchema, ParticipantStatus};

const KEY: &str = "treatymyinfo.key.treaty.instance";
const DATABASES: &str = "treatymyinfo.key.databases";
const PARTICIPANTS: &str = "treatymyinfo.key.participants";
const STATUS: &str = "treatymyinfo.key.status";
const TREATYCLIENT: &str = "treatymyinfo.key.client";

pub fn set_treaty_client(client: &TreatyClient) {
    let client_json = serde_json::to_string(&client).unwrap();
    SessionStorage::set(TREATYCLIENT, client_json).expect("failed to set");
}

pub fn get_treaty_client() -> TreatyClient {
    let client = SessionStorage::get(TREATYCLIENT).unwrap_or_else(|_| String::from(""));
    if client.is_empty() {
        TreatyClient::new("", 0)
    } else {
        let client: TreatyClient = serde_json::from_str(&client).unwrap();
        client
    }
}

/// Saves the JWT to Session Storage
pub fn set_treaty_token(token: Token) {
    let token = serde_json::to_string(&token).unwrap();
    SessionStorage::set(KEY, token).expect("failed to set");
}

/// Gets the JWT from Session Storage
pub fn get_treaty_token() -> Token {
    let token = SessionStorage::get(KEY).unwrap_or_else(|_| String::from(""));
    if token.is_empty() {
        Token::new()
    } else {
        let token: Token = serde_json::from_str(&token).unwrap();
        token
    }
}

/// Saves the treaty instance's Database Schemas to Session Storage
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

/// Gets the treaty instance's Database Schemas from Session Storage
pub fn get_databases() -> Vec<DatabaseSchema> {
    let databases = SessionStorage::get(DATABASES).unwrap_or_else(|_| String::from(""));
    if databases.is_empty() {
        let _x: Vec<DatabaseSchema> = Vec::new();
        return _x;
    }
    let databases: Vec<DatabaseSchema> = serde_json::from_str(&databases).unwrap();
    databases
}

/// Saves the treaty databases Participants to Session Storage
pub fn set_participants(participants: Vec<ParticipantStatus>) {
    let participants_json = serde_json::to_string(&participants).unwrap();
    SessionStorage::set(PARTICIPANTS, participants_json).expect("failed to set");
}

/// Gets the treaty databases Participants to Session Storage
pub fn get_participants() -> Vec<ParticipantStatus> {
    let participants = SessionStorage::get(PARTICIPANTS).unwrap_or_else(|_| String::from(""));
    let participants: Vec<ParticipantStatus> = serde_json::from_str(&participants).unwrap();
    participants
}

/// updates our status on if we're logged in or not
pub fn update_token_login_status(is_logged_in: bool) {
    let mut token = get_treaty_token();
    token.is_logged_in = is_logged_in;
    set_treaty_token(token);
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
