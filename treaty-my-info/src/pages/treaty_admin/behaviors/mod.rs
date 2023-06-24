use yew::{function_component, html, Html};

use crate::pages::treaty_admin::behaviors::{
    deletes_from_host::DeletesFromHost, deletes_to_host::DeletesToHost,
    updates_from_host::UpdatesFromHost, updates_to_host::UpdatesToHost,
};

mod deletes_from_host;
mod deletes_to_host;
mod updates_from_host;
mod updates_to_host;

#[function_component]
pub fn Behaviors() -> Html {
    html! {
        <div>
            <div class="container">
                <div class="box">
                    <p><h1 class="subtitle"> {"Behaviors"} </h1></p>
                    <p>{"We can configure what we want to do if a host sends us update or delete
                    requests, such as ignoring them, logging them for later review, and so on."}</p>
                    <p>{"We can also configure what we want to do if we change or delete data,
                    if we should notify the host or not."}</p>
                    < UpdatesToHost />
                    < DeletesToHost />
                    < UpdatesFromHost />
                    < DeletesFromHost />
                </div>
            </div>
        </div>
    }
}
