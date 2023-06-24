use tracing::debug;
use treaty_types::enums::DatabaseType;
use treaty_tests::harness::get_treaty_client;
use treaty_tests::harness::CoreTestConfig;
use treaty_tests::harness::TreatyClientConfig;

pub fn test_core(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let response = client(&config.test_db_name, &mc);
    debug!("create_db_enable_coop_read_write: got: is_error: {response}");

    assert!(!response);
}

#[tokio::main]
async fn client(db_name: &str, main_client: &TreatyClientConfig) -> bool {
    let mut client = get_treaty_client(main_client).await;

    let is_online = client.is_online().await.unwrap();
    assert!(is_online);

    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    let is_db_created = client.create_user_database(db_name).await.unwrap();

    assert!(is_db_created);

    let enable_coop_features = client.enable_cooperative_features(db_name).await.unwrap();
    let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

    assert!(enable_coop_features);

    let execute_write_drop_is_successful: bool = client
        .execute_write_at_host(db_name, &drop_table_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_drop_is_successful);

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    let execute_write_create_reply_is_successful: bool = client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_create_reply_is_successful);

    let add_record_statement = String::from("INSERT INTO EMPLOYEE (Id, Name) VALUES (1, 'Randy');");

    let execute_write_add_record_is_successful: bool = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_add_record_is_successful);

    let read_record_statement = String::from("SELECT Id, Name FROM EMPLOYEE");
    let read_reply = client
        .execute_read_at_host(db_name, &read_record_statement, database_type)
        .await
        .unwrap();

    read_reply.error.is_some()
}

pub mod http {
    use crate::test_core;
    use treaty_tests::{
        harness::init_trace_to_screen,
        runner::{RunnerConfig, TestRunner},
    };

    #[test]
    fn test() {
        let test_name = "create_db_enable_coop_read_write_http";
        init_trace_to_screen(false);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: None,
            use_internal_logging: false,
        };

        TestRunner::run_http_test(config, test_core);
    }
}

pub mod grpc {
    use crate::test_core;
    use treaty_tests::{
        harness::init_trace_to_screen,
        runner::{RunnerConfig, TestRunner},
    };
    #[test]
    fn test() {
        let test_name = "create_db_enable_coop_read_write_grpc";
        init_trace_to_screen(false);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: None,
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test(config, test_core);
    }

    #[test]
    fn proxy() {
        init_trace_to_screen(false);

        let test_name = "create_db_enable_coop_read_write_grpc-proxy";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test(config, test_core);
    }
}
