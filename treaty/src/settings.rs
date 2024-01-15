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
    /// The backing database name for Treaty if Sqlite, default "treaty.db"
    pub backing_database_name: Option<String>,
    /// The gRPC user/client service port in a format similar to 127.0.0.1:50051
    pub grpc_user_service_addr_port: String,
    /// The gRPC data service port in a format similar to 127.0.0.1:50052
    pub grpc_data_service_addr_port: String,
    /// The gRPC public info service port in a format similar to 127.0.0.1:50059
    pub grpc_info_service_addr_port: String,
    /// The timeout for our gRPC service in seconds
    pub user_grpc_timeout_in_seconds: u32,
    /// The timeout for our gRPC service in seconds
    pub data_grpc_timeout_in_seconds: u32,
    /// The timeout for our gRPC service in seconds
    pub info_grpc_timeout_in_seconds: u32,
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
    /// Specifies when a request is sent to the info service to provide
    /// the user service port number (default is false)
    pub send_user_port_number: bool,
    /// Specifies when a request is sent to the info service to provide
    /// the data service port number (default is true)
    pub send_data_port_number: bool,
    /// Specifies when a request is sent to the info service to provide
    /// the info service port number (default is true)
    pub send_info_port_number: bool,
    /// The time a JWT issued token should be valid for
    pub jwt_valid_time_in_minutes: u32,
    /// Postgres settings, if applicable
    pub postgres_settings: Option<PostgresSettings>,
    /// If we should use Transport Layer Security
    pub use_tls: bool,
    /// Optional TLS settings, if applicable
    pub tls_options: Option<TlsServerClientSettings>,
    /// Optional TLS settings for HTTP
    pub tls_http_options: Option<HttpTlsClientOptions>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TlsServerClientSettings {
    /// The full path to the TLS Cert (.pem) file
    pub tls_cert_path: Option<String>,
    /// The full path to the TLS Key (.key) file
    pub tls_key_path: Option<String>,
    /// The full path to the signed certificate for clients (for the remote connection)
    pub tls_client_cert_path: Option<String>,
    /// The domain the client if applicable for
    pub tls_client_domain: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct HttpTlsClientOptions {
    pub danger_accept_invalid_certs: bool,
    pub tls_sni: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PostgresSettings {
    /// The Postgres hostname
    pub host: String,
    /// The Postgres port
    pub port: Option<u32>,
    /// The username that Treaty will use to connect.
    /// This user will need the ability to create databases, etc.
    pub un: String,
    /// The pw that Treaty will use
    pub pw: String,
    /// The name of the Treaty schema to use
    pub schema_name: String,
    /// The name of the Treaty system database
    pub db_name: String,
    /// Specifies if Treaty should use the schema to store it's settings in-line with
    /// a user datababase. If "true", Treaty will create and use the user defined database
    /// to store it's settings in the "system" schema. If false, Treaty will create a seperate
    /// Treaty system database and use that for storing it's settings
    ///
    /// This setting is mainly used in functional testing for the Treaty codebase.
    pub use_treaty_schema: bool,
}

impl Settings {
    /// parses a settings file at the specified directory with the specified filename
    pub fn new(dir: &str, filename: &str) -> Result<Self, ConfigError> {
        let location = Path::new(dir).join(filename);
        let location = location.to_str().unwrap();
        let error_message = "Could not find settings".to_string();

        let settings = Config::builder()
            .add_source(config::File::with_name(location))
            .add_source(config::Environment::with_prefix("TREATY"))
            .build()
            .expect(&error_message);

        /*
            Some env vars: (sometimes from the .env file)

            TREATY_POSTGRES_HOST="string"
            TREATY_POSTGRES_PORT=5432
            TREATY_POSTGRES_UN="string"
            TREATY_POSTGRES_PW="string"
            TREATY_POSTGRES_TREATY_SCHEMA="treaty"
            TREATY_POSTGRES_DB_NAME="treaty"
            TREATY_POSTGRES_USE_SCHEMA=true
        */

        let i_database_type = settings.get_int(&String::from("database_type"))?;
        let database_type = DatabaseType::from_i64(i_database_type);

        let mut s_db_name: Option<String> = None;

        if database_type == DatabaseType::Sqlite {
            s_db_name = Some(settings.get_string(&String::from("backing_database_name"))?);
        }

        let s_user_service_addr_port =
            settings.get_string(&String::from("grpc_user_service_addr_port"))?;

        let s_user_timeout = settings.get_string(&String::from("user_grpc_timeout_in_seconds"))?;

        let s_data_timeout = settings.get_string(&String::from("data_grpc_timeout_in_seconds"))?;

        let s_info_service_addr_port =
            settings.get_string(&String::from("grpc_info_service_addr_port"))?;

        let s_info_timeout = settings.get_string(&String::from("info_grpc_timeout_in_seconds"))?;

        let user_timeout_in_seconds: u32 = match s_user_timeout.parse() {
            Ok(timeout) => timeout,
            Err(_) => {
                error!("Could not parse: {s_user_timeout:?}, defaulting to 30");
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

        let info_timeout_in_seconds: u32 = match s_info_timeout.parse() {
            Ok(timeout) => timeout,
            Err(_) => {
                error!("Could not parse: {s_info_timeout:?}, defaulting to 30");
                30
            }
        };

        let d_client_service_addr_port = settings.get_string("grpc_data_service_addr_port")?;

        let admin_un = settings.get_string("admin_un")?;

        let admin_pw = settings.get_string("admin_pw")?;

        let http_addr = settings.get_string("http_addr")?;
        let http_port = settings.get_int("http_port")? as u16;

        let use_grpc = settings.get_bool("use_grpc")?;
        let use_http = settings.get_bool("use_http")?;

        let override_ip_with_local = settings.get_bool(&String::from("override_ip_with_local"))?;

        let send_user_port = settings.get_bool("send_user_port_number")?;
        let send_data_port = settings.get_bool("send_data_port_number")?;
        let send_info_port = settings.get_bool("send_info_port_number")?;

        let s_jwt_valid_time_in_minutes: String =
            settings.get_string("jwt_valid_time_in_minutes")?;

        let jwt_valid_time_in_minutes: u32 = match s_jwt_valid_time_in_minutes.parse() {
            Ok(timeout) => timeout,
            Err(_) => {
                error!("Could not parse: {s_jwt_valid_time_in_minutes:?}, defaulting to 20");
                20
            }
        };

        let mut postgres_settings: Option<PostgresSettings> = None;

        if database_type == DatabaseType::Postgres {
            postgres_settings = Some(PostgresSettings {
                host: settings.get_string("TREATY_POSTGRES_HOST")?,
                port: Some(settings.get_int("TREATY_POSTGRES_PORT")? as u32),
                un: settings.get_string("TREATY_POSTGRES_UN")?,
                pw: settings.get_string("TREATY_POSTGRES_PW")?,
                schema_name: settings.get_string("TREATY_POSTGRES_TREATY_SCHEMA")?,
                db_name: settings.get_string("TREATY_POSTGRES_DB_NAME")?,
                use_treaty_schema: settings.get_bool("TREATY_POSTGRES_USE_SCHEMA")?,
            })
        }

        let use_tls = settings.get_bool(&String::from("use_tls"))?;
        let mut tls_options: Option<TlsServerClientSettings> = None;
        let mut tls_http_options: Option<HttpTlsClientOptions> = None;

        if use_tls {
            tls_options = Some(TlsServerClientSettings {
                tls_cert_path: Some(settings.get_string("tls_cert_path")?),
                tls_key_path: Some(settings.get("tls_key_path")?),
                tls_client_cert_path: Some(settings.get("tls_client_cert_path")?),
                tls_client_domain: settings.get("tls_client_domain")?,
            });

            tls_http_options = Some(HttpTlsClientOptions {
                danger_accept_invalid_certs: settings.get_bool("danger_accept_invalid_certs")?,
                tls_sni: settings.get_bool("tls_sni")?,
            });
        }

        Ok(Self {
            admin_un,
            admin_pw,
            database_type,
            backing_database_name: s_db_name,
            grpc_user_service_addr_port: s_user_service_addr_port,
            grpc_data_service_addr_port: d_client_service_addr_port,
            grpc_info_service_addr_port: s_info_service_addr_port,
            user_grpc_timeout_in_seconds: user_timeout_in_seconds,
            data_grpc_timeout_in_seconds: data_timeout_in_seconds,
            info_grpc_timeout_in_seconds: info_timeout_in_seconds,
            http_addr,
            http_port,
            use_grpc,
            use_http,
            override_ip_with_local,
            send_data_port_number: send_data_port,
            send_user_port_number: send_user_port,
            send_info_port_number: send_info_port,
            jwt_valid_time_in_minutes,
            postgres_settings,
            use_tls,
            tls_options,
            tls_http_options,
        })
    }
}
