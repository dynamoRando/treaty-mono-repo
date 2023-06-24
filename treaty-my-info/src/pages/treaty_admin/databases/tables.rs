use crate::log::log_to_console;
use crate::pages::treaty_admin::databases::columns::Columns;
use crate::pages::treaty_admin::databases::get_table_policy::GetTablePolicy;
use crate::pages::treaty_admin::databases::set_table_policy::SetTablePolicy;

use treaty_types::types::treaty_proto::{DatabaseSchema, TableSchema};
use yew::{function_component, html, use_state_eq, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub db: DatabaseSchema,
}

#[function_component]
pub fn Tables(TableProps { db }: &TableProps) -> Html {
    let message = format!("{}{}", "entered tables for: ", db.database_name);
    log_to_console(&message);

    let db = db.clone();
    let tables = db.tables.clone();

    let mut table_names: Vec<String> = Vec::new();

    for table in &*db.tables {
        table_names.push(table.table_name.clone());
    }

    let selected_table = use_state_eq(|| None);

    let get_policy = selected_table.as_ref().map(|table: &TableSchema| {
        html! {
            <GetTablePolicy table={table.clone()} />
        }
    });

    let set_policy = selected_table.as_ref().map(|table: &TableSchema| {
        html! {
            <SetTablePolicy table={table.clone()} />
        }
    });

    let columns = selected_table.as_ref().map(|table: &TableSchema| {
        html! {
            <Columns table={table.clone()} />
        }
    });

    html!(
        <div>
            <div class="container">
                    <div class="box">
                        <h1 class="subtitle"> {"Tables for database: "}{&db.database_name} </h1>

                        <p>{"Tables for the specified database will appear here. Click on a table name to view schema info."}</p>
                            <div class="content">
                                <ul>
                                {
                                    table_names.into_iter().map(|name| {
                                        let table_name = name.clone();
                                        html!{
                                            <div key={table_name.clone()}>
                                                <li onclick={

                                                    let selected_table = selected_table.clone();
                                                    let tables = tables.clone();
                                                    let name = name.clone();

                                                    move |_| {

                                                            let table = &tables
                                                                .iter()
                                                                .find(|x| x.table_name.as_str() == name.clone())
                                                                .unwrap()
                                                                .clone();

                                                                selected_table.set(Some(table.clone()));
                                                        }
                                                    }><span class="mdi mdi-table"></span>{" "}{name.clone()}</li>
                                            </div>}
                                    }).collect::<Html>()
                                }
                                </ul>
                                { get_policy }
                                <p></p>
                                { set_policy }
                            </div>
                    </div>
            </div>
            { columns }
        </div>
    )
}
