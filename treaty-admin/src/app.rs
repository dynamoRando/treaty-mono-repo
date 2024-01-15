use std::collections::HashMap;

use futures_util::{future::ready, stream::StreamExt};
use gloo::timers::future::IntervalStream;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::components::status::Status;
use crate::pages::behaviors::Behaviors;
use crate::pages::contracts::Contracts;
use crate::pages::coop_hosts::CooperativeHosts;
use crate::pages::databases::db::Databases;
use crate::pages::home::Home;
use crate::pages::host_info::HostInfo;
use crate::pages::logs::Logs;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::participants::Participants;
use crate::pages::settings::Settings;
use crate::pages::sql::sqlx::Sql;
use crate::request::{get_status, get_client};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/databases")]
    Databases,
    #[at("/sql")]
    Sql,
    #[at("/contracts")]
    Contracts,
    #[at("/hostinfo")]
    HostInfo,
    #[at("/participants")]
    Participants,
    #[at("/behaviors")]
    Behaviors,
    #[at("/CooperativeHosts")]
    CooperativeHosts,
    #[at("/Settings")]
    Settings,
    #[at("/Logs")]
    Logs,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn check_and_set_login_status(is_logged_in_state: UseStateHandle<bool>) {
    let is_logged_in = get_client().is_logged_in();
    is_logged_in_state.set(is_logged_in);
}

fn check_and_set_status(status: UseStateHandle<String>) {
    let status_string = get_status();
    status.set(status_string);
}

#[function_component]
pub fn App() -> Html {
    let is_logged_in_state = use_state(move || false);
    let login_state = is_logged_in_state.clone();

    let status_state = use_state(move || String::from(""));
    let status = status_state.clone();

    spawn_local(async move {
        IntervalStream::new(1_000)
            .for_each(|_| {
                check_and_set_login_status(login_state.clone());
                ready(())
            })
            .await;
    });

    spawn_local(async move {
        IntervalStream::new(1_000)
            .for_each(|_| {
                check_and_set_status(status.clone());
                ready(())
            })
            .await;
    });

    html! {
        <BrowserRouter>
            <Nav />
            <Status is_logged_in={is_logged_in_state} status_message={status_state}/>
            <main>
                <Switch<Route> render={switch} />
            </main>
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                </div>
            </footer>
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Nav />

            <main>
                <Switch<Route> render={switch} />
            </main>
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                </div>
            </footer>
        </Router>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Databases => {
            html! { <Databases /> }
        }
        Route::Sql => {
            html! { <Sql /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::Contracts => {
            html! { <Contracts /> }
        }
        Route::HostInfo => {
            html! { <HostInfo /> }
        }
        Route::Participants => {
            html! { <Participants /> }
        }
        Route::Behaviors => {
            html! { <Behaviors /> }
        }
        Route::CooperativeHosts => {
            html! { <CooperativeHosts /> }
        }
        Route::Settings => {
            html! { <Settings /> }
        }
        Route::Logs => {
            html! { <Logs /> }
        }
    }
}
