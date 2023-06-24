use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::harness::{TestConfigGrpc};
use crate::harness::{release_port, TreatyClientConfig};
use crate::{
    get_test_temp_dir,
    harness::{
        get_test_temp_dir_main_and_participant,
        grpc::{shutdown_grpc_tests, start_service_with_grpc},
        http::{shutdown_http_tests, start_service_with_http},
        proxy::{setup_proxy_with_users, TreatyProxyTestType},
        sleep_test,
        test_common::{GrpcTestSetup, HttpTestSetup},
        CoreTestConfig, TreatyClientAuth,
    },
};
use tracing::debug;
use treaty_client::TreatyClientType;
use triggered::Trigger;

#[derive(Debug, Clone)]
pub struct RunnerConfig {
    pub test_name: String,
    pub contract_desc: Option<String>,
    pub use_internal_logging: bool,
}

#[derive(Debug, Clone)]
struct ProxyGrpcTriggers {
    pub client: Option<Trigger>,
    pub db: Option<Trigger>,
}

pub struct TestRunner {}

impl TestRunner {
    #[tokio::main]
    pub async fn run_grpc_proxy_test(config: RunnerConfig, test_core: fn(CoreTestConfig)) {
        let db = format!("{}{}", config.test_name, ".db");
        let setup = setup_proxy_with_users(
            &config.test_name,
            false,
            TreatyProxyTestType::Grpc,
        )
        .unwrap();

        debug!("{setup:?}");

        let triggers = ProxyGrpcTriggers {
            client: None,
            db: None,
        };

        let triggers = Arc::new(Mutex::new(triggers));

        {
            let proxy = setup.proxy_info.proxy.clone();
            let triggers = triggers.clone();
            tokio::spawn(async move {
                debug!("starting proxy client");
                let client_trigger = proxy.start_grpc_client().await;
                let mut triggers = triggers.lock().unwrap();
                triggers.client = Some(client_trigger);
                debug!("ending proxy client");
            });
        }

        {
            let proxy = setup.proxy_info.proxy.clone();
            let triggers = triggers.clone();
            tokio::spawn(async move {
                debug!("starting proxy data");
                let data_trigger = proxy.start_grpc_data().await;
                let mut triggers = triggers.lock().unwrap();
                triggers.db = Some(data_trigger);
                debug!("ending proxy data");
            });
        }

        sleep_test();

        {
            let addr = setup.proxy_info.client_addr.clone();
            let mc = TreatyClientConfig {
                addr,
                client_type: TreatyClientType::Grpc,
                host_id: Some(setup.main.host_id.clone()),
                auth: None,
            };

            thread::spawn(move || {
                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: None,
                    test_db_name: db,
                    contract_desc: None,
                    participant_db_addr: None,
                    grpc_test_setup: None,
                    http_test_setup: None,
                    participant_id: None,
                };

                test_core(config);
            })
            .join()
            .unwrap();
        }

        // shutdown proxy services
        triggers.lock().unwrap().client.as_ref().unwrap().trigger();
        triggers.lock().unwrap().db.as_ref().unwrap().trigger();
        release_port(setup.proxy_info.client_addr.port);
        release_port(setup.proxy_info.db_addr.as_ref().unwrap().port);
    }

    #[tokio::main]
    pub async fn run_grpc_proxy_test_multi(config: RunnerConfig, test_core: fn(CoreTestConfig)) {
        let db = format!("{}{}", config.test_name, ".db");
        let setup =
            setup_proxy_with_users(&config.test_name, true, TreatyProxyTestType::Grpc).unwrap();

        let participant_id = setup.part.as_ref().unwrap().host_id.clone();

        debug!("{setup:?}");

        let triggers = ProxyGrpcTriggers {
            client: None,
            db: None,
        };

        let triggers = Arc::new(Mutex::new(triggers));

        {
            let proxy = setup.proxy_info.proxy.clone();
            let triggers = triggers.clone();
            tokio::spawn(async move {
                debug!("starting proxy client");
                let client_trigger = proxy.start_grpc_client().await;
                let mut triggers = triggers.lock().unwrap();
                triggers.client = Some(client_trigger);
                debug!("ending proxy client");
            });
        }

        {
            let proxy = setup.proxy_info.proxy.clone();
            let triggers = triggers.clone();
            tokio::spawn(async move {
                debug!("starting proxy data");
                let data_trigger = proxy.start_grpc_data().await;
                let mut triggers = triggers.lock().unwrap();
                triggers.db = Some(data_trigger);
                debug!("ending proxy data");
            });
        }

        sleep_test();

        {
            let client_addr = setup.proxy_info.client_addr.clone();
            let db_addr = setup.proxy_info.db_addr.clone();

            let m_auth = TreatyClientAuth {
                un: "tester".to_string(),
                pw: "123456".to_string(),
            };

            let p_auth = TreatyClientAuth {
                un: "part".to_string(),
                pw: "123456".to_string(),
            };

            let mc = TreatyClientConfig {
                addr: client_addr.clone(),
                client_type: TreatyClientType::Grpc,
                host_id: Some(setup.main.host_id.clone()),
                auth: Some(m_auth),
            };

            let pc = TreatyClientConfig {
                addr: client_addr.clone(),
                client_type: TreatyClientType::Grpc,
                host_id: Some(setup.part.as_ref().unwrap().host_id.clone()),
                auth: Some(p_auth),
            };

            let (client_trigger, _client_listener) = triggered::trigger();
            let (db_trigger, _db_listener) = triggered::trigger();
            let (tx_main, _rx_main) = mpsc::channel();

            let mtc = TestConfigGrpc {
                client_address: client_addr,
                database_address: db_addr.as_ref().unwrap().clone(),
                client_service_shutdown_trigger: client_trigger,
                database_service_shutdown_trigger: db_trigger,
                client_keep_alive: tx_main,
            };

            let ptc = mtc.clone();

            let grpc_test_setup = GrpcTestSetup {
                main_test_config: mtc,
                participant_test_config: Some(ptc),
                database_name: db.clone(),
                contract_description: config.contract_desc.as_ref().unwrap().clone(),
                main_client: mc.clone(),
                participant_client: Some(pc.clone()),
            };

            let db_addr = setup.proxy_info.db_addr.clone();
            thread::spawn(move || {
                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: Some(pc),
                    test_db_name: db,
                    contract_desc: Some(config.contract_desc.as_ref().unwrap().clone()),
                    participant_db_addr: db_addr.clone(),
                    grpc_test_setup: Some(grpc_test_setup),
                    http_test_setup: None,
                    participant_id: Some(participant_id.clone()),
                };

                test_core(config);
            })
            .join()
            .unwrap();
        }

        // shutdown proxy services
        triggers.lock().unwrap().client.as_ref().unwrap().trigger();
        triggers.lock().unwrap().db.as_ref().unwrap().trigger();
        release_port(setup.proxy_info.client_addr.port);
        release_port(setup.proxy_info.db_addr.as_ref().unwrap().port);
    }

    /// takes a config for a test and will begin an HTTP GRPC test, using the
    /// provided `test_core` function to run
    pub fn run_grpc_test(config: RunnerConfig, test_core: fn(CoreTestConfig)) {
        let db = format!("{}{}", config.test_name, ".db");
        let root_dir = get_test_temp_dir(&config.test_name);
        let main_test_config = start_service_with_grpc(&db, root_dir, config.use_internal_logging);

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let mc = TreatyClientConfig {
                addr: mtc.client_address,
                client_type: TreatyClientType::Grpc,
                host_id: None,
                auth: None,
            };

            thread::spawn(move || {
                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: None,
                    test_db_name: db,
                    contract_desc: None,
                    participant_db_addr: None,
                    grpc_test_setup: None,
                    http_test_setup: None,
                    participant_id: None,
                };

                test_core(config);
            })
            .join()
            .unwrap();
        }

        let instances = vec![&main_test_config];
        shutdown_grpc_tests(instances);
    }

    /// takes a config for a test and will begin an HTTP GRPC test, using the
    /// provided `test_core` function to run
    pub fn run_grpc_test_multi(config: RunnerConfig, test_core: fn(CoreTestConfig)) {
        let db = format!("{}{}", config.test_name, ".db");
        let dirs = get_test_temp_dir_main_and_participant(&config.test_name);

        let main_test_config =
            start_service_with_grpc(&db, dirs.main_dir, config.use_internal_logging);
        let participant_test_config =
            start_service_with_grpc(&db, dirs.participant_dir, config.use_internal_logging);

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();
            let contract = config.contract_desc;

            thread::spawn(move || {
                let mc = TreatyClientConfig {
                    addr: mtc.client_address.clone(),
                    client_type: TreatyClientType::Grpc,
                    host_id: None,
                    auth: None,
                };
                let pc = TreatyClientConfig {
                    addr: ptc.client_address.clone(),
                    client_type: TreatyClientType::Grpc,
                    host_id: None,
                    auth: None,
                };
                let pda = ptc.database_address.clone();

                let grpc_test_setup = GrpcTestSetup {
                    main_test_config: mtc.clone(),
                    participant_test_config: Some(ptc.clone()),
                    database_name: db.clone(),
                    contract_description: contract.as_ref().unwrap().clone(),
                    main_client: mc.clone(),
                    participant_client: Some(pc.clone()),
                };

                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: Some(pc),
                    test_db_name: db,
                    contract_desc: contract,
                    participant_db_addr: Some(pda),
                    grpc_test_setup: Some(grpc_test_setup),
                    http_test_setup: None,
                    participant_id: None,
                };

                test_core(config);
            })
            .join()
            .unwrap();
        }

        let instances = vec![&main_test_config, &participant_test_config];
        shutdown_grpc_tests(instances);
    }

    /// takes a config for a test and will begin an HTTP Treaty test, using the
    /// provided `test_core` function to run for a main and a participant
    pub fn run_http_test_multi(config: RunnerConfig, test_core: fn(CoreTestConfig)) {
        let db = format!("{}{}", config.test_name, ".db");

        let dirs = get_test_temp_dir_main_and_participant(&config.test_name);
        let main_test_config =
            start_service_with_http(&db, dirs.main_dir, config.use_internal_logging);
        let participant_test_config =
            start_service_with_http(&db, dirs.participant_dir, config.use_internal_logging);

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();

            thread::spawn(move || {
                let mc = TreatyClientConfig {
                    addr: mtc.http_address.clone(),
                    client_type: TreatyClientType::Http,
                    host_id: None,
                    auth: None,
                };
                let pc = TreatyClientConfig {
                    addr: ptc.http_address.clone(),
                    client_type: TreatyClientType::Http,
                    host_id: None,
                    auth: None,
                };

                let ptc = ptc.clone();
                let pda = ptc.http_address.clone();
                let contract = config.contract_desc.clone();

                let http_test_setup = HttpTestSetup {
                    main_test_config: mtc.clone(),
                    participant_test_config: ptc,
                    database_name: db.clone(),
                    contract_description: contract.as_ref().unwrap().clone(),
                    main_client: mc.clone(),
                    participant_client: Some(pc.clone()),
                };

                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: Some(pc),
                    test_db_name: db,
                    contract_desc: contract,
                    participant_db_addr: Some(pda),
                    grpc_test_setup: None,
                    http_test_setup: Some(http_test_setup),
                    participant_id: None,
                };

                test_core(config);
            })
            .join()
            .unwrap();
        }

        let tests = vec![&main_test_config, &participant_test_config];

        shutdown_http_tests(tests);
    }

    /// takes a config for a test and will begin an HTTP Treaty test, using the
    /// provided `test_core` function to run
    pub fn run_http_test(config: RunnerConfig, test_core: fn(CoreTestConfig)) {
        let db = format!("{}{}", config.test_name, ".db");

        let dirs = get_test_temp_dir_main_and_participant(&config.test_name);
        let main_test_config =
            start_service_with_http(&db, dirs.main_dir, config.use_internal_logging);

        sleep_test();

        {
            let mtc = main_test_config.clone();

            thread::spawn(move || {
                let mc = TreatyClientConfig {
                    addr: mtc.http_address,
                    client_type: TreatyClientType::Http,
                    host_id: None,
                    auth: None,
                };

                let contract = config.contract_desc.clone();

                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: None,
                    test_db_name: db,
                    contract_desc: contract,
                    participant_db_addr: None,
                    grpc_test_setup: None,
                    http_test_setup: None,
                    participant_id: None,
                };

                test_core(config);
            })
            .join()
            .unwrap();
        }

        let tests = vec![&main_test_config];
        shutdown_http_tests(tests);
    }
}
