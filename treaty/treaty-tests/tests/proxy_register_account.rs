use tracing::debug;
use treaty_proxy::proxy_server::ProxyServer;
use treaty_tests::harness::{
    init_trace_to_screen,
    proxy::{configure_proxy_for_test, get_http_result, TreatyProxyTestType},
};
use treaty_types::proxy::server_messages::{RegisterLoginRequest, RegisterLoginReply};

#[tokio::test]
async fn register_account() {
    init_trace_to_screen(false);

    let setup = configure_proxy_for_test("proxy-i-register-user", TreatyProxyTestType::Grpc);
    let proxy = setup.proxy.clone();

    {
        let proxy = setup.proxy.clone();
        let server = ProxyServer::new(proxy.clone());
        tokio::spawn(async move {
            proxy.start_grpc_client().await;
            proxy.start_grpc_data().await;
            server.start().await.unwrap();
        });
    }

    tokio::spawn(async move {
        let request = RegisterLoginRequest {
            login: "tester".to_string(),
            pw: "1234".to_string(),
        };

        let url = format!(
            "http://{}:{}/account/register",
            proxy.http_endpoint_addr(),
            proxy.http_endpoint_port()
        );

        debug!("{url:?}");
        let result: RegisterLoginReply = get_http_result(url, request).await;
        debug!("{result:?}");
        assert!(result.is_successful);
    })
    .await
    .unwrap();
}
