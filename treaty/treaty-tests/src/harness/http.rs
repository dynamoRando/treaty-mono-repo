use std::env;

use super::TestConfigHttp;
use crate::{
    harness::{
        delete_test_database, delete_test_postgres_database, release_port, sleep_instance,
        start_keepalive_for_test, AddrType, ServiceAddr, TEST_SETTINGS,
    },
    DEFAULT_BACKING_DB_NAME, DEFAULT_GRPC_TIMEOUT_SECONDS, DEFAULT_TEST_PW, DEFAULT_TEST_UN,
    USE_LOCAL_IP,
};
use local_ip_address::local_ip;
use stdext::function_name;
use tracing::{debug, info, trace, warn};
use treaty::{
    service::{Service, ServiceBuilder},
    settings::{HttpTlsClientOptions, PostgresSettings, Settings, TlsServerClientSettings},
};
use treaty_client::TreatyClientType;
use treaty_types::enums::*;

pub async fn start_service_with_http_postgres(test_db_name: &str, use_alt: bool) -> TestConfigHttp {
    match dotenvy::dotenv() {
        Ok(_) => trace!("Loaded .env file"),
        Err(_) => warn!("Could NOT load .env file"),
    }

    let http_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();

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

    trace!("[{}]: Startup http addr: {addr:?}", function_name!());

    let http_addr = ServiceAddr {
        ip4_addr: addr.to_string(),
        port: http_port_num,
        addr_type: AddrType::Client,
    };

    //let addr = format!("{}:", addr);

    let settings = Settings {
        admin_un: DEFAULT_TEST_UN.into(),
        admin_pw: DEFAULT_TEST_PW.into(),
        database_type: DatabaseType::Postgres,
        backing_database_name: Some(DEFAULT_BACKING_DB_NAME.into()),
        grpc_user_service_addr_port: String::from(""),
        grpc_data_service_addr_port: String::from(""),
        grpc_info_service_addr_port: String::from(""),
        user_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        data_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        info_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        http_addr: addr.clone(),
        http_port: http_port_num as u16,
        use_grpc: false,
        use_http: true,
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
    let _ = service.start_services().await.unwrap();
    let keep_alive =
        start_keepalive_for_test(TreatyClientType::Http, http_addr.clone(), http_addr.clone());
    let _ = keep_alive.send(true);

    // sleep_instance();

    sleep_instance();

    TestConfigHttp {
        http_address: http_addr,
        keep_alive,
        tls_opts: None,
    }
}

#[allow(dead_code)]
/// returns a tuple for the addr_port of the client service and the db service
pub async fn start_service_with_http(
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
        backing_database_name: Some(DEFAULT_BACKING_DB_NAME.into()),
        grpc_user_service_addr_port: String::from(""),
        grpc_data_service_addr_port: String::from(""),
        user_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        data_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        http_addr: addr.clone(),
        http_port: http_port_num as u16,
        use_grpc: false,
        use_http: true,
        override_ip_with_local: false,
        grpc_info_service_addr_port: String::from(""),
        info_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        send_user_port_number: false,
        send_data_port_number: true,
        send_info_port_number: true,
        jwt_valid_time_in_minutes: 20,
        postgres_settings: None,
        use_tls: false,
        tls_options: None,
        tls_http_options: None,
    };

    let service = if settings.database_type == DatabaseType::Sqlite {
        ServiceBuilder::build_from_settings(&settings, Some(root_dir.clone())).unwrap()
    } else {
        Service::new(&settings)
    };

    let cwd = service.root().unwrap();

    delete_test_database(test_db_name, &cwd);

    debug!("{:?}", &test_db_name);
    debug!("{:?}", &cwd);

    service.init_db().await;
    service.warn_init_host_info().await;
    service.start_services().await.unwrap();

    let keep_alive =
        start_keepalive_for_test(TreatyClientType::Http, http_addr.clone(), http_addr.clone());
    let _ = keep_alive.send(true);

    // sleep_instance();

    TestConfigHttp {
        http_address: http_addr,
        keep_alive,
        tls_opts: None,
    }
}

#[allow(dead_code)]
/// returns a tuple for the addr_port of the client service and the db service
pub async fn start_service_with_http_tls(
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

    let path = env::current_dir().unwrap();

    let tls_cert = path.join("certs/cert.pem");
    let tls_cert_path = tls_cert.to_str().unwrap().to_string();
    debug!("{}", tls_cert_path);

    let tls_key = path.join("certs/key.pem");
    let tls_key_path = tls_key.to_str().unwrap().to_string();
    debug!("{}", tls_key_path);

    let tls_client_cert = path.join("certs/rootCA.crt");
    let tls_client_cert_path = tls_client_cert.to_str().unwrap().to_string();
    debug!("{}", tls_client_cert_path);

    let tls_client_domain = "treaty";

    let client_tls_opts = HttpTlsClientOptions {
        danger_accept_invalid_certs: true,
        tls_sni: false,
    };

    let tls_settings = TlsServerClientSettings {
        tls_cert_path: Some(tls_cert_path),
        tls_key_path: Some(tls_key_path.clone()),
        tls_client_cert_path: Some(tls_client_cert_path.clone()),
        tls_client_domain: tls_client_domain.to_string(),
    };

    let settings = Settings {
        admin_un: DEFAULT_TEST_UN.into(),
        admin_pw: DEFAULT_TEST_PW.into(),
        database_type: DatabaseType::Sqlite,
        backing_database_name: Some(DEFAULT_BACKING_DB_NAME.into()),
        grpc_user_service_addr_port: String::from(""),
        grpc_data_service_addr_port: String::from(""),
        user_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        data_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        http_addr: addr.clone(),
        http_port: http_port_num as u16,
        use_grpc: false,
        use_http: true,
        override_ip_with_local: false,
        grpc_info_service_addr_port: String::from(""),
        info_grpc_timeout_in_seconds: DEFAULT_GRPC_TIMEOUT_SECONDS,
        send_user_port_number: false,
        send_data_port_number: true,
        send_info_port_number: true,
        jwt_valid_time_in_minutes: 20,
        postgres_settings: None,
        use_tls: true,
        tls_options: Some(tls_settings),
        tls_http_options: Some(client_tls_opts.clone()),
    };

    let service = if settings.database_type == DatabaseType::Sqlite {
        ServiceBuilder::build_from_settings(&settings, Some(root_dir.clone())).unwrap()
    } else {
        Service::new(&settings)
    };

    let cwd = service.root().unwrap();

    delete_test_database(test_db_name, &cwd);

    debug!("{:?}", &test_db_name);
    debug!("{:?}", &cwd);

    service.init_db().await;
    service.warn_init_host_info().await;
    service.start_services().await.unwrap();

    let keep_alive =
        start_keepalive_for_test(TreatyClientType::Http, http_addr.clone(), http_addr.clone());
    let _ = keep_alive.send(true);

    // sleep_instance();

    TestConfigHttp {
        http_address: http_addr,
        keep_alive,
        tls_opts: Some(client_tls_opts.clone()),
        //opt_identity: None,
    }
}

#[allow(dead_code)]
pub async fn shutdown_http(addr: &str, port: u32, use_tls: bool) {
    let addr = addr.to_string();
    let url = if use_tls {
        format!("https://{addr}:{port}/shutdown")
    } else {
        format!("http://{addr}:{port}/shutdown")
    };
    info!("Shutdown Request for http://{addr}:{port}");

    if use_tls {
        warn!("[{}]: WARNING - ACCEPT INVALID CERTS", function_name!());
        warn!("[{}]: WARNING - IGNORE SNI", function_name!());

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .tls_sni(false)
            .build()
            .unwrap();
        let _ = client.get(url);
    } else {
        let _ = reqwest::get(url).await.unwrap();
    }
}

#[allow(dead_code)]
pub async fn shutdown_http_tests(instances: Vec<&TestConfigHttp>) {
    debug!("shutting down http test...");

    for instance in instances {
        if let Err(e) = instance.keep_alive.send(false) {
            warn!("{e}");
        }

        shutdown_http(
            &instance.http_address.ip4_addr,
            instance.http_address.port,
            instance.tls_opts.is_some(),
        )
        .await;
        release_port(instance.http_address.port);
    }
    debug!("shutting down test complete.");
}
