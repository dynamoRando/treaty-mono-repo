use std::time::Duration;

use crate::client_actions::ClientActions;
use async_trait::async_trait;
use stdext::function_name;
use tonic::transport::{Channel, ClientTlsConfig};
use tonic::{codegen::InterceptedService, transport::Certificate};
use tracing::{debug, trace, warn};
use treaty::treaty_proto::{
    user_service_client::UserServiceClient, AcceptPendingActionReply, AcceptPendingActionRequest,
    AcceptPendingContractRequest, AddParticipantRequest, ChangeDeletesFromHostBehaviorRequest,
    ChangeDeletesToHostBehaviorRequest, ChangeHostStatusRequest,
    ChangeUpdatesFromHostBehaviorRequest, ChangeUpdatesToHostBehaviorRequest, Contract,
    CreateUserDatabaseRequest, DeleteUserDatabaseReply, DeleteUserDatabaseRequest,
    EnableCoooperativeFeaturesRequest, ExecuteCooperativeWriteRequest, ExecuteReadRequest,
    ExecuteWriteRequest, GenerateContractRequest, GenerateHostInfoRequest, GetActiveContractReply,
    GetActiveContractRequest, GetBackingDatabaseConfigReply, GetCooperativeHostsReply,
    GetDataHashRequest, GetDatabasesReply, GetDeletesFromHostBehaviorReply,
    GetDeletesFromHostBehaviorRequest, GetDeletesToHostBehaviorReply,
    GetDeletesToHostBehaviorRequest, GetLogicalStoragePolicyRequest, GetParticipantsReply,
    GetParticipantsRequest, GetPendingActionsReply, GetPendingActionsRequest, GetReadRowIdsRequest,
    GetSettingsReply, GetUpdatesFromHostBehaviorReply, GetUpdatesFromHostBehaviorRequest,
    GetUpdatesToHostBehaviorReply, GetUpdatesToHostBehaviorRequest, HasTableRequest, HostInfoReply,
    RevokeReply, SendParticipantContractRequest, SetLogicalStoragePolicyRequest,
    StatementResultset, TestRequest, TokenReply, TreatyError, TryAuthAtParticipantRequest,
};

use self::interceptors::AuthenticationInterceptor;
use treaty_types::enums::*;

mod interceptors;

#[derive(Debug, Clone)]
pub struct GrpcClient {
    /// The HTTP (or HTTPS) address and port for the User service of the `treaty` instance you are talking to. Example: `http://127.0.0.1:50051`
    user_service_address_port: String,
    /// The HTTP (or HTTPS) address and port of the Info service of the `treaty` instance you are talking to.
    info_service_address_port: String,
    /// The timeout for any gRPC call in seconds
    timeout_in_seconds: u32,
    /// The client for making calls
    grpc_client: UserServiceClient<InterceptedService<Channel, AuthenticationInterceptor>>,
    /// Auth Interceptor
    interceptor: AuthenticationInterceptor,
}

#[derive(Debug, Clone, Default)]
pub struct TlsSettings {
    pub pem: String,
    pub domain: Option<String>,
}

impl GrpcClient {
    pub async fn new(
        user_service_address_port: &str,
        info_service_address_port: &str,
        timeout_in_seconds: u32,
        username: &str,
        pw: &str,
        host_id: Option<String>,
        tls: Option<TlsSettings>,
    ) -> Self {
        trace!(
            "[{}]: connecting to: {}",
            function_name!(),
            user_service_address_port
        );

        let tls_opts = tls.clone();
        let channel = match tls_opts {
            Some(settings) => {
                let ca = Certificate::from_pem(settings.pem);

                let tls = match settings.domain {
                    Some(domain) => ClientTlsConfig::new()
                        .ca_certificate(ca)
                        .domain_name(domain),
                    None => ClientTlsConfig::new().ca_certificate(ca),
                };

                Channel::builder(user_service_address_port.parse().unwrap())
                    .tls_config(tls)
                    .unwrap()
                    .timeout(Duration::from_secs(timeout_in_seconds.into()))
                    .connect()
                    .await
                    .unwrap()
            }
            None => Channel::builder(user_service_address_port.parse().unwrap())
                .timeout(Duration::from_secs(timeout_in_seconds.into()))
                .connect()
                .await
                .unwrap(),
        };

        let interceptor = AuthenticationInterceptor::new(
            username,
            pw,
            info_service_address_port,
            timeout_in_seconds.into(),
            host_id,
            tls.clone(),
        )
        .await;

        let user_client = UserServiceClient::with_interceptor(channel, interceptor.clone());

        Self {
            user_service_address_port: user_service_address_port.to_string(),
            info_service_address_port: info_service_address_port.to_string(),
            timeout_in_seconds,
            grpc_client: user_client,
            interceptor: interceptor.clone(),
        }
    }

    pub fn timeout_in_seconds(&self) -> u32 {
        self.timeout_in_seconds
    }

    pub fn user_service_address_port(&self) -> String {
        self.user_service_address_port.clone()
    }

    pub fn info_service_address_port(&self) -> String {
        self.info_service_address_port.clone()
    }
}

#[async_trait]
impl ClientActions for GrpcClient {
    async fn is_online(&mut self) -> Result<bool, TreatyError> {
        let test_string = "is_online";

        let request = TestRequest {
            request_time_utc: "".to_string(),
            request_origin_url: "".to_string(),
            request_origin_ip4: "".to_string(),
            request_origin_ip6: "".to_string(),
            request_port_number: 0,
            request_echo_message: test_string.to_string(),
        };

        let result = self.grpc_client.is_online(request).await.unwrap();

        Ok(result.into_inner().reply_echo_message == test_string)
    }

    async fn get_backing_db_config(
        &mut self,
    ) -> Result<GetBackingDatabaseConfigReply, TreatyError> {
        let response = self
            .grpc_client
            .get_backing_database_config(())
            .await
            .unwrap()
            .into_inner();

        return Ok(response);
    }

    async fn get_host_info(&mut self) -> Result<HostInfoReply, TreatyError> {
        let response = self
            .grpc_client
            .get_host_info(())
            .await
            .unwrap()
            .into_inner();

        return Ok(response);
    }

    async fn get_active_contract(
        &mut self,
        db_name: &str,
    ) -> Result<GetActiveContractReply, TreatyError> {
        let request = GetActiveContractRequest {
            database_name: db_name.to_string(),
        };

        let response = self
            .grpc_client
            .get_active_contract(request)
            .await
            .unwrap()
            .into_inner();

        Ok(response)
    }

    async fn revoke_token(&mut self) -> Result<RevokeReply, TreatyError> {
        let auth = self.interceptor.web_token().unwrap();

        let response = self
            .grpc_client
            .revoke_token(auth)
            .await
            .unwrap()
            .into_inner();

        Ok(response)
    }

    async fn drop_database_forcefully(
        &mut self,
        db_name: &str,
    ) -> Result<DeleteUserDatabaseReply, TreatyError> {
        let request = DeleteUserDatabaseRequest {
            database_name: db_name.to_string(),
        };

        let response = self
            .grpc_client
            .delete_user_database_destructively(request)
            .await
            .unwrap()
            .into_inner();

        Ok(response)
    }

    async fn auth_for_token(&mut self) -> Result<TokenReply, TreatyError> {
        let auth = self.interceptor.basic_auth();

        let response = self
            .grpc_client
            .auth_for_token(auth)
            .await
            .unwrap()
            .into_inner();

        if !response.is_successful {
            warn!("[{}]: could not authenticate for token", function_name!());
        }

        Ok(response)
    }

    async fn accept_pending_action_at_participant(
        &mut self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<AcceptPendingActionReply, TreatyError> {
        let request = AcceptPendingActionRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            row_id,
        };

        let response = self
            .grpc_client
            .accept_pending_action_at_participant(request)
            .await
            .unwrap()
            .into_inner();

        Ok(response)
    }

    async fn get_cooperative_hosts(&mut self) -> Result<GetCooperativeHostsReply, TreatyError> {
        let response = self
            .grpc_client
            .get_cooperative_hosts(tonic::Request::new(()))
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={:?}", function_name!(), response);

        Ok(response)
    }

    async fn get_participants_for_database(
        &mut self,
        db_name: &str,
    ) -> Result<GetParticipantsReply, TreatyError> {
        let request = GetParticipantsRequest {
            database_name: db_name.to_string(),
        };

        let response = self
            .grpc_client
            .get_participants(tonic::Request::new(request))
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={:?}", function_name!(), response);

        Ok(response)
    }

    async fn get_pending_actions_at_participant(
        &mut self,
        db_name: &str,
        table_name: &str,
        action: &str,
    ) -> Result<GetPendingActionsReply, TreatyError> {
        let request = GetPendingActionsRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            action: action.to_string(),
        };

        let response = self
            .grpc_client
            .get_pending_actions_at_participant(tonic::Request::new(request))
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={:?}", function_name!(), response);

        Ok(response)
    }

    async fn get_row_id_at_participant(
        &mut self,
        db_name: &str,
        table_name: &str,
        where_clause: &str,
    ) -> Result<Vec<u32>, TreatyError> {
        let request = GetReadRowIdsRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            where_clause: where_clause.to_string(),
        };

        let response = self
            .grpc_client
            .read_row_id_at_participant(tonic::Request::new(request))
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={:?}", function_name!(), response);

        Ok(response.row_ids)
    }

    async fn get_data_hash_at_participant(
        &mut self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<u64, TreatyError> {
        let request = GetDataHashRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            row_id,
        };

        let response = self
            .grpc_client
            .get_data_hash_at_participant(tonic::Request::new(request))
            .await
            .unwrap()
            .into_inner();
        debug!("RESPONSE={:?}", response);

        Ok(response.data_hash)
    }

    async fn get_data_hash_at_host(
        &mut self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<u64, TreatyError> {
        let request = GetDataHashRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            row_id,
        };

        let response = self
            .grpc_client
            .get_data_hash_at_host(tonic::Request::new(request))
            .await
            .unwrap()
            .into_inner();
        debug!("RESPONSE={:?}", response);

        Ok(response.data_hash)
    }

    async fn get_deletes_to_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
    ) -> Result<GetDeletesToHostBehaviorReply, TreatyError> {
        let request = GetDeletesToHostBehaviorRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        };

        let response = self
            .grpc_client
            .get_deletes_to_host_behavior(tonic::Request::new(request))
            .await
            .unwrap()
            .into_inner();
        debug!("RESPONSE={:?}", response);

        Ok(response)
    }

    async fn change_deletes_to_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
        behavior: DeletesToHostBehavior,
    ) -> Result<bool, TreatyError> {
        let request = ChangeDeletesToHostBehaviorRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            behavior: num::ToPrimitive::to_u32(&behavior).unwrap(),
        };

        let response = self
            .grpc_client
            .change_deletes_to_host_behavior(tonic::Request::new(request))
            .await
            .unwrap()
            .into_inner();
        trace!("RESPONSE={:?}", response);

        Ok(response.is_successful)
    }

    async fn get_updates_to_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
    ) -> Result<GetUpdatesToHostBehaviorReply, TreatyError> {
        let request = GetUpdatesToHostBehaviorRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        };

        let response = self
            .grpc_client
            .get_updates_to_host_behavior(tonic::Request::new(request))
            .await
            .unwrap()
            .into_inner();
        trace!("RESPONSE={:?}", response);

        Ok(response)
    }

    async fn change_updates_to_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
        behavior: UpdatesToHostBehavior,
    ) -> Result<bool, TreatyError> {
        let request = ChangeUpdatesToHostBehaviorRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            behavior: num::ToPrimitive::to_u32(&behavior).unwrap(),
        };

        let response = self
            .grpc_client
            .change_updates_to_host_behavior(tonic::Request::new(request))
            .await
            .unwrap()
            .into_inner();
        debug!("RESPONSE={:?}", response);

        Ok(response.is_successful)
    }

    async fn get_deletes_from_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
    ) -> Result<GetDeletesFromHostBehaviorReply, TreatyError> {
        let request = GetDeletesFromHostBehaviorRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        };

        let response = self
            .grpc_client
            .get_deletes_from_host_behavior(tonic::Request::new(request))
            .await
            .unwrap()
            .into_inner();
        debug!("RESPONSE={:?}", response);

        Ok(response)
    }

    async fn change_deletes_from_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
        behavior: DeletesFromHostBehavior,
    ) -> Result<bool, TreatyError> {
        let request = ChangeDeletesFromHostBehaviorRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            behavior: num::ToPrimitive::to_u32(&behavior).unwrap(),
        };

        let response = self
            .grpc_client
            .change_deletes_from_host_behavior(tonic::Request::new(request))
            .await
            .unwrap()
            .into_inner();
        debug!("RESPONSE={:?}", response);

        Ok(response.is_successful)
    }

    async fn get_updates_from_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
    ) -> Result<GetUpdatesFromHostBehaviorReply, TreatyError> {
        let request = GetUpdatesFromHostBehaviorRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        };

        let response = self
            .grpc_client
            .get_updates_from_host_behavior(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response)
    }

    async fn change_updates_from_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
        behavior: UpdatesFromHostBehavior,
    ) -> Result<bool, TreatyError> {
        let request = ChangeUpdatesFromHostBehaviorRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            behavior: UpdatesFromHostBehavior::to_u32(behavior),
        };

        let response = self
            .grpc_client
            .change_updates_from_host_behavior(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={:?}", function_name!(), response);

        Ok(response.is_successful)
    }

    async fn change_host_status_by_id(
        &mut self,
        host_id: &str,
        status: u32,
    ) -> Result<bool, TreatyError> {
        let request = ChangeHostStatusRequest {
            host_alias: String::from(""),
            host_id: host_id.to_string(),
            status,
        };

        let response = self
            .grpc_client
            .change_host_status(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_successful)
    }

    async fn change_host_status_by_name(
        &mut self,
        host_name: &str,
        status: u32,
    ) -> Result<bool, TreatyError> {
        let request = ChangeHostStatusRequest {
            host_alias: host_name.to_string(),
            host_id: String::from(""),
            status,
        };

        let response = self
            .grpc_client
            .change_host_status(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_successful)
    }

    async fn generate_host_info(&mut self, host_name: &str) -> Result<bool, TreatyError> {
        let request = GenerateHostInfoRequest {
            host_name: host_name.to_string(),
        };

        let response = self
            .grpc_client
            .generate_host_info(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_successful)
    }

    async fn get_databases(&mut self) -> Result<GetDatabasesReply, TreatyError> {
        let response = self
            .grpc_client
            .get_databases(tonic::Request::new(()))
            .await
            .unwrap()
            .into_inner();

        trace!("[{}]: {:#?}", function_name!(), response);

        Ok(response)
    }
    async fn execute_cooperative_write_at_host(
        &mut self,
        db_name: &str,
        cmd: &str,
        participant_alias: &str,
        where_clause: &str,
    ) -> Result<bool, TreatyError> {
        let request = ExecuteCooperativeWriteRequest {
            database_name: db_name.to_string(),
            sql_statement: cmd.to_string(),
            database_type: DatabaseType::to_u32(DatabaseType::Sqlite),
            alias: participant_alias.to_string(),
            participant_id: String::from(""),
            where_clause: where_clause.to_string(),
        };

        trace!("request: {request:?}");

        let response = self
            .grpc_client
            .execute_cooperative_write_at_host(request)
            .await
            .unwrap()
            .into_inner();

        trace!("[{}]: {:?}", function_name!(), response);

        Ok(response.is_successful)
    }
    async fn view_pending_contracts(&mut self) -> Result<Vec<Contract>, TreatyError> {
        let response = self
            .grpc_client
            .review_pending_contracts(tonic::Request::new(()))
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.contracts)
    }

    async fn accept_pending_contract(&mut self, host_alias: &str) -> Result<bool, TreatyError> {
        let request = AcceptPendingContractRequest {
            host_alias: host_alias.to_string(),
        };

        let response = self
            .grpc_client
            .accept_pending_contract(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_successful)
    }

    async fn send_participant_contract(
        &mut self,
        db_name: &str,
        participant_alias: &str,
    ) -> Result<bool, TreatyError> {
        let request = SendParticipantContractRequest {
            database_name: db_name.to_string(),
            participant_alias: participant_alias.to_string(),
        };

        let response = self
            .grpc_client
            .send_participant_contract(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_sent)
    }

    async fn add_participant(
        &mut self,
        db_name: &str,
        participant_alias: &str,
        participant_ip4addr: &str,
        participant_db_port: Option<u32>,
        participant_info_port: u32,
        participant_http_addr: &str,
        participant_http_port: u16,
        participant_id: Option<String>,
    ) -> Result<bool, TreatyError> {
        let request = AddParticipantRequest {
            database_name: db_name.to_string(),
            alias: participant_alias.to_string(),
            ip4_address: participant_ip4addr.to_string(),
            db_port: participant_db_port,
            info_port: participant_info_port,
            http_addr: participant_http_addr.to_string(),
            http_port: participant_http_port as u32,
            id: participant_id,
        };

        let response = self
            .grpc_client
            .add_participant(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_successful)
    }

    async fn generate_contract(
        &mut self,
        db_name: &str,
        host_name: &str,
        desc: &str,
        remote_delete_behavior: RemoteDeleteBehavior,
    ) -> Result<bool, TreatyError> {
        let request = GenerateContractRequest {
            host_name: host_name.to_string(),
            description: desc.to_string(),
            database_name: db_name.to_string(),
            remote_delete_behavior: RemoteDeleteBehavior::to_u32(remote_delete_behavior),
        };

        let response = self
            .grpc_client
            .generate_contract(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_successful)
    }

    async fn has_table(&mut self, db_name: &str, table_name: &str) -> Result<bool, TreatyError> {
        let request = HasTableRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        };

        let response = self
            .grpc_client
            .has_table(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.has_table)
    }

    async fn get_logical_storage_policy(
        &mut self,
        db_name: &str,
        table_name: &str,
    ) -> Result<LogicalStoragePolicy, TreatyError> {
        let request = GetLogicalStoragePolicyRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        };

        let response = self
            .grpc_client
            .get_logical_storage_policy(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        let policy = LogicalStoragePolicy::from_i64(response.policy_mode as i64);

        Ok(policy)
    }

    async fn get_settings(&mut self) -> Result<GetSettingsReply, TreatyError> {
        let response = self
            .grpc_client
            .get_settings(tonic::Request::new(()))
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response)
    }

    async fn set_logical_storage_policy(
        &mut self,
        db_name: &str,
        table_name: &str,
        policy: LogicalStoragePolicy,
    ) -> Result<bool, TreatyError> {
        let request = SetLogicalStoragePolicyRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            policy_mode: LogicalStoragePolicy::to_u32(policy),
        };

        let response = self
            .grpc_client
            .set_logical_storage_policy(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_successful)
    }
    async fn execute_write_at_host(
        &mut self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
        where_clause: &str,
    ) -> Result<bool, TreatyError> {
        let request = ExecuteWriteRequest {
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
            where_clause: where_clause.to_string(),
        };

        let response = self
            .grpc_client
            .execute_write_at_host(request)
            .await
            .unwrap()
            .into_inner();

        trace!("[{}]: RESPONSE={:?}", function_name!(), response);

        Ok(response.is_successful)
    }
    async fn execute_write_at_participant(
        &mut self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
        where_clause: &str,
    ) -> Result<bool, TreatyError> {
        let request = ExecuteWriteRequest {
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
            where_clause: where_clause.to_string(),
        };

        let response = self
            .grpc_client
            .execute_write_at_participant(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_successful)
    }
    async fn try_auth_at_participant(
        &mut self,
        alias: &str,
        id: &str,
        db_name: &str,
    ) -> Result<bool, TreatyError> {
        let request = TryAuthAtParticipantRequest {
            participant_id: id.to_string(),
            participant_alias: alias.to_string(),
            db_name: db_name.to_string(),
        };

        let response = self
            .grpc_client
            .try_auth_at_participant(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={:?}", function_name!(), response);

        Ok(response.is_successful)
    }
    async fn execute_read_at_participant(
        &mut self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
    ) -> Result<StatementResultset, TreatyError> {
        let request = ExecuteReadRequest {
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
        };

        trace!("REQUEST={request:?}");

        let response = self
            .grpc_client
            .execute_read_at_participant(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.results[0].clone())
    }
    async fn execute_read_at_host(
        &mut self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
    ) -> Result<StatementResultset, TreatyError> {
        let request = ExecuteReadRequest {
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
        };

        trace!("[{}]: REQUEST={request:?}", function_name!());

        let response = self
            .grpc_client
            .execute_read_at_host(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.results[0].clone())
    }
    async fn enable_cooperative_features(&mut self, db_name: &str) -> Result<bool, TreatyError> {
        let request = EnableCoooperativeFeaturesRequest {
            database_name: db_name.to_string(),
        };

        let response = self
            .grpc_client
            .enable_coooperative_features(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_successful)
    }
    async fn create_user_database(&mut self, db_name: &str) -> Result<bool, TreatyError> {
        let request = CreateUserDatabaseRequest {
            database_name: db_name.to_string(),
        };

        let response = self
            .grpc_client
            .create_user_database(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_created)
    }

    fn debug(&self) -> String {
        let message = format!("{self:?}");
        message
    }
}
