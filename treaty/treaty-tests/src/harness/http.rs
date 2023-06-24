use local_ip_address::local_ip;
use tracing::{debug, info, warn};
use treaty::{service::Service, settings::Settings};
use treaty_client::TreatyClientType;
use treaty_types::enums::*;
use crate::{
    harness::{
        delete_test_database, release_port, sleep_instance, start_keepalive_for_test, AddrType,
        ServiceAddr, TEST_SETTINGS,
    },
    DEFAULT_BACKING_DB_NAME, DEFAULT_GRPC_TIMEOUT_SECONDS, DEFAULT_TEST_PW, DEFAULT_TEST_UN, USE_LOCAL_IP,
};

use super::TestConfigHttp;

#[allow(dead_code)]
/// returns a tuple for the addr_port of the client service and the db service
pub fn start_service_with_http(
    test_db_name: &str,
    root_dir: String,
    _use_internal_logging: bool,
) -> TestConfigHttp {
    let http_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();

    let addr = if USE_LOCAL_IP {
        let addr = local_ip().unwrap().to_string();
        addr
    } else {
        "127.0.0.1".to_string()
    };

    let http_addr = ServiceAddr {
        ip4_addr: addr.to_string(),
        port: http_port_num,
        addr_type: AddrType::Client,
    };

    let settings = Settings {
        admin_un: DEFAULT_TEST_UN.into(),
        admin_pw: DEFAULT_TEST_PW.into(),
        database_type: DatabaseType::Sqlite,
        backing_database_name: DEFAULT_BACKING_DB_NAME.into(),
        grpc_client_service_addr_port: String::from(""),
        grpc_data_service_addr_port: String::from(""),
        client_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        data_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        http_addr: addr.clone(),
        http_port: http_port_num as u16,
        use_grpc: false,
        use_http: true,
        override_ip_with_local: true,
    };

    let service = Service::new(&root_dir, &settings);
    let cwd = service.root();

    delete_test_database(test_db_name, &cwd);

    debug!("{:?}", &test_db_name);
    debug!("{:?}", &cwd);

    // if use_internal_logging {
    //     service.enable_internal_logging(&root_dir, log::LevelFilter::Debug);
    // }

    service.init_db();
    service.start_services().unwrap();

    let keep_alive = start_keepalive_for_test(TreatyClientType::Grpc, http_addr.clone());
    let _ = keep_alive.send(true);

    sleep_instance();

    TestConfigHttp {
        http_address: http_addr,
        keep_alive,
    }
}

#[allow(dead_code)]
#[tokio::main]
pub async fn shutdown_http(addr: &str, port: u32) {
    {
        let addr = addr.to_string();
        tokio::spawn(async move {
            let url = format!("http://{addr}:{port}/shutdown");
            info!("Shutdown Request for http://{addr}:{port}");
            let _ = reqwest::get(url).await.unwrap();
        });
    }
}

#[allow(dead_code)]
pub fn shutdown_http_tests(instances: Vec<&TestConfigHttp>) {
    debug!("shutting down http test...");

    for instance in instances {
        if let Err(e) = instance.keep_alive.send(false) {
            warn!("{e}");
        }

        release_port(instance.http_address.port);
        shutdown_http(&instance.http_address.ip4_addr, instance.http_address.port);
    }
    debug!("shutting down test complete.");
}
