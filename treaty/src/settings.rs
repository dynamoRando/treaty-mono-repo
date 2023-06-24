use std::path::Path;

use config::Config;
use config::ConfigError;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use tracing::error;
use treaty_types::enums::*;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Settings {
    /// The un for the admin of the treaty instance
    pub admin_un: String,
    /// The pw for the admin of the treaty instance
    pub admin_pw: String,
    /// The backing database technology, default Sqlite
    pub database_type: DatabaseType,
    /// The backing database name for Treaty, default "treaty.db"
    pub backing_database_name: String,
    /// The gRPC user/client service port in a format similar to 127.0.0.1:50051
    pub grpc_client_service_addr_port: String,
    /// The gRPC data service port in a format similar to 127.0.0.1:50052
    pub grpc_data_service_addr_port: String,
    /// The timeout for our gRPC service in seconds
    pub client_grpc_timeout_in_seconds: u32,
    /// The timeout for our gRPC service in seconds
    pub data_grpc_timeout_in_seconds: u32,
    /// The HTTP Address to host the endpoint on
    pub http_addr: String,
    /// The HTTP port
    pub http_port: u16,
    /// Specify if we should start our gRPC endpoints (using Tonic). This starts two gRPC endpoints, a user service
    /// for application developers to talk to and a data service, for other Treaty instances to communicate to
    pub use_grpc: bool,
    /// Specify if we should start an HTTP server (using Rocket)
    pub use_http: bool,
    /// Ignores the IP address provided above in the settings and instead attempts to get the local IP of the machine
    /// to be used. This feature was added to support using Treaty in Docker containers, where the IP address may be
    /// different than the intended default.
    pub override_ip_with_local: bool,
}

impl Settings {
    /// parses a settings file at the specified directory with the specified filename
    pub fn new(dir: &str, filename: &str) -> Result<Self, ConfigError> {
        let location = Path::new(dir).join(filename.clone());
        let location = location.to_str().unwrap();
        let error_message = "Could not find settings".to_string();

        let settings = Config::builder()
            .add_source(config::File::with_name(location))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .expect(&error_message);

        let i_database_type = settings.get_int(&String::from("database_type"))?;
        let database_type = DatabaseType::from_i64(i_database_type);

        let s_db_name = settings.get_string(&String::from("backing_database_name"))?;

        let s_client_service_addr_port =
            settings.get_string(&String::from("grpc_client_service_addr_port"))?;

        let s_client_timeout =
            settings.get_string(&String::from("client_grpc_timeout_in_seconds"))?;

        let s_data_timeout = settings.get_string(&String::from("data_grpc_timeout_in_seconds"))?;

        let client_timeout_in_seconds: u32 = match s_client_timeout.parse() {
            Ok(timeout) => timeout,
            Err(_) => {
                error!("Could not parse: {s_data_timeout:?}, defaulting to 30");
                30
            }
        };

        let data_timeout_in_seconds: u32 = match s_data_timeout.parse() {
            Ok(timeout) => timeout,
            Err(_) => {
                error!("Could not parse: {s_data_timeout:?}, defaulting to 30");
                30
            }
        };

        let d_client_service_addr_port =
            settings.get_string(&String::from("grpc_data_service_addr_port"))?;

        let admin_un = settings.get_string(&String::from("admin_un"))?;

        let admin_pw = settings.get_string(&String::from("admin_pw"))?;

        let http_addr = settings.get_string(&String::from("http_addr"))?;
        let http_port = settings.get_int(&String::from("http_port"))? as u16;

        let use_grpc = settings.get_bool(&String::from("use_grpc"))?;
        let use_http = settings.get_bool(&String::from("use_http"))?;

        let override_ip_with_local = settings.get_bool(&String::from("override_ip_with_local"))?;

        Ok(Self {
            admin_un,
            admin_pw,
            database_type,
            backing_database_name: s_db_name,
            grpc_client_service_addr_port: s_client_service_addr_port,
            grpc_data_service_addr_port: d_client_service_addr_port,
            client_grpc_timeout_in_seconds: client_timeout_in_seconds,
            data_grpc_timeout_in_seconds: data_timeout_in_seconds,
            http_addr,
            http_port,
            use_grpc,
            use_http,
            override_ip_with_local,
        })
    }
}
