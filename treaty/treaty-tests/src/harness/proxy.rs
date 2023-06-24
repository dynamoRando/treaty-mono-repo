use tracing::debug;
use treaty_types::enums::DatabaseType;
use treaty_proxy::{TreatyProxy, TreatyProxySettings};

use crate::get_test_temp_dir;

use super::{get_next_avail_port, AddrType, ServiceAddr};

#[derive(Debug, Clone)]
pub struct TreatyProxyTestSetup {
    pub proxy_info: TreatyProxyInfo,
    pub main: TreatyProxyTestUser,
    pub part: Option<TreatyProxyTestUser>,
}

#[derive(Debug, Clone)]
pub struct TreatyProxyInfo {
    pub proxy: TreatyProxy,
    pub client_addr: ServiceAddr,
    pub db_addr: Option<ServiceAddr>,
}

#[derive(Debug, Clone)]
pub struct TreatyProxyTestUser {
    pub host_id: String,
    pub un: String,
    pub pw: String,
}

#[derive(Debug, Clone)]
pub enum TreatyProxyTestType {
    Grpc,
    Http,
}

/// common test code - sets up a test folder and returns a treaty proxy
pub fn configure_proxy_for_test(
    test_name: &str,
    proxy_type: TreatyProxyTestType,
) -> TreatyProxyInfo {
    let root_dir = get_test_temp_dir(test_name);

    match proxy_type {
        TreatyProxyTestType::Grpc => {
            let client_port = get_next_avail_port();
            let client_addr = format!("127.0.0.1:{}", client_port);
            let db_port = get_next_avail_port();
            let db_addr = format!("127.0.0.1:{}", db_port);
            let proxy_http_port = get_next_avail_port();

            let settings = TreatyProxySettings {
                use_grpc: true,
                use_http: false,
                grpc_client_addr_port: client_addr,
                grpc_db_addr_port: db_addr,
                http_ip: "127.0.0.1".to_string(),
                http_port: 0,
                database_type: DatabaseType::Sqlite,
                database_name: "Proxy.db".to_string(),
                proxy_http_addr: "127.0.0.1".to_string(),
                proxy_http_port: proxy_http_port as usize,
                root_dir,
            };

            let proxy = TreatyProxy::get_proxy_with_config(settings);
            proxy.start();

            let client_addr = ServiceAddr {
                ip4_addr: "127.0.0.1:".to_string(),
                port: client_port,
                addr_type: AddrType::Client,
            };

            let db_addr = ServiceAddr {
                ip4_addr: "127.0.0.1:".to_string(),
                port: db_port,
                addr_type: AddrType::Database,
            };

            TreatyProxyInfo {
                proxy,
                client_addr,
                db_addr: Some(db_addr),
            }
        }
        TreatyProxyTestType::Http => {
            let port = get_next_avail_port();

            let settings = TreatyProxySettings {
                use_grpc: false,
                use_http: true,
                grpc_client_addr_port: "127.0.0.1:0".to_string(),
                grpc_db_addr_port: "127.0.0.1:0".to_string(),
                http_ip: "127.0.0.1".to_string(),
                http_port: 0,
                database_type: DatabaseType::Sqlite,
                database_name: "Proxy.db".to_string(),
                proxy_http_addr: "127.0.0.1".to_string(),
                proxy_http_port: port as usize,
                root_dir,
            };

            let proxy = TreatyProxy::get_proxy_with_config(settings);
            proxy.start();
            let client_addr = ServiceAddr {
                ip4_addr: "127.0.0.1".to_string(),
                port,
                addr_type: AddrType::Client,
            };

            let db_addr = ServiceAddr {
                ip4_addr: "127.0.0.1".to_string(),
                port,
                addr_type: AddrType::Database,
            };

            TreatyProxyInfo {
                proxy,
                client_addr,
                db_addr: Some(db_addr),
            }
        }
    }
}

fn register_proxy_user(proxy: &TreatyProxy, un: &str, pw: &str) -> Option<TreatyProxyTestUser> {
    let result_register = proxy.register_user(un, pw);

    if result_register.is_err() {
        assert!(false);
    }

    let result_setup = proxy.setup_user_folder(false);

    match result_setup {
        Ok(root_dir) => {
            let result_setup_treaty = proxy.setup_treaty_service(un, &root_dir);

            match result_setup_treaty {
                Ok(host_id) => {
                    debug!("A new user was registered with host_id: {host_id:?} with login: {un:?} {pw:?}");
                    assert!(!host_id.is_empty());

                    return Some(TreatyProxyTestUser {
                        host_id,
                        un: un.to_string(),
                        pw: pw.to_string(),
                    });
                }
                Err(_) => assert!(false),
            }
        }
        Err(_) => assert!(false),
    }

    None
}

/// common setup code - sets up the proxy instance and then returns an treaty service for the "test" user
pub fn setup_proxy_with_users(
    test_name: &str,
    main_and_participant: bool,
    proxy_type: TreatyProxyTestType,
) -> Option<TreatyProxyTestSetup> {
    let proxy = configure_proxy_for_test(test_name, proxy_type);

    if main_and_participant {
        if let Some(main) = register_proxy_user(&proxy.proxy, "tester", "123456") {
            if let Some(part) = register_proxy_user(&proxy.proxy, "part", "123456") {
                return Some(TreatyProxyTestSetup {
                    proxy_info: proxy,
                    main,
                    part: Some(part),
                });
            }
        };
    }

    if let Some(main) = register_proxy_user(&proxy.proxy, "tester", "123456") {
        return Some(TreatyProxyTestSetup {
            proxy_info: proxy,
            main,
            part: None,
        });
    }

    None
}


use serde::de;

pub async fn get_http_result<
    'a,
    'b,
    T: de::DeserializeOwned + std::clone::Clone,
    U: de::DeserializeOwned + serde::Serialize + std::clone::Clone,
>(
    url: String,
    request: U,
) -> T {
    let request_json = serde_json::to_string(&request).unwrap();
    let result_json: String = send_http_message(request_json, url).await;
    debug!("{result_json:?}");
    let value: T = serde_json::from_str(&result_json).unwrap();
    value
}

pub async fn send_http_message(json_message: String, url: String) -> String {
    let client = reqwest::Client::new();

    debug!("{json_message}");
    debug!("{url}");

    client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_message)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
