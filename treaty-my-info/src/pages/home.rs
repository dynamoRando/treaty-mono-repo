use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <div class="has-text-centered">
                        <p><h1 class="title is-1">{ "treaty My Info" }</h1></p>
                        <p><h3 class="subtitle">{"version "}{VERSION}</h3></p>
                        <p>{"Welcome! A proxy application for holding and managing multiple treaty instances on behalf of users."}</p>
                        <p>{"For managing a single instance of treaty, leverage the "} <code>{"treaty-admin"}</code> {" application."}</p>
                        <p>{"To get started with "} <code>{"my-info"}</code>{", register an account here, which will create an treaty instance for you."}</p>
                    </div>
                </div>
            </div>

        </div>
    }
}
