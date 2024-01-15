use std::{env, path::Path};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{self, util::SubscriberInitExt};
use treaty_proxy::proxy_server::ProxyServer;
use treaty_proxy::TreatyProxy;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    process_cmd_args(args);

    // SimpleLogger::new().env().init().unwrap();
    // init_log_to_screen_fern(log::LevelFilter::Trace);
    init_to_screen();

    let dir = &cwd();
    let result_proxy = TreatyProxy::get_proxy_from_config(dir);

    match result_proxy {
        Ok(proxy) => {
            let proxy = proxy.clone();
            proxy.start();
            proxy.start_grpc_client().await;
            proxy.start_grpc_data().await;
            proxy.start_grpc_info().await;
            let server = ProxyServer::new(proxy);
            if let Err(e) = server.start().await {
                println!("Error: {e:?}");
            }
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}

fn cwd() -> String {
    let wd = env::current_dir().unwrap();
    let cwd = wd.to_str().unwrap();
    let cur_dir = Path::new(cwd);
    cur_dir.to_str().unwrap().to_string()
}

fn init_to_screen() {
    let filter = EnvFilter::builder().parse_lossy("treaty=trace");

    println!("{filter:?}");

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_env_filter(filter)
        .finish();

    subscriber.init();
}

fn process_cmd_args(args: Vec<String>) {
    if args.len() >= 2 {
        let cmd = args[1].as_str();

        match cmd {
            "-v" => {
                let version = env!("CARGO_PKG_VERSION");
                println!("treaty-proxy version: {}", version);
            }
            _ => {}
        }
    }
}
