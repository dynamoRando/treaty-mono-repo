use std::time::Duration;

use async_trait::async_trait;
use stdext::function_name;
use tonic::transport::Channel;
use tracing::{debug, trace};
use treaty_types::enums::*;
use treaty::{
   
    treaty_proto::{
        user_service_client::UserServiceClient, AcceptPendingActionReply,
        AcceptPendingActionRequest, AcceptPendingContractRequest, AddParticipantRequest,
        AuthRequest, ChangeDeletesFromHostBehaviorRequest, ChangeDeletesToHostBehaviorRequest,
        ChangeHostStatusRequest, ChangeUpdatesFromHostBehaviorRequest,
        ChangeUpdatesToHostBehaviorRequest, Contract, CreateUserDatabaseRequest,
        EnableCoooperativeFeaturesRequest, ExecuteCooperativeWriteRequest, ExecuteReadRequest,
        ExecuteWriteRequest, GenerateContractRequest, GenerateHostInfoRequest,
        GetActiveContractReply, GetActiveContractRequest, GetCooperativeHostsReply,
        GetCooperativeHostsRequest, GetDataHashRequest, GetDatabasesReply, GetDatabasesRequest,
        GetDeletesFromHostBehaviorReply, GetDeletesFromHostBehaviorRequest,
        GetDeletesToHostBehaviorReply, GetDeletesToHostBehaviorRequest,
        GetLogicalStoragePolicyRequest, GetParticipantsReply, GetParticipantsRequest,
        GetPendingActionsReply, GetPendingActionsRequest, GetReadRowIdsRequest,
        GetUpdatesFromHostBehaviorReply, GetUpdatesFromHostBehaviorRequest,
        GetUpdatesToHostBehaviorReply, GetUpdatesToHostBehaviorRequest, HasTableRequest,
        HostInfoReply, RevokeReply, SendParticipantContractRequest, SetLogicalStoragePolicyRequest,
        StatementResultset, TestRequest, TokenReply, TreatyError, TryAuthAtParticipantRequest,
        ViewPendingContractsRequest, GetSettingsReply, GetSettingsRequest,
    },
};

use crate::{client_actions::ClientActions, Auth};

#[derive(Debug, Clone)]
pub struct GrpcClient {
    /// The HTTP (or HTTPS) address and port of the `treaty` instance you are talking to. Example: `http://127.0.0.1:50051`
    addr_port: String,
    timeout_in_seconds: u32,
    grpc_client: Option<UserServiceClient<Channel>>,
    auth: Auth,
    send_jwt_if_available: bool,
    // when talking to a treaty-proxy instance, send the host_id to identify which account you want
    host_id: Option<String>,
}

impl GrpcClient {
    pub async fn new(
        addr_port: &str,
        timeout_in_seconds: u32,
        auth: Auth,
        send_jwt_if_available: bool,
        host_id: Option<String>,
    ) -> Self {
        let endpoint = tonic::transport::Channel::builder(addr_port.parse().unwrap())
            .timeout(Duration::from_secs(timeout_in_seconds.into()));
        let channel = endpoint.connect().await.unwrap();
        let client = UserServiceClient::new(channel);

        Self {
            addr_port: addr_port.to_string(),
            timeout_in_seconds,
            grpc_client: Some(client),
            auth,
            send_jwt_if_available,
            host_id,
        }
    }

    pub fn timeout_in_seconds(&self) -> u32 {
        self.timeout_in_seconds
    }

    pub fn addr_port(&self) -> String {
        self.addr_port.clone()
    }

    fn gen_auth_request(&self) -> AuthRequest {
        let auth: AuthRequest;

        if self.send_jwt_if_available && !self.auth.jwt.is_empty() {
            auth = AuthRequest {
                user_name: String::from(""),
                pw: String::from(""),
                pw_hash: Vec::new(),
                token: Vec::new(),
                jwt: self.auth.jwt.clone(),
                id: self.host_id.clone(),
            };

            // trace!("[{}]: {auth:?}", function_name!());

            return auth;
        }

        auth = AuthRequest {
            user_name: self.auth.user_name.clone(),
            pw: self.auth.pw.clone(),
            pw_hash: Vec::new(),
            token: Vec::new(),
            jwt: String::from(""),
            id: self.host_id.clone(),
        };

        // trace!("[{}]: {:?}", function_name!(), auth);

        auth
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

        let result = self
            .grpc_client
            .as_mut()
            .unwrap()
            .is_online(request)
            .await
            .unwrap();

        Ok(result.into_inner().reply_echo_message == test_string)
    }

    async fn get_host_info(&mut self) -> Result<HostInfoReply, TreatyError> {
        let auth = self.gen_auth_request();

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
            .get_host_info(auth)
            .await
            .unwrap()
            .into_inner();

        return Ok(response);
    }

    async fn get_active_contract(
        &mut self,
        db_name: &str,
    ) -> Result<GetActiveContractReply, TreatyError> {
        let auth = self.gen_auth_request();

        let request = GetActiveContractRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
            .get_active_contract(request)
            .await
            .unwrap()
            .into_inner();

        Ok(response)
    }

    async fn revoke_token(&mut self) -> Result<RevokeReply, TreatyError> {
        let auth = self.gen_auth_request();

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
            .revoke_token(auth)
            .await
            .unwrap()
            .into_inner();

        Ok(response)
    }

    async fn auth_for_token(&mut self) -> Result<TokenReply, TreatyError> {
        let auth = self.gen_auth_request();

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
            .auth_for_token(auth)
            .await
            .unwrap()
            .into_inner();

        if response.is_successful {
            let x = response.clone();
            self.auth.jwt = x.jwt;
        } else {
            self.auth.jwt = "".to_string();
        }

        Ok(response)
    }

    async fn accept_pending_action_at_participant(
        &mut self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<AcceptPendingActionReply, TreatyError> {
        let auth = self.gen_auth_request();

        let request = AcceptPendingActionRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            row_id,
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
            .accept_pending_action_at_participant(request)
            .await
            .unwrap()
            .into_inner();

        Ok(response)
    }

    async fn get_cooperative_hosts(&mut self) -> Result<GetCooperativeHostsReply, TreatyError> {
        let auth = self.gen_auth_request();

        let request = GetCooperativeHostsRequest {
            authentication: Some(auth),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
            .get_cooperative_hosts(tonic::Request::new(request))
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
        let auth = self.gen_auth_request();

        let request = GetParticipantsRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();

        let request = GetPendingActionsRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            action: action.to_string(),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();

        let request = GetReadRowIdsRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            where_clause: where_clause.to_string(),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();

        let request = GetDataHashRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            row_id,
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();

        let request = GetDataHashRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            row_id,
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();
        let request = GetDeletesToHostBehaviorRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();

        let request = ChangeDeletesToHostBehaviorRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            behavior: num::ToPrimitive::to_u32(&behavior).unwrap(),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();
        let request = GetUpdatesToHostBehaviorRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();

        let request = ChangeUpdatesToHostBehaviorRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            behavior: num::ToPrimitive::to_u32(&behavior).unwrap(),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();

        let request = GetDeletesFromHostBehaviorRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();

        let request = ChangeDeletesFromHostBehaviorRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            behavior: num::ToPrimitive::to_u32(&behavior).unwrap(),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();

        let request = GetUpdatesFromHostBehaviorRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();
        let request = ChangeUpdatesFromHostBehaviorRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            behavior: UpdatesFromHostBehavior::to_u32(behavior),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();
        let request = ChangeHostStatusRequest {
            authentication: Some(auth),
            host_alias: String::from(""),
            host_id: host_id.to_string(),
            status,
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
        let auth = self.gen_auth_request();
        let request = ChangeHostStatusRequest {
            authentication: Some(auth),
            host_alias: host_name.to_string(),
            host_id: String::from(""),
            status,
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
            .change_host_status(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_successful)
    }

    async fn generate_host_info(&mut self, host_name: &str) -> Result<bool, TreatyError> {
        let auth = self.gen_auth_request();
        let request = GenerateHostInfoRequest {
            authentication: Some(auth),
            host_name: host_name.to_string(),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
            .generate_host_info(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_successful)
    }

    async fn get_databases(&mut self) -> Result<GetDatabasesReply, TreatyError> {
        let auth = self.gen_auth_request();

        let request = GetDatabasesRequest {
            authentication: Some(auth),
        };

        let client = self.grpc_client.as_mut().unwrap();
        let response = client.get_databases(request).await.unwrap().into_inner();

        trace!("[{}]: {:?}", function_name!(), response);

        Ok(response)
    }
    async fn execute_cooperative_write_at_host(
        &mut self,
        db_name: &str,
        cmd: &str,
        participant_alias: &str,
        where_clause: &str,
    ) -> Result<bool, TreatyError> {
        let auth = self.gen_auth_request();

        let request = ExecuteCooperativeWriteRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            sql_statement: cmd.to_string(),
            database_type: DatabaseType::to_u32(DatabaseType::Sqlite),
            alias: participant_alias.to_string(),
            participant_id: String::from(""),
            where_clause: where_clause.to_string(),
        };

        trace!("request: {request:?}");

        let client = self.grpc_client.as_mut().unwrap();
        let response = client
            .execute_cooperative_write_at_host(request)
            .await
            .unwrap()
            .into_inner();

        trace!("[{}]: {:?}", function_name!(), response);

        Ok(response.is_successful)
    }
    async fn view_pending_contracts(&mut self) -> Result<Vec<Contract>, TreatyError> {
        let auth = self.gen_auth_request();

        let request = ViewPendingContractsRequest {
            authentication: Some(auth),
        };

        let client = self.grpc_client.as_mut().unwrap();

        let response = client
            .review_pending_contracts(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.contracts)
    }

    async fn accept_pending_contract(&mut self, host_alias: &str) -> Result<bool, TreatyError> {
        let auth = self.gen_auth_request();

        let request = AcceptPendingContractRequest {
            authentication: Some(auth),
            host_alias: host_alias.to_string(),
        };

        let client = self.grpc_client.as_mut().unwrap();

        let response = client
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
        let auth = self.gen_auth_request();

        let request = SendParticipantContractRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            participant_alias: participant_alias.to_string(),
        };

        let client = self.grpc_client.as_mut().unwrap();

        let response = client
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
        participant_db_port: u32,
        participant_http_addr: &str,
        participant_http_port: u16,
        participant_id: Option<String>,
    ) -> Result<bool, TreatyError> {
        let auth = self.gen_auth_request();

        let request = AddParticipantRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            alias: participant_alias.to_string(),
            ip4_address: participant_ip4addr.to_string(),
            port: participant_db_port,
            http_addr: participant_http_addr.to_string(),
            http_port: participant_http_port as u32,
            id: participant_id,
        };

        let client = self.grpc_client.as_mut().unwrap();

        let response = client.add_participant(request).await.unwrap().into_inner();
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
        let auth = self.gen_auth_request();

        let request = GenerateContractRequest {
            authentication: Some(auth),
            host_name: host_name.to_string(),
            description: desc.to_string(),
            database_name: db_name.to_string(),
            remote_delete_behavior: RemoteDeleteBehavior::to_u32(remote_delete_behavior),
        };

        let client = self.grpc_client.as_mut().unwrap();

        let response = client
            .generate_contract(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_successful)
    }

    async fn has_table(&mut self, db_name: &str, table_name: &str) -> Result<bool, TreatyError> {
        let auth = self.gen_auth_request();

        let request = HasTableRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        };

        let client = self.grpc_client.as_mut().unwrap();

        let response = client.has_table(request).await.unwrap().into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.has_table)
    }

    async fn get_logical_storage_policy(
        &mut self,
        db_name: &str,
        table_name: &str,
    ) -> Result<LogicalStoragePolicy, TreatyError> {
        let auth = self.gen_auth_request();

        let request = GetLogicalStoragePolicyRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        };

        let client = self.grpc_client.as_mut().unwrap();

        let response = client
            .get_logical_storage_policy(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        let policy = LogicalStoragePolicy::from_i64(response.policy_mode as i64);

        Ok(policy)
    }

    async fn get_settings(
        &mut self,
    ) -> Result<GetSettingsReply, TreatyError> {
        let auth = self.gen_auth_request();

        let request = GetSettingsRequest {
            authentication: Some(auth),
        };

        let client = self.grpc_client.as_mut().unwrap();

        let response = client
            .get_settings(request)
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
        let auth = self.gen_auth_request();

        let request = SetLogicalStoragePolicyRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            policy_mode: LogicalStoragePolicy::to_u32(policy),
        };

        let client = self.grpc_client.as_mut().unwrap();

        let response = client
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
        let auth = self.gen_auth_request();

        let request = ExecuteWriteRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
            where_clause: where_clause.to_string(),
        };

        let client = self.grpc_client.as_mut().unwrap();

        let response = client
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
        let auth = self.gen_auth_request();

        let request = ExecuteWriteRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
            where_clause: where_clause.to_string(),
        };

        let client = self.grpc_client.as_mut().unwrap();

        let response = client
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
        let auth = self.gen_auth_request();

        let request = TryAuthAtParticipantRequest {
            authentication: Some(auth),
            participant_id: id.to_string(),
            participant_alias: alias.to_string(),
            db_name: db_name.to_string(),
        };

        let client = self.grpc_client.as_mut().unwrap();

        let response = client
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
        let auth = self.gen_auth_request();

        let request = ExecuteReadRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
        };

        trace!("REQUEST={request:?}");

        let client = self.grpc_client.as_mut().unwrap();

        let response = client
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
        let auth = self.gen_auth_request();

        let request = ExecuteReadRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
        };

        trace!("[{}]: REQUEST={request:?}", function_name!());

        let client = self.grpc_client.as_mut().unwrap();

        let response = client
            .execute_read_at_host(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.results[0].clone())
    }
    async fn enable_cooperative_features(&mut self, db_name: &str) -> Result<bool, TreatyError> {
        let auth = self.gen_auth_request();

        let request = EnableCoooperativeFeaturesRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
        };

        let client = self.grpc_client.as_mut().unwrap();

        let response = client
            .enable_coooperative_features(request)
            .await
            .unwrap()
            .into_inner();
        trace!("[{}]: RESPONSE={response:?}", function_name!());

        Ok(response.is_successful)
    }
    async fn create_user_database(&mut self, db_name: &str) -> Result<bool, TreatyError> {
        let auth = self.gen_auth_request();

        let request = CreateUserDatabaseRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
        };

        let response = self
            .grpc_client
            .as_mut()
            .unwrap()
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
