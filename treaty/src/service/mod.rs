use std::thread;

use crate::alt_transport::http::{start_http, HttpServer};
use crate::data_service_handler::data_service_handler_actions::DataServiceHandlerActions;
use crate::data_service_handler::DataServiceHandler;
use crate::db_interface::dbi::DbInterface;
use crate::db_interface::dbi_actions::DbiActions;
use crate::db_interface::sqlite::SqliteDbi;
use local_ip_address::local_ip;
use treaty_types::enums::*;

use crate::remote::remote_grpc::RemoteGrpc;
use crate::remote::remote_http::RemoteHttp;
use crate::remote::Remote;
use crate::user_service_handler::user_service_handler_actions::UserServiceHandlerActions;
use crate::user_service_handler::UserServiceHandler;
use crate::{settings::Settings, treaty_proto::TreatyError};
use guid_create::GUID;
use stdext::function_name;
use tracing::{debug, error, trace, warn};
use triggered::Trigger;

/// A service that hosts a Treaty instance. A Treaty instance hosts a User and Data service, implemented by a transport type (Http, gRPC, Websocket, MySQL or Postgres Native),
/// which is implemented via a database interface over some database: Sqlite, MySQL, Postgres, SQL Server, etc.
#[derive(Debug, Clone)]
pub struct Service {
    settings: Settings,
    root_dir: String,
}

/// A struct that contains triggers to shutdown a Treaty service, depending on the type of communications configured for that service
#[derive(Debug, Clone)]
pub struct ServiceShutdown {
    pub grpc_user: Option<Trigger>,
    pub grpc_data: Option<Trigger>,
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

impl Service {
    /// Configures a new Treaty service with the specified root directory and settings
    pub fn new(dir: &str, settings: &Settings) -> Self {
        Self {
            settings: settings.clone(),
            root_dir: dir.into(),
        }
    }

    /// Configures a new Treaty instance, looking for the Settings.toml at the specified dir
    pub fn new_at_dir(dir: &str) -> Result<Self, String> {
        let result = Settings::new(dir, "Settings.toml");
        match result {
            Ok(settings) => Ok(
                Self {
                    settings,
                    root_dir: dir.into(),
                }
            ),
            Err(e) => Err(e.to_string())
        }        
    }

    /// Returns a copy of this service's Settings
    pub fn settings(&self) -> Settings {
        self.settings.clone()
    }

    /// Returns this service's root directory
    pub fn root(&self) -> String {
        self.root_dir.clone()
    }

    /// Initalizes the backing db
    /// with the needed table structures
    pub fn init_db(&self) {
        match self.settings.database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let sqlite = SqliteDbi::new(&self.settings.backing_database_name, &self.root_dir);
                let interface = DbInterface::<SqliteDbi>::new(sqlite);
                interface.init(&self.settings.admin_un, &self.settings.admin_pw);
            }
            DatabaseType::Mysql => todo!(),
            DatabaseType::Postgres => todo!(),
            DatabaseType::Sqlserver => todo!(),
        }
    }

    /// Initalizes the service at the specified directory with the username and hash provided
    /// Note: This is intended to be called by a Treaty-proxy instance upon registration of an
    /// Treaty account - to be called only once
    pub fn init_db_with_admin(&mut self, un: &str, hash: Vec<u8>) {
        match self.settings.database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let sqlite = SqliteDbi::new(&self.settings.backing_database_name, &self.root_dir);
                let interface = DbInterface::<SqliteDbi>::new(sqlite);
                interface.init_with_hash(un, hash);
            }
            DatabaseType::Mysql => todo!(),
            DatabaseType::Postgres => todo!(),
            DatabaseType::Sqlserver => todo!(),
        }
    }

    /// Starts all services (gRPC, HTTP, WS, etc) based on the config settings
    pub fn start_services(mut self) -> Result<ServiceShutdown, TreatyError> {
        if self.settings.override_ip_with_local {
            if self.settings.use_grpc {
                let parts = self.settings.grpc_client_service_addr_port.split(':');
                let parts = parts.collect::<Vec<&str>>();
                let port = parts.last().unwrap().to_string();
                let port = port.parse::<u32>().unwrap();

                let my_local_ip = local_ip().unwrap().to_string();
                let client_service_addr = format!("{}:{}", my_local_ip, port);

                trace!(
                    "[{}]: overriding client ip with local ip {client_service_addr:?}",
                    function_name!()
                );
                self.settings.grpc_client_service_addr_port = client_service_addr;

                let parts = self.settings.grpc_data_service_addr_port.split(':');
                let parts = parts.collect::<Vec<&str>>();
                let port = parts.last().unwrap().to_string();
                let port = port.parse::<u32>().unwrap();

                let my_local_ip = local_ip().unwrap().to_string();
                let data_service_addr = format!("{}:{}", my_local_ip, port);

                trace!(
                    "[{}]: overriding data ip with local ip {data_service_addr:?}",
                    function_name!()
                );
                self.settings.grpc_data_service_addr_port = data_service_addr;
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
            http: None,
        };

        if self.settings.use_grpc {
            let grpc_shutdown = self.start_grpc();
            shutdown.grpc_user = grpc_shutdown.grpc_user;
            shutdown.grpc_data = grpc_shutdown.grpc_data;
        }

        if self.settings.use_http {
            self.start_http()
        }

        Ok(shutdown)
    }

    /// Initalizes the `treaty` instance with a random generated host_id, host_name, and token.
    /// Effectively this is the same thing as a call to `.generate_host_info(host_name)`
    /// and returns the host_id.
    pub fn warn_init_host_info(&self) {
        trace!("[{}]: checking for existing host_id", function_name!());
        self.init_db();
        let db = self.interface();
        let result = db.treaty_get_host_info();
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
                    match db.treaty_generate_host_info(&id.to_string()) {
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
    pub fn host_id(&self) -> String {
        let db = self.interface();
        let result = db.treaty_get_host_info();
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

    /// Returns the User Handler with gRPC backing communications
    pub fn grpc_user_handler(&self) -> impl UserServiceHandlerActions {
        Self::new_grpc_user_handler(&self.root_dir, &self.settings)
    }

    /// Returns the Data Handler 
    pub fn data_handler(&self) -> impl DataServiceHandlerActions {
        Self::new_data_handler(&self.root_dir, &self.settings)
    }

    /// Returns a User Handler configured at the specified directory
    pub fn new_grpc_user_handler_at_dir(
        dir: &str,
    ) -> Result<impl UserServiceHandlerActions, String> {
        let result = Settings::new(dir, "Settings.toml");
        match result {
            Ok(settings) => Ok(Self::new_grpc_user_handler(dir, &settings)),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Returns a User Handler with HTTP backing communications
    pub fn new_http_user_handler_at_dir(dir: &str) -> Result<impl UserServiceHandlerActions, String> {
        let result = Settings::new(dir, "Settings.toml");
        match result {
            Ok(settings) => Ok(Self::new_http_user_handler(dir, &settings)),
            Err(e) => Err(e.to_string())
        }
    }

    /// Returns a Data Handler at the specified directory
    pub fn new_data_handler_at_dir(dir: &str) -> Result<impl DataServiceHandlerActions, String> {
        let result = Settings::new(dir, "Settings.toml"); 
        match result {
            Ok(settings) => Ok(Self::new_data_handler(dir, &settings)),
            Err(e) => Err(e.to_string())
        }
    }

    /// Returns a User Handler with gRPC backing communications based on the settings provided
    pub fn new_grpc_user_handler(dir: &str, settings: &Settings) -> impl UserServiceHandlerActions {
        let database_type = settings.database_type;

        match database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let sqlite = SqliteDbi::new(&settings.backing_database_name, dir);
                let interface = DbInterface::<SqliteDbi>::new(sqlite);
                interface.init(&settings.admin_un, &settings.admin_pw);
                let grpc: RemoteGrpc = RemoteGrpc::new(
                    &settings.grpc_data_service_addr_port,
                    settings.data_grpc_timeout_in_seconds,
                );
                let remote = Remote::new(grpc);
                let user: UserServiceHandler<DbInterface<SqliteDbi>, Remote<RemoteGrpc>> =
                    UserServiceHandler::new(interface, remote, settings.clone());
                user
            }
            DatabaseType::Mysql => todo!(),
            DatabaseType::Postgres => todo!(),
            DatabaseType::Sqlserver => todo!(),
        }
    }

    /// Returns a User Handler with HTTP backing communications based on the settings provided
    pub fn new_http_user_handler(dir: &str, settings: &Settings) -> impl UserServiceHandlerActions {
        let database_type = settings.database_type;

        match database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let sqlite = SqliteDbi::new(&settings.backing_database_name, dir);
                let interface = DbInterface::<SqliteDbi>::new(sqlite);
                interface.init(&settings.admin_un, &settings.admin_pw);
                let http = RemoteHttp::new(&settings.http_addr, settings.http_port as u32);
                let remote = Remote::new(http);
                let user: UserServiceHandler<DbInterface<SqliteDbi>, Remote<RemoteHttp>> =
                    UserServiceHandler::new(interface, remote, settings.clone());
                user
            }
            DatabaseType::Mysql => todo!(),
            DatabaseType::Postgres => todo!(),
            DatabaseType::Sqlserver => todo!(),
        }
    }

    /// Returns a Data Handler with the settings provided
    pub fn new_data_handler(dir: &str, settings: &Settings) -> impl DataServiceHandlerActions {
        let database_type = settings.database_type;

        match database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let sqlite = SqliteDbi::new(&settings.backing_database_name, dir);
                let interface = DbInterface::<SqliteDbi>::new(sqlite);
                interface.init(&settings.admin_un, &settings.admin_pw);
                DataServiceHandler::new(interface, settings.clone())
            }
            DatabaseType::Mysql => todo!(),
            DatabaseType::Postgres => todo!(),
            DatabaseType::Sqlserver => todo!(),
        }
    }

    fn interface(&self) -> impl DbiActions {
        match self.settings.database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let sqlite = SqliteDbi::new(&self.settings.backing_database_name, &self.root_dir);

                DbInterface::<SqliteDbi>::new(sqlite)
            }
            DatabaseType::Mysql => todo!(),
            DatabaseType::Postgres => todo!(),
            DatabaseType::Sqlserver => todo!(),
        }
    }

    fn start_grpc(&self) -> ServiceShutdown {
        // this is a duplicate of the new_X_handler functions above; need to research how to return the trait vs
        // the concrete type: in this case we need the concrete type because we need to start the grpc service
        // but we don't want to add that as a trait since a Handler shouldn't need to worry about the transport layer

        match self.settings.database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let (user_trigger, user_listener) = triggered::trigger();
                let (db_trigger, db_listener) = triggered::trigger();

                {
                    let sqlite =
                        SqliteDbi::new(&self.settings.backing_database_name, &self.root_dir);
                    let interface = DbInterface::<SqliteDbi>::new(sqlite);

                    let gprc = RemoteGrpc::new(
                        &self.settings.grpc_data_service_addr_port,
                        self.settings.data_grpc_timeout_in_seconds,
                    );

                    let remote = Remote::new(gprc);

                    let user = UserServiceHandler::new(interface, remote, self.settings.clone());

                    thread::spawn(move || {
                        user.start_grpc(user_listener).unwrap();
                    });
                }

                {
                    let sqlite =
                        SqliteDbi::new(&self.settings.backing_database_name, &self.root_dir);
                    let interface = DbInterface::<SqliteDbi>::new(sqlite);

                    let data = DataServiceHandler::new(interface, self.settings.clone());

                    thread::spawn(move || {
                        data.start_grpc(db_listener).unwrap();
                    });
                }

                ServiceShutdown {
                    grpc_user: Some(user_trigger),
                    grpc_data: Some(db_trigger),
                    http: None,
                }
            }
            DatabaseType::Mysql => todo!(),
            DatabaseType::Postgres => todo!(),
            DatabaseType::Sqlserver => todo!(),
        }
    }

    fn start_http(&self) {
        let http_server = HttpServer::new(
            self.clone(),
            &self.settings.http_addr,
            self.settings.http_port,
        );
        start_http(http_server);
        debug!(
            "[{}]: http server started on: {}:{}",
            function_name!(),
            &self.settings.http_addr,
            self.settings.http_port
        );
    }
}
