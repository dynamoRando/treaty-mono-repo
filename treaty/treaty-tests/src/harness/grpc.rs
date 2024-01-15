use crate::{
    harness::{
        delete_test_database, delete_test_postgres_database, release_port,
        start_keepalive_for_test, AddrType, ServiceAddr, TEST_SETTINGS,
    },
    DEFAULT_BACKING_DB_NAME, DEFAULT_GRPC_TIMEOUT_SECONDS, DEFAULT_TEST_PW, DEFAULT_TEST_UN,
    USE_LOCAL_IP,
};
use local_ip_address::local_ip;
use std::env;
use tracing::{debug, trace, warn};
use treaty::{
    service::Service,
    settings::{PostgresSettings, Settings, TlsServerClientSettings},
};
use treaty_client::grpc::TlsSettings;
use treaty_client::TreatyClientType;
use treaty_types::enums::*;

use super::TestConfigGrpc;

#[allow(dead_code)]
/// returns a tuple for the addr_port of the client service and the db service
pub async fn start_service_with_grpc_tls(
    test_db_name: &str,
    root_dir: String,
    _use_internal_logging: bool,
) -> TestConfigGrpc {
    let client_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();
    let db_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();
    let info_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();

    let addr = if USE_LOCAL_IP {
        let addr = local_ip().unwrap().to_string();
        addr
    } else {
        "127.0.0.1".to_string()
    };

    let addr = format!("{}:", addr);

    let client_address_port = format!("{}{}", String::from(addr.clone()), client_port_num);
    let db_address_port = format!("{}{}", String::from(addr.clone()), db_port_num);
    let info_address_port = format!("{}{}", String::from(addr.clone()), info_port_num);

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

    let info_addr = ServiceAddr {
        ip4_addr: addr.clone().to_string(),
        port: info_port_num,
        addr_type: AddrType::Info,
    };

    let path = env::current_dir().unwrap();

    let tls_cert = path.join("certs/cert.pem");
    let tls_cert_path = tls_cert.to_str().unwrap().to_string();
    debug!("{}", tls_cert_path);

    let tls_key = path.join("certs/key.pem");
    let tls_key_path = tls_key.to_str().unwrap().to_string();
    debug!("{}", tls_key_path);

    let tls_client_cert = path.join("certs/rootCA.crt");
    let tls_client_cert_path = tls_client_cert.to_str().unwrap().to_string();

    let tls_client_domain = "localhost";

    let tls_settings = TlsServerClientSettings {
        tls_cert_path: Some(tls_cert_path),
        tls_key_path: Some(tls_key_path),
        tls_client_cert_path: Some(tls_client_cert_path.clone()),
        tls_client_domain: tls_client_domain.to_string(),
    };

    let settings = Settings {
        admin_un: DEFAULT_TEST_UN.into(),
        admin_pw: DEFAULT_TEST_PW.into(),
        database_type: DatabaseType::Sqlite,
        backing_database_name: Some(DEFAULT_BACKING_DB_NAME.into()),
        grpc_user_service_addr_port: client_address_port,
        grpc_data_service_addr_port: db_address_port,
        grpc_info_service_addr_port: info_address_port,
        user_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        data_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        info_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        http_addr: String::from(""),
        http_port: 0,
        use_grpc: true,
        use_http: false,
        override_ip_with_local: false,
        send_data_port_number: true,
        send_info_port_number: true,
        send_user_port_number: false,
        jwt_valid_time_in_minutes: 20,
        postgres_settings: None,
        use_tls: true,
        tls_options: Some(tls_settings),
        tls_http_options: None,
    };

    let service = Service::new_with_dir(&root_dir, &settings);

    debug!("{:?}", &service);
    debug!("{:?}", &root_dir);

    delete_test_database(test_db_name, &root_dir);

    service.init_db().await;
    service.warn_init_host_info().await;
    let shutdown = service.start_services().await.unwrap();
    let keep_alive = start_keepalive_for_test(
        TreatyClientType::Grpc,
        client_addr.clone(),
        info_addr.clone(),
    );
    let _ = keep_alive.send(true);

    // sleep_instance();

    let pem = std::fs::read_to_string(tls_client_cert_path.clone()).unwrap();
    let tls_settings = TlsSettings {
        pem,
        domain: Some(tls_client_domain.to_string()),
    };

    TestConfigGrpc {
        client_address: client_addr,
        database_address: db_addr,
        info_address: info_addr,
        client_service_shutdown_trigger: shutdown.grpc_user.unwrap(),
        database_service_shutdown_trigger: shutdown.grpc_data.unwrap(),
        client_keep_alive: keep_alive,
        tls_settings: Some(tls_settings),
    }
}

#[allow(dead_code)]
/// returns a tuple for the addr_port of the client service and the db service
pub async fn start_service_with_grpc(
    test_db_name: &str,
    root_dir: String,
    _use_internal_logging: bool,
) -> TestConfigGrpc {
    let client_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();
    let db_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();
    let info_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();

    let addr = if USE_LOCAL_IP {
        let addr = local_ip().unwrap().to_string();
        addr
    } else {
        "127.0.0.1".to_string()
    };

    let addr = format!("{}:", addr);

    let client_address_port = format!("{}{}", String::from(addr.clone()), client_port_num);
    let db_address_port = format!("{}{}", String::from(addr.clone()), db_port_num);
    let info_address_port = format!("{}{}", String::from(addr.clone()), info_port_num);

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

    let info_addr = ServiceAddr {
        ip4_addr: addr.clone().to_string(),
        port: info_port_num,
        addr_type: AddrType::Info,
    };

    let settings = Settings {
        admin_un: DEFAULT_TEST_UN.into(),
        admin_pw: DEFAULT_TEST_PW.into(),
        database_type: DatabaseType::Sqlite,
        backing_database_name: Some(DEFAULT_BACKING_DB_NAME.into()),
        grpc_user_service_addr_port: client_address_port,
        grpc_data_service_addr_port: db_address_port,
        grpc_info_service_addr_port: info_address_port,
        user_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        data_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        info_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        http_addr: String::from(""),
        http_port: 0,
        use_grpc: true,
        use_http: false,
        override_ip_with_local: false,
        send_data_port_number: true,
        send_info_port_number: true,
        send_user_port_number: false,
        jwt_valid_time_in_minutes: 20,
        postgres_settings: None,
        use_tls: false,
        tls_options: None,
        tls_http_options: None,
    };

    let service = Service::new_with_dir(&root_dir, &settings);

    debug!("{:?}", &service);
    debug!("{:?}", &root_dir);

    delete_test_database(test_db_name, &root_dir);

    service.init_db().await;
    service.warn_init_host_info().await;
    let shutdown = service.start_services().await.unwrap();
    let keep_alive = start_keepalive_for_test(
        TreatyClientType::Grpc,
        client_addr.clone(),
        info_addr.clone(),
    );
    let _ = keep_alive.send(true);

    // sleep_instance();

    TestConfigGrpc {
        client_address: client_addr,
        database_address: db_addr,
        info_address: info_addr,
        client_service_shutdown_trigger: shutdown.grpc_user.unwrap(),
        database_service_shutdown_trigger: shutdown.grpc_data.unwrap(),
        client_keep_alive: keep_alive,
        tls_settings: None,
    }
}

#[allow(dead_code)]
pub fn shutdown_grpc_tests(instances: Vec<&TestConfigGrpc>) {
    debug!("shutting down grpc test...");

    for instance in instances {
        if let Err(e) = instance.client_keep_alive.send(false) {
            warn!("{e}")
        }

        instance.client_service_shutdown_trigger.trigger();
        instance.database_service_shutdown_trigger.trigger();

        release_port(instance.client_address.port);
        release_port(instance.database_address.port);
    }

    debug!("shutting down test complete.");
}

pub async fn start_service_with_grpc_postgres(test_db_name: &str, use_alt: bool) -> TestConfigGrpc {
    match dotenvy::dotenv() {
        Ok(_) => trace!("Loaded .env file"),
        Err(_) => warn!("Could NOT load .env file"),
    }

    let client_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();
    let db_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();
    let info_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();

    let port = (env::var("TREATY_POSTGRES_PORT").unwrap())
        .parse::<u32>()
        .unwrap();

    // let postgres_settings = Some(PostgresSettings {
    //     host: env::var("TREATY_POSTGRES_HOST").unwrap(),
    //     port: Some(port),
    //     un: env::var("TREATY_POSTGRES_UN").unwrap(),
    //     pw: env::var("TREATY_POSTGRES_PW").unwrap(),
    //     schema_name: env::var("TREATY_POSTGRES_TREATY_SCHEMA").unwrap(),
    //     db_name: env::var("TREATY_POSTGRES_DB_NAME").unwrap(),
    // });

    let host = if use_alt {
        format!("alt.{}", env::var("TREATY_POSTGRES_HOST").unwrap())
    } else {
        env::var("TREATY_POSTGRES_HOST").unwrap()
    };

    let use_schema = env::var("TREATY_POSTGRES_USE_SCHEMA").unwrap();
    let use_schema: bool = use_schema.parse().unwrap();

    let postgres_settings = Some(PostgresSettings {
        host: host,
        port: Some(port),
        un: env::var("TREATY_POSTGRES_UN").unwrap(),
        pw: env::var("TREATY_POSTGRES_PW").unwrap(),
        schema_name: env::var("TREATY_POSTGRES_TREATY_SCHEMA").unwrap(),
        db_name: test_db_name.to_string(),
        use_treaty_schema: use_schema,
    });

    let addr = if USE_LOCAL_IP {
        let addr = local_ip().unwrap().to_string();
        addr
    } else {
        "127.0.0.1".to_string()
    };

    let addr = format!("{}:", addr);

    let client_address_port = format!("{}{}", String::from(addr.clone()), client_port_num);
    let db_address_port = format!("{}{}", String::from(addr.clone()), db_port_num);
    let info_address_port = format!("{}{}", String::from(addr.clone()), info_port_num);

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

    let info_addr = ServiceAddr {
        ip4_addr: addr.clone().to_string(),
        port: info_port_num,
        addr_type: AddrType::Info,
    };

    let settings = Settings {
        admin_un: DEFAULT_TEST_UN.into(),
        admin_pw: DEFAULT_TEST_PW.into(),
        database_type: DatabaseType::Postgres,
        backing_database_name: Some(DEFAULT_BACKING_DB_NAME.into()),
        grpc_user_service_addr_port: client_address_port,
        grpc_data_service_addr_port: db_address_port,
        grpc_info_service_addr_port: info_address_port,
        user_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        data_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        info_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        http_addr: String::from(""),
        http_port: 0,
        use_grpc: true,
        use_http: false,
        override_ip_with_local: false,
        send_data_port_number: true,
        send_info_port_number: true,
        send_user_port_number: false,
        jwt_valid_time_in_minutes: 20,
        postgres_settings: postgres_settings.clone(),
        use_tls: false,
        tls_options: None,
        tls_http_options: None,
    };

    let service = Service::new(&settings);

    debug!("{:?}", &service);

    delete_test_postgres_database(test_db_name, postgres_settings.as_ref().unwrap()).await;

    service.init_db().await;
    service.warn_init_host_info().await;
    let shutdown = service.start_services().await.unwrap();
    let keep_alive = start_keepalive_for_test(
        TreatyClientType::Grpc,
        client_addr.clone(),
        info_addr.clone(),
    );
    let _ = keep_alive.send(true);

    // sleep_instance();

    TestConfigGrpc {
        client_address: client_addr,
        database_address: db_addr,
        info_address: info_addr,
        client_service_shutdown_trigger: shutdown.grpc_user.unwrap(),
        database_service_shutdown_trigger: shutdown.grpc_data.unwrap(),
        client_keep_alive: keep_alive,
        tls_settings: None,
    }
}
