use stdext::function_name;
use tracing::debug;
use tracing::trace;
use treaty_tests::harness::get_treaty_client;
use treaty_tests::harness::CoreTestConfig;
use treaty_tests::harness::TreatyClientConfig;
use treaty_types::enums::DatabaseType;
use treaty_types::enums::LogicalStoragePolicy;
use treaty_types::enums::RemoteDeleteBehavior;

pub async fn test_core(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let response = client(&config.test_db_name, &mc).await;
    debug!("has settings: got: {response}");

    assert!(response);
}

async fn client(db_name: &str, config: &TreatyClientConfig) -> bool {
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    debug!("{config:?}");

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

    let logical_storage_policy = LogicalStoragePolicy::HostOnly;

    client
        .set_logical_storage_policy(db_name, "EMPLOYEE", logical_storage_policy)
        .await
        .unwrap();

    let behavior = RemoteDeleteBehavior::Ignore;

    client
        .generate_contract(db_name, "tester", "desc", behavior)
        .await
        .unwrap();

    client
        .execute_write_at_host(
            db_name,
            "INSERT INTO EMPLOYEE ( Id, Name ) VALUES ( 1234, 'Rando');",
            DatabaseType::to_u32(DatabaseType::Sqlite),
            "",
        )
        .await
        .unwrap();

    client.has_table(db_name, "EMPLOYEE").await.unwrap();

    let settings = client.get_settings().await.unwrap().settings_json;

    trace!("[{}]: {settings:?}", function_name!());

    settings.is_some()
}

pub mod http {
    use crate::test_core;
    use treaty_tests::runner::{RunnerConfig, TestRunner};

    #[tokio::test]
    async fn test() {
        let test_name = "get_settings_http";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: None,
            use_internal_logging: false,
        };

        TestRunner::run_http_test(config, test_core).await;
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

        let test_name = "get_settings_grpc";
        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: None,
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test(config, test_core).await;
    }

    #[tokio::test]
    async fn postgres() {
        let test_name = "get_settings_grpc_postgres";
        init_trace_to_screen(false, Some(String::from("get_settings=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_postgres(config, test_core).await;
    }

    #[tokio::test]
    async fn proxy() {
        let test_name = "get_settings_grpc-proxy";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test(config, test_core).await;
    }
}
