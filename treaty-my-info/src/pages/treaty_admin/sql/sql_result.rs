use crate::pages::treaty_admin::sql::sqlx::SqlProps;
use yew::{function_component, html, Html};

#[function_component]
pub fn SqlResult(SqlProps { sql_result_state }: &SqlProps) -> Html {
    let mut text = String::from("");

    if sql_result_state.is_some() {
        let sql_result_state = sql_result_state.as_ref();
        text = sql_result_state.unwrap().clone();
    }

    html! {
        <div>
        <h1 class="subtitle"> {"SQL Results"} </h1>
        <label for="sql_result">{ "Results" }</label>
        <p>
        <textarea class="textarea" rows="5" cols="60" id ="sql_Result" placeholder="SQL Results Will Be Displayed Here As Markdown Table" value={text} readonly=true/>
        </p>
        </div>
    }
}
