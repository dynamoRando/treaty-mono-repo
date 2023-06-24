use std::collections::HashMap;

use futures_util::{future::ready, stream::StreamExt};
use gloo::timers::future::IntervalStream;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::components::status::Status;
use crate::components::treaty_nav::TreatyNav;

use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::treaty_admin::behaviors::Behaviors;
use crate::pages::treaty_admin::contracts::Contracts;
use crate::pages::treaty_admin::coop_hosts::CooperativeHosts;

use crate::pages::register::Register;
use crate::pages::site_admin::SiteAdmin;
use crate::pages::treaty_admin::databases::db::TreatyDb;
use crate::pages::treaty_admin::host_info::HostInfo;
use crate::pages::treaty_admin::logs::Logs;
use crate::pages::treaty_admin::participants::Participants;
use crate::pages::treaty_admin::settings::Settings;
use crate::pages::treaty_admin::sql::sqlx::Sql;
use crate::request::proxy::get_proxy_token;
use crate::request::treaty::get_status;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/info/register")]
    Register,
    #[at("/info/login")]
    Login,
    #[at("/info/site-admin")]
    SiteAdmin,
    #[at("/my/db")]
    MyTreatyDb,
    #[at("/my/sql")]
    MyTreatySql,
    #[at("/my/contracts")]
    MyTreatyContracts,
    #[at("/my/hostinfo")]
    MyTreatyHostInfo,
    #[at("/my/part")]
    MyTreatyPart,
    #[at("/my/behavior")]
    MyTreatyBehavior,
    #[at("/my/coophost")]
    MyTreatyCoopHost,
    #[at("/my/settings")]
    MyTreatySettings,
    #[at("/my/logs")]
    MyTreatyLogs,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn App() -> Html {
    let is_proxy_logged_in_state = use_state(move || false);
    let login_state = is_proxy_logged_in_state.clone();

    let status_state = use_state(move || String::from(""));

    spawn_local(async move {
        IntervalStream::new(1_000)
            .for_each(|_| {
                check_and_set_login_status(login_state.clone());
                ready(())
            })
            .await;
    });

    {
        let status_state = status_state.clone();
        spawn_local(async move {
            IntervalStream::new(1_000)
                .for_each(|_| {
                    check_and_set_status(status_state.clone());
                    ready(())
                })
                .await;
        });
    }

    html! {
        <BrowserRouter>
            <Nav />
            <TreatyNav is_logged_in={is_proxy_logged_in_state.clone()} status_message={status_state.clone()}/>
            <Status is_logged_in={is_proxy_logged_in_state} status_message={status_state}/>
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

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Register => html! { <Register /> },
        Route::Login => html! { <Login /> },
        Route::SiteAdmin => html! { <SiteAdmin /> },
        Route::MyTreatyDb => html! { <TreatyDb /> },
        Route::MyTreatySql => html! { <Sql /> },
        Route::NotFound => html! { <PageNotFound /> },
        Route::MyTreatyContracts => html! { <Contracts /> },
        Route::MyTreatyHostInfo => html! { <HostInfo /> },
        Route::MyTreatyPart => html! { <Participants /> },
        Route::MyTreatyBehavior => html! { <Behaviors /> },
        Route::MyTreatyCoopHost => html! { <CooperativeHosts /> },
        Route::MyTreatySettings => html! { <Settings /> },
        Route::MyTreatyLogs => html! { <Logs /> },
    }
}

fn check_and_set_login_status(is_logged_in_state: UseStateHandle<bool>) {
    let is_logged_in = get_proxy_token().is_logged_in;
    is_logged_in_state.set(is_logged_in);
}

fn check_and_set_status(status: UseStateHandle<String>) {
    let status_string = get_status();
    status.set(status_string);
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
            // <Nav />

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
