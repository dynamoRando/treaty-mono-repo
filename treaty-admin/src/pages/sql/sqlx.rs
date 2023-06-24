use crate::pages::sql::{enter_sql::EnterSql, sql_result::SqlResult};
use yew::{function_component, html, use_state_eq, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct SqlProps {
    pub sql_result_state: UseStateHandle<Option<String>>,
}

#[function_component]
pub fn Sql() -> Html {
    let result_state = use_state_eq(move || None);
    html! {
        <div>
            <div class="container">
                <div class="box">
                    <p>{"Execute and review SQL results from this page."}</p>
                    < EnterSql sql_result_state={result_state.clone()}/>
                    < SqlResult sql_result_state={result_state.clone()}/>
                </div>
            </div>
        </div>
    }
}
