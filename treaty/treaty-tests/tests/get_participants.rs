use treaty_tests::common_contract_setup::main_and_participant_setup;
use treaty_tests::harness::get_treaty_client;
use treaty_tests::harness::CoreTestConfig;

pub fn test_core(config: CoreTestConfig) {
    go(config)
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    // let pc = config.participant_client.as_ref().unwrap().clone();
    let db_name = config.test_db_name.clone();
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);

    let mut mc = get_treaty_client(&mc).await;

    let mut main_has_participant: bool = false;

    let data = mc.get_participants_for_database(&db_name).await.unwrap();

    for participant in &data.participants {
        if participant.participant.as_ref().unwrap().alias == "participant" {
            main_has_participant = true;
        }
    }

    assert!(main_has_participant);
}

pub mod http {
    use crate::test_core;
    use treaty_tests::{
        harness::init_trace_to_screen,
        runner::{RunnerConfig, TestRunner},
    };

    #[test]
    fn test() {
        init_trace_to_screen(false);

        let test_name = "get_participants_for_db_http";
        let contract = String::from("");

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
    use treaty_tests::{runner::{RunnerConfig, TestRunner}, harness::init_trace_to_screen};

    #[test]
    fn test() {
        init_trace_to_screen(false);

        let test_name = "get_participants_for_db_grpc";
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

        let test_name = "get_participants_for_db_grpc-proxy";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test_multi(config, test_core);
    }
}
