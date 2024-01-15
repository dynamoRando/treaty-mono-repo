use tracing::trace;
use treaty_tests::common_contract_setup::main_and_participant_setup;
use treaty_tests::harness::get_treaty_client;
use treaty_tests::harness::CoreTestConfig;
use treaty_tests::harness::TreatyClientConfig;
use treaty_types::enums::DatabaseType;
use treaty_types::enums::UpdatesFromHostBehavior;

pub async fn test_core(config: CoreTestConfig) {
    go(config).await
}

async fn go(config: CoreTestConfig) {
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);

    let db = config.test_db_name.clone();
    let mca = config.main_client.clone();
    let pca = config.participant_client.as_ref().unwrap().clone();

    let update_statement = "UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999";

    {
        let new_behavior = UpdatesFromHostBehavior::QueueForReviewAndLog;

        let db = db.clone();
        let pca = pca.clone();

        let update_at_participant_is_successful =
            participant_changes_update_behavior(&db, &pca, new_behavior).await;
        assert!(update_at_participant_is_successful);
    }

    {
        let db = db.clone();
        let mca = mca.clone();

        let can_read_rows = main_read_updated_row_should_fail(&db, &mca, update_statement).await;

        assert!(!can_read_rows);
    }

    {
        let db = db.clone();
        let pca = pca.clone();

        let has_and_accept_update =
            participant_get_and_approve_pending_update(&db, "EMPLOYEE", &pca, update_statement)
                .await;
        assert!(has_and_accept_update);
    }

    {
        let mca = mca.clone();
        let can_read_rows = main_read_updated_row_should_succed(&db, &mca).await;

        assert!(can_read_rows);
    }
}

async fn participant_changes_update_behavior(
    db_name: &str,
    participant_client_addr: &TreatyClientConfig,
    behavior: UpdatesFromHostBehavior,
) -> bool {
    let mut client = get_treaty_client(participant_client_addr).await;

    let change_update_behavior = client
        .change_updates_from_host_behavior(db_name, "EMPLOYEE", behavior)
        .await;

    change_update_behavior.unwrap()
}

async fn main_read_updated_row_should_fail(
    db_name: &str,
    main_client_addr: &TreatyClientConfig,
    update_statement: &str,
) -> bool {
    let mut client = get_treaty_client(main_client_addr).await;

    let update_result = client
        .execute_cooperative_write_at_host(db_name, update_statement, "participant", "ID = 999")
        .await;

    trace!("{update_result:?}");

    assert!(update_result.unwrap());

    let cmd = String::from("SELECT NAME FROM EMPLOYEE WHERE Id = 999");
    let read_result = client
        .execute_read_at_host(db_name, &cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
        .await;

    let results = read_result.unwrap();

    let row = results.rows.first().unwrap();

    let value = &row.values[1].value.clone();

    trace!("{value:?}");

    let expected_value = "TESTER".as_bytes().to_vec();

    trace!("{expected_value:?}");

    *value == expected_value
}

async fn participant_get_and_approve_pending_update(
    db_name: &str,
    table_name: &str,
    participant_client_addr: &TreatyClientConfig,
    update_statement: &str,
) -> bool {
    let mut has_statement = false;
    let mut statement_row_id = 0;
    let mut client = get_treaty_client(participant_client_addr).await;

    let pending_updates = client
        .get_pending_actions_at_participant(db_name, table_name, "UPDATE")
        .await
        .unwrap();

    for statement in &pending_updates.pending_statements {
        if statement.statement == update_statement {
            has_statement = true;
            statement_row_id = statement.row_id;
        }
    }

    assert!(has_statement);

    if has_statement {
        trace!("has statement");

        // need to accept the statement
        let accept_update_result = client
            .accept_pending_action_at_participant(db_name, table_name, statement_row_id)
            .await
            .unwrap();

        return accept_update_result.is_successful;
    }

    false
}

async fn main_read_updated_row_should_succed(
    db_name: &str,
    main_client_addr: &TreatyClientConfig,
) -> bool {
    let mut client = get_treaty_client(main_client_addr).await;

    let cmd = String::from("SELECT NAME FROM EMPLOYEE WHERE Id = 999");
    let read_result = client
        .execute_read_at_host(db_name, &cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
        .await;

    let results = read_result.unwrap();

    let row = results.rows.first().unwrap();

    let value = &row.values[1].value.clone();

    trace!("{value:?}");

    let expected_value = "TESTER".as_bytes().to_vec();

    trace!("{expected_value:?}");

    *value == expected_value
}

pub mod http {
    use crate::test_core;
    use treaty_tests::harness::init_trace_to_screen;
    use treaty_tests::runner::{RunnerConfig, TestRunner};

    #[tokio::test]
    async fn test() {
        init_trace_to_screen(false, None);

        let test_name = "updates_from_host_queue_with_log_http";
        let contract = String::from("insert read remote row");

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(contract),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_multi(config, test_core).await;
    }

    #[tokio::test]
    async fn postgres() {
        let test_name = "updates_from_host_queue_with_log_http_postgres";
        init_trace_to_screen(false, Some(String::from("validate_update_with_log=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_postgres_multi(config, test_core).await;
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

        let test_name = "updates_from_host_queue_with_log_grpc";
        let contract = String::from("insert read remote row");
        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(contract),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_multi(config, test_core).await;
    }

    #[tokio::test]
    async fn postgres() {
        let test_name = "updates_from_host_queue_with_log_postgres_grpc";
        init_trace_to_screen(false, Some(String::from("validate_update_with_log=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_postgres_multi(config, test_core).await;
    }

    #[tokio::test]
    async fn proxy() {
        init_trace_to_screen(false, None);

        let test_name = "updates_from_host_queue_with_log_grpc-proxy";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test_multi(config, test_core).await;
    }

    #[tokio::test]
    pub async fn postgres_sqlite() {
        let test_name = "updates_from_host_queue_with_log_grpc_postgres_sqlite";
        init_trace_to_screen(false, Some(String::from("validate_update_with_log=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_postgres_to_sqlite_multi(config, test_core).await;
    }

    #[tokio::test]
    pub async fn sqlite_postgres() {
        let test_name = "updates_from_host_queue_with_log_grpc_sqlite_postgres";
        init_trace_to_screen(false, Some(String::from("validate_update_with_log=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_sqlite_to_postgres_multi(config, test_core).await;
    }
}
