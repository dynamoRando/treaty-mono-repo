use crate::{
    pages::{
        databases::{add::Create, enable_coop::EnableCoop, tables::Tables},
        home,
    },
    request::{get_database, get_databases, get_client},
};
use treaty_types::types::treaty_proto::DatabaseSchema;

use yew::{function_component, html, use_state_eq, Callback, Html};

/// Represents viewing and configuring schema data for tables in an Treaty instance
#[function_component]
pub fn Databases() -> Html {
    let databases = get_databases();
    let mut database_names: Vec<String> = Vec::new();

    for db in &databases {
        database_names.push(db.database_name.clone());
    }

    let x = use_state_eq(move || {
        let x: Vec<String> = Vec::new();
        x
    });

    let selected_database = use_state_eq(|| None);

    let tables = selected_database.as_ref().map(|db: &DatabaseSchema| {
        html! {
            <Tables db={db.clone()} />
        }
    });

    let reload_db_onclick = Callback::from(move |_| {
        let client = get_client();
        let addr = format!("http:// {}:{}", client.user_addr(), client.user_port());
        home::databases(&addr, x.clone());
    });

    html! {
        <div>
            <div class="container">
                <div class="box">
                        <h1 class="subtitle"> {"Databases"} </h1>
                        <p>{"View database schema information and configure properties of schema objects from this page."}</p>
                        <p>{"After loading, click on a database to view details."}</p>
                        <button type="button" class="button is-primary" id="get_databases" value="Reload databases"
                        onclick={reload_db_onclick}>
                        <span class="mdi mdi-database-refresh">{" Reload"}</span>
                        </button>
                        <h2 class="subtitle">{"Icon Key"}</h2>
                        <div class="table-container">
                            <table class="table is-narrow">
                                <thead>
                                    <tr>
                                        <th>{"Icon"}</th>
                                        <th>{"Value"}</th>
                                    </tr>
                                </thead>
                                <tr>
                                    <td><span class="mdi mdi-database"></span></td>
                                    <td>{"Database"}</td>
                                </tr>
                                <tr>
                                    <td><span class="mdi mdi-handshake"></span></td>
                                    <td>{"Cooperation Is Enabled"}</td>
                                </tr>
                                <tr>
                                    <td><span class="mdi mdi-account-multiple"></span></td>
                                    <td>{"Has Participants"}</td>
                                </tr>
                            </table>
                        </div>
                        <div class="content">
                            <ul>
                                {
                                    database_names.clone().into_iter().map(|name| {

                                    let db_name = name.clone();
                                    let db = db_name.clone();

                                    let database = get_database(&db_name);

                                    let cooperation_enabled = database.cooperation_enabled;
                                    let has_participants = database.has_participants;

                                    html!{
                                    <div key={db_name.clone()}>
                                        <li onclick={
                                            let selected_database = selected_database.clone();

                                            move |_| {
                                                    let database = get_database(&db_name);
                                                    selected_database.set(Some(database));
                                                }
                                            }>
                                            <span class="mdi mdi-database"></span>
                                            {
                                                if cooperation_enabled {
                                                    html!{
                                                        <span class="mdi mdi-handshake"></span>
                                                    }
                                                } else {
                                                    html!{
                                                        <a></a>
                                                    }
                                                }
                                            }
                                            {
                                                if has_participants {
                                                    html!{
                                                        <span class="mdi mdi-account-multiple"></span>
                                                    }
                                                } else {
                                                    html!{
                                                        <a></a>
                                                    }
                                                }
                                            }
                                            {" "}
                                            {db.clone()}
                                        </li>
                                    </div>
                                }
                                    }).collect::<Html>()
                                }
                            </ul>
                        </div>
                </div>
            </div>
            { tables }
            < Create />
            < EnableCoop />
        </div>
    }
}
