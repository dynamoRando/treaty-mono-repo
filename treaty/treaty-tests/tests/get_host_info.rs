use tracing::debug;
use treaty_tests::harness::CoreTestConfig;
use treaty_tests::harness::TreatyClientConfig;

pub async fn test_core(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let db = config.test_db_name;

    let response = client(&db, &mc).await;
    debug!("create_user_database: got: {response}");

    assert!(response);
}

async fn client(db_name: &str, config: &TreatyClientConfig) -> bool {
    let mut client = treaty_tests::harness::get_treaty_client(config).await;

    client.create_user_database(db_name).await.unwrap();
    client.enable_cooperative_features(db_name).await.unwrap();
    client.generate_host_info("main").await.unwrap();

    let host_info = client.get_host_info().await.unwrap();

    host_info.host_info.unwrap().host_name == "main"
}

pub mod http {
    use crate::test_core;
    use treaty_tests::runner::{RunnerConfig, TestRunner};

    #[tokio::test]
    async fn test() {
        let test_name = "get_host_info_http";

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
        let test_name = "get_host_info_grpc";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: None,
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test(config, test_core).await;
    }

    #[tokio::test]
    async fn postgres() {
        let test_name = "get_host_info_grpc_postgres";
        init_trace_to_screen(false, Some(String::from("get_host_info=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_postgres(config, test_core).await;
    }

    #[tokio::test]
    async fn proxy() {
        let test_name = "get_host_info_grpc-proxy";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: None,
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test(config, test_core).await;
    }
}
