use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Nav() -> Html {
    let navbar_active = use_state_eq(|| false);

    let toggle_navbar = {
        let navbar_active = navbar_active.clone();

        Callback::from(move |_| {
            navbar_active.set(!*navbar_active);
        })
    };

    let active_class = if !*navbar_active { "is-active" } else { "" };

    html! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <h1 class="navbar-item is-size-3">{ "Treaty Admin" }</h1>

                <button class={classes!("navbar-burger", "burger", active_class)}
                    aria-label="menu" aria-expanded="false"
                    onclick={toggle_navbar}
                >
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </button>
            </div>
            <div class={classes!("navbar-menu", active_class)}>
                <div class="navbar-start">
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                        { "Home" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Databases}>
                    { "Databases" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Sql}>
                    { "Sql" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Contracts}>
                    { "Contracts" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::HostInfo}>
                    { "Host Info" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Participants}>
                    { "Participants" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Behaviors}>
                    { "Behaviors" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::CooperativeHosts}>
                    { "Cooperative Hosts" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Settings}>
                    { "Settings" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Logs}>
                    { "Logs" }
                    </Link<Route>>
                </div>
            </div>
        </nav>
    }
}
