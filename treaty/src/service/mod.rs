use std::thread;

use crate::alt_transport::http::{start_http, HttpServer};
use crate::data_service_handler::DataServiceHandler;
use crate::db_interface::dbi::DbInterface;
use crate::db_interface::dbi_actions::DbiActions;
use crate::db_interface::postgres::PostgresDbi;
use crate::db_interface::sqlite::SqliteDbi;
use crate::info_service_handler::InfoServiceHandler;
use local_ip_address::local_ip;
use tokio::runtime::Handle;
use treaty_types::enums::*;

use crate::remote::remote_grpc::RemoteGrpc;
use crate::remote::Remote;
use crate::user_service_handler::UserServiceHandler;
use crate::{settings::Settings, treaty_proto::TreatyError};
use guid_create::GUID;
use stdext::function_name;
use tracing::{debug, error, trace, warn};
use triggered::Trigger;

pub mod handler_builder;

/// A struct that contains triggers to shutdown a Treaty service, depending on the type of communications configured for that service
#[derive(Debug, Clone)]
pub struct ServiceShutdown {
    pub grpc_user: Option<Trigger>,
    pub grpc_data: Option<Trigger>,
    pub grpc_info: Option<Trigger>,
    pub http: Option<Trigger>,
}

impl ServiceShutdown {
    /// Attempts to shutdown all services
    pub fn all(&self) {
        if let Some(shutdown) = &self.grpc_user {
            shutdown.trigger();
        }

        if let Some(shutdown) = &self.grpc_data {
            shutdown.trigger()
        }

        if let Some(shutdown) = &self.http {
            shutdown.trigger();
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServiceBuilder {}

impl ServiceBuilder {
    pub fn build_from_settings(
        settings: &Settings,
        root_dir: Option<String>,
    ) -> Result<Service, String> {
        if settings.database_type == DatabaseType::Sqlite && root_dir.is_none() {
            return Err("Sqlite database requires a root directory".to_string());
        }

        Ok(Service {
            settings: settings.clone(),
            root_dir,
        })
    }
}

/// A service that hosts a Treaty instance. A Treaty instance hosts a User and Data service, implemented by a transport type (Http, gRPC, Websocket, or Postgres Native),
/// which is implemented via a database interface over some database: Sqlite or Postgres.
#[derive(Debug)]
pub struct Service {
    settings: Settings,
    root_dir: Option<String>,
}

impl Service {
    /// Configures a new Treaty service with the specified root directory and settings
    pub fn new(settings: &Settings) -> Self {
        Self {
            settings: settings.clone(),
            root_dir: None,
        }
    }

    /// Configures a new Treaty instance, looking for the Settings.toml at the specified dir
    pub fn new_at_dir(dir: &str) -> Result<Self, String> {
        let result = Settings::new(dir, "Settings.toml");
        match result {
            Ok(settings) => Ok(Self {
                settings,
                root_dir: Some(dir.into()),
            }),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn new_with_dir(dir: &str, settings: &Settings) -> Self {
        Self {
            settings: settings.clone(),
            root_dir: Some(dir.to_string()),
        }
    }

    /// Returns a copy of this service's Settings
    pub fn settings(&self) -> Settings {
        self.settings.clone()
    }

    /// Returns this service's root directory
    pub fn root(&self) -> Option<String> {
        self.root_dir.clone()
    }

    /// Initalizes the backing db
    /// with the needed table structures
    pub async fn init_db(&self) {
        trace!("[{}]: Init db..", function_name!());

        match self.settings.database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let sqlite = SqliteDbi::new(
                    self.settings.backing_database_name.as_ref().unwrap(),
                    self.root_dir.as_ref().unwrap(),
                );
                let interface = DbInterface::<SqliteDbi>::new(sqlite);
                interface
                    .init(&self.settings.admin_un, &self.settings.admin_pw)
                    .await;
            }

            DatabaseType::Postgres => {
                let settings = &self.settings.postgres_settings.as_ref().unwrap();
                let postgres = PostgresDbi::new(
                    &settings.host,
                    settings.port,
                    &settings.un,
                    &settings.pw,
                    &settings.schema_name,
                    &settings.db_name,
                    settings.use_treaty_schema,
                );

                let interface = DbInterface::<PostgresDbi>::new(postgres);
                interface
                    .init(&self.settings.admin_un, &self.settings.admin_pw)
                    .await;
            }
        }
    }

    /// Initalizes the service at the specified directory with the username and hash provided
    /// Note: This is intended to be called by a Treaty-proxy instance upon registration of an
    /// Treaty account - to be called only once
    pub async fn init_db_with_admin(&mut self, un: &str, hash: Vec<u8>) {
        match self.settings.database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let sqlite = SqliteDbi::new(
                    self.settings.backing_database_name.as_ref().unwrap(),
                    self.root_dir.as_ref().unwrap(),
                );
                let interface = DbInterface::<SqliteDbi>::new(sqlite);
                interface.init_with_hash(un, hash).await;
            }

            DatabaseType::Postgres => todo!(),
        }
    }

    fn rewrite_ip(original_ip: &str) -> String {
        let parts = original_ip.split(':');
        let parts = parts.collect::<Vec<&str>>();
        let port = parts.last().unwrap().to_string();
        let port = port.parse::<u32>().unwrap();

        let my_local_ip = local_ip().unwrap().to_string();
        let addr = format!("{}:{}", my_local_ip, port);

        trace!("[{}]: rewrite ip with local ip {addr:?}", function_name!());
        addr
    }

    /// Starts all services (gRPC, HTTP, WS, etc) based on the config settings
    pub async fn start_services(mut self) -> Result<ServiceShutdown, TreatyError> {
        if self.settings.override_ip_with_local {
            if self.settings.use_grpc {
                // overide user service
                let user_service_addr =
                    Self::rewrite_ip(&self.settings.grpc_user_service_addr_port);

                trace!(
                    "[{}]: overriding client ip with local ip {user_service_addr:?}",
                    function_name!()
                );
                self.settings.grpc_user_service_addr_port = user_service_addr;

                // override data service
                let data_service_addr =
                    Self::rewrite_ip(&self.settings.grpc_data_service_addr_port);

                trace!(
                    "[{}]: overriding data ip with local ip {data_service_addr:?}",
                    function_name!()
                );
                self.settings.grpc_data_service_addr_port = data_service_addr;

                // override info service
                let info_service_addr =
                    Self::rewrite_ip(&self.settings.grpc_info_service_addr_port);

                trace!(
                    "[{}]: overriding info ip with local ip {info_service_addr:?}",
                    function_name!()
                );
                self.settings.grpc_info_service_addr_port = info_service_addr;
            }

            if self.settings.use_http {
                let my_local_ip = local_ip().unwrap().to_string();
                trace!(
                    "[{}]: overriding http ip with local ip {my_local_ip:?}",
                    function_name!()
                );
                self.settings.http_addr = my_local_ip;
            }
        }

        let mut shutdown = ServiceShutdown {
            grpc_user: None,
            grpc_data: None,
            grpc_info: None,
            http: None,
        };

        if self.settings.use_grpc {
            let grpc_shutdown = self.start_grpc();
            shutdown.grpc_user = grpc_shutdown.grpc_user;
            shutdown.grpc_data = grpc_shutdown.grpc_data;
            shutdown.grpc_info = grpc_shutdown.grpc_info
        }

        if self.settings.use_http {
            let handle = Handle::current();
            self.start_http(Some(handle)).await
        }

        Ok(shutdown)
    }

    /// Initalizes the `treaty` instance with a random generated host_id, host_name, and token.
    /// Effectively this is the same thing as a call to `.generate_host_info(host_name)`
    /// and returns the host_id.
    pub async fn warn_init_host_info(&self) {
        trace!("[{}]: checking for existing host_id", function_name!());
        self.init_db().await;
        let db = self.interface();
        let result = db.treaty_get_host_info().await;
        match result {
            Ok(opt_host_info) => match opt_host_info {
                Some(host_info) => {
                    let host_id = host_info.id;
                    warn!(
                        "[{}]: a host id has already been set: {host_id}",
                        function_name!()
                    );
                    warn!("[{}]: setting a host_id has been ignored", function_name!());
                }
                None => {
                    let id = GUID::rand();
                    match db.treaty_generate_host_info(&id.to_string()).await {
                        Ok(_) => {
                            trace!("[{}]: host_id has been set", function_name!());
                        }
                        Err(e) => {
                            error!("[{}]: {e:?}", function_name!());
                        }
                    }
                }
            },
            Err(e) => error!("[{}]: {e:?}", function_name!()),
        }
    }

    /// Returns this services's Host Id
    pub async fn host_id(&self) -> String {
        let db = self.interface();
        let result = db.treaty_get_host_info().await;
        match result {
            Ok(opt_host_info) => match opt_host_info {
                Some(host_info) => {
                    let host_id = host_info.id;
                    return host_id;
                }
                None => {
                    warn!("No host info has been set");
                }
            },
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
            }
        }

        String::from("")
    }

    // fn interface<D: DbiActions + Clone + Send + Sync>(&self) -> DbInterface<D> {
    fn interface(&self) -> Box<dyn DbiActions + Send> {
        // fn interface<D: DbiActions + Clone + Send + Sync>(&self) -> Box<DbInterface<D>> {
        match self.settings.database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let sqlite = SqliteDbi::new(
                    self.settings.backing_database_name.as_ref().unwrap(),
                    self.root_dir.as_ref().unwrap(),
                );

                Box::new(DbInterface::<SqliteDbi>::new(sqlite))
            }

            DatabaseType::Postgres => {
                let settings = &self.settings.postgres_settings.as_ref().unwrap();
                let postgres = PostgresDbi::new(
                    &settings.host,
                    settings.port,
                    &settings.un,
                    &settings.pw,
                    &settings.schema_name,
                    &settings.db_name,
                    settings.use_treaty_schema,
                );

                Box::new(DbInterface::<PostgresDbi>::new(postgres))
            }
        }
    }

    fn start_grpc(&self) -> ServiceShutdown {
        match self.settings.database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let (user_trigger, user_listener) = triggered::trigger();
                let (db_trigger, db_listener) = triggered::trigger();
                let (info_trigger, info_listener) = triggered::trigger();

                {
                    let sqlite = SqliteDbi::new(
                        self.settings.backing_database_name.as_ref().unwrap(),
                        self.root_dir.as_ref().unwrap(),
                    );
                    let interface = DbInterface::<SqliteDbi>::new(sqlite);

                    let gprc = match &self.settings.tls_options {
                        Some(tls_settings) => RemoteGrpc::new(
                            &self.settings.grpc_data_service_addr_port,
                            &self.settings.grpc_info_service_addr_port,
                            self.settings.data_grpc_timeout_in_seconds,
                            Some(tls_settings.clone().into()),
                        ),
                        None => RemoteGrpc::new(
                            &self.settings.grpc_data_service_addr_port,
                            &self.settings.grpc_info_service_addr_port,
                            self.settings.data_grpc_timeout_in_seconds,
                            None,
                        ),
                    };

                    let remote = Remote::new(gprc);

                    let user = UserServiceHandler::new(interface, remote, self.settings.clone());

                    thread::spawn(move || {
                        user.start_grpc(user_listener).unwrap();
                    });
                }

                {
                    let sqlite = SqliteDbi::new(
                        self.settings.backing_database_name.as_ref().unwrap(),
                        self.root_dir.as_ref().unwrap(),
                    );
                    let interface = DbInterface::<SqliteDbi>::new(sqlite);

                    let data = DataServiceHandler::new(interface, self.settings.clone());

                    thread::spawn(move || {
                        data.start_grpc(db_listener).unwrap();
                    });
                }

                {
                    let sqlite = SqliteDbi::new(
                        self.settings.backing_database_name.as_ref().unwrap(),
                        self.root_dir.as_ref().unwrap(),
                    );
                    let interface = DbInterface::<SqliteDbi>::new(sqlite);

                    let info = InfoServiceHandler::new(interface, self.settings.clone());

                    thread::spawn(move || {
                        info.start_grpc(info_listener).unwrap();
                    });
                }

                ServiceShutdown {
                    grpc_user: Some(user_trigger),
                    grpc_data: Some(db_trigger),
                    grpc_info: Some(info_trigger),
                    http: None,
                }
            }

            DatabaseType::Postgres => {
                let (user_trigger, user_listener) = triggered::trigger();
                let (db_trigger, db_listener) = triggered::trigger();
                let (info_trigger, info_listener) = triggered::trigger();

                {
                    let settings = &self.settings.postgres_settings.as_ref().unwrap();
                    let postgres = PostgresDbi::new(
                        &settings.host,
                        settings.port,
                        &settings.un,
                        &settings.pw,
                        &settings.schema_name,
                        &settings.db_name,
                        settings.use_treaty_schema,
                    );

                    let gprc = match &self.settings.tls_options {
                        Some(tls_settings) => RemoteGrpc::new(
                            &self.settings.grpc_data_service_addr_port,
                            &self.settings.grpc_info_service_addr_port,
                            self.settings.data_grpc_timeout_in_seconds,
                            Some(tls_settings.clone().into()),
                        ),
                        None => RemoteGrpc::new(
                            &self.settings.grpc_data_service_addr_port,
                            &self.settings.grpc_info_service_addr_port,
                            self.settings.data_grpc_timeout_in_seconds,
                            None,
                        ),
                    };

                    let remote = Remote::new(gprc);

                    let user = UserServiceHandler::new(postgres, remote, self.settings.clone());

                    thread::spawn(move || {
                        user.start_grpc(user_listener).unwrap();
                    });
                }

                {
                    let settings = &self.settings.postgres_settings.as_ref().unwrap();
                    let postgres = PostgresDbi::new(
                        &settings.host,
                        settings.port,
                        &settings.un,
                        &settings.pw,
                        &settings.schema_name,
                        &settings.db_name,
                        settings.use_treaty_schema,
                    );
                    let data = DataServiceHandler::new(postgres, self.settings.clone());

                    thread::spawn(move || {
                        data.start_grpc(db_listener).unwrap();
                    });
                }

                {
                    let settings = &self.settings.postgres_settings.as_ref().unwrap();
                    let postgres = PostgresDbi::new(
                        &settings.host,
                        settings.port,
                        &settings.un,
                        &settings.pw,
                        &settings.schema_name,
                        &settings.db_name,
                        settings.use_treaty_schema,
                    );

                    let info = InfoServiceHandler::new(postgres, self.settings.clone());

                    thread::spawn(move || {
                        info.start_grpc(info_listener).unwrap();
                    });
                }

                ServiceShutdown {
                    grpc_user: Some(user_trigger),
                    grpc_data: Some(db_trigger),
                    grpc_info: Some(info_trigger),
                    http: None,
                }
            }
        }
    }

    async fn start_http(&self, option_handle: Option<Handle>) {
        let http_server = HttpServer::new(
            self.root_dir.clone(),
            self.settings.clone(),
            &self.settings.http_addr,
            self.settings.http_port,
            option_handle,
        )
        .await;
        debug!(
            "[{}]: Service is starting http server on: {}:{}",
            function_name!(),
            &self.settings.http_addr,
            self.settings.http_port
        );

        start_http(http_server).await;
    }
}
