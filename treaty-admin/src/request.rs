use gloo::storage::{SessionStorage, Storage};
use treaty_client_wasm::client::TreatyClient;
use treaty_types::types::treaty_proto::{DatabaseSchema, ParticipantStatus};
use yew::{platform::spawn_local, AttrValue, Callback};

use crate::log::log_to_console;

const DATABASES: &str = "treatyadmin.key.databases";
const PARTICIPANTS: &str = "treatyadmin.key.participants";
const STATUS: &str = "treatyadmin.key.status";
const CLIENT: &str = "treatyadmin.key.client";

/// sends an HTTP POST to the specified URL with the treaty-message as JSON, returning JSON if successful,
/// otherwise a string describing the error that occurred
pub fn post(url: String, body: String, callback: Callback<Result<AttrValue, String>>) {
    let message = format!("{}{}", "outgoing message: ", body);
    log_to_console(message);

    let message = format!("{}{}", "outgoing URL: ", url);
    log_to_console(message);

    spawn_local(async move {
        let mut client = get_client();
        let result = client.send_http_message(&body, &url).await;

        match result {
            Ok(response) => {
                set_client(&client);
                callback.emit(Ok(AttrValue::from(response)));
            }
            Err(e) => {
                set_client(&client);
                callback.emit(Err(e))
            }
        }
    });
}

pub fn set_client(client: &TreatyClient) {
    let client_json = serde_json::to_string(&client).unwrap();
    SessionStorage::set(CLIENT, client_json).expect("failed to set");
}

pub fn get_client() -> TreatyClient {
    let client = SessionStorage::get(CLIENT).unwrap_or_else(|_| String::from(""));
    if client.is_empty() {
        TreatyClient::new("", 0, "", 0, "", "", None)
    } else {
        let client: TreatyClient = serde_json::from_str(&client).unwrap();
        client
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
