use tracing::trace;
use treaty_tests::{
    common_contract_setup::main_and_participant_setup,
    harness::{get_treaty_client, CoreTestConfig},
};
use treaty_types::enums::DatabaseType;

pub mod grpc {

    use treaty_tests::{
        harness::init_trace_to_screen,
        runner::{RunnerConfig, TestRunner},
    };

    use crate::test_core;

    #[tokio::test]
    async fn test() {
        let test_name = "add_read_update_remote_grpc";
        init_trace_to_screen(false, None);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_multi(config, test_core).await;
    }

    #[tokio::test]
    async fn proxy() {
        let test_name = "add_read_update_remote_grpc-proxy";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test_multi(config, test_core).await;
    }

    #[tokio::test]
    async fn postgres() {
        let test_name = "add_read_update_remote_grpc_postgres";
        init_trace_to_screen(false, Some(String::from("db_setup_insert=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_postgres_multi(config, test_core).await;
    }
}

pub mod http {

    #[tokio::test]
    async fn test() {
        use crate::test_core;
        use treaty_tests::{
            harness::init_trace_to_screen,
            runner::{RunnerConfig, TestRunner},
        };

        let test_name = "add_read_update_remote_http";
        init_trace_to_screen(false, None);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_multi(config, test_core).await;
    }
}

async fn test_core(config: CoreTestConfig) {
    go(config).await;
}

async fn go(config: CoreTestConfig) {
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);
    let db_name = &config.test_db_name;
    let mut client = get_treaty_client(&config.main_client).await;

    let update_result = client
        .execute_cooperative_write_at_host(
            db_name,
            "UPDATE EMPLOYEE SET NAME = 'Bob' WHERE Id = 999;",
            "participant",
            "Id = 999",
        )
        .await
        .unwrap();

    assert!(update_result);

    let new_data = client
        .execute_read_at_host(
            db_name,
            "SELECT Name FROM EMPLOYEE",
            DatabaseType::to_u32(DatabaseType::Sqlite),
        )
        .await
        .unwrap();

    trace!("{new_data:#?}");

    let new_value = new_data
        .rows
        .first()
        .unwrap()
        .values
        .iter()
        .find(|v| v.column.as_ref().unwrap().column_name.to_lowercase() == "name")
        .unwrap()
        .value
        .clone();

    trace!("assert left: {new_value:#?}");
    let expected_value = "Bob".as_bytes().to_vec();
    trace!("assert right: {expected_value:#?}");

    assert!(new_value == expected_value);
}
