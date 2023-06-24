use core::time;
use lazy_static::lazy_static;
use std::fs;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::{path::Path, sync::Mutex};
use test_common::GrpcTestSetup;
use test_common::HttpTestSetup;
use tracing::info;
use tracing::trace;
use treaty_client::client_actions::ClientActions;
use treaty_client::grpc::GrpcClient;
use treaty_client::http::HttpClient;
use treaty_client::Auth;
use treaty_client::TreatyClient;
use treaty_client::TreatyClientType;
use triggered::Trigger;

use crate::get_test_temp_dir;
use crate::DEFAULT_GRPC_TIMEOUT_SECONDS;
use crate::DEFAULT_TEST_PW;
use crate::DEFAULT_TEST_UN;

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
}

#[derive(Debug, Clone)]
pub struct TreatyClientConfig {
    pub addr: ServiceAddr,
    pub client_type: TreatyClientType,
    pub host_id: Option<String>,
    pub auth: Option<TreatyClientAuth>,
}

#[derive(Debug, Clone)]
pub struct TreatyClientAuth {
    pub un: String,
    pub pw: String,
}

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
    pub client_service_shutdown_trigger: Trigger,
    pub database_service_shutdown_trigger: Trigger,
    pub client_keep_alive: Sender<bool>,
}

#[derive(Debug, Clone)]
pub struct TestConfigHttp {
    pub http_address: ServiceAddr,
    pub keep_alive: Sender<bool>,
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
    pub fn to_full_string_with_http(&self) -> String {
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
    sleep_test_for_seconds(2);
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

pub fn init_trace_to_screen(enable: bool) {
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::EnvFilter;

    let filter = EnvFilter::builder().parse_lossy("treaty=trace");

    println!("{filter:?}");

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_env_filter(filter)
        .finish();

    if enable {
        subscriber.init();
    }
}

/*
pub fn init_log_to_screen_fern(level: log::LevelFilter) {
    use ignore_result::Ignore;

    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Blue)
        .error(Color::BrightRed)
        .warn(Color::Magenta);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(level)
        .level_for("tokio", log::LevelFilter::Off)
        .level_for("hyper", log::LevelFilter::Off)
        .level_for("rocket", log::LevelFilter::Off)
        .level_for("h2", log::LevelFilter::Off)
        .level_for("tower", log::LevelFilter::Off)
        .level_for("_", log::LevelFilter::Off)
        .chain(std::io::stdout())
        .apply()
        .ignore();
}
*/

pub fn start_keepalive_for_test(client_type: TreatyClientType, addr: ServiceAddr) -> Sender<bool> {
    let (tx_main, rx_main) = mpsc::channel();

    // main - normal database setup
    thread::spawn(move || {
        let _ = keep_alive(client_type, addr, rx_main);
    })
    .join()
    .unwrap();

    tx_main
}

async fn keep_alive(client_type: TreatyClientType, addr: ServiceAddr, reciever: Receiver<bool>) {
    let sleep_in_seconds = 1;

    match client_type {
        TreatyClientType::Grpc => {
            let addr_port = addr.to_full_string_with_http();
            let timeout_in_seconds = DEFAULT_GRPC_TIMEOUT_SECONDS;

            let auth = Auth {
                user_name: DEFAULT_TEST_UN.into(),
                pw: DEFAULT_TEST_PW.into(),
                jwt: String::from(""),
            };

            let send_jwt_if_available = true;

            let mut client = TreatyClient::<GrpcClient>::new_grpc(
                &addr_port,
                timeout_in_seconds,
                auth,
                send_jwt_if_available,
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
            let val = *self.ports.iter().max().unwrap() + 1;
            self.ports.push(val);
            val
        }
    }

    pub fn get_current_port(&self) -> u32 {
        if self.ports.is_empty() {
            self.max_port
        } else {
            *self.ports.iter().max().unwrap()
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

pub async fn get_treaty_client(config: &TreatyClientConfig) -> Box<dyn ClientActions> {
    trace!("get_treaty_client: {config:?}");

    match config.client_type {
        TreatyClientType::Grpc => {
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
                let addr_port = config.addr.to_full_string_with_http();
                let timeout_in_seconds = DEFAULT_GRPC_TIMEOUT_SECONDS;

                let auth = Auth {
                    user_name: un,
                    pw,
                    jwt: String::from(""),
                };

                let send_jwt_if_available = true;

                let client = TreatyClient::<GrpcClient>::new_grpc(
                    &addr_port,
                    timeout_in_seconds,
                    auth,
                    send_jwt_if_available,
                    None,
                )
                .await;
                // return HarnessClientType::Grpc(client);
                return Box::new(client);
            }

            let addr_port = config.addr.to_full_string_with_http();
            let timeout_in_seconds = DEFAULT_GRPC_TIMEOUT_SECONDS;

            let auth = Auth {
                user_name: un,
                pw,
                jwt: String::from(""),
            };

            let send_jwt_if_available = true;
            let host_id = Some(config.host_id.as_ref().unwrap().clone());

            let client = TreatyClient::<GrpcClient>::new_grpc(
                &addr_port,
                timeout_in_seconds,
                auth,
                send_jwt_if_available,
                host_id,
            )
            .await;
            // return HarnessClientType::Grpc(client);
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

            let auth = Auth {
                user_name: un,
                pw,
                jwt: String::from(""),
            };

            if config.host_id.is_none() {
                let addr = &config.addr.ip4_addr.clone();
                let port = &config.addr.port;
                let timeout_in_seconds: u32 = 60;
                let send_jwt_if_available = true;

                let client = TreatyClient::<HttpClient>::new_http(
                    addr,
                    *port,
                    auth,
                    timeout_in_seconds,
                    send_jwt_if_available,
                    None,
                )
                .await;
                // return HarnessClientType::Http(client);
                return Box::new(client);
            }
            let addr = &config.addr.ip4_addr.clone();
            let port = &config.addr.port;
            let timeout_in_seconds: u32 = 60;
            let send_jwt_if_available = true;
            let host_id = config.host_id.as_ref().unwrap();

            let client = TreatyClient::<HttpClient>::new_http(
                addr,
                *port,
                auth,
                timeout_in_seconds,
                send_jwt_if_available,
                Some(host_id.into()),
            )
            .await;
            // return HarnessClientType::Http(client);
            return Box::new(client);
        }
    }
}
