use treaty_types::types::treaty_proto::TableSchema;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ColumnProps {
    pub table: TableSchema,
}

#[function_component]
pub fn Columns(ColumnProps { table }: &ColumnProps) -> Html {
    let table_name = &table.table_name;
    let db_name = &table.database_name;

    let mut col_names: Vec<String> = Vec::new();

    for col in &table.columns {
        col_names.push(col.column_name.clone());
    }

    html! {
            <div class="container">
                <div class="box">
                    <h1 class="subtitle"> {"Columns for table "}{&table_name} {" in database "}{&db_name}</h1>
                        <div class="content">
                            <ul>
                            {
                                col_names.into_iter().map(|name| {
                                    let col_name = name.clone();
                                    html!{<div key={col_name.clone()}>
                                    <li><span class="mdi mdi-table-column"></span>{" "}{col_name.clone()}</li></div>}
                                }).collect::<Html>()
                            }</ul>
                        </div>
                </div>
           </div>
    }
}
