use stdext::function_name;
use tracing::debug;
use treaty_tests::harness::get_treaty_client;
use treaty_tests::harness::tokio_sleep_core;
use treaty_tests::harness::CoreTestConfig;
use treaty_tests::harness::TreatyClientConfig;
use treaty_types::enums::DatabaseType;
use treaty_types::enums::LogicalStoragePolicy;
use treaty_types::enums::RemoteDeleteBehavior;

pub async fn test_core(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let db = config.test_db_name;

    tokio_sleep_core().await;

    let response = client(&db, &mc).await;
    assert!(response);
}

async fn client(db_name: &str, config: &TreatyClientConfig) -> bool {
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    let mut client = get_treaty_client(config).await;

    let db_config = client.get_backing_db_config().await.unwrap();

    let db_type = db_config.database_type;
    let db_type = DatabaseType::from_u32(db_type);
    let use_schema = db_config.use_schema;

    debug!(
        "[{}]: Test running for backing database type: {db_type:?} with schema setting {use_schema:?}.",
        function_name!()
    );

    if (db_type == DatabaseType::Postgres && use_schema == false) || db_type == DatabaseType::Sqlite
    {
        debug!("[{}]: create user database", function_name!());
        assert!(client.create_user_database(db_name).await.unwrap());
    } else {
        debug!("[{}]: SKIPPING create user database", function_name!());

        /*
        We skip testing for creating a user database because of the difference in implementation for a database system
        that can hold multiple schemas and one that cannot. In a database system that can not hold multiple schemas (MySQL, Sqlite),
        the `Treaty` schema and the user database we're testing against are two different databases, and we want to make sure
        that we can create the user database successfully, and that if we try to re-create the user database we don't overwrite
        blindly that database.

        In a database system that can have multiple schemas in one database (Postgres, MS SQL Server), we keep the `Treaty` schema
        in the actual user database itself, and keep the user defined tables in the default schema (`public` for Postgres, `dbo` for MS SQL Server).
        */
    }

    debug!("[{}]: enable cooperative features", function_name!());
    assert!(client.enable_cooperative_features(db_name).await.unwrap());

    assert!(client
        .execute_write_at_host(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type, "")
        .await
        .unwrap());

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    debug!("[{}]: execute write at host", function_name!());
    assert!(client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap());

    let logical_storage_policy = LogicalStoragePolicy::ParticipantOwned;

    debug!("[{}]: set logical storage policy", function_name!());
    assert!(client
        .set_logical_storage_policy(db_name, "EMPLOYEE", logical_storage_policy)
        .await
        .unwrap());

    let behavior = RemoteDeleteBehavior::Ignore;

    debug!("[{}]: generate contract", function_name!());
    assert!(client
        .generate_contract(db_name, "tester", "desc", behavior)
        .await
        .unwrap());

    let result = client.auth_for_token().await.unwrap();

    debug!("{result:?}");

    assert!(result.is_successful);

    debug!("[{}]: get databases", function_name!());
    let databases = client.get_databases().await.unwrap();

    let has_db = databases
        .databases
        .iter()
        .any(|x| x.database_name.as_str() == db_name);

    has_db
}

pub mod http {

    use crate::test_core;
    use treaty_tests::{
        harness::init_trace_to_screen,
        runner::{RunnerConfig, TestRunner},
    };

    #[tokio::test]
    async fn test() {
        let test_name = "auth_token_http";
        init_trace_to_screen(false, None);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: None,
            use_internal_logging: false,
        };

        TestRunner::run_http_test(config, test_core).await;
    }

    #[tokio::test]
    pub async fn postgres() {
        let test_name = "auth_token_http_postgres";
        init_trace_to_screen(false, Some(String::from("auth_for_token=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_postgres(config, test_core).await;
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

        let test_name = "auth_token_grpc";
        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: None,
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test(config, test_core).await;
    }

    #[tokio::test]
    async fn proxy() {
        let test_name = "auth_token_grpc-proxy";
        init_trace_to_screen(false, None);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test(config, test_core).await;
    }

    #[tokio::test]
    async fn postgres() {
        let test_name = "auth_token_grpc_postgres";
        init_trace_to_screen(false, Some(String::from("auth_for_token=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_postgres(config, test_core).await;
    }
}
