use crate::get_test_temp_dir;
use crate::DEFAULT_GRPC_TIMEOUT_SECONDS;
use crate::DEFAULT_TEST_PW;
use crate::DEFAULT_TEST_UN;
use core::panic;
use core::time;
use lazy_static::lazy_static;
use std::fs;
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use std::{path::Path, sync::Mutex};
use stdext::function_name;
use test_common::GrpcTestSetup;
use test_common::HttpTestSetup;
use tokio_postgres::NoTls;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::trace;
use treaty::settings::HttpTlsClientOptions;
use treaty::settings::PostgresSettings;
use treaty_client::client_actions::ClientActions;
use treaty_client::grpc::GrpcClient;
use treaty_client::grpc::TlsSettings;
use treaty_client::http::HttpClient;
use treaty_client::TreatyClient;
use treaty_client::TreatyClientType;
use triggered::Trigger;

#[doc(hidden)]
pub mod grpc;
#[doc(hidden)]
pub mod http;
#[doc(hidden)]
pub mod proxy;
#[doc(hidden)]
pub mod test_common;

// http://oostens.me/posts/singletons-in-rust/
// we want to increment for all tests the ports used
// so that way we can run multiple client/servers

pub enum HarnessClientType {
    Grpc(TreatyClient<GrpcClient>),
    Http(TreatyClient<HttpClient>),
}

#[derive(Debug, Clone)]
pub enum AddrType {
    Client,
    Database,
    Info,
}

#[derive(Debug, Clone)]
pub struct TreatyClientConfig {
    pub user_address_port: ServiceAddr,
    pub info_address_port: ServiceAddr,
    pub client_type: TreatyClientType,
    pub host_id: Option<String>,
    pub auth: Option<TreatyClientAuth>,
    pub tls: bool,
    pub tls_settings: Option<TlsSettings>,
    pub tls_http: Option<HttpTlsClientOptions>,
}

#[derive(Debug, Clone)]
pub struct TreatyClientAuth {
    pub un: String,
    pub pw: String,
}

/// Represents an IP address, port number, and the purpose of
/// the address at a Treaty instance
#[derive(Debug, Clone)]
pub struct ServiceAddr {
    pub ip4_addr: String,
    pub port: u32,
    pub addr_type: AddrType,
}

#[derive(Debug, Clone)]
pub struct TestConfigGrpc {
    pub client_address: ServiceAddr,
    pub database_address: ServiceAddr,
    pub info_address: ServiceAddr,
    pub client_service_shutdown_trigger: Trigger,
    pub database_service_shutdown_trigger: Trigger,
    pub client_keep_alive: Sender<bool>,
    pub tls_settings: Option<TlsSettings>,
}

#[derive(Debug, Clone)]
pub struct TestConfigHttp {
    pub http_address: ServiceAddr,
    pub keep_alive: Sender<bool>,
    pub tls_opts: Option<HttpTlsClientOptions>,
}

#[derive(Debug, Clone)]
pub struct TestDirectoryConfig {
    pub root_dir: String,
    pub main_dir: String,
    pub participant_dir: String,
}

#[derive(Debug, Clone)]
pub struct CoreTestConfig {
    pub main_client: TreatyClientConfig,
    pub participant_client: Option<TreatyClientConfig>,
    pub test_db_name: String,
    pub contract_desc: Option<String>,
    pub participant_db_addr: Option<ServiceAddr>,
    pub participant_info_addr: Option<ServiceAddr>,
    pub grpc_test_setup: Option<GrpcTestSetup>,
    pub http_test_setup: Option<HttpTestSetup>,
    pub participant_id: Option<String>,
}

impl ServiceAddr {
    #[allow(dead_code)]
    pub fn to_full_string(&self) -> String {
        format!("{}{}", self.ip4_addr, self.port)
    }
    #[allow(dead_code)]
    pub fn to_full_string_with_http(&self, tls: bool) -> String {
        if tls {
            return format!("{}{}", String::from("https://"), self.to_full_string());
        }
        format!("{}{}", String::from("http://"), self.to_full_string())
    }
}

lazy_static! {
    pub static ref TEST_SETTINGS: Mutex<TestSettings> = Mutex::new(TestSettings {
        max_port: 9000,
        ports: Vec::new()
    });
}

pub fn release_port(port: u32) {
    TEST_SETTINGS.lock().unwrap().release_port(port);
}

pub fn get_next_avail_port() -> u32 {
    return TEST_SETTINGS.lock().unwrap().get_next_avail_port();
}

pub fn sleep_test_for_seconds(seconds: u32) {
    let time = time::Duration::from_secs(seconds as u64);
    info!("sleeping for {} seconds...", seconds.to_string());
    thread::sleep(time);
    // tokio::time::sleep(time).await;
}

pub fn sleep_test() {
    sleep_test_for_seconds(1);
}

pub fn sleep_instance() {
    sleep_test_for_seconds(1);
}

pub async fn tokio_sleep_core() {
    let seconds = 5;
    info!("sleeping for {} seconds...", seconds.to_string());
    tokio::time::sleep(Duration::from_secs(5)).await;
}

/*
/// overrides Treaty's default logger to log to screen for the specified logging level with Simple Logger
pub fn init_log_to_screen(level: log::LevelFilter) {
    let res_log = SimpleLogger::new().with_level(level).init();
    if let Err(e) = res_log {
        error!("{e}");
    }
}
 */

/// Specify if you want tracing to appear on screen.
/// The optional filter is if you want traces from the test module itself to appear as
/// part of the traces.
/// Note that this needs to be the name of the test module, example: `auth_for_token=trace`
/// where `auth_for_token` is the name of the test module, and `trace` is the
/// tracing level.
pub fn init_trace_to_screen(enable: bool, opt_filter: Option<String>) {
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::EnvFilter;

    // let filter = EnvFilter::builder().parse_lossy("treaty=trace,rocket=trace");

    let filter = match opt_filter {
        Some(filter) => {
            let filter_string = format!("treaty=trace,treaty_tests=trace,{}", filter);
            println!("EnvFilter passed: {}", filter_string);
            EnvFilter::builder().parse_lossy(filter_string)
        }
        None => EnvFilter::builder().parse_lossy("treaty=trace,treaty_tests=trace"),
    };

    println!("EnvFilter: {filter:?}");

    /*
     let subscriber = tracing_subscriber::fmt()
         .compact()
         .with_file(true)
         .with_line_number(true)
         .with_target(true)
         .with_max_level(Level::TRACE)
         //.with_env_filter(filter)
         .finish();
    */

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        //.with_max_level(Level::TRACE)
        .with_env_filter(filter)
        .finish();

    if enable {
        subscriber.init();
    }
}

pub fn start_keepalive_for_test(
    client_type: TreatyClientType,
    user_address: ServiceAddr,
    info_address: ServiceAddr,
) -> Sender<bool> {
    trace!("[{}]: Starting keepalive", function_name!());

    let (tx_main, rx_main) = mpsc::channel();

    // main - normal database setup
    thread::spawn(move || {
        let _ = keep_alive(client_type, user_address, rx_main, info_address);
    })
    .join()
    .unwrap();

    tx_main
}

async fn keep_alive(
    client_type: TreatyClientType,
    user_address: ServiceAddr,
    reciever: Receiver<bool>,
    info_address: ServiceAddr,
) {
    let sleep_in_seconds = 1;

    match client_type {
        TreatyClientType::Grpc => {
            let mut client = TreatyClient::<GrpcClient>::new_grpc(
                &user_address.to_full_string_with_http(false),
                &info_address.to_full_string_with_http(false),
                DEFAULT_GRPC_TIMEOUT_SECONDS,
                DEFAULT_TEST_UN,
                DEFAULT_TEST_PW,
                None,
                None,
            )
            .await;

            while reciever.try_recv().unwrap() {
                let time = time::Duration::from_secs(sleep_in_seconds as u64);
                tokio::time::sleep(time).await;
                let _ = client.is_online().await;
            }
        }
        TreatyClientType::Http => {
            todo!();
        }
    };
}

/// returns a tuple for the root directory, the "main" directory, and the "participant" directory
/// in the temp folder
pub fn get_test_temp_dir_main_and_participant(test_name: &str) -> TestDirectoryConfig {
    let root_dir = get_test_temp_dir(test_name);

    let main_path = Path::new(&root_dir).join("main");

    if main_path.exists() {
        fs::remove_dir_all(&main_path).unwrap();
    }

    fs::create_dir_all(&main_path).unwrap();

    let main_dir = main_path.as_os_str().to_str().unwrap();

    let participant_path = Path::new(&root_dir).join("participant");

    if participant_path.exists() {
        fs::remove_dir_all(&participant_path).unwrap();
    }

    fs::create_dir_all(&participant_path).unwrap();

    let participant_dir = participant_path.as_os_str().to_str().unwrap();

    TestDirectoryConfig {
        root_dir,
        main_dir: main_dir.to_string(),
        participant_dir: participant_dir.to_string(),
    }
}

pub struct TestSettings {
    max_port: u32,
    ports: Vec<u32>,
}

impl TestSettings {
    /// tracks the next defined port available in the collection
    /// note: this will sleep the thread for 1 second
    pub fn get_next_avail_port(&mut self) -> u32 {
        sleep_test_for_seconds(1);

        if self.ports.is_empty() {
            self.max_port += 1;
            self.ports.push(self.max_port);
            self.max_port
        } else {
            let mut port_num = *self.ports.iter().max().unwrap() + 1;

            while !self.port_is_available(port_num) {
                port_num = port_num + 1;
                trace!(
                    "[{}]: port number {port_num:?} is not available, incrementing",
                    function_name!()
                );
            }

            self.ports.push(port_num);

            let ports = self.ports.clone();

            println!("get_next_avail_port ports: {ports:#?}");

            port_num
        }
    }

    pub fn port_is_available(&self, port: u32) -> bool {
        let port = format!("127.0.0.1:{port:?}");
        match TcpListener::bind(port.clone()) {
            Ok(_) => {
                trace!("[{}]: port number {port:?} is available", function_name!());
                true
            }
            Err(e) => {
                error!("[{}]: Error: {e:?}", function_name!());
                trace!(
                    "[{}]: port number {port:?} is NOT available",
                    function_name!()
                );
                false
            }
        }
    }

    pub fn release_port(&mut self, port: u32) {
        if self.ports.contains(&port) {
            let index = self.ports.iter().position(|x| *x == port).unwrap();
            self.ports.remove(index);
        }
    }
}

pub fn delete_test_database(db_name: &str, cwd: &str) {
    let db_path = Path::new(&cwd).join(db_name);

    if db_path.exists() {
        fs::remove_file(&db_path).unwrap();
    }
}

fn get_postgres_connection_string(postgres: &PostgresSettings) -> String {
    match postgres.port {
        Some(port) => format!(
            "host={} user={} password={} port={}",
            postgres.host, postgres.un, postgres.pw, port
        ),
        None => format!(
            "host={} user={} password={} ",
            postgres.host, postgres.un, postgres.pw,
        ),
    }
}

pub async fn delete_test_postgres_database(db_name: &str, postgres: &PostgresSettings) {
    let postgres = postgres.clone();
    let result_client =
        match tokio_postgres::connect(&get_postgres_connection_string(&postgres), NoTls).await {
            Ok(pair) => {
                let postgres = postgres.clone();
                tokio::spawn(async move {
                    if let Err(e) = pair.1.await {
                        error!("connection error: {} at {postgres:?}", e);
                        panic!();
                    }
                });

                Ok(pair.0)
            }
            Err(e) => {
                error!("[{}]: {postgres:?} {e:?}", function_name!());
                Err("Couldn't create connection")
            }
        };

    let postgres = postgres.clone();
    if let Err(e) = result_client {
        panic!("{postgres:?} {e:?}");
    }

    if let Ok(client) = result_client {
        let sql = format!("DROP DATABASE IF EXISTS {};", db_name);
        debug!("{sql:?}");
        if let Err(e) = client.execute(&sql, &[]).await {
            error!(
                "[{}]: postgres: {postgres:?} error: {e:?}",
                function_name!()
            );
            panic!("Error droppping database");
        }
    }
}

pub async fn get_treaty_client(config: &TreatyClientConfig) -> Box<dyn ClientActions + Send> {
    trace!("get_treaty_client: {config:?}");

    match config.client_type {
        TreatyClientType::Grpc => {
            trace!("[{}]: Getting GRPC client...", function_name!());
            let un: String;
            let pw: String;

            if config.auth.is_none() {
                un = String::from("tester");
                pw = String::from("123456");
            } else {
                un = config.auth.as_ref().unwrap().un.clone();
                pw = config.auth.as_ref().unwrap().pw.clone();
            }

            if config.host_id.is_none() {
                let user_address_port = config
                    .user_address_port
                    .to_full_string_with_http(config.tls);
                let info_address_port = config
                    .info_address_port
                    .to_full_string_with_http(config.tls);
                let timeout_in_seconds = DEFAULT_GRPC_TIMEOUT_SECONDS;

                let client = TreatyClient::<GrpcClient>::new_grpc(
                    &user_address_port,
                    &info_address_port,
                    timeout_in_seconds,
                    &un,
                    &pw,
                    None,
                    config.tls_settings.clone(),
                )
                .await;
                trace!("[{}]: Getting client with no host id", function_name!());
                trace!("[{}]: {client:?}", function_name!());
                return Box::new(client);
            }

            let user_address_port = config
                .user_address_port
                .to_full_string_with_http(config.tls);
            let info_address_port = config
                .info_address_port
                .to_full_string_with_http(config.tls);
            let timeout_in_seconds = DEFAULT_GRPC_TIMEOUT_SECONDS;

            let host_id = Some(config.host_id.as_ref().unwrap().clone());

            let client = TreatyClient::<GrpcClient>::new_grpc(
                &user_address_port,
                &info_address_port,
                timeout_in_seconds,
                &un,
                &pw,
                host_id,
                config.tls_settings.clone(),
            )
            .await;
            trace!("[{}]: Getting client with host id", function_name!());
            trace!("[{}]: {client:?}", function_name!());
            return Box::new(client);
        }
        TreatyClientType::Http => {
            let un: String;
            let pw: String;

            if config.auth.is_none() {
                un = String::from("tester");
                pw = String::from("123456");
            } else {
                un = config.auth.as_ref().unwrap().un.clone();
                pw = config.auth.as_ref().unwrap().pw.clone();
            }

            let use_tls = match config.tls_settings {
                Some(_) => true,
                None => false,
            };

            if config.host_id.is_none() {
                let user_address = &config.user_address_port.ip4_addr.clone();
                let info_address = &config.info_address_port.ip4_addr.clone();
                let user_port = config.user_address_port.port;
                let info_port = config.info_address_port.port;

                let timeout_in_seconds: u32 = 60;

                let client = TreatyClient::<HttpClient>::new_http(
                    user_address,
                    user_port,
                    info_address,
                    info_port,
                    &un,
                    &pw,
                    timeout_in_seconds,
                    None,
                    use_tls,
                    config.tls_http.clone(),
                )
                .await;
                trace!("[{}]: {client:?}", function_name!());
                return Box::new(client);
            }

            let user_address = &config.user_address_port.ip4_addr.clone();
            let info_address = &config.info_address_port.ip4_addr.clone();
            let user_port = config.user_address_port.port;
            let info_port = config.info_address_port.port;

            let timeout_in_seconds: u32 = 60;
            let host_id = config.host_id.as_ref().unwrap();

            let client = TreatyClient::<HttpClient>::new_http(
                user_address,
                user_port,
                info_address,
                info_port,
                &un,
                &pw,
                timeout_in_seconds,
                Some(host_id.to_string()),
                use_tls,
                config.tls_http.clone(),
            )
            .await;
            trace!("[{}]: {client:?}", function_name!());
            return Box::new(client);
        }
    }
}
