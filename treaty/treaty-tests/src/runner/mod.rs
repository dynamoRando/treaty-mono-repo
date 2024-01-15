use std::sync::{mpsc, Arc, Mutex};

use crate::harness::grpc::start_service_with_grpc_tls;
use crate::harness::http::{start_service_with_http_postgres, start_service_with_http_tls};
use crate::harness::{grpc::start_service_with_grpc_postgres, TestConfigGrpc};
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
use futures::Future;
use tracing::debug;
use treaty_client::grpc::TlsSettings;
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
    pub info: Option<Trigger>,
}

pub struct TestRunner {}

impl TestRunner {
    pub async fn run_grpc_proxy_test<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send,
    {
        let db = format!("{}{}", config.test_name, ".db");
        let setup = setup_proxy_with_users(&config.test_name, false, TreatyProxyTestType::Grpc)
            .await
            .unwrap();

        debug!("{setup:?}");

        let triggers = ProxyGrpcTriggers {
            client: None,
            db: None,
            info: None,
        };

        let triggers = Arc::new(Mutex::new(triggers));

        {
            let proxy = setup.proxy_info.proxy.clone();
            let triggers = triggers.clone();
            let handle = tokio::spawn(async move {
                debug!("starting proxy client");
                let client_trigger = proxy.start_grpc_client().await;
                let mut triggers = triggers.lock().unwrap();
                triggers.client = Some(client_trigger);
                debug!("ending proxy client");
            });

            handle.await.unwrap();
        }

        {
            let proxy = setup.proxy_info.proxy.clone();
            let triggers = triggers.clone();
            let handle = tokio::spawn(async move {
                debug!("starting proxy data");
                let data_trigger = proxy.start_grpc_data().await;
                let mut triggers = triggers.lock().unwrap();
                triggers.db = Some(data_trigger);
                debug!("ending proxy data");
            });

            handle.await.unwrap();
        }

        {
            let proxy = setup.proxy_info.proxy.clone();
            let triggers = triggers.clone();
            let handle = tokio::spawn(async move {
                debug!("starting proxy info");
                let data_trigger = proxy.start_grpc_info().await;
                let mut triggers = triggers.lock().unwrap();
                triggers.db = Some(data_trigger);
                debug!("ending proxy info");
            });

            handle.await.unwrap();
        }

        sleep_test();

        {
            let client_addr = setup.proxy_info.client_addr.clone();
            let info_addr = setup.proxy_info.info_addr.clone();

            let mc = TreatyClientConfig {
                user_address_port: client_addr,
                info_address_port: info_addr,
                client_type: TreatyClientType::Grpc,
                host_id: Some(setup.main.host_id.clone()),
                auth: None,
                tls: false,
                tls_settings: None,
                tls_http: None,
            };

            let handle = tokio::spawn(async move {
                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: None,
                    test_db_name: db,
                    contract_desc: None,
                    participant_db_addr: None,
                    participant_info_addr: None,
                    grpc_test_setup: None,
                    http_test_setup: None,
                    participant_id: None,
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        // shutdown proxy services
        triggers.lock().unwrap().client.as_ref().unwrap().trigger();
        triggers.lock().unwrap().db.as_ref().unwrap().trigger();
        if let Some(info_trigger) = triggers.lock().unwrap().info.clone() {
            info_trigger.trigger();
        }
        release_port(setup.proxy_info.client_addr.port);
        release_port(setup.proxy_info.db_addr.as_ref().unwrap().port);
        release_port(setup.proxy_info.db_addr.as_ref().unwrap().port);
    }

    pub async fn run_grpc_proxy_test_multi<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let db = format!("{}{}", config.test_name, ".db");
        let setup = setup_proxy_with_users(&config.test_name, true, TreatyProxyTestType::Grpc)
            .await
            .unwrap();

        let participant_id = setup.part.as_ref().unwrap().host_id.clone();

        debug!("{setup:?}");

        let triggers = ProxyGrpcTriggers {
            client: None,
            db: None,
            info: None,
        };

        let triggers = Arc::new(Mutex::new(triggers));

        {
            let proxy = setup.proxy_info.proxy.clone();
            let triggers = triggers.clone();
            let handle = tokio::spawn(async move {
                debug!("starting proxy client");
                let client_trigger = proxy.start_grpc_client().await;
                let mut triggers = triggers.lock().unwrap();
                triggers.client = Some(client_trigger);
                debug!("ending proxy client");
            });

            handle.await.unwrap();
        }

        {
            let proxy = setup.proxy_info.proxy.clone();
            let triggers = triggers.clone();
            let handle = tokio::spawn(async move {
                debug!("starting proxy data");
                let data_trigger = proxy.start_grpc_data().await;
                let mut triggers = triggers.lock().unwrap();
                triggers.db = Some(data_trigger);
                debug!("ending proxy data");
            });

            handle.await.unwrap();
        }

        {
            let proxy = setup.proxy_info.proxy.clone();
            let triggers = triggers.clone();
            let handle = tokio::spawn(async move {
                debug!("starting proxy info");
                let data_trigger = proxy.start_grpc_info().await;
                let mut triggers = triggers.lock().unwrap();
                triggers.db = Some(data_trigger);
                debug!("ending proxy info");
            });

            handle.await.unwrap();
        }

        sleep_test();

        {
            let client_addr = setup.proxy_info.client_addr.clone();
            let db_addr = setup.proxy_info.db_addr.clone();
            let info_addr = setup.proxy_info.info_addr.clone();

            let m_auth = TreatyClientAuth {
                un: "tester".to_string(),
                pw: "123456".to_string(),
            };

            let p_auth = TreatyClientAuth {
                un: "part".to_string(),
                pw: "123456".to_string(),
            };

            let mc = TreatyClientConfig {
                user_address_port: client_addr.clone(),
                info_address_port: info_addr.clone(),
                client_type: TreatyClientType::Grpc,
                host_id: Some(setup.main.host_id.clone()),
                auth: Some(m_auth),
                tls: false,
                tls_settings: None,
                tls_http: None,
            };

            let pc = TreatyClientConfig {
                user_address_port: client_addr.clone(),
                info_address_port: info_addr.clone(),
                client_type: TreatyClientType::Grpc,
                host_id: Some(setup.part.as_ref().unwrap().host_id.clone()),
                auth: Some(p_auth),
                tls: false,
                tls_settings: None,
                tls_http: None,
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
                info_address: info_addr.clone(),
                tls_settings: None,
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
            let handle = tokio::spawn(async move {
                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: Some(pc),
                    test_db_name: db,
                    contract_desc: Some(config.contract_desc.as_ref().unwrap().clone()),
                    participant_db_addr: db_addr.clone(),
                    participant_info_addr: Some(info_addr.clone()),
                    grpc_test_setup: Some(grpc_test_setup),
                    http_test_setup: None,
                    participant_id: Some(participant_id.clone()),
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        // shutdown proxy services
        triggers.lock().unwrap().client.as_ref().unwrap().trigger();
        triggers.lock().unwrap().db.as_ref().unwrap().trigger();
        release_port(setup.proxy_info.client_addr.port);
        release_port(setup.proxy_info.info_addr.port);
        release_port(setup.proxy_info.db_addr.as_ref().unwrap().port);
    }

    /// takes a config for a test and will begin an HTTP GRPC test, using the
    /// provided `test_core` function to run
    pub async fn run_grpc_test<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let db = format!("{}{}", config.test_name, ".db");
        let root_dir = get_test_temp_dir(&config.test_name);
        let main_test_config =
            start_service_with_grpc(&db, root_dir, config.use_internal_logging).await;

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let mc = TreatyClientConfig {
                user_address_port: mtc.client_address,
                info_address_port: mtc.info_address,
                client_type: TreatyClientType::Grpc,
                host_id: None,
                auth: None,
                tls: false,
                tls_settings: None,
                tls_http: None,
            };

            let handle = tokio::spawn(async move {
                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: None,
                    test_db_name: db,
                    contract_desc: None,
                    participant_db_addr: None,
                    participant_info_addr: None,
                    grpc_test_setup: None,
                    http_test_setup: None,
                    participant_id: None,
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        let instances = vec![&main_test_config];
        shutdown_grpc_tests(instances);
    }

    pub async fn run_grpc_test_postgres_to_sqlite_multi<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let main_test_config = start_service_with_grpc_postgres(&config.test_name, false).await;

        // this is the sqlite format
        // let db = format!("{}{}", config.test_name, ".db");

        let db = config.test_name.clone();
        let root_dir = get_test_temp_dir(&config.test_name);
        let participant_test_config =
            start_service_with_grpc(&db, root_dir, config.use_internal_logging).await;

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();
            let contract = config.contract_desc;

            let handle = tokio::spawn(async move {
                let mc = TreatyClientConfig {
                    user_address_port: mtc.client_address.clone(),
                    info_address_port: mtc.info_address.clone(),
                    client_type: TreatyClientType::Grpc,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
                };
                let pc = TreatyClientConfig {
                    user_address_port: ptc.client_address.clone(),
                    info_address_port: ptc.info_address.clone(),
                    client_type: TreatyClientType::Grpc,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
                };
                let pda = ptc.database_address.clone();
                let pia = ptc.info_address.clone();

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
                    test_db_name: db.clone(),
                    contract_desc: contract,
                    participant_db_addr: Some(pda),
                    participant_info_addr: Some(pia),
                    grpc_test_setup: Some(grpc_test_setup),
                    http_test_setup: None,
                    participant_id: None,
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        let instances = vec![&main_test_config, &participant_test_config];
        shutdown_grpc_tests(instances);
    }

    pub async fn run_grpc_test_sqlite_to_postgres_multi<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let db = config.test_name.clone();

        // this is the sqlite format
        // let db = format!("{}{}", config.test_name, ".db");

        let root_dir = get_test_temp_dir(&config.test_name);
        let main_test_config =
            start_service_with_grpc(&db, root_dir, config.use_internal_logging).await;
        let participant_test_config =
            start_service_with_grpc_postgres(&config.test_name, false).await;

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();
            let contract = config.contract_desc;

            let handle = tokio::spawn(async move {
                let mc = TreatyClientConfig {
                    user_address_port: mtc.client_address.clone(),
                    info_address_port: mtc.info_address.clone(),
                    client_type: TreatyClientType::Grpc,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
                };
                let pc = TreatyClientConfig {
                    user_address_port: ptc.client_address.clone(),
                    info_address_port: ptc.info_address.clone(),
                    client_type: TreatyClientType::Grpc,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
                };
                let pda = ptc.database_address.clone();
                let pia = ptc.info_address.clone();

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
                    test_db_name: db.clone(),
                    contract_desc: contract,
                    participant_db_addr: Some(pda),
                    participant_info_addr: Some(pia),
                    grpc_test_setup: Some(grpc_test_setup),
                    http_test_setup: None,
                    participant_id: None,
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        let instances = vec![&main_test_config, &participant_test_config];
        shutdown_grpc_tests(instances);
    }

    /// takes a config for a test and will begin an HTTP Treaty test, using the
    /// provided `test_core` function to run
    pub async fn run_http_test_postgres<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let db = config.test_name.clone();
        let main_test_config = start_service_with_http_postgres(&config.test_name, false).await;

        sleep_test();

        {
            let mtc = main_test_config.clone();

            let handle = tokio::spawn(async move {
                let mc = TreatyClientConfig {
                    user_address_port: mtc.http_address.clone(),
                    info_address_port: mtc.http_address.clone(),
                    client_type: TreatyClientType::Http,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
                };

                let contract = config.contract_desc.clone();

                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: None,
                    test_db_name: db,
                    contract_desc: contract,
                    participant_db_addr: None,
                    participant_info_addr: None,
                    grpc_test_setup: None,
                    http_test_setup: None,
                    participant_id: None,
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        let tests = vec![&main_test_config];
        shutdown_http_tests(tests).await;
    }

    pub async fn run_http_test_postgres_multi<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let db = config.test_name.clone();
        let main_test_config = start_service_with_http_postgres(&config.test_name, false).await;
        let participant_test_config =
            start_service_with_http_postgres(&config.test_name, true).await;

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();

            let handle = tokio::spawn(async move {
                let mc = TreatyClientConfig {
                    user_address_port: mtc.http_address.clone(),
                    info_address_port: mtc.http_address.clone(),
                    client_type: TreatyClientType::Http,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
                };
                let pc = TreatyClientConfig {
                    user_address_port: ptc.http_address.clone(),
                    info_address_port: ptc.http_address.clone(),
                    client_type: TreatyClientType::Http,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
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
                    test_db_name: db.clone(),
                    contract_desc: contract,
                    participant_db_addr: Some(pda.clone()),
                    participant_info_addr: Some(pda.clone()),
                    grpc_test_setup: None,
                    http_test_setup: Some(http_test_setup),
                    participant_id: None,
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        let instances = vec![&main_test_config, &participant_test_config];
        shutdown_http_tests(instances).await;
    }

    pub async fn run_grpc_test_postgres_multi<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let db = config.test_name.clone();
        let main_test_config = start_service_with_grpc_postgres(&config.test_name, false).await;
        let participant_test_config =
            start_service_with_grpc_postgres(&config.test_name, true).await;

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();
            let contract = config.contract_desc;

            let handle = tokio::spawn(async move {
                let mc = TreatyClientConfig {
                    user_address_port: mtc.client_address.clone(),
                    info_address_port: mtc.info_address.clone(),
                    client_type: TreatyClientType::Grpc,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
                };
                let pc = TreatyClientConfig {
                    user_address_port: ptc.client_address.clone(),
                    info_address_port: ptc.info_address.clone(),
                    client_type: TreatyClientType::Grpc,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
                };
                let pda = ptc.database_address.clone();
                let pia = ptc.info_address.clone();

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
                    test_db_name: db.clone(),
                    contract_desc: contract,
                    participant_db_addr: Some(pda),
                    participant_info_addr: Some(pia),
                    grpc_test_setup: Some(grpc_test_setup),
                    http_test_setup: None,
                    participant_id: None,
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        let instances = vec![&main_test_config, &participant_test_config];
        shutdown_grpc_tests(instances);
    }

    /// Takes a config for a test and will begin an HTTP GRPC test using Postgres as the database, using the
    /// provided `test_core` function to run
    pub async fn run_grpc_test_postgres<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let main_test_config = start_service_with_grpc_postgres(&config.test_name, false).await;

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let mc = TreatyClientConfig {
                user_address_port: mtc.client_address,
                info_address_port: mtc.info_address,
                client_type: TreatyClientType::Grpc,
                host_id: None,
                auth: None,
                tls: false,
                tls_settings: None,
                tls_http: None,
            };

            let handle = tokio::spawn(async move {
                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: None,
                    test_db_name: config.test_name.clone(),
                    contract_desc: None,
                    participant_db_addr: None,
                    participant_info_addr: None,
                    grpc_test_setup: None,
                    http_test_setup: None,
                    participant_id: None,
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        let instances = vec![&main_test_config];
        shutdown_grpc_tests(instances);
    }

    /// takes a config for a test and will begin an HTTP GRPC test, using the
    /// provided `test_core` function to run
    pub async fn run_grpc_test_multi_tls<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let db = format!("{}{}", config.test_name, ".db");
        let dirs = get_test_temp_dir_main_and_participant(&config.test_name);

        let main_test_config =
            start_service_with_grpc_tls(&db, dirs.main_dir, config.use_internal_logging).await;
        let participant_test_config =
            start_service_with_grpc_tls(&db, dirs.participant_dir, config.use_internal_logging)
                .await;

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();
            let contract = config.contract_desc;

            let handle = tokio::spawn(async move {
                let mc = TreatyClientConfig {
                    user_address_port: mtc.client_address.clone(),
                    info_address_port: mtc.info_address.clone(),
                    client_type: TreatyClientType::Grpc,
                    host_id: None,
                    auth: None,
                    tls: true,
                    tls_settings: mtc.tls_settings.clone(),
                    tls_http: None,
                };
                let pc = TreatyClientConfig {
                    user_address_port: ptc.client_address.clone(),
                    info_address_port: ptc.info_address.clone(),
                    client_type: TreatyClientType::Grpc,
                    host_id: None,
                    auth: None,
                    tls: true,
                    tls_settings: ptc.tls_settings.clone(),
                    tls_http: None,
                };
                let pda = ptc.database_address.clone();
                let pia = ptc.info_address.clone();

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
                    participant_info_addr: Some(pia),
                    grpc_test_setup: Some(grpc_test_setup),
                    http_test_setup: None,
                    participant_id: None,
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        let instances = vec![&main_test_config, &participant_test_config];
        shutdown_grpc_tests(instances);
    }

    /// takes a config for a test and will begin an HTTP GRPC test, using the
    /// provided `test_core` function to run
    pub async fn run_grpc_test_multi<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let db = format!("{}{}", config.test_name, ".db");
        let dirs = get_test_temp_dir_main_and_participant(&config.test_name);

        let main_test_config =
            start_service_with_grpc(&db, dirs.main_dir, config.use_internal_logging).await;
        let participant_test_config =
            start_service_with_grpc(&db, dirs.participant_dir, config.use_internal_logging).await;

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();
            let contract = config.contract_desc;

            let handle = tokio::spawn(async move {
                let mc = TreatyClientConfig {
                    user_address_port: mtc.client_address.clone(),
                    info_address_port: mtc.info_address.clone(),
                    client_type: TreatyClientType::Grpc,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
                };
                let pc = TreatyClientConfig {
                    user_address_port: ptc.client_address.clone(),
                    info_address_port: ptc.info_address.clone(),
                    client_type: TreatyClientType::Grpc,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
                };
                let pda = ptc.database_address.clone();
                let pia = ptc.info_address.clone();

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
                    participant_info_addr: Some(pia),
                    grpc_test_setup: Some(grpc_test_setup),
                    http_test_setup: None,
                    participant_id: None,
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        let instances = vec![&main_test_config, &participant_test_config];
        shutdown_grpc_tests(instances);
    }

    /// takes a config for a test and will begin an HTTP Treaty test, using the
    /// provided `test_core` function to run for a main and a participant
    pub async fn run_http_test_multi_tls<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let db = format!("{}{}", config.test_name, ".db");

        let dirs = get_test_temp_dir_main_and_participant(&config.test_name);
        let main_test_config =
            start_service_with_http_tls(&db, dirs.main_dir, config.use_internal_logging).await;

        sleep_test();

        let participant_test_config =
            start_service_with_http_tls(&db, dirs.participant_dir, config.use_internal_logging)
                .await;

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();

            let handle = tokio::spawn(async move {
                let mc = TreatyClientConfig {
                    user_address_port: mtc.http_address.clone(),
                    info_address_port: mtc.http_address.clone(),
                    client_type: TreatyClientType::Http,
                    host_id: None,
                    auth: None,
                    tls: true,
                    tls_settings: Some(TlsSettings::default()),
                    tls_http: mtc.tls_opts.clone(),
                };
                let pc = TreatyClientConfig {
                    user_address_port: ptc.http_address.clone(),
                    info_address_port: ptc.http_address.clone(),
                    client_type: TreatyClientType::Http,
                    host_id: None,
                    auth: None,
                    tls: true,
                    tls_settings: Some(TlsSettings::default()),
                    tls_http: ptc.tls_opts.clone(),
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
                    participant_db_addr: Some(pda.clone()),
                    participant_info_addr: Some(pda.clone()),
                    grpc_test_setup: None,
                    http_test_setup: Some(http_test_setup),
                    participant_id: None,
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        let tests = vec![&main_test_config, &participant_test_config];

        shutdown_http_tests(tests).await;
    }

    /// takes a config for a test and will begin an HTTP Treaty test, using the
    /// provided `test_core` function to run for a main and a participant
    pub async fn run_http_test_multi<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let db = format!("{}{}", config.test_name, ".db");

        let dirs = get_test_temp_dir_main_and_participant(&config.test_name);
        let main_test_config =
            start_service_with_http(&db, dirs.main_dir, config.use_internal_logging).await;

        sleep_test();

        let participant_test_config =
            start_service_with_http(&db, dirs.participant_dir, config.use_internal_logging).await;

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();

            let handle = tokio::spawn(async move {
                let mc = TreatyClientConfig {
                    user_address_port: mtc.http_address.clone(),
                    info_address_port: mtc.http_address.clone(),
                    client_type: TreatyClientType::Http,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
                };
                let pc = TreatyClientConfig {
                    user_address_port: ptc.http_address.clone(),
                    info_address_port: ptc.http_address.clone(),
                    client_type: TreatyClientType::Http,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
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
                    participant_db_addr: Some(pda.clone()),
                    participant_info_addr: Some(pda.clone()),
                    grpc_test_setup: None,
                    http_test_setup: Some(http_test_setup),
                    participant_id: None,
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        let tests = vec![&main_test_config, &participant_test_config];

        shutdown_http_tests(tests).await;
    }

    /// takes a config for a test and will begin an HTTP Treaty test, using the
    /// provided `test_core` function to run
    pub async fn run_http_test<Fut>(
        config: RunnerConfig,
        f: impl FnOnce(CoreTestConfig) -> Fut + Send + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let db = format!("{}{}", config.test_name, ".db");

        let dirs = get_test_temp_dir_main_and_participant(&config.test_name);
        let main_test_config =
            start_service_with_http(&db, dirs.main_dir, config.use_internal_logging).await;

        sleep_test();

        {
            let mtc = main_test_config.clone();

            let handle = tokio::spawn(async move {
                let mc = TreatyClientConfig {
                    user_address_port: mtc.http_address.clone(),
                    info_address_port: mtc.http_address.clone(),
                    client_type: TreatyClientType::Http,
                    host_id: None,
                    auth: None,
                    tls: false,
                    tls_settings: None,
                    tls_http: None,
                };

                let contract = config.contract_desc.clone();

                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: None,
                    test_db_name: db,
                    contract_desc: contract,
                    participant_db_addr: None,
                    participant_info_addr: None,
                    grpc_test_setup: None,
                    http_test_setup: None,
                    participant_id: None,
                };

                f(config).await;
            });

            handle.await.unwrap();
        }

        let tests = vec![&main_test_config];
        shutdown_http_tests(tests).await;
    }
}
