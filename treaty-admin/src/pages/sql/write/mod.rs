use treaty_types::types::treaty_proto::{
    ExecuteCooperativeWriteReply, ExecuteCooperativeWriteRequest, ExecuteWriteReply,
    ExecuteWriteRequest,
};
use yew::{AttrValue, Callback, UseStateHandle};

use crate::{
    log::log_to_console,
    request::{self, clear_status, set_status, get_client},
};

pub fn write(db_name: String, text: String, state: UseStateHandle<Option<String>>, endpoint: &str) {
    let client = get_client();
    
    let request = ExecuteWriteRequest {
        database_name: db_name,
        sql_statement: text,
        database_type: 1,
        where_clause: "".to_string(),
    };

    let write_request_json = serde_json::to_string(&request).unwrap();
    let url = format!("{}{}", client.user_addr_port(), endpoint);

    let callback = Callback::from(move |response: Result<AttrValue, String>| {
        if let Ok(ref x) = response {
            log_to_console(x.to_string());
            clear_status();

            let write_reply: ExecuteWriteReply = serde_json::from_str(x).unwrap();

            let mut result_message = String::new();

            result_message += &format!("Is result successful: {}", write_reply.is_successful);

            result_message += "\n";
            result_message += &format!("Total rows affected: {}", write_reply.total_rows_affected);

            if write_reply.is_error {
                result_message += "\n";
                result_message += &format!("Error Message: {}", write_reply.error.unwrap().message);
            }

            let sql_table_text = result_message.clone();

            state.set(Some(sql_table_text));
        } else {
            set_status(response.err().unwrap());
        }
    });

    request::post(url, write_request_json, callback);
}

pub fn cooperative_write(
    db_name: String,
    text: String,
    participant_alias: String,
    state: UseStateHandle<Option<String>>,
    endpoint: &str,
) {
    let client = get_client();

    let request = ExecuteCooperativeWriteRequest {
        database_name: db_name,
        sql_statement: text,
        database_type: 1,
        where_clause: "".to_string(),
        alias: participant_alias,
        participant_id: "".to_string(),
    };

    let write_request_json = serde_json::to_string(&request).unwrap();
    let url = format!("{}{}", client.user_addr_port(), endpoint);

    let callback = Callback::from(move |response: Result<AttrValue, String>| {
        if let Ok(ref x) = response {
            log_to_console(x.to_string());
            clear_status();

            let write_reply: ExecuteCooperativeWriteReply = serde_json::from_str(x).unwrap();

            let mut result_message = String::new();

            result_message =
                result_message + &format!("Is result successful: {}", write_reply.is_successful);

            result_message += "\n";
            result_message = result_message
                + &format!("Total rows affected: {}", write_reply.total_rows_affected);
            result_message += "\n";

            let sql_table_text = result_message;

            state.set(Some(sql_table_text));
        }
    });

    request::post(url, write_request_json, callback);
}
