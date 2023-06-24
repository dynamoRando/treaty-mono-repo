use crate::TreatyProxy;
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{debug, warn};
use treaty::{
    data_service_handler::data_service_handler_actions::DataServiceHandlerActions,
    defaults,
    treaty_proto::{data_service_server::DataService, user_service_server::UserService, *},
    user_service_handler::user_service_handler_actions::UserServiceHandlerActions,
};

#[derive(Clone, Debug)]
#[allow(dead_code)]
/// Implements the `UserServiceHandler` definition from the protobuf file
pub struct ProxyUserServiceHandlerGrpc {
    root_folder: String,
    database_name: String,
    addr_port: String,
    own_db_addr_port: String,
    proxy: TreatyProxy,
}

impl ProxyUserServiceHandlerGrpc {
    pub fn new(
        root_folder: String,
        database_name: String,
        addr_port: String,
        own_db_addr_port: String,
        proxy: TreatyProxy,
    ) -> Self {
        Self {
            root_folder,
            database_name,
            addr_port,
            own_db_addr_port,
            proxy,
        }
    }

    /// Checks to see if the auth request:
    /// - Exists
    /// - Exists and has a host id
    /// - Exists, has a host id, and we have an account with that host id
    ///
    /// If the above conditions are true, it will return the `core` for that account,
    /// otherwise, we will return an `AuthResult` with an error message
    fn validate_auth_request(
        &self,
        auth: &Option<AuthRequest>,
    ) -> Result<impl UserServiceHandlerActions, AuthResult> {
        if auth.is_some() {
            let auth = auth.as_ref().unwrap().clone();
            if auth.id.is_some() {
                let id = auth.id.unwrap();
                let result_has_core = self.proxy.get_treaty_grpc_user_handler(&id);

                if result_has_core.is_ok() {
                    return Ok(result_has_core.unwrap());
                } else {
                    return Err(AuthResult {
                        is_authenticated: false,
                        message: Some(format!(
                            "Host Id: {id} was not found at treaty-proxy instance"
                        )),
                    });
                }
            } else {
                return Err(AuthResult {
                    is_authenticated: false,
                    message: Some("No Host Id provided for treaty-proxy instance".to_string()),
                });
            }
        }

        Err(AuthResult {
            is_authenticated: false,
            message: Some("No authentication provided".to_string()),
        })
    }
}

#[tonic::async_trait]
impl UserService for ProxyUserServiceHandlerGrpc {
    async fn is_online(
        &self,
        request: Request<TestRequest>,
    ) -> Result<Response<TestReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let reply = TestReply {
            reply_time_utc: Utc::now().to_rfc2822(),
            reply_echo_message: request.into_inner().request_echo_message,
            treaty_version: defaults::VERSION.to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn get_logs_by_last_number(
        &self,
        request: Request<GetLogsByLastNumberRequest>,
    ) -> Result<Response<GetLogsByLastNumberReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!()

        // let request = request.into_inner().clone();
        // let auth_result = self.validate_auth_request(&request.authentication);

        // match auth_result {
        //     Ok(core) => {
        //         let response = core.get_last_log_entries(request).await;
        //         return Ok(Response::new(response));
        //     }
        //     Err(auth_result) => {
        //         let reply = GetLogsByLastNumberReply {
        //             authentication_result: Some(auth_result),
        //             logs: Vec::new(),
        //         };

        //         return Ok(Response::new(reply));
        //     }
        // }
    }

    async fn delete_user_database(
        &self,
        request: Request<DeleteUserDatabaseRequest>,
    ) -> Result<Response<DeleteUserDatabaseReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!()
    }

    async fn get_settings(
        &self,
        request: Request<GetSettingsRequest>,
    ) -> Result<Response<GetSettingsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_settings(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetSettingsReply {
                    authentication_result: Some(auth_result),
                    settings_json: None,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_cooperative_hosts(
        &self,
        request: Request<GetCooperativeHostsRequest>,
    ) -> Result<Response<GetCooperativeHostsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_cooperative_hosts(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetCooperativeHostsReply {
                    authentication_result: Some(auth_result),
                    hosts: Vec::new(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_updates_from_host_behavior(
        &self,
        request: Request<GetUpdatesFromHostBehaviorRequest>,
    ) -> Result<Response<GetUpdatesFromHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_updates_from_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetUpdatesFromHostBehaviorReply {
                    authentication_result: Some(auth_result),
                    behavior: None,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_updates_to_host_behavior(
        &self,
        request: Request<GetUpdatesToHostBehaviorRequest>,
    ) -> Result<Response<GetUpdatesToHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_updates_to_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetUpdatesToHostBehaviorReply {
                    authentication_result: Some(auth_result),
                    behavior: None,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_deletes_from_host_behavior(
        &self,
        request: Request<GetDeletesFromHostBehaviorRequest>,
    ) -> Result<Response<GetDeletesFromHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_deletes_from_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetDeletesFromHostBehaviorReply {
                    authentication_result: Some(auth_result),
                    behavior: None,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_deletes_to_host_behavior(
        &self,
        request: Request<GetDeletesToHostBehaviorRequest>,
    ) -> Result<Response<GetDeletesToHostBehaviorReply>, Status> {
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_deletes_to_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetDeletesToHostBehaviorReply {
                    authentication_result: Some(auth_result),
                    behavior: None,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    #[allow(dead_code, unused_variables)]
    async fn get_versions(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<VersionReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        // need to write an HTTP version as well
        todo!()
    }

    async fn get_host_info(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<HostInfoReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&Some(request.clone()));

        match auth_result {
            Ok(core) => {
                let response = core.get_host_info(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = HostInfoReply {
                    authentication_result: Some(auth_result),
                    host_info: None,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn revoke_token(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<RevokeReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&Some(request.clone()));

        match auth_result {
            Ok(core) => {
                let response = core.revoke_token(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = RevokeReply {
                    is_successful: false,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn auth_for_token(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<TokenReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&Some(request.clone()));

        match auth_result {
            Ok(core) => {
                let response = core.auth_for_token(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = TokenReply {
                    is_successful: false,
                    expiration_utc: "".to_string(),
                    jwt: "".to_string(),
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_active_contract(
        &self,
        request: Request<GetActiveContractRequest>,
    ) -> Result<Response<GetActiveContractReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_active_contract(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetActiveContractReply {
                    authentication_result: Some(auth_result),
                    contract: None,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_participants(
        &self,
        request: Request<GetParticipantsRequest>,
    ) -> Result<Response<GetParticipantsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_participants(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetParticipantsReply {
                    authentication_result: Some(auth_result),
                    participants: Vec::new(),
                    is_error: true,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_databases(
        &self,
        request: Request<GetDatabasesRequest>,
    ) -> Result<Response<GetDatabasesReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_databases(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetDatabasesReply {
                    authentication_result: Some(auth_result),
                    databases: Vec::new(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn accept_pending_action_at_participant(
        &self,
        request: Request<AcceptPendingActionRequest>,
    ) -> Result<Response<AcceptPendingActionReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.accept_pending_action_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = AcceptPendingActionReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_pending_actions_at_participant(
        &self,
        request: Request<GetPendingActionsRequest>,
    ) -> Result<Response<GetPendingActionsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_pending_actions_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetPendingActionsReply {
                    authentication_result: Some(auth_result),
                    pending_statements: Vec::new(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    #[allow(dead_code, unused_variables)]
    async fn get_data_log_table_status_at_participant(
        &self,
        request: Request<GetDataLogTableStatusRequest>,
    ) -> Result<Response<GetDataLogTableStatusReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        unimplemented!();
    }

    #[allow(dead_code, unused_variables)]
    async fn set_data_log_table_status_at_participant(
        &self,
        request: Request<SetDataLogTableStatusRequest>,
    ) -> Result<Response<SetDataLogTableStatusReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        unimplemented!();
    }

    async fn generate_host_info(
        &self,
        request: Request<GenerateHostInfoRequest>,
    ) -> Result<Response<GenerateHostInfoReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.generate_host_info(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GenerateHostInfoReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn create_user_database(
        &self,
        request: Request<CreateUserDatabaseRequest>,
    ) -> Result<Response<CreateUserDatabaseReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.create_user_database(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = CreateUserDatabaseReply {
                    authentication_result: Some(auth_result),
                    is_created: false,
                    message: "".to_string(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn enable_coooperative_features(
        &self,
        request: Request<EnableCoooperativeFeaturesRequest>,
    ) -> Result<Response<EnableCoooperativeFeaturesReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.enable_coooperative_features(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = EnableCoooperativeFeaturesReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    message: "".to_string(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    #[allow(dead_code, unused_variables)]
    async fn execute_read_at_host(
        &self,
        request: Request<ExecuteReadRequest>,
    ) -> Result<Response<ExecuteReadReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.execute_read_at_host(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = ExecuteReadReply {
                    authentication_result: Some(auth_result),
                    total_resultsets: 0,
                    results: Vec::new(),
                    is_error: true,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn execute_read_at_participant(
        &self,
        request: Request<ExecuteReadRequest>,
    ) -> Result<Response<ExecuteReadReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.execute_read_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = ExecuteReadReply {
                    authentication_result: Some(auth_result),
                    total_resultsets: 0,
                    results: Vec::new(),
                    is_error: true,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn execute_write_at_host(
        &self,
        request: Request<ExecuteWriteRequest>,
    ) -> Result<Response<ExecuteWriteReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.execute_write_at_host(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = ExecuteWriteReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    is_error: true,
                    total_rows_affected: 0,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn execute_write_at_participant(
        &self,
        request: Request<ExecuteWriteRequest>,
    ) -> Result<Response<ExecuteWriteReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.execute_write_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = ExecuteWriteReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    is_error: true,
                    total_rows_affected: 0,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn execute_cooperative_write_at_host(
        &self,
        request: Request<ExecuteCooperativeWriteRequest>,
    ) -> Result<Response<ExecuteCooperativeWriteReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.execute_cooperative_write_at_host(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = ExecuteCooperativeWriteReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    total_rows_affected: 0,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn has_table(
        &self,
        request: Request<HasTableRequest>,
    ) -> Result<Response<HasTableReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.has_table(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = HasTableReply {
                    authentication_result: Some(auth_result),
                    has_table: false,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn set_logical_storage_policy(
        &self,
        request: Request<SetLogicalStoragePolicyRequest>,
    ) -> Result<Response<SetLogicalStoragePolicyReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.set_logical_storage_policy(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = SetLogicalStoragePolicyReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    message: "".to_string(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_logical_storage_policy(
        &self,
        request: Request<GetLogicalStoragePolicyRequest>,
    ) -> Result<Response<GetLogicalStoragePolicyReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_logical_storage_policy(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetLogicalStoragePolicyReply {
                    authentication_result: Some(auth_result),
                    policy_mode: 0,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn generate_contract(
        &self,
        request: Request<GenerateContractRequest>,
    ) -> Result<Response<GenerateContractReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.generate_contract(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GenerateContractReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    message: "".to_string(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn add_participant(
        &self,
        request: Request<AddParticipantRequest>,
    ) -> Result<Response<AddParticipantReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.add_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = AddParticipantReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    message: "".to_string(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn send_participant_contract(
        &self,
        request: Request<SendParticipantContractRequest>,
    ) -> Result<Response<SendParticipantContractReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.send_participant_contract(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = SendParticipantContractReply {
                    authentication_result: Some(auth_result),
                    is_sent: false,
                    contract_status: 0,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn review_pending_contracts(
        &self,
        request: Request<ViewPendingContractsRequest>,
    ) -> Result<Response<ViewPendingContractsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        debug!("review_pending_contracts: {request:?}");

        match auth_result {
            Ok(core) => {
                let response = core.review_pending_contracts(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = ViewPendingContractsReply {
                    authentication_result: Some(auth_result),
                    contracts: Vec::new(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn accept_pending_contract(
        &self,
        request: Request<AcceptPendingContractRequest>,
    ) -> Result<Response<AcceptPendingContractReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.accept_pending_contract(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = AcceptPendingContractReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    message: "".to_string(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    #[allow(dead_code, unused_variables)]
    async fn reject_pending_contract(
        &self,
        _request: tonic::Request<RejectPendingContractRequest>,
    ) -> Result<tonic::Response<RejectPendingContractReply>, tonic::Status> {
        unimplemented!();
    }

    async fn change_host_status(
        &self,
        request: tonic::Request<ChangeHostStatusRequest>,
    ) -> Result<tonic::Response<ChangeHostStatusReply>, tonic::Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.change_host_status(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = ChangeHostStatusReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    status: 0,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn try_auth_at_participant(
        &self,
        request: tonic::Request<TryAuthAtParticipantRequest>,
    ) -> Result<tonic::Response<TryAuthAtPartipantReply>, tonic::Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.try_auth_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = TryAuthAtPartipantReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    message: "".to_string(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn change_updates_from_host_behavior(
        &self,
        request: Request<ChangeUpdatesFromHostBehaviorRequest>,
    ) -> Result<Response<ChangesUpdatesFromHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.change_updates_from_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = ChangesUpdatesFromHostBehaviorReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    message: "".to_string(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn change_deletes_from_host_behavior(
        &self,
        request: Request<ChangeDeletesFromHostBehaviorRequest>,
    ) -> Result<Response<ChangeDeletesFromHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.change_deletes_from_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = ChangeDeletesFromHostBehaviorReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    message: "".to_string(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn change_updates_to_host_behavior(
        &self,
        request: Request<ChangeUpdatesToHostBehaviorRequest>,
    ) -> Result<Response<ChangeUpdatesToHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.change_updates_to_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = ChangeUpdatesToHostBehaviorReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    message: "".to_string(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn change_deletes_to_host_behavior(
        &self,
        request: Request<ChangeDeletesToHostBehaviorRequest>,
    ) -> Result<Response<ChangeDeletesToHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.change_deletes_to_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = ChangeDeletesToHostBehaviorReply {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    message: "".to_string(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn read_row_id_at_participant(
        &self,
        request: Request<GetReadRowIdsRequest>,
    ) -> Result<Response<GetReadRowIdsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.read_row_id_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetReadRowIdsReply {
                    authentication_result: Some(auth_result),
                    row_ids: Vec::new(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_data_hash_at_host(
        &self,
        request: Request<GetDataHashRequest>,
    ) -> Result<Response<GetDataHashReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_data_hash_at_host(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetDataHashReply {
                    authentication_result: Some(auth_result),
                    data_hash: 0,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_data_hash_at_participant(
        &self,
        request: Request<GetDataHashRequest>,
    ) -> Result<Response<GetDataHashReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_data_hash_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetDataHashReply {
                    authentication_result: Some(auth_result),
                    data_hash: 0,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }
}

#[allow(dead_code, unused_variables)]
pub struct ProxyDataServiceHandlerGrpc {
    root_folder: String,
    database_name: String,
    addr_port: String,
    proxy: TreatyProxy,
}

impl ProxyDataServiceHandlerGrpc {
    #[allow(dead_code, unused_variables)]
    pub fn new(
        root_folder: String,
        database_name: String,
        addr_port: String,
        proxy: TreatyProxy,
    ) -> Self {
        Self {
            root_folder,
            database_name,
            addr_port,
            proxy,
        }
    }

    /// Checks to see if the auth request:
    /// - Exists
    /// - Exists and has a host id
    /// - Exists, has a host id, and we have an account with that host id
    ///
    /// If the above conditions are true, it will return the `core` for that account,
    /// otherwise, we will return an `AuthResult` with an error message
    #[allow(dead_code, unused_variables)]
    fn validate_auth_request(
        &self,
        auth: &Option<AuthRequest>,
    ) -> Result<impl DataServiceHandlerActions, AuthResult> {
        if auth.is_some() {
            let auth = auth.as_ref().unwrap().clone();
            let un = auth.user_name.clone();
            if auth.id.is_some() {
                let id = auth.id.unwrap();
                let result_has_core = self.proxy.get_treaty_data_handler(&id);

                if result_has_core.is_ok() {
                    return Ok(result_has_core.unwrap());
                } else {
                    return Err(AuthResult {
                        is_authenticated: false,
                        message: Some(format!(
                            "Host Id: {id} was not found at treaty-proxy instance"
                        )),
                    });
                }
            } else {
                return Err(AuthResult {
                    is_authenticated: false,
                    message: Some("No Host Id provided for treaty-proxy instance".to_string()),
                });
            }
        }

        Err(AuthResult {
            is_authenticated: false,
            message: Some("No authentication provided".to_string()),
        })
    }
}

#[tonic::async_trait]
impl DataService for ProxyDataServiceHandlerGrpc {
    async fn is_online(
        &self,
        request: Request<TestRequest>,
    ) -> Result<Response<TestReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let item = request.into_inner().request_echo_message;

        Ok(Response::new(TestReply {
            reply_time_utc: Utc::now().to_rfc2822(),
            reply_echo_message: item,
            treaty_version: defaults::VERSION.to_string(),
        }))
    }

    async fn create_partial_database(
        &self,
        request: Request<CreatePartialDatabaseRequest>,
    ) -> Result<Response<CreatePartialDatabaseResult>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.create_partial_database(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = CreatePartialDatabaseResult {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    database_name: "".to_string(),
                    database_id: "".to_string(),
                    is_error: true,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn create_table_in_database(
        &self,
        request: Request<CreateTableRequest>,
    ) -> Result<Response<CreateTableResult>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.create_table_in_database(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = CreateTableResult {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    database_name: "".to_string(),
                    result_message: "".to_string(),
                    table_name: "".to_string(),
                    table_id: "".to_string(),
                    database_id: "".to_string(),
                    is_error: true,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn insert_command_into_table(
        &self,
        request: Request<InsertDataRequest>,
    ) -> Result<Response<InsertDataResult>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.insert_command_into_table(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = InsertDataResult {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    data_hash: 0,
                    message: "".to_string(),
                    row_id: 0,
                    is_error: true,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn update_command_into_table(
        &self,
        request: Request<UpdateDataRequest>,
    ) -> Result<Response<UpdateDataResult>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.update_command_into_table(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = UpdateDataResult {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    message: "".to_string(),
                    rows: Vec::new(),
                    update_status: 0,
                    is_error: true,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn delete_command_into_table(
        &self,
        request: Request<DeleteDataRequest>,
    ) -> Result<Response<DeleteDataResult>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.delete_command_into_table(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = DeleteDataResult {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    message: "".to_string(),
                    rows: Vec::new(),
                    is_error: true,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_row_from_partial_database(
        &self,
        request: Request<GetRowFromPartialDatabaseRequest>,
    ) -> Result<Response<GetRowFromPartialDatabaseResult>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.get_row_from_partial_database(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = GetRowFromPartialDatabaseResult {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    row: None,
                    result_message: "".to_string(),
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn save_contract(
        &self,
        request: Request<SaveContractRequest>,
    ) -> Result<Response<SaveContractResult>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();

        let auth = AuthRequest {
            user_name: "".to_string(),
            pw: "".to_string(),
            pw_hash: Vec::new(),
            token: Vec::new(),
            jwt: "".to_string(),
            id: request.id.clone(),
        };

        let auth_result = self.validate_auth_request(&Some(auth));

        match auth_result {
            Ok(core) => {
                let response = core.save_contract(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = SaveContractResult {
                    is_saved: false,
                    contract_status: 0,
                    participant_info: None,
                    is_error: true,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn accept_contract(
        &self,
        request: Request<ParticipantAcceptsContractRequest>,
    ) -> Result<Response<ParticipantAcceptsContractResult>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();

        let id = request.id.as_ref().unwrap().clone();

        let auth = AuthRequest {
            user_name: "".to_string(),
            pw: "".to_string(),
            pw_hash: Vec::new(),
            token: Vec::new(),
            jwt: "".to_string(),
            id: Some(id),
        };

        let auth_result = self.validate_auth_request(&Some(auth));

        match auth_result {
            Ok(core) => {
                let response = core.accept_contract(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = ParticipantAcceptsContractResult {
                    contract_acceptance_is_acknowledged: false,
                    is_error: true,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn update_row_data_hash_for_host(
        &self,
        request: Request<UpdateRowDataHashForHostRequest>,
    ) -> Result<Response<UpdateRowDataHashForHostResult>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.update_row_data_hash_for_host(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = UpdateRowDataHashForHostResult {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn notify_host_of_removed_row(
        &self,
        request: Request<NotifyHostOfRemovedRowRequest>,
    ) -> Result<Response<NotifyHostOfRemovedRowResult>, Status> {
        debug!(
            "notify_host_of_removed_row: Request from {:?}",
            request.remote_addr()
        );

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.notify_host_of_removed_row(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = NotifyHostOfRemovedRowResult {
                    authentication_result: Some(auth_result),
                    is_successful: false,
                    error: None,
                };

                warn!("notify_host_of_removed_row: {reply:?}");

                return Ok(Response::new(reply));
            }
        }
    }

    async fn try_auth(
        &self,
        request: Request<TryAuthRequest>,
    ) -> Result<Response<TryAuthResult>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let request = request.into_inner().clone();
        let auth_result = self.validate_auth_request(&request.authentication);

        match auth_result {
            Ok(core) => {
                let response = core.try_auth(request).await;
                return Ok(Response::new(response));
            }
            Err(auth_result) => {
                let reply = TryAuthResult {
                    authentication_result: Some(auth_result),
                };

                return Ok(Response::new(reply));
            }
        }
    }
}
