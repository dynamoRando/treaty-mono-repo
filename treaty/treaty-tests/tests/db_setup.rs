use treaty_tests::{common_contract_setup::main_and_participant_setup, harness::CoreTestConfig};

pub fn test_core(config: CoreTestConfig) {
    go(config);
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let result = main_and_participant_setup(config).await;
    assert!(result);
}

pub mod grpc {
    use treaty_tests::{
        harness::init_trace_to_screen,
        runner::{RunnerConfig, TestRunner},
    };

    use crate::test_core;

    #[test]
    pub fn test() {
        let test_name = "sa_contract_grpc";
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
        let test_name = "sa_contract_grpc-proxy";
        init_trace_to_screen(false);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test_multi(config, test_core);
    }
}

pub mod http {
    use treaty_tests::{
        harness::init_trace_to_screen,
        runner::{RunnerConfig, TestRunner},
    };

    use crate::test_core;

    #[test]
    fn test() {
        let test_name = "sa_contract_http";
        init_trace_to_screen(false);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_multi(config, test_core);
    }
}
