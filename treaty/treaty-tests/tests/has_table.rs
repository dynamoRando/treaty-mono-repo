use treaty_types::enums::DatabaseType;
use treaty_tests::harness::get_treaty_client;
use treaty_tests::harness::CoreTestConfig;
use treaty_tests::harness::TreatyClientConfig;

pub fn test_core(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let db = config.test_db_name;
    let response = client(&db, &mc);

    assert!(response);
}

#[cfg(test)]
#[tokio::main]
async fn client(db_name: &str, config: &TreatyClientConfig) -> bool {
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);
    let mut client = get_treaty_client(config).await;

    client.create_user_database(db_name).await.unwrap();
    client.enable_cooperative_features(db_name).await.unwrap();

    let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

    client
        .execute_write_at_host(db_name, &drop_table_statement, database_type, "")
        .await
        .unwrap();

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    client.has_table(db_name, "EMPLOYEE").await.unwrap()
}

pub mod http {
    use crate::test_core;
    use treaty_tests::{
        runner::{RunnerConfig, TestRunner},
    };
    #[test]
    fn test() {
        let test_name = "has_table_http";

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
        init_trace_to_screen(false);

        let test_name = "has_table_gprc";
        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: None,
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test(config, test_core);
    }

    #[test]
    fn proxy() {
        let test_name = "has_table_gprc-proxy";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test(config, test_core);
    }
}
