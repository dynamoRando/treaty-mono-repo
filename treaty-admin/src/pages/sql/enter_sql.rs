use crate::{
    pages::{
        common::select_database::SelectDatabase,
        sql::{read::read, sqlx::SqlProps, write::cooperative_write, write::write},
    },
    request::{
        clear_status, get_client, get_database, get_databases, get_token, set_status,
        update_token_login_status,
    },
};
use treaty_http_endpoints::client::{
    COOPERATIVE_WRITE_SQL_AT_HOST, READ_SQL_AT_HOST, READ_SQL_AT_PARTICIPANT, WRITE_SQL_AT_HOST,
    WRITE_SQL_AT_PARTICIPANT,
};

use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, use_state_eq, Callback, Html};

#[function_component]
pub fn EnterSql(SqlProps { sql_result_state }: &SqlProps) -> Html {
    let sql_result_state = sql_result_state.clone();
    let databases = get_databases();
    let mut database_names: Vec<String> = Vec::new();

    let participant_aliases = use_state_eq(move || {
        let list: Vec<String> = Vec::new();
        Some(list)
    });

    let participant_dropdown_enabled = use_state_eq(move || false);

    for database in &databases {
        database_names.push(database.database_name.clone());
    }

    let active_database = use_state_eq(move || String::from(""));
    let active_participant = use_state_eq(move || None);

    // text-box
    let ui_enter_sql_text = use_node_ref();

    // drop-down
    let ui_active_participant = use_node_ref();

    let onclick_db = {
        let participant_aliases = participant_aliases.clone();
        let participant_dropdown_enabled = participant_dropdown_enabled.clone();
        Callback::from(move |db_name: String| {
            if db_name.is_empty() || db_name == "SELECT DATABASE" {
            } else {
                let list: Vec<String> = Vec::new();
                participant_aliases.set(Some(list));
                participant_dropdown_enabled.set(false);
                let database = get_database(&db_name);
                let cooperation_enabled = database.cooperation_enabled;
                let participant_dropdown_enabled = participant_dropdown_enabled.clone();
                if cooperation_enabled {
                    let participant_aliases = participant_aliases.clone();
                    let token = get_token();
                    //
                    let mut client = get_client();
                    spawn_local(async move {
                        let reply = client.get_participants(token.auth(), &db_name).await;

                        match reply {
                            Ok(response) => {
                                clear_status();
                                let participant_aliases = participant_aliases.clone();

                                let is_authenticated = response
                                    .authentication_result
                                    .as_ref()
                                    .unwrap()
                                    .is_authenticated;
                                update_token_login_status(is_authenticated);

                                if is_authenticated {
                                    if !response.is_error {
                                        let participants = response.participants;

                                        let mut aliases: Vec<String> = Vec::new();
                                        for p in &participants {
                                            aliases.push(
                                                p.participant.as_ref().unwrap().alias.clone(),
                                            );
                                        }

                                        participant_dropdown_enabled.set(true);
                                        participant_aliases.set(Some(aliases));
                                    } else {
                                        let message = format!(
                                            "{} - {}",
                                            response.error.as_ref().unwrap().message,
                                            response.error.as_ref().unwrap().help.as_ref().unwrap_or(&"".to_string())
                                        );
                                        set_status(message);
                                    }
                                }
                            }
                            Err(e) => {
                                set_status(e);
                            }
                        }
                    });
                }
            }
        })
    };

    let onchange_participant = {
        let active_participant = active_participant.clone();
        let ui_active_participant = ui_active_participant.clone();

        Callback::from(move |_| {
            let selected_particpant = ui_active_participant.cast::<HtmlInputElement>();
            let ui_active_participant = ui_active_participant.clone();

            if selected_particpant.is_some() {
                let selected_participant_val = ui_active_participant
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                active_participant.set(Some(selected_participant_val));
            }
        })
    };

    let onclick_read_at_host = {
        let sql_result_state = sql_result_state.clone();
        let ui_enter_sql_text = ui_enter_sql_text.clone();
        let active_database = active_database.clone();
        Callback::from(move |_| {
            let active_database = active_database.clone();
            let db_name = active_database;
            let text = ui_enter_sql_text
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();
            read(
                db_name.to_string(),
                text,
                sql_result_state.clone(),
                READ_SQL_AT_HOST,
            )
        })
    };

    let onclick_read_at_part = {
        let sql_result_state = sql_result_state.clone();
        let ui_enter_sql_text = ui_enter_sql_text.clone();
        let active_database = active_database.clone();
        Callback::from(move |_| {
            let active_database = active_database.clone();
            let db_name = active_database;
            let text = ui_enter_sql_text
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();
            read(
                db_name.to_string(),
                text,
                sql_result_state.clone(),
                READ_SQL_AT_PARTICIPANT,
            )
        })
    };

    let onclick_write_at_host = {
        let sql_result_state = sql_result_state.clone();
        let ui_enter_sql_text = ui_enter_sql_text.clone();
        let active_database = active_database.clone();
        Callback::from(move |_| {
            let active_database = active_database.clone();
            let text = ui_enter_sql_text
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();
            write(
                active_database.to_string(),
                text,
                sql_result_state.clone(),
                WRITE_SQL_AT_HOST,
            )
        })
    };

    let onclick_write_at_part = {
        let sql_result_state = sql_result_state.clone();
        let ui_enter_sql_text = ui_enter_sql_text.clone();
        let active_database = active_database.clone();
        Callback::from(move |_| {
            let active_database = active_database.clone();
            let text = ui_enter_sql_text
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();
            write(
                active_database.to_string(),
                text,
                sql_result_state.clone(),
                WRITE_SQL_AT_PARTICIPANT,
            )
        })
    };

    let onclick_coop_write_at_host = {
        let sql_result_state = sql_result_state;
        let ui_enter_sql_text = ui_enter_sql_text.clone();
        let active_database = active_database.clone();
        Callback::from(move |_| {
            let active_database = active_database.clone();

            let alias = active_participant.as_ref().unwrap().clone();

            let db_name = active_database;
            let text = ui_enter_sql_text
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();
            cooperative_write(
                db_name.to_string(),
                text,
                alias,
                sql_result_state.clone(),
                COOPERATIVE_WRITE_SQL_AT_HOST,
            )
        })
    };

    html! {
        <div>
            <h1 class="subtitle"> {"Execute SQL"} </h1>
                <p>
                    <p><label for="execute_sql_dbs">{ "Select Database " }</label></p>
                    <p>< SelectDatabase active_db_name={active_database} onclick_db={onclick_db}/></p>
                </p>
            <p><label for="execute_sql">{ "Enter SQL" }</label></p>
            <p>
                <textarea class="textarea" rows="5" cols="60"  id ="execute_sql" placeholder="SELECT * FROM TABLE_NAME" ref={&ui_enter_sql_text}/>
            </p>

            <p><h3> {"Choose Participant"} </h3></p>
            <p>{"Select the participant to execute on, if applicable."}</p>
            <p>
                <label for="select_participant_for_execute">{ "Select Participant " }</label>

                <div class="select is-multiple">
                    <select
                        name="select_participant_for_execute"
                        id="select_participant_for_execute"
                        ref={&ui_active_participant}
                        onchange={&onchange_participant}
                        disabled={!*participant_dropdown_enabled}
                    >
                    <option value="SELECT PARTICIPANT">{"SELECT PARTICIPANT"}</option>
                    {
                        (*participant_aliases).as_ref().unwrap().clone().into_iter().map(|name| {
                            // console::log_1(&name.clone().into());
                            html!{
                            <option value={name.clone()}>{name.clone()}</option>}
                        }).collect::<Html>()
                    }
                    </select>
                </div>

                <p>{"The following commands denote if you wish to execute your SQL action (read or write) against the specified type of database (host or partial). To write data to a participant, use Cooperative Write."}</p>
            </p>
            <div class="buttons">
                <button class="button is-primary" type="button" id="read_at_host" value="Read On Host Db" onclick={&onclick_read_at_host}>
                    <span class="mdi mdi-database">{" Read At Host"}</span>
                </button>
                <button class="button is-primary" type="button" id="read_at_part" value="Read On Partial Db" onclick={&onclick_read_at_part}>
                    <span class="mdi mdi-database-outline">{" Read At Partial"}</span>
                </button>
                <button class="button is-warning" type="button" id="write_at_host" value="Write On Host Db" onclick={&onclick_write_at_host}>
                    <span class="mdi mdi-database">{" Write At Host"}</span>
                </button>
                <button class="button is-warning" type="button" id="write_at_part" value="Write On Part Db" onclick={&onclick_write_at_part}>
                    <span class="mdi mdi-database-outline">{" Write At Partial"}</span>
                </button>
                <button class="button is-warning" type="button" id="coop_write_at_part" value="Cooperative Write On Host Db" onclick={&onclick_coop_write_at_host}>
                    <span class="mdi mdi-database-export"></span><span class="mdi mdi-database-import-outline"></span>{" Cooperative Write At Host"}
                </button>
            </div>
        </div>
    }
}
