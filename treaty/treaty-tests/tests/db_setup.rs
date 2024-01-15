use treaty_tests::{common_contract_setup::main_and_participant_setup, harness::CoreTestConfig};

pub async fn test_core(config: CoreTestConfig) {
    go(config).await;
}

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

    #[tokio::test]
    pub async fn test() {
        let test_name = "sa_contract_grpc";
        init_trace_to_screen(false, Some(String::from("db_setup=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_multi(config, test_core).await;
    }

    #[tokio::test]
    pub async fn tls() {
        let test_name = "sa_contract_grpc_tls";
        init_trace_to_screen(false, Some(String::from("db_setup=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_multi_tls(config, test_core).await;
    }

    #[tokio::test]
    pub async fn proxy() {
        let test_name = "sa_contract_grpc-proxy";
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
        let test_name = "sa_contract_grpc_postgres";
        init_trace_to_screen(false, Some(String::from("db_setup=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_postgres_multi(config, test_core).await;
    }

    #[tokio::test]
    pub async fn postgres_sqlite() {
        let test_name = "sa_contract_grpc_postgres_sqlite";
        init_trace_to_screen(false, Some(String::from("db_setup=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_postgres_to_sqlite_multi(config, test_core).await;
    }

    #[tokio::test]
    pub async fn sqlite_postgres() {
        let test_name = "sa_contract_grpc_sqlite_postgres";
        init_trace_to_screen(false, Some(String::from("db_setup=trace")));

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
        let test_name = "sa_contract_http";
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
        let test_name = "sa_contract_http_tls";
        init_trace_to_screen(
            false,
            Some(String::from("db_setup=trace,rocket=trace,reqwest=trace")),
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
        let test_name = "sa_contract_http_postgres";
        init_trace_to_screen(false, Some(String::from("db_setup=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_postgres_multi(config, test_core).await;
    }
}
