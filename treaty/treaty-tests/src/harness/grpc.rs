use local_ip_address::local_ip;
use tracing::{debug, warn};
use treaty::{service::Service, settings::Settings};
use treaty_client::TreatyClientType;
use treaty_types::enums::*;
use crate::{
    harness::{
        delete_test_database, release_port, start_keepalive_for_test, AddrType, ServiceAddr,
        TEST_SETTINGS,
    },
    DEFAULT_BACKING_DB_NAME, DEFAULT_GRPC_TIMEOUT_SECONDS, DEFAULT_TEST_PW, DEFAULT_TEST_UN, USE_LOCAL_IP,
};

use super::TestConfigGrpc;

#[allow(dead_code)]
/// returns a tuple for the addr_port of the client service and the db service
pub fn start_service_with_grpc(
    test_db_name: &str,
    root_dir: String,
    _use_internal_logging: bool,
) -> TestConfigGrpc {
    let client_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();
    let db_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();

    let addr = if USE_LOCAL_IP {
        let addr = local_ip().unwrap().to_string();
        addr
    } else {
        "127.0.0.1".to_string()
    };

    let addr = format!("{}:", addr);

    let client_address_port = format!("{}{}", String::from(addr.clone()), client_port_num);
    let db_address_port = format!("{}{}", String::from(addr.clone()), db_port_num);

    let client_addr = ServiceAddr {
        ip4_addr: addr.clone().to_string(),
        port: client_port_num,
        addr_type: AddrType::Client,
    };

    let db_addr = ServiceAddr {
        ip4_addr: addr.clone().to_string(),
        port: db_port_num,
        addr_type: AddrType::Database,
    };

    let settings = Settings {
        admin_un: DEFAULT_TEST_UN.into(),
        admin_pw: DEFAULT_TEST_PW.into(),
        database_type: DatabaseType::Sqlite,
        backing_database_name: DEFAULT_BACKING_DB_NAME.into(),
        grpc_client_service_addr_port: client_address_port,
        grpc_data_service_addr_port: db_address_port,
        client_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        data_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        http_addr: String::from(""),
        http_port: 0,
        use_grpc: true,
        use_http: false,
        override_ip_with_local: true,
    };

    let service = Service::new(&root_dir, &settings);

    debug!("{:?}", &service);
    debug!("{:?}", &root_dir);

    delete_test_database(test_db_name, &root_dir);

    service.init_db();
    let shutdown = service.start_services().unwrap();
    let keep_alive = start_keepalive_for_test(TreatyClientType::Grpc, client_addr.clone());
    let _ = keep_alive.send(true);

    // sleep_instance();

    TestConfigGrpc {
        client_address: client_addr,
        database_address: db_addr,
        client_service_shutdown_trigger: shutdown.grpc_user.unwrap(),
        database_service_shutdown_trigger: shutdown.grpc_data.unwrap(),
        client_keep_alive: keep_alive,
    }
}

#[allow(dead_code)]
pub fn shutdown_grpc_tests(instances: Vec<&TestConfigGrpc>) {
    debug!("shutting down grpc test...");

    for instance in instances {
        if let Err(e) = instance.client_keep_alive.send(false) {
            warn!("{e}")
        }

        release_port(instance.client_address.port);
        release_port(instance.database_address.port);

        instance.client_service_shutdown_trigger.trigger();
        instance.database_service_shutdown_trigger.trigger();
    }

    debug!("shutting down test complete.");
}
