use std::sync::{Mutex, Arc};

use tracing::debug;
use treaty::treaty_proto::{AuthRequest, CreateUserDatabaseRequest, CreateUserDatabaseReply};

use treaty_proxy::proxy_server::ProxyServer;
use treaty_tests::harness::{proxy::{configure_proxy_for_test, TreatyProxyTestType, get_http_result}, init_trace_to_screen};
use treaty_types::proxy::{server_messages::{RegisterLoginRequest, ExecuteReply, RegisterLoginReply, ExecuteRequest}, request_type::RequestType};


struct TestId {
    pub id: Option<String>,
}

#[tokio::test]
async fn create_database() {
    init_trace_to_screen(false);

    let id = TestId { id: None };
    let id = Mutex::new(id);
    let id = Arc::new(id);

    let setup = configure_proxy_for_test("proxy-i-register-make-db", TreatyProxyTestType::Grpc);
    {
        let proxy = setup.proxy.clone();
        let server = ProxyServer::new(proxy.clone());
        tokio::spawn(async move {
            proxy.start_grpc_client().await;
            proxy.start_grpc_data().await;
            server.start().await.unwrap();
        });
    }

    {
        let proxy = setup.proxy.clone();
        let id = id.clone();
        tokio::spawn(async move {
            let proxy = proxy.clone();
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

            if result.is_successful {
                if let Ok(mut x) = id.lock() {
                    let id = result.host_id.as_ref().unwrap().clone();
                    x.id = Some(id);
                }
            }
        })
        .await
        .unwrap();
    }
    {
        let id = id.clone();
        let proxy = setup.proxy.clone();
        let request_type: u16 = RequestType::CreateUserDatabase.into();
        let mut _id: Option<String> = None;
        if let Ok(_lock) = id.lock() {
            _id = Some(_lock.id.as_ref().unwrap().clone());
        }

        let _auth = AuthRequest {
            user_name: "tester".to_string(),
            pw: "1234".to_string(),
            pw_hash: Vec::new(),
            token: Vec::new(),
            jwt: "".to_string(),
            id: _id,
        };

        let _request = CreateUserDatabaseRequest {
            authentication: Some(_auth),
            database_name: "hopeitworks.db".to_string(),
        };

        let _request_json = serde_json::to_string(&_request).unwrap();

        tokio::spawn(async move {
            let request = ExecuteRequest {
                login: Some("tester".to_string()),
                pw: Some("1234".to_string()),
                jwt: None,
                request_type,
                request_json: _request_json,
            };

            let url = format!(
                "http://{}:{}/execute",
                proxy.http_endpoint_addr(),
                proxy.http_endpoint_port()
            );

            debug!("{url:?}");
            let result: ExecuteReply = get_http_result(url, request).await;
            debug!("{result:?}");
            assert!(result.login_success && result.execute_success);

            let db_result: CreateUserDatabaseReply =
                serde_json::from_str(&result.reply.as_ref().unwrap().clone()).unwrap();
            assert!(db_result.is_created);
        })
        .await
        .unwrap();
    }
}
