use crate::request::get_databases;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, Callback, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
/// Represents a state handle with the selected database name
/// and a callback that returns the selected database name on click
pub struct SelectDatabaseProps {
    pub active_db_name: Option<UseStateHandle<String>>,
    pub onclick_db: Option<Callback<String>>,
}

#[function_component]
pub fn SelectDatabase(
    SelectDatabaseProps {
        active_db_name,
        onclick_db,
    }: &SelectDatabaseProps,
) -> Html {
    let databases = get_databases();
    let mut database_names: Vec<String> = Vec::new();

    for database in &databases {
        database_names.push(database.database_name.clone());
    }

    let ui_active_database = use_node_ref();
    let active_db_name = active_db_name.clone();

    let local_onclick = {
        let active_db_name = active_db_name;
        let ui_active_database = ui_active_database.clone();
        let onclick_db = onclick_db.clone();
        Callback::from(move |_| {
            let active_db_name = active_db_name.clone();
            let onclick_db = onclick_db.clone();
            let selected_db = ui_active_database.cast::<HtmlInputElement>();

            if selected_db.is_some() {
                let selected_db_val = ui_active_database
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();

                if let Some(x) = active_db_name {
                    x.set(selected_db_val.clone());
                }

                if let Some(y) = onclick_db {
                    y.emit(selected_db_val);
                }
            }
        })
    };

    html! {
        <div>
            <div class="select is-multiple">
                <select
                    name="execute_sql_dbs"
                    id="execute_sql_dbs"
                    ref={&ui_active_database}
                    onclick={local_onclick}
                >
                <option value="SELECT DATABASE">{"SELECT DATABASE"}</option>
                {
                    database_names.into_iter().map(|name| {
                        html!{
                        <option value={name.clone()}>{name.clone()}</option>}
                    }).collect::<Html>()
                }
                </select>
            </div>
        </div>
    }
}
