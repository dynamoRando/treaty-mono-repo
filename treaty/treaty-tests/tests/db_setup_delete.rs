use treaty_tests::{
    common_contract_setup::main_and_participant_setup,
    harness::{get_treaty_client, CoreTestConfig},
};

pub mod grpc {
    use treaty_tests::{
        harness::init_trace_to_screen,
        runner::{RunnerConfig, TestRunner},
    };

    use crate::test_core;

    #[test]
    fn grpc() {
        let test_name = "add_read_delete_remote_gprc";
        init_trace_to_screen(false);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_multi(config, test_core);
    }

    #[test]
    fn proxy() {
        let test_name = "add_read_delete_remote_gprc-proxy";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test_multi(config, test_core);
    }
}

pub mod http {

    #[test]
    fn http() {
        use crate::test_core;
        use treaty_tests::{
            harness::init_trace_to_screen,
            runner::{RunnerConfig, TestRunner},
        };

        let test_name = "add_read_delete_remote_http";
        init_trace_to_screen(false);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_multi(config, test_core);
    }
}

fn test_core(config: CoreTestConfig) {
    go(config);
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);

    let mut client = get_treaty_client(&config.main_client).await;
    let can_delete = client
        .execute_cooperative_write_at_host(
            &config.test_db_name,
            "DELETE FROM EMPLOYEE WHERE Id = 999",
            "participant",
            "Id = 999",
        )
        .await
        .unwrap();

    assert!(can_delete);
}
