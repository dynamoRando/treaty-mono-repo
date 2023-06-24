use treaty_tests::common_contract_setup::main_and_participant_setup;
use treaty_tests::harness::get_treaty_client;
use treaty_tests::harness::CoreTestConfig;

pub fn test_core(config: CoreTestConfig) {
    go(config)
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let pc = config.participant_client.as_ref().unwrap().clone();
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);

    let mut client = get_treaty_client(&pc).await;

    let hosts = client.get_cooperative_hosts().await.unwrap();

    let mut has_host: bool = false;

    for host in &hosts.hosts {
        if host.host.as_ref().unwrap().host_name == "tester" {
            has_host = true;
        }
    }

    assert!(has_host);
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
        let test_name = "get_coop_hosts_http";
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
    use treaty_tests::runner::{RunnerConfig, TestRunner};

    #[test]
    fn test() {
        let test_name = "get_coop_hosts_gprc";
        let contract = String::from("insert read remote row");
        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some(contract),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_test_multi(config, test_core);
    }

    #[test]
    fn proxy() {
        let test_name = "get_coop_hosts_grpc-proxy";

        let config = RunnerConfig {
            test_name: test_name.to_string(),
            contract_desc: Some("".to_string()),
            use_internal_logging: false,
        };

        TestRunner::run_grpc_proxy_test_multi(config, test_core);
    }
}
