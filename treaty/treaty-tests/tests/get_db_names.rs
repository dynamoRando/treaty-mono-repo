use tracing::{trace, debug, warn};
use treaty_client::TreatyClientType;
use treaty_tests::common_contract_setup::main_and_participant_setup;
use treaty_tests::harness::{CoreTestConfig, get_treaty_client};

pub fn test_core(config: CoreTestConfig) {
    go(config)
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let pc = config.participant_client.as_ref().unwrap().clone();
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);

    let mut pc = get_treaty_client(&pc).await;
    let mut mc = get_treaty_client(&mc).await;

    // create main dbs
    {
        mc.create_user_database("get_db_names2.db").await.unwrap();
        mc.create_user_database("get_db_names3.db").await.unwrap();
    }

    // create part dbs
    {
        pc.create_user_database("part_example.db").await.unwrap();
        pc.create_user_database("part_example2.db").await.unwrap();
        pc.create_user_database("part_example3.db").await.unwrap();
    }

    // participant should have all databases
    {
        let mut has_all_databases = true;

        let result = pc.get_databases().await;

        let dbs_reply = result.unwrap();

        let mut actual_db_names: Vec<String> = Vec::new();

        trace!("actual names");

        for db in &dbs_reply.databases {
            trace!("{}", db.database_name.clone());
            actual_db_names.push(db.database_name.clone());
        }

        let expected_db_names: [&str; 5];

        match config
            .participant_client
            .as_ref()
            .unwrap()
            .clone()
            .client_type
        {
            TreatyClientType::Grpc => {
                if config
                    .participant_client
                    .as_ref()
                    .unwrap()
                    .host_id
                    .is_none()
                {
                    expected_db_names = [
                        "part_example.db",
                        "part_example2.db",
                        "part_example3.db",
                        "get_db_names_grpc.dbpart",
                        "treaty.db",
                    ];
                } else {
                    expected_db_names = [
                        "get_db_names_grpc-proxy.dbpart",
                        "part_example3.db",
                        "part_example2.db",
                        "part_example.db",
                        "treaty.db",
                    ];
                }
            }
            TreatyClientType::Http => {
                expected_db_names = [
                    "part_example.db",
                    "part_example2.db",
                    "part_example3.db",
                    "get_db_names_http.dbpart",
                    "treaty.db",
                ];
            }
        }

        trace!("expected names");
        for name in &expected_db_names {
            trace!("{name}");
        }

        debug!("actual: {:?}", actual_db_names);
        debug!("expected: {:?}", expected_db_names);

        for name in &expected_db_names {
            if !actual_db_names.iter().any(|n| n == name) {
                warn!("missing database: {:?}", name);
                has_all_databases = false;
            }
        }

        assert!(has_all_databases);
    }

    // main should have all databases
    {
        let mut has_all_databases = true;
        let result = mc.get_databases().await;

        let dbs_reply = result.unwrap();

        let mut actual_db_names: Vec<String> = Vec::new();

        trace!("actual names");

        for db in &dbs_reply.databases {
            trace!("{}", db.database_name.clone());
            actual_db_names.push(db.database_name.clone());
        }

        let expected_db_names: [&str; 4];

        match config.main_client.client_type {
            TreatyClientType::Grpc => {
                if config
                    .participant_client
                    .as_ref()
                    .unwrap()
                    .host_id
                    .is_none()
                {
                    expected_db_names = [
                        "get_db_names2.db",
                        "get_db_names3.db",
                        "get_db_names_grpc.db",
                        "treaty.db",
                    ];
                } else {
                    expected_db_names = [
                        "get_db_names2.db",
                        "get_db_names3.db",
                        "get_db_names_grpc-proxy.db",
                        "treaty.db",
                    ];
                }
            }
            TreatyClientType::Http => {
                expected_db_names = [
                    "get_db_names2.db",
                    "get_db_names3.db",
                    "get_db_names_http.db",
                    "treaty.db",
                ];
            }
        }

        trace!("expected names");
        for name in &expected_db_names {
            trace!("{name}");
        }

        for name in &expected_db_names {
            if !actual_db_names.contains(&(*name).to_string()) {
                warn!("missing database: {:?}", name);
                has_all_databases = false;
            }
        }

        assert!(has_all_databases);
    }
}

pub mod http {
    use crate::test_core;
    use treaty_tests::{
        runner::{RunnerConfig, TestRunner}, harness::init_trace_to_screen,
    };

    #[test]
    fn test() {
        init_trace_to_screen(false);

        let test_name = "get_db_names_http";
        let contract = String::from("");

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(contract),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_multi(config, test_core);
    }
}

pub mod grpc {
    use crate::test_core;
    use treaty_tests::{runner::{RunnerConfig, TestRunner}, harness::init_trace_to_screen};

    #[test]
    fn test() {
        init_trace_to_screen(false);

        let test_name = "get_db_names_grpc";
        let contract = String::from("");
        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(contract),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_multi(config, test_core);
    }

    #[test]
    fn proxy() {
        init_trace_to_screen(false);

        let test_name = "get_db_names_grpc-proxy";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test_multi(config, test_core);
    }
}
