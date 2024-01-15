use chrono::{DateTime, Utc};
use config::Config;
use repo::{sqlite_repo::SqliteRepo, RepoActions};
use std::{env, fs, path::Path};
use stdext::function_name;
use thiserror::Error;
use tonic::transport::Server;
use tracing::warn;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace};
use treaty::info_service_handler::info_service_handler_actions::InfoServiceHandlerActions;
use treaty::service::handler_builder::HandlerBuilder;
use treaty::service::ServiceBuilder;
use treaty::{
    crypt::{self, hash},
    data_service_handler::data_service_handler_actions::DataServiceHandlerActions,
    jwt::create_jwt,
    service::Service,
    treaty_proto::{
        data_service_server::DataServiceServer, info_service_server::InfoServiceServer,
        user_service_server::UserServiceServer,
    },
    user_service_handler::user_service_handler_actions::UserServiceHandlerActions,
};
use treaty_types::enums::*;
use treaty_types::proxy::server_messages::AuthForTokenReply;
use triggered::{Listener, Trigger};
use user_info::UserInfo;
use uuid::Uuid;

use crate::proxy_grpc::{
    data::ProxyDataServiceHandlerGrpc, info::ProxyInfoServiceHandlerGrpc,
    user::ProxyUserServiceHandlerGrpc,
};
const SETTINGS: &str = "Settings.toml";
const PROXY_DB: &str = "Proxy.db";

mod grpc_client;

mod proxy_grpc;
pub mod proxy_server;
#[doc(hidden)]
pub mod repo;
mod sql_text;
mod user_info;

/*

`treaty-proxy` is a crate designed to emulate multiple Treaty instances as a "proxy" for an actual Treaty instance. It's intended to be a
SaaS-like offering to allow multiple users to host their accounts on one server. This is accomplished by abstracting each Treaty instance
by folder and keeping track of each instance in the `Proxy.db` database. Databases leverage Sqlite or Postgres to persist information.

`treaty-proxy` has two main structs:

TreatyProxy: This struct hosts the gRPC endpoints that enable the original `treaty` UserService and DataService handlers; but abstracted by the Treaty instance's HostId.
    Messages sent to these endpoints must include the HostId of the actual Treaty instance they wish to interact with.

ProxyServer:  This struct hosts HTTP endpoints that is intended for use by the `treaty-my-info` wasm front-end.
    This is the admin endpoint for the actual `treaty-proxy` instance. Messages sent to this endpoint can emulate talking to TreatyProxy by
    means of wrapping the actual protobuf message inside of the `ExecuteRequest` message found in the `server_messages` module. The message
    is denoted by the `RequestType` enum and is expected to be a seralized JSON representation of the message.

*/

#[derive(Error, Debug, PartialEq)]
pub enum TreatyProxyErr {
    #[error("Could not find Settings.toml in dir: `{0}`")]
    SettingsNotFound(String),
    #[error("User already exists: `{0}`")]
    UserAlreadyExists(String),
    #[error("Db Error: `{0}`")]
    DbError(String),
    #[error("Folder already exists: `{0}`")]
    FolderAlreadyExists(String),
    #[error("User `{0}` not found")]
    UserNotFound(String),
    #[error("Host Id `{0}` not found")]
    HostIdNotFound(String),
    #[error("No rows affected")]
    NoRowsAffected,
    #[error("User `{0}` folder not set")]
    UserFolderNotSet(String),
}

#[derive(Debug, Clone)]
pub struct TreatyProxy {
    settings: TreatyProxySettings,
    db: TreatyProxyDbBuilder,
}

#[derive(Debug, Clone)]
pub struct TreatyProxyDbBuilder {
    settings: TreatyProxySettings,
}

impl TreatyProxyDbBuilder {
    pub fn new(settings: &TreatyProxySettings) -> Self {
        Self {
            settings: settings.clone(),
        }
    }

    pub fn build(&self) -> impl RepoActions {
        let db_type = self.settings.database_type;
        let db_name = self.settings.database_name.clone();
        let dir = self.settings.root_dir.clone();

        match db_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => SqliteRepo::new(&dir, &db_name),

            DatabaseType::Postgres => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
struct GrpcServiceSettings {
    pub root_folder: String,
    pub database_name: String,
    pub addr_port: String,
    pub own_db_addr_port: String,
    pub proxy: TreatyProxy,
}

#[derive(Debug, Clone)]
pub struct TreatyProxySettings {
    pub use_grpc: bool,
    pub use_http: bool,
    pub grpc_user_addr_port: String,
    pub grpc_db_addr_port: String,
    pub grpc_info_addr_port: String,
    pub http_ip: String,
    pub http_port: usize,
    pub root_dir: String,
    pub database_type: DatabaseType,
    pub database_name: String,
    // for the webpage to talk to this instance
    pub proxy_http_addr: String,
    pub proxy_http_port: usize,
    pub send_user_port_number: bool,
    pub send_info_port_number: bool,
    pub send_data_port_number: bool,
    pub jwt_timeout_in_minutes: u32,
}

impl TreatyProxy {
    pub fn db(&self) -> impl RepoActions {
        self.db.build()
    }

    pub fn get_proxy_with_config(settings: TreatyProxySettings) -> Self {
        let db = TreatyProxyDbBuilder {
            settings: settings.clone(),
        };
        TreatyProxy { settings, db }
    }

    pub fn http_endpoint_addr(&self) -> String {
        self.settings.proxy_http_addr.clone()
    }

    pub fn http_endpoint_port(&self) -> u16 {
        self.settings.proxy_http_port as u16
    }

    pub fn get_proxy_from_config_with_dir(
        config_dir: &str,
        root_dir: &str,
    ) -> Result<Self, TreatyProxyErr> {
        let result_settings = Self::get_settings(config_dir);

        match result_settings {
            Ok(mut settings) => {
                settings.root_dir = root_dir.to_string();

                let db = TreatyProxyDbBuilder {
                    settings: settings.clone(),
                };
                let service = TreatyProxy { settings, db };

                Ok(service)
            }
            Err(e) => Err(e),
        }
    }

    /// reads the specified directory's Settings.toml and returns
    /// a new instance of a treatyProxy
    pub fn get_proxy_from_config(dir: &str) -> Result<Self, TreatyProxyErr> {
        let result_settings = Self::get_settings(dir);
        match result_settings {
            Ok(settings) => {
                let db = TreatyProxyDbBuilder {
                    settings: settings.clone(),
                };
                let service = TreatyProxy { settings, db };
                Ok(service)
            }
            Err(e) => Err(e),
        }
    }

    fn get_settings(dir: &str) -> Result<TreatyProxySettings, TreatyProxyErr> {
        let config = Path::new(&dir).join(SETTINGS);

        debug!("[{}]: {config:?}", function_name!());

        if !Path::exists(&config) {
            return Err(TreatyProxyErr::SettingsNotFound(dir.to_string()));
        }

        let config = config.to_str().unwrap();

        let settings = Config::builder()
            .add_source(config::File::with_name(config))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .expect("Could not find {SETTINGS} file in {dir}");

        let result_db_name = settings.get_string("backing_database_name");

        let db_name: String = match result_db_name {
            Ok(name) => name,
            Err(_) => {
                error!("missing setting: 'backing_database_name', using default {PROXY_DB}");
                PROXY_DB.to_string()
            }
        };

        let result_client_addr = settings.get_string("client_addr_port");

        let client_addr = match result_client_addr {
            Ok(addr) => addr,
            Err(_) => {
                error!("missing setting: 'client_addr_port', using default 127.0.0.1:50051");
                "127.0.0.1:50051".to_string()
            }
        };

        let result_db_addr = settings.get_string("db_addr_port");

        let db_addr = match result_db_addr {
            Ok(addr) => addr,
            Err(_) => {
                error!("missing setting: 'db_addr_port', using default 127.0.0.1:50052");
                "127.0.0.1:50052".to_string()
            }
        };

        let result_info_addr = settings.get_string("info_addr_port");

        let info_addr = match result_info_addr {
            Ok(addr) => addr,
            Err(_) => {
                error!("missing setting: 'info_addr_port', using default 127.0.0.1:50059");
                "127.0.0.1:50059".to_string()
            }
        };

        let result_http_addr = settings.get_string("http_addr");

        let http_addr = match result_http_addr {
            Ok(addr) => addr,
            Err(_) => {
                error!("missing setting: 'http_addr', using default 127.0.0.1");
                "127.0.0.1".to_string()
            }
        };

        let result_http_port = settings.get_string("http_port");

        let http_port = match result_http_port {
            Ok(port) => port,
            Err(_) => {
                error!("missing setting: 'http_port',  using default 50055");
                "50055".to_string()
            }
        };

        let result_proxy_http_addr = settings.get_string("proxy_http_addr");

        let proxy_http_addr = match result_proxy_http_addr {
            Ok(addr) => addr,
            Err(_) => {
                error!("missing setting: 'http_addr', using default 127.0.0.1");
                "127.0.0.1".to_string()
            }
        };

        let result_proxy_http_port = settings.get_string("proxy_http_port");

        let proxy_http_port = match result_proxy_http_port {
            Ok(port) => port,
            Err(_) => {
                error!("missing setting: 'http_port',  using default 50055");
                "50055".to_string()
            }
        };

        let result_db_type = settings.get_string("database_type");

        let db_type = match result_db_type {
            Ok(x) => x,
            Err(_) => {
                error!("missing setting: 'database_type', using default 1 ");
                "1".to_string()
            }
        };

        let db_type = DatabaseType::from_u32(db_type.parse().unwrap());

        let s_jwt_valid_time_in_minutes: String = settings
            .get_string(&String::from("jwt_valid_time_in_minutes"))
            .unwrap();

        let jwt_valid_time_in_minutes: u32 = match s_jwt_valid_time_in_minutes.parse() {
            Ok(timeout) => timeout,
            Err(_) => {
                error!("Could not parse: {s_jwt_valid_time_in_minutes:?}, defaulting to 20");
                20
            }
        };

        Ok(TreatyProxySettings {
            use_grpc: true,
            use_http: true,
            grpc_user_addr_port: client_addr,
            grpc_db_addr_port: db_addr,
            http_ip: http_addr,
            http_port: http_port.parse().unwrap(),
            root_dir: dir.to_string(),
            database_type: db_type,
            database_name: db_name,
            proxy_http_addr,
            proxy_http_port: proxy_http_port.parse().unwrap(),
            grpc_info_addr_port: info_addr,
            send_user_port_number: false,
            send_info_port_number: true,
            send_data_port_number: true,
            jwt_timeout_in_minutes: jwt_valid_time_in_minutes,
        })
    }

    pub fn output_settings(&self) {
        let settings = &self.settings.clone();
        info!("{settings:?}");
    }

    /// initalizes the backing database
    pub fn start(&self) {
        let version = env!("CARGO_PKG_VERSION");
        info!("treaty-proxy version: {}", version);

        self.db().config();
    }

    pub fn start_server() {
        todo!()
    }

    pub async fn start_grpc_data_at_addr(&self, addr: &str) {
        let (_, client_listener) = triggered::trigger();

        let client = ProxyDataServiceHandlerGrpc::new(
            self.settings.root_dir.clone(),
            self.settings.database_name.clone(),
            addr.to_string(),
            self.clone(),
        );

        let service = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        info!("Data Proxy Service Starting At: {addr}");

        let addr = addr.parse().unwrap();

        Server::builder()
            .add_service(DataServiceServer::new(client))
            .add_service(service)
            .serve_with_shutdown(addr, client_listener)
            .await
            .unwrap();

        info!("Data Proxy Service Ending...");
    }

    async fn start_grpc_data_with_settings(settings: GrpcServiceSettings, listener: Listener) {
        let client = ProxyDataServiceHandlerGrpc::new(
            settings.root_folder.clone(),
            settings.database_name.clone(),
            settings.addr_port.clone(),
            settings.proxy.clone(),
        );

        let addr = settings.addr_port.clone();

        let service = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        info!("Data Proxy Service Starting At: {addr}");

        let addr = addr.parse().unwrap();

        Server::builder()
            .add_service(DataServiceServer::new(client))
            .add_service(service)
            .serve_with_shutdown(addr, listener)
            .await
            .unwrap();

        info!("Data Proxy Service Ending...");
    }

    async fn start_grpc_user_service_with_settings(
        settings: GrpcServiceSettings,
        listener: Listener,
    ) {
        let client = ProxyUserServiceHandlerGrpc::new(
            settings.root_folder.clone(),
            settings.database_name.clone(),
            settings.addr_port.clone(),
            settings.own_db_addr_port.to_string(),
            settings.proxy.clone(),
        );

        let addr = settings.addr_port.clone();

        let service = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        info!("Client Proxy Service Starting At: {addr}");

        let addr = addr.parse().unwrap();

        Server::builder()
            .add_service(UserServiceServer::new(client))
            .add_service(service)
            .serve_with_shutdown(addr, listener)
            .await
            .unwrap();

        info!("Cient Proxy Service Ending...");
    }

    async fn start_grpc_info_service_with_settings(
        settings: GrpcServiceSettings,
        listener: Listener,
        info_addr_port: &str,
    ) {
        let client = ProxyInfoServiceHandlerGrpc::new(
            settings.root_folder.clone(),
            settings.database_name.clone(),
            settings.addr_port.clone(),
            info_addr_port.to_string(),
            settings.own_db_addr_port.to_string(),
            settings.proxy.clone(),
        );

        let addr = info_addr_port;

        let service = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        info!("Info Proxy Service Starting At: {addr}");

        let addr = addr.parse().unwrap();

        Server::builder()
            .add_service(InfoServiceServer::new(client))
            .add_service(service)
            .serve_with_shutdown(addr, listener)
            .await
            .unwrap();

        info!("Info Proxy Service Ending...");
    }

    pub async fn start_grpc_info_client_with_trigger(&self) -> Trigger {
        let (trigger, listener) = triggered::trigger();

        let settings = GrpcServiceSettings {
            root_folder: self.settings.root_dir.clone(),
            database_name: self.settings.database_name.clone(),
            addr_port: self.settings.grpc_user_addr_port.clone(),
            own_db_addr_port: self.settings.grpc_db_addr_port.clone(),
            proxy: self.clone(),
        };

        let info_addr = self.settings.grpc_info_addr_port.clone();

        info!("Client Proxy Info Service Starting At: {info_addr}");

        tokio::spawn(async move {
            Self::start_grpc_info_service_with_settings(settings, listener, &info_addr).await
        });

        trigger
    }

    pub async fn start_grpc_user_client_with_trigger(&self) -> Trigger {
        let (trigger, listener) = triggered::trigger();

        let settings = GrpcServiceSettings {
            root_folder: self.settings.root_dir.clone(),
            database_name: self.settings.database_name.clone(),
            addr_port: self.settings.grpc_user_addr_port.clone(),
            own_db_addr_port: self.settings.grpc_db_addr_port.clone(),
            proxy: self.clone(),
        };

        tokio::spawn(async move {
            Self::start_grpc_user_service_with_settings(settings, listener).await
        });

        trigger
    }

    pub async fn start_grpc_data_with_trigger(&self) -> Trigger {
        let (trigger, listener) = triggered::trigger();

        let settings = GrpcServiceSettings {
            root_folder: self.settings.root_dir.clone(),
            database_name: self.settings.database_name.clone(),
            addr_port: self.settings.grpc_db_addr_port.clone(),
            own_db_addr_port: self.settings.grpc_db_addr_port.clone(),
            proxy: self.clone(),
        };

        tokio::spawn(async move { Self::start_grpc_data_with_settings(settings, listener).await });

        trigger
    }

    pub async fn start_grpc_client(&self) -> Trigger {
        self.start_grpc_user_client_with_trigger().await
    }

    pub async fn start_grpc_data(&self) -> Trigger {
        self.start_grpc_data_with_trigger().await
    }

    pub async fn start_grpc_info(&self) -> Trigger {
        self.start_grpc_info_client_with_trigger().await
    }

    pub fn revoke_tokens_for_login(&self, un: &str) {
        self.db().revoke_tokens_for_login(un);
    }

    pub fn auth_for_token(&self, un: &str, pw: &str) -> Result<AuthForTokenReply, TreatyProxyErr> {
        if self.verify_login(un, pw)? {
            if self.db().login_has_token(un) {
                self.db().revoke_tokens_for_login(un);
                debug!("revoked existing tokens for login {un}");
            }

            let token_data = self.create_token_for_login(un);
            let jwt = token_data.0;
            let expiration_utc = token_data.1.to_string();

            let u = self.db().get_user(un)?;

            return Ok(AuthForTokenReply {
                is_successful: true,
                expiration_utc: Some(expiration_utc),
                jwt: Some(jwt),
                id: u.id,
            });
        }

        Ok(AuthForTokenReply {
            is_successful: false,
            expiration_utc: None,
            jwt: None,
            id: None,
        })
    }

    fn create_token_for_login(&self, login: &str) -> (String, DateTime<Utc>) {
        let token_data = create_jwt("treaty-proxy", login, self.settings.jwt_timeout_in_minutes);
        self.db()
            .save_token(login, &token_data.0, token_data.1)
            .unwrap();
        token_data
    }

    pub fn verify_login(&self, un: &str, pw: &str) -> Result<bool, TreatyProxyErr> {
        if self.db().has_user(un) {
            let u = self.db().get_user(un)?;

            let mut padded = [0u8; 128];
            u.hash.iter().enumerate().for_each(|(i, val)| {
                padded[i] = *val;
            });

            return Ok(crypt::verify(padded, pw));
        } else {
            warn!("Proxy user not found: {}", un);
        }
        Ok(false)
    }

    pub fn verify_token(&self, jwt: &str) -> Result<bool, TreatyProxyErr> {
        Ok(self.db().verify_token(jwt))
    }

    /// checks to see if the specified user name already exists
    /// if not, it will save the un, hash the pw, and init
    /// a new `treaty` directory for the account and init the `treaty` instance
    pub fn register_user(&self, un: &str, pw: &str) -> Result<(), TreatyProxyErr> {
        let hash = hash(pw);
        self.db().register_user(un, &hash.0)
    }

    pub fn get_host_id_for_token(&self, token: &str) -> Result<Option<String>, TreatyProxyErr> {
        if self.db().verify_token(token) {
            let u = self.db().get_user_with_token(token)?;
            return Ok(u.id);
        }

        Ok(None)
    }

    pub fn get_host_id_for_user(&self, un: &str) -> Result<Option<String>, TreatyProxyErr> {
        let u = self.db().get_user(un)?;
        Ok(u.id)
    }

    /// sets up the treaty instnce for the user. intended to be called after `register_user`
    pub async fn create_treaty_instance(
        &self,
        un: &str,
        overwrite_existing: bool,
    ) -> Result<String, TreatyProxyErr> {
        let full_folder_path = self.setup_user_folder(overwrite_existing)?;
        let host_id = self.setup_treaty_service(un, &full_folder_path).await?;

        let mut u = self.db().get_user(un)?;
        u.id = Some(host_id.clone());

        if u.folder.is_none() {
            u.folder = Some(full_folder_path);
        }

        self.db().update_user(&u)?;

        trace!("create_treaty_instance: {u:?}");

        Ok(host_id)
    }

    /// sets up a brand new treaty service for the specified user and updates the treaty folder for this user
    /// intended to be called after a user is registered
    pub async fn setup_treaty_service(
        &self,
        un: &str,
        full_folder_path: &str,
    ) -> Result<String, TreatyProxyErr> {
        trace!(
            "[{}]: un: {} dir: {}",
            function_name!(),
            un,
            full_folder_path
        );

        let settings = self.get_default_treaty_settings(un);

        let mut u = self.db().get_user(un)?;

        if u.folder.is_none() {
            u.folder = Some(full_folder_path.to_string());
            self.db().update_user(&u).unwrap();
        }

        let mut service = if settings.database_type == DatabaseType::Sqlite {
            ServiceBuilder::build_from_settings(&settings, Some(full_folder_path.to_string()))
                .unwrap()
        } else {
            Service::new(&settings)
        };

        // trace!("[{}]: {service:?}", function_name!());

        service.init_db_with_admin(un, u.hash.clone()).await;
        service.warn_init_host_info().await;

        let host_id = service.host_id().await;

        if u.id.is_none() {
            u.id = Some(host_id.clone());
            self.db().update_user(&u).unwrap();
        }

        trace!("[{}]: {u:?}", function_name!());

        Ok(host_id)
    }

    pub async fn get_treaty_service_for_existing_user(
        &self,
        un: &str,
    ) -> Result<Service, TreatyProxyErr> {
        let u = self.db().get_user(un)?;
        let full_folder_path = self.get_user_root_dir(&u)?;

        let settings = self.get_default_treaty_settings(un);
        let service = if settings.database_type == DatabaseType::Sqlite {
            ServiceBuilder::build_from_settings(&settings, Some(full_folder_path.to_string()))
                .unwrap()
        } else {
            Service::new(&settings)
        };
        service.init_db().await;
        Ok(service)
    }

    pub async fn get_treaty_data_handler(
        &self,
        id: &str,
    ) -> Result<Box<dyn DataServiceHandlerActions + Send + Sync>, TreatyProxyErr> {
        let service = self.get_treaty_service_for_existing_host(id)?;
        let service =
            HandlerBuilder::build_data(service.root().as_deref(), &service.settings()).await;
        Ok(service)
    }

    pub async fn get_treaty_info_handler(
        &self,
        id: &str,
    ) -> Result<Box<dyn InfoServiceHandlerActions + Send + Sync>, TreatyProxyErr> {
        let service = self.get_treaty_service_for_existing_host(id)?;
        let service =
            HandlerBuilder::build_info(service.root().as_deref(), &service.settings()).await;
        Ok(service)
    }

    pub async fn get_treaty_grpc_user_handler(
        &self,
        id: &str,
    ) -> Result<Box<dyn UserServiceHandlerActions + Send + Sync>, TreatyProxyErr> {
        let service = self.get_treaty_service_for_existing_host(id)?;
        let service = HandlerBuilder::build_user(
            service.root().as_deref(),
            &service.settings(),
            CommunicationType::Grpc,
        )
        .await;
        Ok(service)
    }

    pub fn get_treaty_service_for_existing_host(
        &self,
        id: &str,
    ) -> Result<Service, TreatyProxyErr> {
        let u = self.db().get_host(id)?;
        let full_folder_path = self.get_user_root_dir(&u)?;
        let settings = self.get_default_treaty_settings(&u.username);
        let service = if settings.database_type == DatabaseType::Sqlite {
            ServiceBuilder::build_from_settings(&settings, Some(full_folder_path.to_string()))
                .unwrap()
        } else {
            Service::new(&settings)
        };
        Ok(service)
    }

    fn get_default_treaty_settings(&self, un: &str) -> treaty::settings::Settings {
        treaty::settings::Settings {
            admin_un: un.to_string(),
            admin_pw: "".to_string(),
            database_type: DatabaseType::Sqlite,
            backing_database_name: Some("treaty.db".to_string()),
            grpc_user_service_addr_port: self.settings.grpc_user_addr_port.clone(),
            grpc_data_service_addr_port: self.settings.grpc_db_addr_port.clone(),
            user_grpc_timeout_in_seconds: 60,
            data_grpc_timeout_in_seconds: 60,
            http_addr: self.settings.http_ip.clone(),
            http_port: self.settings.http_port as u16,
            use_grpc: true,
            use_http: false,
            override_ip_with_local: false,
            grpc_info_service_addr_port: self.settings.grpc_info_addr_port.clone(),
            info_grpc_timeout_in_seconds: 60,
            send_user_port_number: false,
            send_data_port_number: true,
            send_info_port_number: true,
            jwt_valid_time_in_minutes: 20,
            postgres_settings: None,
            use_tls: false,
            tls_options: None,
            tls_http_options: None,
        }
    }

    fn get_user_root_dir(&self, u: &UserInfo) -> Result<String, TreatyProxyErr> {
        trace!("[{}]: {u:?}", function_name!());

        if u.folder.as_ref().is_none() {
            return Err(TreatyProxyErr::UserFolderNotSet(u.username.clone()));
        }

        return Ok(Path::new(&self.settings.root_dir)
            .join(u.folder.as_ref().unwrap())
            .to_str()
            .unwrap()
            .to_string());
    }

    pub fn setup_user_folder(&self, overwrite_existing: bool) -> Result<String, TreatyProxyErr> {
        let folder_id = Uuid::new_v4().to_string();
        let folder_path = Path::new(&self.settings.root_dir).join(folder_id);

        if Path::exists(&folder_path) && !overwrite_existing {
            return Err(TreatyProxyErr::FolderAlreadyExists(
                folder_path.to_str().unwrap().to_string(),
            ));
        }

        if folder_path.exists() && overwrite_existing {
            fs::remove_dir_all(&folder_path).unwrap();
        }

        fs::create_dir_all(&folder_path).unwrap();

        Ok(folder_path.to_str().unwrap().to_string())
    }
}

pub fn get_test_temp_dir(test_name: &str) -> String {
    let dir = env::temp_dir();
    let tmp = dir.as_os_str().to_str().unwrap();
    let path = Path::new(&tmp).join("TREATY_TESTS").join(test_name);

    if path.exists() {
        fs::remove_dir_all(&path).unwrap();
    }

    fs::create_dir_all(&path).unwrap();

    return path.as_path().to_str().unwrap().to_string();
}

#[test]
fn test_output_settings() {
    use std::env;

    let cwd = env::current_dir().unwrap().to_str().unwrap().to_string();
    let proxy = TreatyProxy::get_proxy_from_config(&cwd).unwrap();
    proxy.output_settings();
}

#[test]
pub fn test_new_with_sqlite() {
    let proxy = test_setup("treaty-proxy-db-unit-test-new");
    let result = proxy.register_user("test", "1234");
    assert_eq!(result, Ok(()));
}

#[test]
pub fn test_register_twice() {
    let proxy = test_setup("treaty-proxy-db-unit-test-register");

    let first_result = proxy.register_user("test", "1234");
    let second_result = proxy.register_user("test", "1234");

    assert_eq!(first_result, Ok(()));
    assert_eq!(
        second_result,
        Err(TreatyProxyErr::UserAlreadyExists(
            "User 'test' already exists".to_string()
        ))
    );
}

#[tokio::test]
pub async fn test_register_and_setup_user_and_host() {
    let proxy = test_treaty_common_setup("treaty-proxy-unit-test-reg-setup-run-host")
        .await
        .unwrap();
    let service = proxy
        .get_treaty_service_for_existing_user("test")
        .await
        .unwrap();
    let host_id = service.host_id().await;
    debug!("{host_id:?}");
    assert!(!host_id.is_empty());

    let service = proxy
        .get_treaty_service_for_existing_host(&host_id)
        .unwrap();
    let id = service.host_id().await;
    assert_eq!(host_id, id);
}

#[tokio::test]
pub async fn test_register_and_setup_user() {
    let proxy = test_treaty_common_setup("treaty-proxy-unit-test-reg-setup-run")
        .await
        .unwrap();
    let service = proxy
        .get_treaty_service_for_existing_user("test")
        .await
        .unwrap();
    let host_id = service.host_id().await;
    debug!("{host_id:?}");
    assert!(!host_id.is_empty());
}

#[tokio::test]
pub async fn test_get_treaty_for_user() {
    test_treaty_common_setup("treaty-proxy-unit-test-get-treaty-for-user")
        .await
        .unwrap();
    assert!(true);
}

#[cfg(test)]
/// common test code - sets up a test folder and returns a treaty proxy
pub fn test_setup(test_name: &str) -> TreatyProxy {
    let root_dir = get_test_temp_dir(test_name);
    let config_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let proxy = TreatyProxy::get_proxy_from_config_with_dir(&config_dir, &root_dir).unwrap();
    proxy.start();
    proxy
}

#[cfg(test)]
/// common setup code - sets up the proxy instance and then returns an treaty service for the "test" user
pub async fn test_treaty_common_setup(test_name: &str) -> Option<TreatyProxy> {
    let proxy = test_setup(test_name);
    let result_register = proxy.register_user("test", "1234");

    if result_register.is_err() {
        assert!(false);
    }

    let result_setup = proxy.setup_user_folder(false);

    match result_setup {
        Ok(root_dir) => {
            let result_setup_treaty = proxy.setup_treaty_service("test", &root_dir).await;

            match result_setup_treaty {
                Ok(host_id) => {
                    debug!("{host_id:?}");
                    assert!(!host_id.is_empty());
                    return Some(proxy);
                }
                Err(_) => assert!(false),
            }
        }
        Err(_) => assert!(false),
    }

    None
}
