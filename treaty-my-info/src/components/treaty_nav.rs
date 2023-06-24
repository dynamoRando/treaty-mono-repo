use crate::{
    app::Route,
    request::proxy::{get_proxy_token, get_un},
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct TreatyStatusProps {
    pub is_logged_in: UseStateHandle<bool>,
    pub status_message: UseStateHandle<String>,
}

#[function_component]
pub fn TreatyNav(props: &TreatyStatusProps) -> Html {
    /*
        let navigator = use_navigator().unwrap();

        if !has_proxy_token() {
            navigator.push(&Route::Login);

            html! {
                <div>
                    <p>{"You are not logged in, redirecting to login page."}</p>
                </div>
            }
        } else {
            html! {
                <div>
                    <p>{"my treaty placeholder"}</p>
                </div>
            }
        }
    */

    let navbar_active = use_state_eq(|| false);
    navbar_active.set(*props.is_logged_in);

    let toggle_navbar = {
        let navbar_active = navbar_active.clone();

        Callback::from(move |_| {
            navbar_active.set(!*navbar_active);
        })
    };
    let active_class = if !*navbar_active { "is-invisible" } else { "" };

    let un = get_un();

    html!(
        <div>
            <nav class="navbar is-light">
                <div class="navbar-brand">
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
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyTreatyDb}>
                            { "Databases" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyTreatySql}>
                        { "Sql" }
                        </Link<Route>>
                         <Link<Route> classes={classes!("navbar-item")} to={Route::MyTreatyContracts}>
                        { "Contracts" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyTreatyHostInfo}>
                        { "Host Info" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyTreatyPart}>
                        { "Participants" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyTreatyBehavior}>
                        { "Behaviors" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyTreatyCoopHost}>
                        { "Cooperative Hosts" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyTreatySettings}>
                        { "Settings" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyTreatyLogs}>
                        { "Logs" }
                        </Link<Route>>
                    </div>
                </div>
                    {
                        if *props.is_logged_in {
                            let token = get_proxy_token();
                            let id = token.id.as_ref().unwrap().clone();
                            html! {
                                <div>
                                    <div class="navbar-item">
                                        <input type="text" class="input" size=36
                                        id ="account_id" placeholder="Account Id"
                                        value={id} readonly=true />
                                    <div class="navbar-item">
                                            <div class="buttons">
                                                <button class="button is-warning">
                                                {"User Name: "}{un}
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                        else {
                            html! {
                                <div class="navbar-item">
                                    <button class="button is-light">
                                        <span class="mdi mdi-account-cancel">{" Not Logged In"}</span>
                                    </button>
                                </div>
                                }
                        }
                    }
            </nav>
        </div>
    )
}
