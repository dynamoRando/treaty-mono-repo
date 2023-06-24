use tracing::trace;
use treaty_types::enums::DatabaseType;
use treaty_types::enums::UpdatesFromHostBehavior;
use treaty_types::enums::UpdatesToHostBehavior;
use treaty_tests::common_contract_setup::main_and_participant_setup;
use treaty_tests::harness::get_treaty_client;
use treaty_tests::harness::CoreTestConfig;

pub fn test_core(config: CoreTestConfig) {
    go(config)
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);

    let db_name = config.test_db_name.clone();

    let pc = config.participant_client.as_ref().unwrap().clone();
    let mut pc = get_treaty_client(&pc).await;

    let mc = config.main_client.clone();
    let mut mc = get_treaty_client(&mc).await;

    // check that we can change update_to_host_behavior, and validate that behavior
    {
        let new_behavior = UpdatesToHostBehavior::SendDataHashChange;
        let change_ok = pc
            .change_updates_to_host_behavior(&db_name, "EMPLOYEE", new_behavior)
            .await
            .unwrap();

        assert!(change_ok);

        // lets update and make sure data hashes match
        let statement = String::from("UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999");
        let change_ok = pc
            .execute_write_at_participant(
                &db_name,
                &statement,
                DatabaseType::to_u32(DatabaseType::Sqlite),
                "ID = 999",
            )
            .await
            .unwrap();

        assert!(change_ok);

        // validate that main can read
        let cmd = String::from("SELECT NAME FROM EMPLOYEE WHERE Id = 999");
        let read_result = mc
            .execute_read_at_host(&db_name, &cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
            .await;

        let results = read_result.unwrap();
        let row = results.rows.first().unwrap();
        let value = &row.values[1].value.clone();

        trace!("{value:?}");

        let expected_value = "TESTER".as_bytes().to_vec();

        trace!("{expected_value:?}");
        assert!(*value == expected_value);

        // lets validate that data hashes at both location match
        let mut row_id_at_participant: u32;
        let mut participant_data_hash: u64;
        let mut host_data_hash: u64;

        let row_ids = pc
            .get_row_id_at_participant(&db_name, "EMPLOYEE", "NAME = 'TESTER'")
            .await
            .unwrap();

        row_id_at_participant = *row_ids.first().unwrap();

        participant_data_hash = pc
            .get_data_hash_at_participant(&db_name, "EMPLOYEE", row_id_at_participant)
            .await
            .unwrap();

        host_data_hash = mc
            .get_data_hash_at_host(&db_name, "EMPLOYEE", row_id_at_participant)
            .await
            .unwrap();

        assert_eq!(participant_data_hash, host_data_hash);

        // change to not send updates back to host, this means that data hashes should not match
        // this is a repeat of the above test, but with the expectation that the hashes don't match

        let new_behavior = UpdatesToHostBehavior::DoNothing;
        let change_ok = pc
            .change_updates_to_host_behavior(&db_name, "EMPLOYEE", new_behavior)
            .await
            .unwrap();

        assert!(change_ok);

        let statement = String::from("UPDATE EMPLOYEE SET NAME = 'FOOBAR' WHERE ID = 999");
        let change_ok = pc
            .execute_write_at_participant(
                &db_name,
                &statement,
                DatabaseType::to_u32(DatabaseType::Sqlite),
                "ID = 999",
            )
            .await
            .unwrap();

        assert!(change_ok);

        let row_ids = pc
            .get_row_id_at_participant(&db_name, "EMPLOYEE", "NAME = 'FOOBAR'")
            .await
            .unwrap();

        row_id_at_participant = *row_ids.first().unwrap();

        participant_data_hash = pc
            .get_data_hash_at_participant(&db_name, "EMPLOYEE", row_id_at_participant)
            .await
            .unwrap();

        host_data_hash = mc
            .get_data_hash_at_host(&db_name, "EMPLOYEE", row_id_at_participant)
            .await
            .unwrap();

        assert_ne!(participant_data_hash, host_data_hash);
    }

    // check that we can change and get update_from_host_behavior
    {
        let new_update_behavior = UpdatesFromHostBehavior::Ignore;
        let change_ok = pc
            .change_updates_from_host_behavior(&db_name, "EMPLOYEE", new_update_behavior)
            .await
            .unwrap();

        assert!(change_ok);

        let behavior = pc
            .get_updates_from_host_behavior(&db_name, "EMPLOYEE")
            .await
            .unwrap();

        let response_value = UpdatesFromHostBehavior::from_u32(behavior.behavior.unwrap());

        assert!(response_value == new_update_behavior)
    }

    // main client should fail since we have denied the host from making changes
    {
        let cmd = String::from("UPDATE EMPLOYEE SET NAME = 'Fail' WHERE Id = 999");
        let update_result = mc
            .execute_cooperative_write_at_host(&db_name, &cmd, "participant", "Id = 999")
            .await
            .unwrap();

        assert!(!update_result)
    }

    // setup record
    let new_update_behavior = UpdatesFromHostBehavior::AllowOverwrite;
    pc.change_updates_from_host_behavior(&db_name, "EMPLOYEE", new_update_behavior)
        .await
        .unwrap();

    let update_statement = "UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999";
    mc.execute_cooperative_write_at_host(&db_name, update_statement, "participant", "ID = 999")
        .await
        .unwrap();

    // lets check that queued updates work
    let behaviors: Vec<UpdatesFromHostBehavior> = vec![
        UpdatesFromHostBehavior::QueueForReview,
        UpdatesFromHostBehavior::QueueForReviewAndLog,
    ];

    for new_behavior in behaviors {
        {
            let change_ok = pc
                .change_updates_from_host_behavior(&db_name, "EMPLOYEE", new_behavior)
                .await
                .unwrap();

            assert!(change_ok);

            // main tries to update but discovered that update did not succeed (this is intentional)
            let update_statement = "UPDATE EMPLOYEE SET NAME = 'FUNBAR' WHERE ID = 999";
            let change_ok = mc
                .execute_cooperative_write_at_host(
                    &db_name,
                    update_statement,
                    "participant",
                    "ID = 999",
                )
                .await
                .unwrap();

            assert!(change_ok);

            // try to read the value back, this should be wrong
            let cmd = String::from("SELECT NAME FROM EMPLOYEE WHERE Id = 999");
            let results = mc
                .execute_read_at_host(&db_name, &cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
                .await
                .unwrap();

            let row = results.rows.first().unwrap();
            let value = &row.values[1].value.clone();

            trace!("{value:?}");

            let expected_value = "FUNBAR".as_bytes().to_vec();

            trace!("{expected_value:?}");

            let are_values_equal = *value == expected_value;
            assert!(!are_values_equal);

            let mut has_statement: bool = false;
            let mut statement_row_id: u32 = 0;

            // participant later accepts update
            let pending_updates = pc
                .get_pending_actions_at_participant(&db_name, "EMPLOYEE", "UPDATE")
                .await
                .unwrap();

            for statement in &pending_updates.pending_statements {
                if statement.statement == update_statement {
                    has_statement = true;
                    statement_row_id = statement.row_id;
                }
            }

            assert!(has_statement);

            // need to accept the statement
            let accept_update_result = pc
                .accept_pending_action_at_participant(&db_name, "EMPLOYEE", statement_row_id)
                .await
                .unwrap();

            assert!(accept_update_result.is_successful);

            // main reads again and finds update has succeeded
            let cmd = String::from("SELECT NAME FROM EMPLOYEE WHERE Id = 999");
            let read_result = mc
                .execute_read_at_host(&db_name, &cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
                .await;

            let results = read_result.unwrap();
            let row = results.rows.first().unwrap();
            let value = &row.values[1].value.clone();

            trace!("{value:?}");

            let expected_value = "FUNBAR".as_bytes().to_vec();

            trace!("{expected_value:?}");

            let are_values_equal = *value == expected_value;
            assert!(are_values_equal);

            // setup record
            let new_update_behavior = UpdatesFromHostBehavior::AllowOverwrite;
            pc.change_updates_from_host_behavior(&db_name, "EMPLOYEE", new_update_behavior)
                .await
                .unwrap();

            let update_statement = "UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999";
            mc.execute_cooperative_write_at_host(
                &db_name,
                update_statement,
                "participant",
                "ID = 999",
            )
            .await
            .unwrap();
        }
    }
}

pub mod http {
    use treaty_tests::harness::init_trace_to_screen;

    #[test]
    fn test() {
        use crate::test_core;
        use treaty_tests::runner::{RunnerConfig, TestRunner};
        init_trace_to_screen(false);

        let test_name = "get_update_from_part_http";
        let contract = String::from("insert read remote row");

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(contract),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_multi(config, test_core);
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

        let test_name = "get_update_from_part_gprc";
        let contract = String::from("");
        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(contract),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_multi(config, test_core);
    }

    #[test]
    fn proxy() {
        init_trace_to_screen(false);

        let test_name = "get_update_from_part_gprc-proxy";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test_multi(config, test_core);
    }
}
