use yew::{function_component, html, Html};

use crate::pages::host_info::{generate_info::GenerateInfo, get_info::GetInfo};

mod generate_info;
mod get_info;

#[function_component]
pub fn HostInfo() -> Html {
    html! {
        <div>
            <div class="container">
                <div class="box">
                    < GetInfo />
                    < GenerateInfo />
                </div>
            </div>
        </div>
    }
}
