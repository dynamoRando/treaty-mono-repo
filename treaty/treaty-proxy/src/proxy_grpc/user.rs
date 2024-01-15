use crate::TreatyProxy;
use chrono::Utc;
use stdext::function_name;
use tonic::{Request, Response, Status};
use tracing::{debug, trace, warn};
use treaty::{
    defaults,
    treaty_proto::{user_service_server::UserService, *},
    user_service_handler::user_service_handler_actions::UserServiceHandlerActions,
};
use treaty_http_endpoints::headers::TREATY_AUTH_HEADER_METADATA_BIN;

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
    async fn validate_auth_request(
        &self,
        request: &tonic::metadata::MetadataMap,
    ) -> Result<Box<dyn UserServiceHandlerActions + Send + Sync>, AuthResult> {
        trace!("[{}]: metadata: {request:?}", function_name!());

        if request.contains_key(TREATY_AUTH_HEADER_METADATA_BIN) {
            let kv = request.get_bin(TREATY_AUTH_HEADER_METADATA_BIN);
            if let Some(key_value) = kv {
                let bytes = key_value.to_bytes().unwrap();
                let metadata: AuthRequestMetadata = bincode::deserialize(&bytes).unwrap();
                trace!("[{}: {metadata:?}]", function_name!());
                if metadata.id.is_some() {
                    let id = metadata.id.unwrap();
                    let result_has_core = self.proxy.get_treaty_grpc_user_handler(&id).await;

                    if result_has_core.is_ok() {
                        trace!("[{}]: Caller is authenticated.", function_name!());
                        return Ok(result_has_core.unwrap());
                    } else {
                        warn!(
                            "[{}]: No host id found at treaty-proxy instance",
                            function_name!()
                        );
                        return Err(AuthResult {
                            is_authenticated: false,
                            message: Some(format!(
                                "Host Id: {id} was not found at treaty-proxy instance"
                            )),
                        });
                    }
                } else {
                    warn!(
                        "[{}]: No host id provided at treaty-proxy instance",
                        function_name!()
                    );
                    return Err(AuthResult {
                        is_authenticated: false,
                        message: Some("No Host Id provided for treaty-proxy instance".to_string()),
                    });
                }
            }
        }
        warn!(
            "[{}]: No authentication provided at treaty-proxy instance",
            function_name!()
        );
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

    async fn get_backing_database_config(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetBackingDatabaseConfigReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let auth_result = self.validate_auth_request(&request.metadata()).await;

        match auth_result {
            Ok(core) => {
                let response = core.get_backing_database_config().await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetBackingDatabaseConfigReply {
                    database_type: 0,
                    use_schema: false,
                    error: Some(TreatyError {
                        message: String::from("Could not get database type"),
                        help: None,
                        number: 0,
                    }),
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_logs_by_last_number(
        &self,
        request: Request<GetLogsByLastNumberRequest>,
    ) -> Result<Response<GetLogsByLastNumberReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!()

        // let request = request.into_inner().clone();
        // let auth_result = self.validate_auth_request(&request.metadata()).await;

        // match auth_result {
        //     Ok(core) => {
        //         let response = core.get_last_log_entries(request).await;
        //         return Ok(Response::new(response));
        //     }
        //     Err(auth_result) => {
        //         let reply = GetLogsByLastNumberReply {
        //
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

    async fn delete_user_database_destructively(
        &self,
        request: Request<DeleteUserDatabaseRequest>,
    ) -> Result<Response<DeleteUserDatabaseReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!()
    }

    async fn get_settings(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetSettingsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let auth_result = self.validate_auth_request(&request.metadata()).await;

        match auth_result {
            Ok(core) => {
                let response = core.get_settings().await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetSettingsReply {
                    settings_json: None,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn get_cooperative_hosts(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetCooperativeHostsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let auth_result = self.validate_auth_request(&request.metadata()).await;

        match auth_result {
            Ok(core) => {
                let response = core.get_cooperative_hosts().await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetCooperativeHostsReply {
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
        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.get_updates_from_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetUpdatesFromHostBehaviorReply {
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
        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.get_updates_to_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetUpdatesToHostBehaviorReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.get_deletes_from_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetDeletesFromHostBehaviorReply {
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
        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.get_deletes_to_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetDeletesToHostBehaviorReply {
                    behavior: None,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    #[allow(dead_code, unused_variables)]
    async fn get_versions(&self, request: Request<()>) -> Result<Response<VersionReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        // need to write an HTTP version as well
        todo!()
    }

    async fn get_host_info(&self, request: Request<()>) -> Result<Response<HostInfoReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let auth_result = self.validate_auth_request(&request.metadata()).await;

        match auth_result {
            Ok(core) => {
                let response = core.get_host_info().await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = HostInfoReply {
                    host_info: None,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn revoke_token(
        &self,
        request: Request<AuthRequestWebToken>,
    ) -> Result<Response<RevokeReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

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
        request: Request<AuthRequestBasic>,
    ) -> Result<Response<TokenReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.get_active_contract(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetActiveContractReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.get_participants(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetParticipantsReply {
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
        request: Request<()>,
    ) -> Result<Response<GetDatabasesReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let auth_result = self.validate_auth_request(&request.metadata()).await;

        match auth_result {
            Ok(core) => {
                let response = core.get_databases().await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetDatabasesReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.accept_pending_action_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = AcceptPendingActionReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.get_pending_actions_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetPendingActionsReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.generate_host_info(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GenerateHostInfoReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.create_user_database(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = CreateUserDatabaseReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.enable_coooperative_features(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = EnableCoooperativeFeaturesReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.execute_read_at_host(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = ExecuteReadReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.execute_read_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = ExecuteReadReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.execute_write_at_host(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = ExecuteWriteReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.execute_write_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = ExecuteWriteReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.execute_cooperative_write_at_host(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = ExecuteCooperativeWriteReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.has_table(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = HasTableReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.set_logical_storage_policy(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = SetLogicalStoragePolicyReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.get_logical_storage_policy(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetLogicalStoragePolicyReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.generate_contract(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GenerateContractReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.add_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = AddParticipantReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.send_participant_contract(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = SendParticipantContractReply {
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
        request: Request<()>,
    ) -> Result<Response<ViewPendingContractsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let auth_result = self.validate_auth_request(&request.metadata()).await;

        debug!("review_pending_contracts: {request:?}");

        match auth_result {
            Ok(core) => {
                let response = core.review_pending_contracts().await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = ViewPendingContractsReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.accept_pending_contract(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = AcceptPendingContractReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.change_host_status(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = ChangeHostStatusReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.try_auth_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = TryAuthAtPartipantReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.change_updates_from_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = ChangesUpdatesFromHostBehaviorReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.change_deletes_from_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = ChangeDeletesFromHostBehaviorReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.change_updates_to_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = ChangeUpdatesToHostBehaviorReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.change_deletes_to_host_behavior(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = ChangeDeletesToHostBehaviorReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.read_row_id_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetReadRowIdsReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.get_data_hash_at_host(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetDataHashReply {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.get_data_hash_at_participant(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetDataHashReply {
                    data_hash: 0,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }
}
