use std::pin::Pin;

use crate::info_service_handler::info_service_handler_actions::InfoServiceHandlerActions;
use crate::service::handler_builder::HandlerBuilder;
use crate::settings::Settings;
use rocket::config::{Ident, TlsConfig};
use rocket::fairing::Kind;
use rocket::http::uri::Origin;
use rocket::http::{Header, Method};
use rocket::log::LogLevel;
use rocket::request::{FromRequest, Outcome};
use rocket::{
    fairing::{Fairing, Info},
    get,
    http::Status,
};
use rocket::{routes, Config, Shutdown};
use rocket::{Request, Response};
use tokio::runtime::Handle;

use tracing::{info, trace, warn};
use treaty_http_endpoints::headers::{
    TREATY_AUTH_HEADER, TREATY_AUTH_HEADER_AUTHOR, TREATY_AUTH_HEADER_BASIC,
    TREATY_AUTH_HEADER_METADATA, TREATY_AUTH_HEADER_WEB_TOKEN,
};
use treaty_types::auth::AuthRequestData;
use treaty_types::types::treaty_proto::{
    AuthRequestAuthor, AuthRequestBasic, AuthRequestBinary, AuthRequestMetadata,
    AuthRequestWebToken,
};

use crate::data_service_handler::data_service_handler_actions::DataServiceHandlerActions;

use crate::user_service_handler::user_service_handler_actions::UserServiceHandlerActions;
use crate::{
    data_service_handler::DataServiceHandler, db_interface::dbi_actions::DbiActions,
    remote::remote_actions::RemoteActions, user_service_handler::UserServiceHandler,
};

use stdext::function_name;

pub(crate) mod client;
pub(crate) mod data;
pub(crate) mod info;

#[derive(Debug, Clone)]
pub struct ApiToken<'r>(&'r str);

impl ApiToken<'_> {
    pub fn jwt(&self) -> String {
        self.0.replace("Bearer ", "").trim().to_string()
    }

    pub fn creds(&self) -> (String, String) {
        let creds = self.0.replace("Basic", "");
        let creds = creds.trim();
        let parts = creds.split(':');
        let parts = parts.collect::<Vec<&str>>();
        if parts.len() == 2 {
            let login = parts[0];
            let pw = parts[1];
            (login.to_string(), pw.to_string())
        } else {
            (String::from(""), String::from(""))
        }
    }
}

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiToken<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            if key.is_empty() {
                return false;
            }

            if key.contains("Bearer") || key.contains("Basic") {
                return true;
            }

            false
        }

        match req.headers().get_one("Authorization") {
            None => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiToken(key)),
            Some(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}

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

pub struct HttpServer {
    pub dir: Option<String>,
    pub settings: Settings,
    pub addr: String,
    pub port: u16,
    pub option_handle: Option<Handle>,
    user: Pin<Box<dyn UserServiceHandlerActions + Send + Sync>>,
    data: Pin<Box<dyn DataServiceHandlerActions + Send + Sync>>,
    info: Pin<Box<dyn InfoServiceHandlerActions + Send + Sync>>,
}

impl HttpServer {
    pub async fn new(
        dir: Option<String>,
        settings: Settings,
        addr: &str,
        port: u16,
        option_handle: Option<Handle>,
    ) -> Self {
        let data = HandlerBuilder::build_data_pin(dir.as_deref(), &settings).await;

        let user = HandlerBuilder::build_user_pin(
            dir.as_deref(),
            &settings,
            treaty_types::enums::CommunicationType::Http,
        )
        .await;

        let info = HandlerBuilder::build_info_pin(dir.as_deref(), &settings).await;

        Self {
            dir,
            settings,
            addr: addr.into(),
            port,
            option_handle,
            user,
            data,
            info,
        }
    }

    pub async fn user(&self) -> &Pin<Box<dyn UserServiceHandlerActions + Send + Sync>> {
        &self.user
    }

    pub async fn data(&self) -> &Pin<Box<dyn DataServiceHandlerActions + Send + Sync>> {
        &self.data
    }

    pub async fn info(&self) -> &Pin<Box<dyn InfoServiceHandlerActions + Send + Sync>> {
        &self.info
    }
}

pub async fn start_http(http_server: HttpServer) {
    // We should change this such that the HTTP server when starting
    // constructs the handler services (such as user/data/info)
    // rather than having our treaty service construct it and then pass
    // it to the server.

    trace!(
        "[{}]: Transport attempting to start HttpServer",
        function_name!()
    );

    let opt_handle = http_server.option_handle.clone();

    trace!("[{}]: Server has been set.", function_name!());
    trace!("[{}]: Server handle: {opt_handle:?}", function_name!());

    if let Some(handle) = opt_handle {
        trace!(
            "[{}]: attempting to start server on existing handle: {handle:?}",
            function_name!()
        );

        let flavor = handle.runtime_flavor();
        trace!("[{}]: Runtime flavor: {flavor:?}", function_name!());

        handle.spawn(async move {
            trace!("[{}]: Spawning HTTP using handle", function_name!());
            start(http_server).await.unwrap();
        });
    } else {
        trace!(
            "[{}]: attempting to start server without handle",
            function_name!()
        );
        tokio::spawn(async move {
            trace!("[{}]: Spawning HTTP", function_name!());
            start(http_server).await.unwrap();
        });
    }
}

async fn start(http_server: HttpServer) -> Result<(), rocket::Error> {
    trace!(
        "[{}]: http_server addr: {}",
        function_name!(),
        http_server.addr
    );

    let http_server_addr = if http_server.addr.ends_with(":") {
        http_server.addr.replace(":", "")
    } else {
        http_server.addr.clone()
    };

    let config = if http_server.settings.use_tls {
        trace!("[{}]: Server is starting with TLS", function_name!());

        let settings = http_server.settings.tls_options.as_ref().unwrap().clone();
        let cert_path = settings.tls_cert_path.as_ref().unwrap().clone();
        let key_path = settings.tls_key_path.as_ref().unwrap().clone();

        let tls_config = TlsConfig::from_paths(cert_path, key_path);
        let identity = Ident::try_new("treaty").unwrap();

        let config = Config {
            port: http_server.port,
            address: http_server_addr.parse().unwrap(),
            log_level: LogLevel::Normal,
            tls: Some(tls_config),
            cli_colors: false,
            ident: identity,
            ..Config::debug_default()
        };
        config
    } else {
        let config = Config {
            port: http_server.port,
            address: http_server_addr.parse().unwrap(),
            log_level: LogLevel::Normal,
            cli_colors: false,
            ..Config::debug_default()
        };
        config
    };

    let user_service = HandlerBuilder::build_user_pin(
        http_server.dir.as_deref(),
        &http_server.settings,
        treaty_types::enums::CommunicationType::Http,
    )
    .await;

    trace!(
        "[{}]: attempting to start rocket server: {config:?}",
        function_name!()
    );

    let _ = rocket::custom(config)
        .attach(CORS)
        .attach(TreatyAuth { user_service })
        .mount(
            "/",
            routes![
                index,
                client::status,
                client::version,
                shutdown,
                unauthorized,
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
                client::db_type,
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
                client::database::delete_database_forcefully,
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
                info::save_contract,
                info::participant_accepts_contract,
                info::auth_for_token,
                info::info_try_auth_web_token,
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

pub struct TreatyAuth {
    user_service: Pin<Box<dyn UserServiceHandlerActions + Send + Sync>>,
}

fn get_auth_data_from_request(request: &rocket::Request<'_>) -> AuthRequestData {
    let mut request_data: AuthRequestData = AuthRequestData {
        headers: Vec::new(),
        basic: None,
        web_token: None,
        binary: None,
        metadata: None,
        author: None,
    };

    if request.headers().contains(TREATY_AUTH_HEADER_BASIC) {
        request_data
            .headers
            .push(TREATY_AUTH_HEADER_BASIC.to_string());

        let value = request.headers().get_one(TREATY_AUTH_HEADER_BASIC);
        if let Some(v) = value {
            let basic: AuthRequestBasic = serde_json::from_str(v).unwrap();
            request_data.basic = Some(basic);
            trace!("[{}]: Basic auth header found", function_name!());
        }
    } else {
        trace!("[{}]: No Basic Auth header found", function_name!());
    }

    if request.headers().contains(TREATY_AUTH_HEADER_WEB_TOKEN) {
        request_data
            .headers
            .push(TREATY_AUTH_HEADER_WEB_TOKEN.to_string());

        let value = request.headers().get_one(TREATY_AUTH_HEADER_WEB_TOKEN);
        if let Some(v) = value {
            let web_token: AuthRequestWebToken = serde_json::from_str(v).unwrap();
            request_data.web_token = Some(web_token);
            trace!("[{}]: Web Token header found", function_name!());
        }
    } else {
        trace!("[{}]: No Web Token header found", function_name!());
    }

    if request.headers().contains(TREATY_AUTH_HEADER) {
        request_data.headers.push(TREATY_AUTH_HEADER.to_string());

        let value = request.headers().get_one(TREATY_AUTH_HEADER);
        if let Some(v) = value {
            let binary: AuthRequestBinary = serde_json::from_str(v).unwrap();
            request_data.binary = Some(binary);
            trace!("[{}]: Auth bin header found", function_name!());
        }
    } else {
        trace!("[{}]: No Auth bin header found", function_name!());
    }

    if request.headers().contains(TREATY_AUTH_HEADER_AUTHOR) {
        request_data
            .headers
            .push(TREATY_AUTH_HEADER_AUTHOR.to_string());

        let value = request.headers().get_one(TREATY_AUTH_HEADER_AUTHOR);
        if let Some(v) = value {
            let author: AuthRequestAuthor = serde_json::from_str(v).unwrap();
            request_data.author = Some(author);
            trace!("[{}]: Author header found", function_name!());
        }
    } else {
        trace!("[{}]: No Author header found", function_name!());
    }

    if request.headers().contains(TREATY_AUTH_HEADER_METADATA) {
        request_data
            .headers
            .push(TREATY_AUTH_HEADER_AUTHOR.to_string());

        let value = request.headers().get_one(TREATY_AUTH_HEADER_METADATA);
        if let Some(v) = value {
            let metadata: AuthRequestMetadata = serde_json::from_str(v).unwrap();
            request_data.metadata = Some(metadata);
            trace!("[{}]: Metadata header found", function_name!());
        }
    } else {
        trace!("[{}]: No Metadata header found", function_name!());
    }

    request_data
}

#[rocket::async_trait]
impl Fairing for TreatyAuth {
    fn info(&self) -> Info {
        Info {
            name: "Inspect for Authentication",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut rocket::Request<'_>, _data: &mut rocket::Data<'_>) {
        let path = req.uri().path().to_string();
        trace!("[{}]: PATH: {path:?}", function_name!());
        if path.contains("/info/") || path.contains("/shutdown") {
            trace!("Ignoring authorization for /info or /shutdown service request.");
            return;
        } else {
            let auth_data = get_auth_data_from_request(req);
            let is_authenticated = self.user_service.authenticate_request(&auth_data).await;

            if is_authenticated {
                trace!("[{}]: Caller is authenticated.", function_name!());
                return;
            }
        }

        warn!("Caller is NOT authenticated");
        let u = Origin::parse("/unauthorized").unwrap();
        req.set_uri(u);
        req.set_method(Method::Get);
    }
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

#[get("/unauthorized")]
fn unauthorized() -> &'static str {
    warn!("The caller was not authorized.");
    "Unauthorized"
}
