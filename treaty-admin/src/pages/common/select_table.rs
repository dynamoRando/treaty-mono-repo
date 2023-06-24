use crate::request::get_databases;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, Callback, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
/// Represents a state handle with the selected database name
/// and a callback that returns the selected database name on click
pub struct SelectTableProps {
    pub active_database_name: Option<UseStateHandle<String>>,
    pub active_table_name: Option<UseStateHandle<String>>,
    pub onclick_table: Option<Callback<String>>,
}

#[function_component]
pub fn SelectTable(
    SelectTableProps {
        active_database_name,
        active_table_name,
        onclick_table,
    }: &SelectTableProps,
) -> Html {
    let active_database_name = active_database_name.clone();
    let active_table_name = active_table_name.clone();

    let mut table_names: Vec<String> = Vec::new();

    if let Some(x) = active_database_name {
        table_names.clear();
        let databases = get_databases();

        let db_name = x;

        if !db_name.is_empty() {
            let database = databases
                .iter()
                .find(|x| x.database_name == *db_name)
                .unwrap()
                .clone();

            for table in &database.tables {
                table_names.push(table.table_name.clone());
            }
        }
    }

    let ui_active_table = use_node_ref();

    let local_onclick = {
        let active_table_name = active_table_name;
        let ui_active_table = ui_active_table.clone();
        let onclick_table = onclick_table.clone();
        Callback::from(move |_| {
            let active_table_name = active_table_name.clone();
            let onclick_table = onclick_table.clone();
            let selected_table = ui_active_table.cast::<HtmlInputElement>();
            if selected_table.is_some() {
                let selected_table_val =
                    ui_active_table.cast::<HtmlInputElement>().unwrap().value();

                if let Some(x) = active_table_name {
                    x.set(selected_table_val.clone());
                }

                if let Some(x) = onclick_table {
                    x.emit(selected_table_val);
                }
            }
        })
    };

    html! {
        <div>
            <div class="select is-multiple">
                <select
                    name="select_table"
                    id="select_table"
                    ref={&ui_active_table}
                    onclick={local_onclick}
                >
                <option value="SELECT TABLE">{"SELECT TABLE"}</option>
                {
                    table_names.into_iter().map(|name| {
                        html!{
                        <option value={name.clone()}>{name.clone()}</option>}
                    }).collect::<Html>()
                }
                </select>
            </div>
        </div>
    }
}
