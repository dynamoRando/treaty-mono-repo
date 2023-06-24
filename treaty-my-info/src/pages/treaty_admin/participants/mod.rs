use yew::{function_component, html, use_state_eq, Html, Properties, UseStateHandle};

use crate::pages::treaty_admin::participants::{add::AddParticipant, view::ViewParticipants};

pub mod add;
pub mod view;

#[derive(Properties, PartialEq)]
pub struct ActiveDbProps {
    pub active_db: UseStateHandle<String>,
}

#[function_component]
pub fn Participants() -> Html {
    let active_db = use_state_eq(move || String::from(""));

    html! {
        <div>
            <div class="container">
                <div class="box">
                    < ViewParticipants active_db={active_db.clone()}/>
                    < AddParticipant active_db={active_db.clone()}/>
                </div>
            </div>
        </div>
    }
}
