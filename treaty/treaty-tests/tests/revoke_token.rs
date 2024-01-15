use tracing::debug;
use treaty_tests::harness::get_treaty_client;
use treaty_tests::harness::CoreTestConfig;
use treaty_tests::harness::TreatyClientConfig;
use treaty_types::enums::DatabaseType;
use treaty_types::enums::LogicalStoragePolicy;
use treaty_types::enums::RemoteDeleteBehavior;

pub async fn test_core(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let db = config.test_db_name;
    let response = client(&db, &mc).await;
    assert!(response);
}

async fn client(db_name: &str, config: &TreatyClientConfig) -> bool {
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    let mut client = get_treaty_client(config).await;

    debug!("!! ------------ REVOKE TOKEN: Create User Database ------------ !!");
    client.create_user_database(db_name).await.unwrap();

    debug!("!! ------------ REVOKE TOKEN: Enable Cooperative Features ------------ !!");
    client.enable_cooperative_features(db_name).await.unwrap();
    client
        .execute_write_at_host(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type, "")
        .await
        .unwrap();

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    debug!("!! ------------ REVOKE TOKEN: Create TAble ------------ !!");
    client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    let logical_storage_policy = LogicalStoragePolicy::ParticipantOwned;

    debug!("!! ------------ REVOKE TOKEN: Set Logical Storage Policy ------------ !!");
    client
        .set_logical_storage_policy(db_name, "EMPLOYEE", logical_storage_policy)
        .await
        .unwrap();

    let behavior = RemoteDeleteBehavior::Ignore;

    debug!("!! ------------ REVOKE TOKEN: Generate Contract ------------ !!");
    client
        .generate_contract(db_name, "tester", "desc", behavior)
        .await
        .unwrap();

    debug!("!! ------------ REVOKE TOKEN: Auth for token ------------ !!");
    let result = client.auth_for_token().await.unwrap();

    debug!("{result:?}");
    debug!("!! ------------ REVOKE TOKEN: Get Databases ------------ !!");
    let _ = client.get_databases().await.unwrap();

    debug!("!! ------------ REVOKE TOKEN: Revoke token ------------ !!");
    let revoke = client.revoke_token().await.unwrap();

    revoke.is_successful
}

pub mod http {
    use crate::test_core;
    use treaty_tests::runner::{RunnerConfig, TestRunner};

    #[tokio::test]
    async fn test() {
        let test_name = "revoke_token_http";

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

        let test_name = "revoke_token_grpc";
        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: None,
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test(config, test_core).await;
    }

    #[tokio::test]
    async fn postgres() {
        let test_name = "revoke_token_postgres_grpc";
        init_trace_to_screen(false, Some(String::from("revoke_token=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_postgres(config, test_core).await;
    }

    #[tokio::test]
    async fn proxy() {
        let test_name = "revoke_token_grpc-proxy";
        init_trace_to_screen(false, None);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test(config, test_core).await;
    }
}
