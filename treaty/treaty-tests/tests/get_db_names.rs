use stdext::function_name;
use tracing::{debug, trace, warn};
use treaty::treaty_proto::GetDatabasesReply;
use treaty_client::TreatyClientType;
use treaty_tests::common_contract_setup::main_and_participant_setup;
use treaty_tests::harness::{get_treaty_client, CoreTestConfig};

pub async fn test_core(config: CoreTestConfig) {
    go(config).await
}

async fn go(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let pc = config.participant_client.as_ref().unwrap().clone();
    let result = main_and_participant_setup(config.clone()).await;
    trace!("[{}]: initial setup complete.", function_name!());
    assert!(result);

    let mut pc = get_treaty_client(&pc).await;
    let mut mc = get_treaty_client(&mc).await;

    let mut ignore_db_suffix = false;

    // create main dbs
    {
        let intial_db_created = mc.create_user_database("get_db_names2.db").await.unwrap();

        trace!(
            "[{}]: initial_db_created: {intial_db_created:?}",
            function_name!()
        );

        if !intial_db_created {
            // this might be a database system that doesn't support `.` in the name, so
            // attempt the test without them
            ignore_db_suffix = true;
            warn!("[{}]: We might need to make sure these databases don't exist first by dropping them.", function_name!());

            trace!("[{}]: drop database forcefully", function_name!());
            mc.drop_database_forcefully("get_db_names2").await.unwrap();
            trace!("[{}]: create user database", function_name!());
            mc.create_user_database("get_db_names2").await.unwrap();
            assert!(has_database(
                mc.get_databases().await.unwrap(),
                "get_db_names2"
            ));
        }

        if !ignore_db_suffix {
            mc.create_user_database("get_db_names3.db").await.unwrap();
        } else {
            trace!("[{}]: drop database forcefully", function_name!());
            mc.drop_database_forcefully("get_db_names3").await.unwrap();
            trace!("[{}]: create user database", function_name!());
            mc.create_user_database("get_db_names3").await.unwrap();
            assert!(has_database(
                mc.get_databases().await.unwrap(),
                "get_db_names3"
            ));
        }
    }

    // create part dbs
    {
        if !ignore_db_suffix {
            pc.create_user_database("part_example.db").await.unwrap();
            pc.create_user_database("part_example2.db").await.unwrap();
            pc.create_user_database("part_example3.db").await.unwrap();
        } else {
            trace!("[{}]: drop database forcefully", function_name!());
            pc.drop_database_forcefully("part_example").await.unwrap();
            pc.create_user_database("part_example").await.unwrap();
            assert!(has_database(
                pc.get_databases().await.unwrap(),
                "part_example"
            ));

            trace!("[{}]: drop database forcefully", function_name!());
            pc.drop_database_forcefully("part_example2").await.unwrap();
            pc.create_user_database("part_example2").await.unwrap();
            assert!(has_database(
                pc.get_databases().await.unwrap(),
                "part_example2"
            ));

            trace!("[{}]: drop database forcefully", function_name!());

            pc.drop_database_forcefully("part_example3").await.unwrap();
            pc.create_user_database("part_example3").await.unwrap();
            assert!(has_database(
                mc.get_databases().await.unwrap(),
                "part_example3"
            ));
        }
    }

    // participant should have all databases
    {
        let mut has_all_databases = true;

        trace!("[{}]: get databases...", function_name!());
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
                    if !ignore_db_suffix {
                        expected_db_names = [
                            "part_example.db",
                            "part_example2.db",
                            "part_example3.db",
                            "get_db_names_grpc.dbpart",
                            "treaty.db",
                        ];
                    } else {
                        expected_db_names = [
                            "part_example",
                            "part_example2",
                            "part_example3",
                            "get_db_names_grpc_postgres",
                            "get_db_names_grpc_postgres",
                        ];
                    }
                } else {
                    if !ignore_db_suffix {
                        expected_db_names = [
                            "get_db_names_grpc-proxy.dbpart",
                            "part_example3.db",
                            "part_example2.db",
                            "part_example.db",
                            "treaty.db",
                        ];
                    } else {
                        expected_db_names = [
                            "get_db_names_grpc-proxy",
                            "part_example3",
                            "part_example2",
                            "part_example",
                            "get_db_names_grpc_postgres",
                        ];
                    }
                }
            }
            TreatyClientType::Http => {
                if config
                    .participant_client
                    .as_ref()
                    .unwrap()
                    .host_id
                    .is_none()
                {
                    if !ignore_db_suffix {
                        expected_db_names = [
                            "part_example.db",
                            "part_example2.db",
                            "part_example3.db",
                            "get_db_names_http.dbpart",
                            "treaty.db",
                        ];
                    } else {
                        expected_db_names = [
                            "part_example",
                            "part_example2",
                            "part_example3",
                            "get_db_names_http_postgres",
                            "get_db_names_http_postgres",
                        ];
                    }
                } else {
                    if !ignore_db_suffix {
                        expected_db_names = [
                            "get_db_names_http-proxy.dbpart",
                            "part_example3.db",
                            "part_example2.db",
                            "part_example.db",
                            "treaty.db",
                        ];
                    } else {
                        expected_db_names = [
                            "get_db_names_http-proxy",
                            "part_example3",
                            "part_example2",
                            "part_example",
                            "get_db_names_http_postgres",
                        ];
                    }
                }
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
                    if !ignore_db_suffix {
                        expected_db_names = [
                            "get_db_names2.db",
                            "get_db_names3.db",
                            "get_db_names_grpc.db",
                            "treaty.db",
                        ];
                    } else {
                        expected_db_names = [
                            "get_db_names2",
                            "get_db_names3",
                            "get_db_names_grpc_postgres",
                            "get_db_names_grpc_postgres",
                        ];
                    }
                } else {
                    if !ignore_db_suffix {
                        expected_db_names = [
                            "get_db_names2.db",
                            "get_db_names3.db",
                            "get_db_names_grpc-proxy.db",
                            "treaty.db",
                        ];
                    } else {
                        expected_db_names = [
                            "get_db_names2",
                            "get_db_names3",
                            "get_db_names_grpc-proxy",
                            "get_db_names_grpc_postgres",
                        ];
                    }
                }
            }
            TreatyClientType::Http => {
                if config
                    .participant_client
                    .as_ref()
                    .unwrap()
                    .host_id
                    .is_none()
                {
                    if !ignore_db_suffix {
                        expected_db_names = [
                            "get_db_names2.db",
                            "get_db_names3.db",
                            "get_db_names_http.db",
                            "treaty.db",
                        ];
                    } else {
                        expected_db_names = [
                            "get_db_names2",
                            "get_db_names3",
                            "get_db_names_http_postgres",
                            "get_db_names_http_postgres",
                        ];
                    }
                } else {
                    if !ignore_db_suffix {
                        expected_db_names = [
                            "get_db_names2.db",
                            "get_db_names3.db",
                            "get_db_names_http-proxy.db",
                            "treaty.db",
                        ];
                    } else {
                        expected_db_names = [
                            "get_db_names2",
                            "get_db_names3",
                            "get_db_names_http-proxy",
                            "get_db_names_http_postgres",
                        ];
                    }
                }
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

fn has_database(db_reply: GetDatabasesReply, db_name: &str) -> bool {
    let mut actual_db_names: Vec<String> = Vec::new();
    for db in &db_reply.databases {
        trace!("{}", db.database_name.clone());
        actual_db_names.push(db.database_name.clone());
    }

    actual_db_names.into_iter().any(|d| d == db_name)
}

pub mod http {
    use crate::test_core;
    use treaty_tests::{
        harness::init_trace_to_screen,
        runner::{RunnerConfig, TestRunner},
    };

    #[tokio::test]
    async fn test() {
        init_trace_to_screen(false, None);

        let test_name = "get_db_names_http";
        let contract = String::from("");

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(contract),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_multi(config, test_core).await;
    }
}

pub mod grpc {
    use crate::test_core;
    use treaty_tests::{
        harness::init_trace_to_screen,
        runner::{RunnerConfig, TestRunner},
    };

    #[tokio::test]
    async fn test() {
        init_trace_to_screen(false, None);

        let test_name = "get_db_names_grpc";
        let contract = String::from("");
        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(contract),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_multi(config, test_core).await;
    }

    #[tokio::test]
    async fn proxy() {
        init_trace_to_screen(false, None);

        let test_name = "get_db_names_grpc-proxy";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test_multi(config, test_core).await;
    }
}
