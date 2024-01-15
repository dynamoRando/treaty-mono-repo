pub mod reject_host_test_core {
    use treaty_tests::{
        common_contract_setup::main_and_participant_setup,
        harness::{get_treaty_client, CoreTestConfig, TreatyClientConfig},
    };
    use treaty_types::enums::HostStatus;

    pub async fn test_core(config: CoreTestConfig) {
        go(config).await
    }

    async fn go(config: CoreTestConfig) {
        let result = main_and_participant_setup(config.clone()).await;
        assert!(result);

        let db = config.test_db_name.clone();
        let pc = config.participant_client.as_ref().unwrap().clone();
        let mc = config.main_client.clone();
        let reject = participant_rejects_host(&pc).await;
        assert!(reject);

        let should_fail = main_read_should_fail(&db, &mc).await;
        println!("should_fail: {should_fail:?}");

        assert!(!should_fail);
    }

    async fn participant_rejects_host(config: &TreatyClientConfig) -> bool {
        let mut client = get_treaty_client(config).await;

        let host_status = HostStatus::Deny;

        let reject_host_result = client
            .change_host_status_by_name("tester", HostStatus::to_u32(host_status))
            .await;

        reject_host_result.unwrap()
    }

    async fn main_read_should_fail(db_name: &str, config: &TreatyClientConfig) -> bool {
        let mut client = get_treaty_client(config).await;

        client
            .try_auth_at_participant("participant", "", db_name)
            .await
            .unwrap()
    }
}

pub mod http {
    use crate::reject_host_test_core::test_core;
    use treaty_tests::harness::init_trace_to_screen;
    use treaty_tests::runner::{RunnerConfig, TestRunner};

    #[tokio::test]
    async fn test() {
        let test_name = "reject_host_http";
        init_trace_to_screen(false, None);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("contract".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_multi(config, test_core).await;
    }

    #[tokio::test]
    async fn postgres() {
        let test_name = "reject_host_http_postgres";
        init_trace_to_screen(false, Some(String::from("reject_host=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_http_test_postgres_multi(config, test_core).await;
    }
}

pub mod grpc {
    use crate::reject_host_test_core::test_core;
    use treaty_tests::harness::init_trace_to_screen;
    use treaty_tests::runner::{RunnerConfig, TestRunner};

    #[tokio::test]
    async fn test() {
        let test_name = "reject_host_grpc";
        init_trace_to_screen(false, None);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_multi(config, test_core).await;
    }

    #[tokio::test]
    async fn postgres() {
        let test_name = "reject_host_grpc_postgres";
        init_trace_to_screen(false, Some(String::from("reject_host=trace")));

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(String::from("contract")),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_postgres_multi(config, test_core).await;
    }

    #[tokio::test]
    async fn proxy() {
        let test_name = "reject_host_grpc-proxy";
        init_trace_to_screen(false, None);

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test_multi(config, test_core).await;
    }
}
