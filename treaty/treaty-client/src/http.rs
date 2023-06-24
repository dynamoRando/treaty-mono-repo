use crate::{client_actions::ClientActions, Auth};
use async_trait::async_trait;
use reqwest::Client;
use serde::de;
use stdext::function_name;
use tracing::trace;

use treaty_http_endpoints::client::{
    ACCEPT_PENDING_ACTION, ACCEPT_PENDING_CONTRACT, ADD_PARTICIPANT, AUTH_FOR_TOKEN,
    CHANGE_DELETES_FROM_HOST_BEHAVIOR, CHANGE_DELETES_TO_HOST_BEHAVIOR, CHANGE_HOST_STATUS_ID,
    CHANGE_HOST_STATUS_NAME, CHANGE_UPDATES_FROM_HOST_BEHAVIOR, CHANGE_UPDATES_TO_HOST_BEHAVIOR,
    COOPERATIVE_WRITE_SQL_AT_HOST, ENABLE_COOPERATIVE_FEATURES, GENERATE_CONTRACT,
    GENERATE_HOST_INFO, GET_ACTIVE_CONTRACT, GET_COOP_HOSTS, GET_DATABASES, GET_DATA_HASH_AT_HOST,
    GET_DATA_HASH_AT_PARTICIPANT, GET_DELETES_FROM_HOST_BEHAVIOR, GET_DELETES_TO_HOST_BEHAVIOR,
    GET_HOST_INFO, GET_PARTICIPANTS, GET_PENDING_ACTIONS, GET_POLICY, GET_ROW_AT_PARTICIPANT,
    GET_SETTINGS, GET_UPDATES_FROM_HOST_BEHAVIOR, GET_UPDATES_TO_HOST_BEHAVIOR, HAS_TABLE,
    IS_ONLINE, NEW_DATABASE, READ_SQL_AT_HOST, READ_SQL_AT_PARTICIPANT, REVOKE_TOKEN,
    SEND_CONTRACT_TO_PARTICIPANT, SET_POLICY, TRY_AUTH_PARTICIPANT, VIEW_PENDING_CONTRACTS,
    WRITE_SQL_AT_HOST, WRITE_SQL_AT_PARTICIPANT
};
use treaty_types::enums::*;
use treaty::{
    treaty_proto::{
        AcceptPendingActionReply, AcceptPendingActionRequest, AcceptPendingContractReply,
        AcceptPendingContractRequest, AddParticipantReply, AddParticipantRequest, AuthRequest,
        ChangeDeletesFromHostBehaviorReply, ChangeDeletesFromHostBehaviorRequest,
        ChangeDeletesToHostBehaviorReply, ChangeDeletesToHostBehaviorRequest,
        ChangeHostStatusReply, ChangeHostStatusRequest, ChangeUpdatesFromHostBehaviorRequest,
        ChangeUpdatesToHostBehaviorReply, ChangeUpdatesToHostBehaviorRequest,
        ChangesUpdatesFromHostBehaviorReply, Contract, CreateUserDatabaseReply,
        CreateUserDatabaseRequest, EnableCoooperativeFeaturesReply,
        EnableCoooperativeFeaturesRequest, ExecuteCooperativeWriteReply,
        ExecuteCooperativeWriteRequest, ExecuteReadReply, ExecuteReadRequest, ExecuteWriteReply,
        ExecuteWriteRequest, GenerateContractReply, GenerateContractRequest, GenerateHostInfoReply,
        GenerateHostInfoRequest, GetActiveContractReply, GetActiveContractRequest,
        GetCooperativeHostsReply, GetCooperativeHostsRequest, GetDataHashReply, GetDataHashRequest,
        GetDatabasesReply, GetDatabasesRequest, GetDeletesFromHostBehaviorReply,
        GetDeletesFromHostBehaviorRequest, GetDeletesToHostBehaviorReply,
        GetDeletesToHostBehaviorRequest, GetLogicalStoragePolicyReply,
        GetLogicalStoragePolicyRequest, GetParticipantsReply, GetParticipantsRequest,
        GetPendingActionsReply, GetPendingActionsRequest, GetReadRowIdsReply, GetReadRowIdsRequest,
        GetSettingsReply, GetSettingsRequest, GetUpdatesFromHostBehaviorReply,
        GetUpdatesFromHostBehaviorRequest, GetUpdatesToHostBehaviorReply,
        GetUpdatesToHostBehaviorRequest, HasTableReply, HasTableRequest, HostInfoReply,
        RevokeReply, SendParticipantContractReply, SendParticipantContractRequest,
        SetLogicalStoragePolicyReply, SetLogicalStoragePolicyRequest, StatementResultset,
        TestReply, TestRequest, TokenReply, TreatyError, TryAuthAtParticipantRequest,
        TryAuthAtPartipantReply, ViewPendingContractsReply, ViewPendingContractsRequest,
    },
};

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub addr: String,
    pub port: u32,
    pub http_client: Option<Client>,
    pub auth: Auth,
    timeout_in_seconds: u32,
    send_jwt_if_available: bool,
    // when talking to a treaty-proxy instance, send the host_id to identify which account you want
    host_id: Option<String>,
}

impl HttpClient {
    pub async fn new(
        addr: &str,
        port: u32,
        auth: Auth,
        timeout_in_seconds: u32,
        send_jwt_if_available: bool,
        host_id: Option<String>,
    ) -> Self {
        let http = reqwest::Client::new();

        Self {
            addr: addr.to_string(),
            port,
            http_client: Some(http),
            auth,
            timeout_in_seconds,
            send_jwt_if_available,
            host_id,
        }
    }

    pub fn timeout_in_seconds(&self) -> u32 {
        self.timeout_in_seconds
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

            trace!("[{}]: {auth:?}", function_name!());

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

        trace!("[{}]: {:?}", function_name!(), auth);

        auth
    }

    fn get_http_url(&self, action_url: &str) -> String {
        let http_base = format!("{}{}:{}", "http://", self.addr, self.port);

        let result = format!("{http_base}{action_url}");
        trace!("[{}]: {}", function_name!(), result);
        result
    }

    async fn send_http_message(&self, json_message: String, url: String) -> String {
        let client = self.http_client.as_ref().unwrap();

        trace!("[{}]: {json_message}", function_name!());
        trace!("[{}]: {url}", function_name!());

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

    pub async fn get_http_result<
        'a,
        'b,
        T: de::DeserializeOwned + std::clone::Clone,
        U: de::DeserializeOwned + serde::Serialize + std::clone::Clone,
    >(
        &mut self,
        url: String,
        request: U,
    ) -> T {
        let request_json = serde_json::to_string(&request).unwrap();
        let result_json: String = self.send_http_message(request_json, url).await;
        trace!("[{}]: {result_json:?}", function_name!());
        let value: T = serde_json::from_str(&result_json).unwrap();
        value
    }
}

#[async_trait]
impl ClientActions for HttpClient {
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

        let url = self.get_http_url(IS_ONLINE);
        let result: TestReply = self.get_http_result(url, request).await;
        Ok(result.reply_echo_message == test_string)
    }

    async fn get_host_info(&mut self) -> Result<HostInfoReply, TreatyError> {
        let request = self.gen_auth_request();

        let url = self.get_http_url(GET_HOST_INFO);
        let result = self.get_http_result(url, request).await;
        Ok(result)
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

        let url = self.get_http_url(GET_ACTIVE_CONTRACT);
        let result = self.get_http_result(url, request).await;

        Ok(result)
    }

    async fn revoke_token(&mut self) -> Result<RevokeReply, TreatyError> {
        let auth = self.gen_auth_request();

        let url = self.get_http_url(REVOKE_TOKEN);
        let result = self.get_http_result(url, auth).await;

        Ok(result)
    }

    async fn auth_for_token(&mut self) -> Result<TokenReply, TreatyError> {
        let auth = self.gen_auth_request();

        let url = self.get_http_url(AUTH_FOR_TOKEN);
        let result: TokenReply = self.get_http_result(url, auth).await;

        if result.is_successful {
            let x = result.clone();
            self.auth.jwt = x.jwt;
        } else {
            self.auth.jwt = "".to_string();
        }

        Ok(result)
    }

    async fn get_settings(&mut self) -> Result<GetSettingsReply, TreatyError> {
        let auth = self.gen_auth_request();

        let request = GetSettingsRequest {
            authentication: Some(auth),
        };

        let url = self.get_http_url(GET_SETTINGS);
        let result = self.get_http_result(url, request).await;

        Ok(result)
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

        let url = self.get_http_url(ACCEPT_PENDING_ACTION);
        let result = self.get_http_result(url, request).await;

        Ok(result)
    }

    async fn get_cooperative_hosts(&mut self) -> Result<GetCooperativeHostsReply, TreatyError> {
        let auth = self.gen_auth_request();

        let request = GetCooperativeHostsRequest {
            authentication: Some(auth),
        };
        let url = self.get_http_url(GET_COOP_HOSTS);
        let result = self.get_http_result(url, request).await;

        Ok(result)
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

        let url = self.get_http_url(GET_PARTICIPANTS);
        let result = self.get_http_result(url, request).await;

        Ok(result)
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

        let url = self.get_http_url(GET_PENDING_ACTIONS);
        let result = self.get_http_result(url, request).await;

        Ok(result)
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

        let url = self.get_http_url(GET_ROW_AT_PARTICIPANT);
        let result: GetReadRowIdsReply = self.get_http_result(url, request).await;

        Ok(result.row_ids)
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

        let url = self.get_http_url(GET_DATA_HASH_AT_PARTICIPANT);
        let result: GetDataHashReply = self.get_http_result(url, request).await;

        Ok(result.data_hash)
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

        let url = self.get_http_url(GET_DATA_HASH_AT_HOST);
        let result: GetDataHashReply = self.get_http_result(url, request).await;

        Ok(result.data_hash)
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

        let url = self.get_http_url(GET_DELETES_TO_HOST_BEHAVIOR);
        let result: GetDeletesToHostBehaviorReply = self.get_http_result(url, request).await;

        Ok(result)
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

        let url = self.get_http_url(CHANGE_DELETES_TO_HOST_BEHAVIOR);
        let result: ChangeDeletesToHostBehaviorReply = self.get_http_result(url, request).await;
        Ok(result.is_successful)
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

        let url = self.get_http_url(GET_UPDATES_TO_HOST_BEHAVIOR);
        let result: GetUpdatesToHostBehaviorReply = self.get_http_result(url, request).await;

        Ok(result)
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

        let url = self.get_http_url(CHANGE_UPDATES_TO_HOST_BEHAVIOR);
        let result: ChangeUpdatesToHostBehaviorReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
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

        let url = self.get_http_url(GET_DELETES_FROM_HOST_BEHAVIOR);
        let result: GetDeletesFromHostBehaviorReply = self.get_http_result(url, request).await;

        Ok(result)
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

        let url = self.get_http_url(CHANGE_DELETES_FROM_HOST_BEHAVIOR);
        let result: ChangeDeletesFromHostBehaviorReply = self.get_http_result(url, request).await;
        Ok(result.is_successful)
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

        let url = self.get_http_url(GET_UPDATES_FROM_HOST_BEHAVIOR);
        let result = self.get_http_result(url, request).await;
        Ok(result)
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

        let url = self.get_http_url(CHANGE_UPDATES_FROM_HOST_BEHAVIOR);
        let result: ChangesUpdatesFromHostBehaviorReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
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

        let url = self.get_http_url(CHANGE_HOST_STATUS_ID);
        let result: ChangesUpdatesFromHostBehaviorReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
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

        let url = self.get_http_url(CHANGE_HOST_STATUS_NAME);
        let result: ChangeHostStatusReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
    }
    async fn generate_host_info(&mut self, host_name: &str) -> Result<bool, TreatyError> {
        let auth = self.gen_auth_request();
        let request = GenerateHostInfoRequest {
            authentication: Some(auth),
            host_name: host_name.to_string(),
        };
        let url = self.get_http_url(GENERATE_HOST_INFO);
        let result: GenerateHostInfoReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
    }

    async fn get_databases(&mut self) -> Result<GetDatabasesReply, TreatyError> {
        let auth = self.gen_auth_request();

        let request = GetDatabasesRequest {
            authentication: Some(auth),
        };

        let url = self.get_http_url(GET_DATABASES);
        let result = self.get_http_result(url, request).await;
        Ok(result)
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
        let url = self.get_http_url(COOPERATIVE_WRITE_SQL_AT_HOST);
        let result: ExecuteCooperativeWriteReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
    }
    async fn view_pending_contracts(&mut self) -> Result<Vec<Contract>, TreatyError> {
        let auth = self.gen_auth_request();

        let request = ViewPendingContractsRequest {
            authentication: Some(auth),
        };

        let url = self.get_http_url(VIEW_PENDING_CONTRACTS);
        let result: ViewPendingContractsReply = self.get_http_result(url, request).await;

        Ok(result.contracts)
    }

    async fn accept_pending_contract(&mut self, host_alias: &str) -> Result<bool, TreatyError> {
        let auth = self.gen_auth_request();

        let request = AcceptPendingContractRequest {
            authentication: Some(auth),
            host_alias: host_alias.to_string(),
        };

        let url = self.get_http_url(ACCEPT_PENDING_CONTRACT);
        let result: AcceptPendingContractReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
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

        let url = self.get_http_url(SEND_CONTRACT_TO_PARTICIPANT);
        let result: SendParticipantContractReply = self.get_http_result(url, request).await;

        Ok(result.is_sent)
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

        let url = self.get_http_url(ADD_PARTICIPANT);
        let result: AddParticipantReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
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

        let url = self.get_http_url(GENERATE_CONTRACT);
        let result: GenerateContractReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
    }
    async fn has_table(&mut self, db_name: &str, table_name: &str) -> Result<bool, TreatyError> {
        let auth = self.gen_auth_request();

        let request = HasTableRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        };

        let url = self.get_http_url(HAS_TABLE);
        let result: HasTableReply = self.get_http_result(url, request).await;

        Ok(result.has_table)
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

        let url = self.get_http_url(GET_POLICY);
        let result: GetLogicalStoragePolicyReply = self.get_http_result(url, request).await;

        Ok(LogicalStoragePolicy::from_i64(result.policy_mode as i64))
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
        let url = self.get_http_url(SET_POLICY);
        let result: SetLogicalStoragePolicyReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
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

        let url = self.get_http_url(WRITE_SQL_AT_HOST);
        let result: ExecuteWriteReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
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

        let url = self.get_http_url(WRITE_SQL_AT_PARTICIPANT);
        let result: ExecuteWriteReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
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

        let url = self.get_http_url(TRY_AUTH_PARTICIPANT);
        let result: TryAuthAtPartipantReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
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

        let url = self.get_http_url(READ_SQL_AT_PARTICIPANT);
        let result: ExecuteReadReply = self.get_http_result(url, request).await;

        Ok(result.results[0].clone())
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
        let url = self.get_http_url(READ_SQL_AT_HOST);
        let result: ExecuteReadReply = self.get_http_result(url, request).await;

        Ok(result.results[0].clone())
    }
    async fn enable_cooperative_features(&mut self, db_name: &str) -> Result<bool, TreatyError> {
        let auth = self.gen_auth_request();

        let request = EnableCoooperativeFeaturesRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
        };

        let url = self.get_http_url(ENABLE_COOPERATIVE_FEATURES);
        let result: EnableCoooperativeFeaturesReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
    }
    async fn create_user_database(&mut self, db_name: &str) -> Result<bool, TreatyError> {
        let auth = self.gen_auth_request();

        let request = CreateUserDatabaseRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
        };

        let url = self.get_http_url(NEW_DATABASE);
        let result: CreateUserDatabaseReply = self.get_http_result(url, request).await;

        Ok(result.is_created)
    }

    fn debug(&self) -> String {
        let message = format!("{self:?}");
        message
    }
}
