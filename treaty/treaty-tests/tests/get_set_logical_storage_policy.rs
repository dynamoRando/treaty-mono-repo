use treaty_types::enums::DatabaseType;
use treaty_types::enums::LogicalStoragePolicy;
use treaty_tests::harness::get_treaty_client;
use treaty_tests::harness::CoreTestConfig;
use treaty_tests::harness::TreatyClientConfig;

pub fn test_core(config: CoreTestConfig) {
    let db = config.test_db_name.clone();

    let policy = LogicalStoragePolicy::ParticpantOwned;
    let i_policy = LogicalStoragePolicy::to_u32(policy);
    let response = client(&db, config.main_client, i_policy);
    assert_eq!(i_policy, response);
}

#[tokio::main]
async fn client(db_name: &str, client: TreatyClientConfig, policy_num: u32) -> u32 {

    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    let mut client = get_treaty_client(&client).await;

    let create_db_is_successful = client.create_user_database(db_name).await.unwrap();

    assert!(create_db_is_successful);

    let enable_coop_features_is_successful =
        client.enable_cooperative_features(db_name).await.unwrap();

    let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

    assert!(enable_coop_features_is_successful);

    let drop_table_is_successful = client
        .execute_write_at_host(db_name, &drop_table_statement, database_type, "")
        .await
        .unwrap();

    assert!(drop_table_is_successful);

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    let create_table_is_successful = client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    assert!(create_table_is_successful);

    let add_record_statement = String::from("INSERT INTO EMPLOYEE (Id, Name) VALUES (1, 'Randy');");

    let execute_write_is_successful = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_is_successful);

    let read_record_statement = String::from("SELECT Id, Name FROM EMPLOYEE");

    let result = client
        .execute_read_at_host(db_name, &read_record_statement, database_type)
        .await
        .unwrap();

    assert!(!result.error.is_some());

    let _set_policy_is_successful = client
        .set_logical_storage_policy(
            db_name,
            "EMPLOYEE",
            LogicalStoragePolicy::from_i64(policy_num as i64),
        )
        .await
        .unwrap();

    let policy_response = client
        .get_logical_storage_policy(db_name, "EMPLOYEE")
        .await
        .unwrap();

    LogicalStoragePolicy::to_u32(policy_response)
}

pub mod http {
    use crate::test_core;
    use treaty_tests::runner::{RunnerConfig, TestRunner};

    #[test]
    fn test() {
        let test_name = "get_set_logical_storage_policy_http";

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
    use treaty_tests::runner::{RunnerConfig, TestRunner};
    #[test]
    fn test() {
        let test_name = "get_set_logical_storage_policy_grpc";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: None,
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test(config, test_core);
    }

    #[test]
    fn proxy() {
        // treaty_test_harness::init_log_to_screen_fern(tracing::LevelFilter::Debug);

        let test_name = "get_set_logical_storage_policy_grpc-proxy";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test(config, test_core);
    }
}
