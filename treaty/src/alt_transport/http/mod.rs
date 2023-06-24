use lazy_static::lazy_static;
use rocket::fairing::Kind;
use rocket::http::Header;
use rocket::log::LogLevel;

use rocket::{
    fairing::{Fairing, Info},
    get,
    http::Status,
};
use rocket::{routes, Config, Shutdown};
use rocket::{Request, Response};
use std::sync::Mutex;
use std::thread;
use tracing::{info, trace};

use crate::data_service_handler::data_service_handler_actions::DataServiceHandlerActions;





use crate::user_service_handler::user_service_handler_actions::UserServiceHandlerActions;
use crate::{
    data_service_handler::DataServiceHandler, db_interface::dbi_actions::DbiActions,
    remote::remote_actions::RemoteActions, service::Service,
    user_service_handler::UserServiceHandler,
};

use stdext::function_name;

pub(crate) mod client;
pub(crate) mod data;

struct HttpHandler<
    D: DbiActions + Clone + Send + Sync + 'static,
    R: RemoteActions + Clone + Send + Sync + 'static,
> {
    user: UserServiceHandler<D, R>,
    db: DataServiceHandler<D>,
    addr: String,
    port: u16,
}

impl<
        D: DbiActions + Clone + Send + Sync + 'static,
        R: RemoteActions + Clone + Send + Sync + 'static,
    > HttpHandler<D, R>
{
    #[allow(dead_code, unused_variables)]
    pub fn new(
        user: UserServiceHandler<D, R>,
        db: DataServiceHandler<D>,
        addr: &str,
        port: u16,
    ) -> Self {
        Self {
            user,
            db,
            addr: addr.into(),
            port,
        }
    }
}

lazy_static! {
    static ref SERVER: Mutex<HttpServer> = Mutex::new(HttpServer::default());
}

#[derive(Debug, Clone, Default)]
pub struct HttpServer {
    pub service: Option<Service>,
    pub addr: String,
    pub port: u16,
}

impl HttpServer {
    pub fn new(service: Service, addr: &str, port: u16) -> Self {
        Self {
            service: Some(service),
            addr: addr.into(),
            port,
        }
    }

    pub fn user(&self) -> impl UserServiceHandlerActions {
        Service::new_http_user_handler(
            &self.service.as_ref().unwrap().root(),
            &self.service.as_ref().unwrap().settings(),
        )
    }

    pub fn data(&self) -> impl DataServiceHandlerActions {
        Service::new_data_handler(
            &self.service.as_ref().unwrap().root(),
            &self.service.as_ref().unwrap().settings(),
        )
    }
}

pub fn start_http(http_server: HttpServer) {
    trace!(
        "[{}]: attempting to start server: {http_server:?}",
        function_name!()
    );
    *SERVER.lock().unwrap() = http_server;

    thread::spawn(move || {
        let _ = start();
    });
}

#[rocket::main]
async fn start() -> Result<(), rocket::Error> {
    // let config = Config {
    //     port: get_port(),
    //     address: get_addr().parse().unwrap(),
    //     log_level: LogLevel::Debug,
    //     ..Config::debug_default()
    // };

    let http_server = (*SERVER.lock().unwrap()).clone();

    let config = Config {
        port: http_server.port,
        address: http_server.addr.parse().unwrap(),
        log_level: LogLevel::Normal,
        cli_colors: false,
        ..Config::debug_default()
    };

    trace!(
        "[{}]: attempting to start server: {config:?}",
        function_name!()
    );

    let _ = rocket::custom(config)
        .attach(CORS)
        .mount(
            "/",
            routes![
                index,
                client::status,
                client::version,
                shutdown,
                client::host::generate_host_info,
                client::host::get_host_info,
                client::host::get_cooperative_hosts,
                client::change_host_status_id,
                client::change_host_status_name,
                client::try_auth_at_participant,
                client::auth_for_token,
                client::revoke_token,
                client::get_settings,
                //client::logs::get_logs_by_last_entries,
                client::database::new_database,
                client::database::has_table,
                client::database::post_get_databases,
                client::database::get_logical_storage_policy,
                client::database::set_logical_storage_policy,
                client::database::get_active_contact,
                client::database::get_row_id_at_participant,
                client::database::get_data_hash_at_participant,
                client::database::get_data_hash_at_host,
                client::database::participant::add_participant,
                client::database::participant::send_contract_to_participant,
                client::database::participant::get_participants,
                client::database::generate_contract,
                client::database::enable_coooperative_features,
                client::database::actions::accept_pending_action_at_participant,
                client::database::actions::get_pending_actions_at_participant,
                client::database::behavior::change_deletes_to_host_behavior,
                client::database::behavior::change_updates_to_host_behavior,
                client::database::behavior::change_deletes_from_host_behavior,
                client::database::behavior::change_updates_from_host_behavior,
                client::database::behavior::get_deletes_to_host_behavior,
                client::database::behavior::get_updates_to_host_behavior,
                client::database::behavior::get_deletes_from_host_behavior,
                client::database::behavior::get_updates_from_host_behavior,
                client::sql::read_at_host,
                client::sql::write_at_host,
                client::sql::cooperative_write_at_host,
                client::sql::write_at_participant,
                client::sql::read_at_participant,
                client::contract::review_pending_contracts,
                client::contract::accept_pending_contract,
                data::status,
                data::version,
                data::try_auth,
                data::contract::save_contract,
                data::contract::participant_accepts_contract,
                data::io::remove_row_at_participant,
                data::io::notify_host_of_removed_row,
                data::io::update_row_at_participant,
                data::io::insert_row_at_participant,
                data::io::get_row_at_participant,
                data::io::notify_host_of_updated_hash,
            ],
        )
        .manage(http_server)
        .launch()
        .await?;

    Ok(())
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_status(Status::Ok)
    }
}

#[get("/")]
fn index() -> &'static str {
    "Treaty HTTP endpoint online"
}

#[get("/shutdown")]
fn shutdown(shutdown: Shutdown) -> &'static str {
    shutdown.notify();
    let msg = "Shutting down http...";
    info!("{}", msg);
    msg
}
