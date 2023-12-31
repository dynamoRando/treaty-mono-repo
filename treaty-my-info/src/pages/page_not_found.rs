use yew::{function_component, html, Html};

#[function_component]
pub fn PageNotFound() -> Html {
    html! {
        <section class="hero is-danger is-bold is-large">
            <div class="hero-body">
                <div class="container">
                    <h1 class="title">
                        { "Page not found" }
                    </h1>
                    <h2 class="subtitle">
                        { "Page does not seem to exist" }
                    </h2>
                </div>
            </div>
        </section>
    }
}
