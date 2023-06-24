use crate::{proxy_server::process::process_request, TreatyProxy, TreatyProxyErr};
use rocket::{http::Status, post, serde::json::Json, State};
use tracing::debug;
use treaty_types::proxy::server_messages::{ExecuteReply, ExecuteRequest};

#[post("/execute", format = "application/json", data = "<request>")]
pub async fn execute_request(
    request: Json<ExecuteRequest>,
    state: &State<TreatyProxy>,
) -> (Status, Json<ExecuteReply>) {
    debug!("{request:?}");

    let request = request.into_inner();

    if request.pw.is_some() && request.login.is_some() {
        let state = state.inner().clone();
        let result_is_authorized = state.verify_login(
            request.login.as_ref().unwrap(),
            &request.pw.clone().unwrap(),
        );
        match result_is_authorized {
            Ok(is_auth) => {
                if !is_auth {
                    let response = ExecuteReply {
                        login_success: false,
                        execute_success: false,
                        reply: None,
                    };

                    return (Status::Ok, Json(response));
                }
            }
            Err(_) => {
                let response = ExecuteReply {
                    login_success: false,
                    execute_success: false,
                    reply: None,
                };

                return (Status::Ok, Json(response));
            }
        }
    }

    if request.jwt.is_some() {
        let result_is_authorized = state.verify_token(&request.jwt.clone().unwrap());
        match result_is_authorized {
            Ok(is_auth) => {
                if !is_auth {
                    let response = ExecuteReply {
                        login_success: false,
                        execute_success: false,
                        reply: None,
                    };

                    return (Status::Ok, Json(response));
                }
            }
            Err(_) => {
                let response = ExecuteReply {
                    login_success: false,
                    execute_success: false,
                    reply: None,
                };

                return (Status::Ok, Json(response));
            }
        }
    }

    let response = ExecuteReply {
        login_success: false,
        execute_success: false,
        reply: None,
    };

    if let Some(login) = request.login.clone() {
        debug!("getting host via login: {}", login);
        let result_id = state.get_host_id_for_user(&login);
        let response = execute(result_id, state, &request).await;
        return (Status::Ok, Json(response));
    }

    if let Some(jwt) = request.jwt.clone() {
        debug!("getting host via token: {}", jwt);
        let result_id = state.get_host_id_for_token(&jwt);
        let response = execute(result_id, state, &request).await;
        return (Status::Ok, Json(response));
    }

    (Status::Ok, Json(response))
}

async fn execute(
    result_id: Result<Option<String>, TreatyProxyErr>,
    proxy: &TreatyProxy,
    request: &ExecuteRequest,
) -> ExecuteReply {
    match result_id {
        Ok(id) => match id {
            Some(id) => {
                let result_core = proxy.get_treaty_grpc_user_handler(&id);
                match result_core {
                    Ok(core) => {
                        let core = Box::new(core);
                        let result_json_reply = process_request(request, core).await;
                        match result_json_reply {
                            Ok(reply) => ExecuteReply {
                                login_success: true,
                                execute_success: true,
                                reply: Some(reply),
                            },
                            Err(e) => ExecuteReply {
                                login_success: true,
                                execute_success: false,
                                reply: Some(e),
                            },
                        }
                    }
                    Err(e) => ExecuteReply {
                        login_success: false,
                        execute_success: false,
                        reply: Some(e.to_string()),
                    },
                }
            }
            None => ExecuteReply {
                login_success: false,
                execute_success: false,
                reply: None,
            },
        },
        Err(e) => ExecuteReply {
            login_success: false,
            execute_success: false,
            reply: Some(e.to_string()),
        },
    }
}
