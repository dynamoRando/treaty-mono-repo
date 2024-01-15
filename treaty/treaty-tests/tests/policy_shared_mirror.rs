use tracing::trace;
use treaty_tests::common_contract_setup::main_and_participant_setup;
use treaty_tests::harness::get_treaty_client;
use treaty_tests::harness::CoreTestConfig;
use treaty_tests::harness::TreatyClientConfig;
use treaty_types::enums::DatabaseType;

pub async fn test_core(config: CoreTestConfig) {
    go(config).await;
}

async fn go(config: CoreTestConfig) {
    let result = main_and_participant_setup(config.clone()).await;

    assert!(result);

    client(&config.test_db_name, config.main_client.clone()).await;
    participant(
        &config.test_db_name,
        config.participant_client.as_ref().unwrap().clone(),
    )
    .await;
    write_client(&config.test_db_name, config.main_client.clone()).await;
    check_participant(
        &config.test_db_name,
        config.participant_client.as_ref().unwrap().clone(),
    )
    .await;
}

async fn client(db_name: &str, client: TreatyClientConfig) {
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);
    let mut client = get_treaty_client(&client).await;

    let add_record_statement =
        String::from("INSERT INTO SHARED_ENTRIES (Id, Notes) VALUES (1, 'Shared Entry 1');");

    let execute_write_is_successful = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_is_successful);

    let add_record_statement =
        String::from("INSERT INTO SHARED_ENTRIES (Id, Notes) VALUES (2, 'Shared Entry 2');");

    let execute_write_is_successful = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_is_successful);

    let add_record_statement =
        String::from("INSERT INTO SHARED_ENTRIES (Id, Notes) VALUES (3, 'Shared Entry 3');");

    let execute_write_is_successful = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_is_successful);

    let add_record_statement =
        String::from("INSERT INTO MIRRORED_ENTRIES (Id, Notes) VALUES (1, 'Mirrored Entry 1');");

    let execute_write_is_successful = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_is_successful);

    let add_record_statement =
        String::from("INSERT INTO MIRRORED_ENTRIES (Id, Notes) VALUES (2, 'Mirrored Entry 2');");

    let execute_write_is_successful = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_is_successful);

    let add_record_statement =
        String::from("INSERT INTO MIRRORED_ENTRIES (Id, Notes) VALUES (3, 'Mirrored Entry 3');");

    let execute_write_is_successful = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_is_successful);
}

async fn participant(db_name: &str, client: TreatyClientConfig) {
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);
    let mut client = get_treaty_client(&client).await;

    let read_record_statement = String::from("SELECT Id, Notes FROM SHARED_ENTRIES");

    let result = client
        .execute_read_at_participant(db_name, &read_record_statement, database_type)
        .await
        .unwrap();

    assert!(!result.error.is_some());
    assert!(!result.rows.is_empty());

    let read_record_statement = String::from("SELECT Id, Notes FROM MIRRORED_ENTRIES");

    let result = client
        .execute_read_at_participant(db_name, &read_record_statement, database_type)
        .await
        .unwrap();

    assert!(!result.error.is_some());
    assert!(!result.rows.is_empty());
}

async fn write_client(db_name: &str, client: TreatyClientConfig) {
    // execute update and delete statements at the host
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);
    let mut client = get_treaty_client(&client).await;

    let add_record_statement =
        String::from("UPDATE SHARED_ENTRIES SET NOTES = 'Modified' WHERE Id = 2;");

    let execute_write_is_successful = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "Id = 2")
        .await
        .unwrap();

    assert!(execute_write_is_successful);

    let add_record_statement = String::from("DELETE FROM SHARED_ENTRIES WHERE Id = 3;");

    let execute_write_is_successful = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "Id = 3")
        .await
        .unwrap();

    assert!(execute_write_is_successful);

    let add_record_statement =
        String::from("UPDATE MIRRORED_ENTRIES SET NOTES = 'Modified' WHERE Id = 2;");

    let execute_write_is_successful = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "Id = 2")
        .await
        .unwrap();

    assert!(execute_write_is_successful);

    let add_record_statement = String::from("DELETE FROM MIRRORED_ENTRIES WHERE Id = 3;");

    let execute_write_is_successful = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "Id = 3")
        .await
        .unwrap();

    assert!(execute_write_is_successful);
}

async fn check_participant(db_name: &str, client: TreatyClientConfig) {
    // check that at the participant that for the shared table that there are records in the queue
    // and for the mirrored table that there are records in the logs table
    let mut client = get_treaty_client(&client).await;

    let cmd = "SELECT * FROM SHARED_ENTRIES_COOP_DATA_QUEUE;";
    let read_result = client
        .execute_read_at_participant(&db_name, cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
        .await
        .unwrap();

    trace!("{read_result:?}");

    assert!(!read_result.rows.is_empty());

    let cmd = "SELECT * FROM MIRRORED_ENTRIES_COOP_DATA_LOG;";
    let read_result = client
        .execute_read_at_participant(&db_name, cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
        .await
        .unwrap();

    trace!("{read_result:?}");

    assert!(!read_result.rows.is_empty());
}

pub mod grpc {
    use treaty_tests::{
        harness::init_trace_to_screen,
        runner::{RunnerConfig, TestRunner},
    };

    use crate::test_core;

    #[tokio::test]
    pub async fn test() {
        let test_name = "sa_contract_share_mirror_grpc";
        init_trace_to_screen(false, Some(String::from("policy_shared_mirror=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_multi(config, test_core).await;
    }

    
    #[tokio::test]
    pub async fn tls() {
        let test_name = "sa_contract_share_mirror_grpc_tls";
        init_trace_to_screen(false, Some(String::from("policy_shared_mirror=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_multi_tls(config, test_core).await;
    }

    #[tokio::test]
    pub async fn proxy() {
        let test_name = "sa_contract_share_mirror_grpc-proxy";
        init_trace_to_screen(false, None);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test_multi(config, test_core).await;
    }

    #[tokio::test]
    pub async fn postgres() {
        let test_name = "sa_contract_grpc_share_mirror_postgres";
        init_trace_to_screen(false, Some(String::from("policy_shared_mirror=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_postgres_multi(config, test_core).await;
    }

    #[tokio::test]
    pub async fn postgres_sqlite() {
        let test_name = "sa_contract_grpc_share_mirror_postgres_sqlite";
        init_trace_to_screen(false, Some(String::from("policy_shared_mirror=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_postgres_to_sqlite_multi(config, test_core).await;
    }

    #[tokio::test]
    pub async fn sqlite_postgres() {
        let test_name = "sa_contract_grpc_sqlite_share_mirror_postgres";
        init_trace_to_screen(false, Some(String::from("policy_shared_mirror=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_sqlite_to_postgres_multi(config, test_core).await;
    }
}

pub mod http {
    use treaty_tests::{
        harness::init_trace_to_screen,
        runner::{RunnerConfig, TestRunner},
    };

    use crate::test_core;

    #[tokio::test]
    async fn test() {
        let test_name = "sa_contract_share_mirror_http";
        init_trace_to_screen(false, None);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_multi(config, test_core).await;
    }

    #[tokio::test]
    async fn tls() {
        let test_name = "sa_contract_share_mirror_http_tls";
        init_trace_to_screen(
            false,
            Some(String::from(
                "policy_shared_mirror=trace,rocket=trace,reqwest=trace",
            )),
        );

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_multi_tls(config, test_core).await;
    }

    #[tokio::test]
    pub async fn postgres() {
        let test_name = "sa_contract_http_share_mirror_postgres";
        init_trace_to_screen(false, Some(String::from("policy_shared_mirror=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_postgres_multi(config, test_core).await;
    }
}
