
use treaty_types::{types::treaty_proto::{ExecuteReadRequest, ExecuteReadReply}, formatter};
use yew::{AttrValue, Callback, UseStateHandle};

use crate::{
    log::log_to_console,
    request::{self, clear_status, set_status, get_client},
};

pub fn read(db_name: String, text: String, state: UseStateHandle<Option<String>>, endpoint: &str) {
    let client = get_client();

    let request = ExecuteReadRequest {
        database_name: db_name.into(),
        sql_statement: text.into(),
        database_type: 1,
    };

    let read_request_json = serde_json::to_string(&request).unwrap();
    let url = format!("{}{}", client.user_addr_port(), endpoint);

    let callback = Callback::from(move |response: Result<AttrValue, String>| {
        if let Ok(ref x) = response {
            log_to_console(x.to_string());
            clear_status();

            let read_reply: ExecuteReadReply = serde_json::from_str(x).unwrap();

                let result = read_reply.results.first().unwrap();
                if !result.error.is_some() {
                    let rows = result.clone().rows;
                    let sql_table_text = formatter::rows_to_string_markdown_table(&rows);

                    state.set(Some(sql_table_text));
                } else {
                    let mut message = String::new();
                    message += "ERROR: ";
                    message += &result.error.as_ref().unwrap().message.clone();

                    state.set(Some(message));
                }
            
        } else {
            set_status(response.err().unwrap());
        }
    });

    request::post(url, read_request_json, callback);
}
