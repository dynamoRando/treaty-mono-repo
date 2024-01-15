use crate::client_actions::ClientActions;
use async_trait::async_trait;
use serde::de;
use stdext::function_name;
use tracing::{trace, warn};
use treaty::{
    settings::HttpTlsClientOptions,
    treaty_proto::{
        AcceptPendingActionReply, AcceptPendingActionRequest, AcceptPendingContractReply,
        AcceptPendingContractRequest, AddParticipantReply, AddParticipantRequest,
        AuthRequestAuthor, AuthRequestBasic, AuthRequestMetadata, AuthRequestWebToken,
        ChangeDeletesFromHostBehaviorReply, ChangeDeletesFromHostBehaviorRequest,
        ChangeDeletesToHostBehaviorReply, ChangeDeletesToHostBehaviorRequest,
        ChangeHostStatusReply, ChangeHostStatusRequest, ChangeUpdatesFromHostBehaviorRequest,
        ChangeUpdatesToHostBehaviorReply, ChangeUpdatesToHostBehaviorRequest,
        ChangesUpdatesFromHostBehaviorReply, Contract, CreateUserDatabaseReply,
        CreateUserDatabaseRequest, DeleteUserDatabaseReply, DeleteUserDatabaseRequest,
        EnableCoooperativeFeaturesReply, EnableCoooperativeFeaturesRequest,
        ExecuteCooperativeWriteReply, ExecuteCooperativeWriteRequest, ExecuteReadReply,
        ExecuteReadRequest, ExecuteWriteReply, ExecuteWriteRequest, GenerateContractReply,
        GenerateContractRequest, GenerateHostInfoReply, GenerateHostInfoRequest,
        GetActiveContractReply, GetActiveContractRequest, GetBackingDatabaseConfigReply,
        GetCooperativeHostsReply, GetDataHashReply, GetDataHashRequest, GetDatabasesReply,
        GetDeletesFromHostBehaviorReply, GetDeletesFromHostBehaviorRequest,
        GetDeletesToHostBehaviorReply, GetDeletesToHostBehaviorRequest,
        GetLogicalStoragePolicyReply, GetLogicalStoragePolicyRequest, GetParticipantsReply,
        GetParticipantsRequest, GetPendingActionsReply, GetPendingActionsRequest,
        GetReadRowIdsReply, GetReadRowIdsRequest, GetSettingsReply,
        GetUpdatesFromHostBehaviorReply, GetUpdatesFromHostBehaviorRequest,
        GetUpdatesToHostBehaviorReply, GetUpdatesToHostBehaviorRequest, HasTableReply,
        HasTableRequest, HostInfoReply, RevokeReply, SendParticipantContractReply,
        SendParticipantContractRequest, SetLogicalStoragePolicyReply,
        SetLogicalStoragePolicyRequest, StatementResultset, TestReply, TestRequest, TokenReply,
        TreatyError, TryAuthAtParticipantRequest, TryAuthAtPartipantReply, TryAuthResult,
        ViewPendingContractsReply,
    },
};
use treaty_http_endpoints::{
    client::{
        ACCEPT_PENDING_ACTION, ACCEPT_PENDING_CONTRACT, ADD_PARTICIPANT, AUTH_FOR_TOKEN,
        CHANGE_DELETES_FROM_HOST_BEHAVIOR, CHANGE_DELETES_TO_HOST_BEHAVIOR, CHANGE_HOST_STATUS_ID,
        CHANGE_HOST_STATUS_NAME, CHANGE_UPDATES_FROM_HOST_BEHAVIOR,
        CHANGE_UPDATES_TO_HOST_BEHAVIOR, COOPERATIVE_WRITE_SQL_AT_HOST, DB_TYPE,
        DELETE_DATABASE_FORCE, ENABLE_COOPERATIVE_FEATURES, GENERATE_CONTRACT, GENERATE_HOST_INFO,
        GET_ACTIVE_CONTRACT, GET_COOP_HOSTS, GET_DATABASES, GET_DATA_HASH_AT_HOST,
        GET_DATA_HASH_AT_PARTICIPANT, GET_DELETES_FROM_HOST_BEHAVIOR, GET_DELETES_TO_HOST_BEHAVIOR,
        GET_HOST_INFO, GET_PARTICIPANTS, GET_PENDING_ACTIONS, GET_POLICY, GET_ROW_AT_PARTICIPANT,
        GET_SETTINGS, GET_UPDATES_FROM_HOST_BEHAVIOR, GET_UPDATES_TO_HOST_BEHAVIOR, HAS_TABLE,
        IS_ONLINE, NEW_DATABASE, READ_SQL_AT_HOST, READ_SQL_AT_PARTICIPANT, REVOKE_TOKEN,
        SEND_CONTRACT_TO_PARTICIPANT, SET_POLICY, TRY_AUTH_PARTICIPANT, VIEW_PENDING_CONTRACTS,
        WRITE_SQL_AT_HOST, WRITE_SQL_AT_PARTICIPANT,
    },
    headers::{
        TREATY_AUTH_HEADER_AUTHOR, TREATY_AUTH_HEADER_BASIC, TREATY_AUTH_HEADER_METADATA,
        TREATY_AUTH_HEADER_WEB_TOKEN,
    },
    info::{INFO_AUTH_FOR_TOKEN, INFO_TRY_AUTH},
};
use treaty_types::enums::*;

#[derive(Debug, Clone)]
pub struct HttpClient {
    user_service_address: String,
    user_service_port: u32,
    info_service_addres: String,
    info_service_port: u32,
    basic_auth: AuthRequestBasic,
    token_auth: Option<AuthRequestWebToken>,
    timeout_in_seconds: u32,
    // when talking to a treaty-proxy instance, send the host_id to identify which account you want
    host_id: Option<String>,
    /// use TLS
    use_tls: bool,
    /// Optional: Tls Client settings, used for local tests
    opt_tls_settings: Option<HttpTlsClientOptions>,
}

impl HttpClient {
    pub async fn new(
        user_service_address: &str,
        user_service_port: u32,
        info_service_address: &str,
        info_service_port: u32,
        username: &str,
        pw: &str,
        timeout_in_seconds: u32,
        host_id: Option<String>,
        use_tls: bool,
        opt_tls_settings: Option<HttpTlsClientOptions>,
    ) -> Self {
        let basic_auth = AuthRequestBasic {
            user_name: username.to_string(),
            pw: pw.to_string(),
        };

        Self {
            user_service_address: user_service_address.to_string(),
            user_service_port,
            info_service_addres: info_service_address.to_string(),
            info_service_port,
            basic_auth,
            token_auth: None,
            timeout_in_seconds,
            host_id,
            use_tls,
            opt_tls_settings,
        }
    }

    pub fn timeout_in_seconds(&self) -> u32 {
        self.timeout_in_seconds
    }

    fn get_http_url(&self, action_url: &str) -> String {
        let http_base = if self.use_tls {
            format!(
                "{}{}:{}",
                "https://", self.user_service_address, self.user_service_port
            )
        } else {
            format!(
                "{}{}:{}",
                "http://", self.user_service_address, self.user_service_port
            )
        };

        let result = format!("{http_base}{action_url}");
        trace!("[{}]: {}", function_name!(), result);
        result
    }

    async fn validate_web_token(&mut self) -> bool {
        if let Some(web_token) = self.token_auth.clone() {
            trace!("[{}]: {web_token:?}", function_name!());
            let json_message = serde_json::to_string(&web_token).unwrap();
            let author = AuthRequestAuthor { author_type: 1 };

            let metadata = AuthRequestMetadata {
                id: self.host_id.clone(),
                db_name: None,
            };

            let http_base = if self.use_tls {
                format!(
                    "{}{}:{}",
                    "https://", self.info_service_addres, self.info_service_port
                )
            } else {
                format!(
                    "{}{}:{}",
                    "http://", self.info_service_addres, self.info_service_port
                )
            };

            let action_url = INFO_TRY_AUTH;
            let url = format!("{http_base}{action_url}");

            let client = match &self.opt_tls_settings {
                Some(opts) => {
                    if opts.danger_accept_invalid_certs {
                        warn!("[{}]: WARNING - ACCEPT INVALID CERTS!", function_name!());
                    }

                    if !opts.tls_sni {
                        warn!(
                            "[{}]: WARNING - IGNORING TLS Server Name Indicator",
                            function_name!()
                        );
                    }

                    reqwest::Client::builder()
                        .danger_accept_invalid_certs(opts.danger_accept_invalid_certs)
                        .tls_sni(opts.tls_sni)
                        .build()
                        .unwrap()
                }
                None => reqwest::Client::new(),
            };

            let json_response = client
                .post(url)
                .header("Content-Type", "application/json")
                .body(json_message)
                .header(
                    TREATY_AUTH_HEADER_WEB_TOKEN,
                    serde_json::to_string(&self.token_auth.as_ref().unwrap().clone()).unwrap(),
                )
                .header(
                    TREATY_AUTH_HEADER_AUTHOR,
                    serde_json::to_string(&author).unwrap(),
                )
                .header(
                    TREATY_AUTH_HEADER_METADATA,
                    serde_json::to_string(&metadata).unwrap(),
                )
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            let response: TryAuthResult = serde_json::from_str(&json_response).unwrap();
            trace!("[{}]: {response:?}", function_name!());
            return response.is_authenticated;
        }

        trace!("[{}]: Web token was not validated", function_name!());
        false
    }

    async fn update_token(&mut self) {
        if !self.validate_web_token().await {
            let metadata = AuthRequestMetadata {
                id: self.host_id.clone(),
                db_name: None,
            };
            let author = AuthRequestAuthor { author_type: 1 };

            let client = match &self.opt_tls_settings {
                Some(opts) => {
                    if opts.danger_accept_invalid_certs {
                        warn!("[{}]: WARNING - ACCEPT INVALID CERTS!", function_name!());
                    }

                    if !opts.tls_sni {
                        warn!(
                            "[{}]: WARNING - IGNORING TLS Server Name Indicator",
                            function_name!()
                        );
                    }

                    reqwest::Client::builder()
                        .danger_accept_invalid_certs(opts.danger_accept_invalid_certs)
                        .tls_sni(opts.tls_sni)
                        .build()
                        .unwrap()
                }
                None => reqwest::Client::new(),
            };

            let json_message = serde_json::to_string(&self.basic_auth).unwrap();
            let http_base = if self.use_tls {
                format!(
                    "{}{}:{}",
                    "https://", self.info_service_addres, self.info_service_port
                )
            } else {
                format!(
                    "{}{}:{}",
                    "http://", self.info_service_addres, self.info_service_port
                )
            };

            let action_url = INFO_AUTH_FOR_TOKEN;
            let url = format!("{http_base}{action_url}");
            let json_response = client
                .post(url)
                .header("Content-Type", "application/json")
                .body(json_message)
                .header(
                    TREATY_AUTH_HEADER_BASIC,
                    serde_json::to_string(&self.basic_auth.clone()).unwrap(),
                )
                .header(
                    TREATY_AUTH_HEADER_AUTHOR,
                    serde_json::to_string(&author).unwrap(),
                )
                .header(
                    TREATY_AUTH_HEADER_METADATA,
                    serde_json::to_string(&metadata).unwrap(),
                )
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            trace!("[{}]: {json_response:?}", function_name!());
            let response: TokenReply = serde_json::from_str(&json_response).unwrap();
            if response.is_successful {
                let token = AuthRequestWebToken {
                    jwt: response.jwt.clone(),
                };

                self.token_auth = Some(token);
            }
        }
    }

    async fn send_http_message(&mut self, json_message: String, url: String) -> String {
        self.update_token().await;
        let is_web_token_valid = self.validate_web_token().await;

        let client = match &self.opt_tls_settings {
            Some(opts) => {
                if opts.danger_accept_invalid_certs {
                    warn!("[{}]: WARNING - ACCEPT INVALID CERTS!", function_name!());
                }

                if !opts.tls_sni {
                    warn!(
                        "[{}]: WARNING - IGNORING TLS Server Name Indicator",
                        function_name!()
                    );
                }

                reqwest::Client::builder()
                    .danger_accept_invalid_certs(opts.danger_accept_invalid_certs)
                    .tls_sni(opts.tls_sni)
                    .build()
                    .unwrap()
            }
            None => reqwest::Client::new(),
        };

        trace!("[{}]: {json_message}", function_name!());
        trace!("[{}]: {url}", function_name!());

        let author = AuthRequestAuthor { author_type: 1 };

        if is_web_token_valid {
            return client
                .post(url)
                .header("Content-Type", "application/json")
                .body(json_message)
                .header(
                    TREATY_AUTH_HEADER_WEB_TOKEN,
                    serde_json::to_string(&self.token_auth.as_ref().unwrap().clone()).unwrap(),
                )
                .header(
                    TREATY_AUTH_HEADER_AUTHOR,
                    serde_json::to_string(&author).unwrap(),
                )
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
        } else {
            return client
                .post(url)
                .header("Content-Type", "application/json")
                .body(json_message)
                .header(
                    TREATY_AUTH_HEADER_BASIC,
                    serde_json::to_string(&self.basic_auth).unwrap(),
                )
                .header(
                    TREATY_AUTH_HEADER_AUTHOR,
                    serde_json::to_string(&author).unwrap(),
                )
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
        }
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
        trace!("[{}]: {request_json:?}", function_name!());
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

    async fn get_backing_db_config(
        &mut self,
    ) -> Result<GetBackingDatabaseConfigReply, TreatyError> {
        let url = self.get_http_url(DB_TYPE);
        let result: GetBackingDatabaseConfigReply =
            self.get_http_result(url, String::from("")).await;
        Ok(result)
    }

    async fn get_host_info(&mut self) -> Result<HostInfoReply, TreatyError> {
        let url = self.get_http_url(GET_HOST_INFO);
        let result = self.get_http_result(url, String::from("")).await;
        Ok(result)
    }

    async fn get_active_contract(
        &mut self,
        db_name: &str,
    ) -> Result<GetActiveContractReply, TreatyError> {
        let request = GetActiveContractRequest {
            database_name: db_name.to_string(),
        };

        let url = self.get_http_url(GET_ACTIVE_CONTRACT);
        let result = self.get_http_result(url, request).await;

        Ok(result)
    }

    async fn revoke_token(&mut self) -> Result<RevokeReply, TreatyError> {
        if let Some(token) = self.token_auth.clone() {
            let url = self.get_http_url(REVOKE_TOKEN);
            let result = self.get_http_result(url, token).await;

            return Ok(result);
        }

        Err(TreatyError {
            message: "Token does not exist".to_string(),
            help: None,
            number: 0,
        })
    }

    async fn auth_for_token(&mut self) -> Result<TokenReply, TreatyError> {
        let url = self.get_http_url(AUTH_FOR_TOKEN);
        let token = self.basic_auth.clone();
        let result: TokenReply = self.get_http_result(url, token).await;

        if result.is_successful {
            let reply = result.clone();
            self.token_auth = Some(AuthRequestWebToken { jwt: reply.jwt });
        } else {
            self.token_auth = None;
        }

        Ok(result)
    }

    async fn get_settings(&mut self) -> Result<GetSettingsReply, TreatyError> {
        let url = self.get_http_url(GET_SETTINGS);
        let result = self.get_http_result(url, String::from("")).await;

        Ok(result)
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

        let url = self.get_http_url(ACCEPT_PENDING_ACTION);
        let result = self.get_http_result(url, request).await;

        Ok(result)
    }

    async fn get_cooperative_hosts(&mut self) -> Result<GetCooperativeHostsReply, TreatyError> {
        let url = self.get_http_url(GET_COOP_HOSTS);
        let result = self.get_http_result(url, String::from("")).await;

        Ok(result)
    }

    async fn get_participants_for_database(
        &mut self,
        db_name: &str,
    ) -> Result<GetParticipantsReply, TreatyError> {
        let request = GetParticipantsRequest {
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
        let request = GetPendingActionsRequest {
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
        let request = GetReadRowIdsRequest {
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
        let request = GetDataHashRequest {
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
        let request = GetDataHashRequest {
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
        let request = GetDeletesToHostBehaviorRequest {
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
        let request = ChangeDeletesToHostBehaviorRequest {
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
        let request = GetUpdatesToHostBehaviorRequest {
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
        let request = ChangeUpdatesToHostBehaviorRequest {
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
        let request = GetDeletesFromHostBehaviorRequest {
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
        let request = ChangeDeletesFromHostBehaviorRequest {
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
        let request = GetUpdatesFromHostBehaviorRequest {
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
        let request = ChangeUpdatesFromHostBehaviorRequest {
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
        let request = ChangeHostStatusRequest {
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
        let request = ChangeHostStatusRequest {
            host_alias: host_name.to_string(),
            host_id: String::from(""),
            status,
        };

        let url = self.get_http_url(CHANGE_HOST_STATUS_NAME);
        let result: ChangeHostStatusReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
    }
    async fn generate_host_info(&mut self, host_name: &str) -> Result<bool, TreatyError> {
        let request = GenerateHostInfoRequest {
            host_name: host_name.to_string(),
        };
        let url = self.get_http_url(GENERATE_HOST_INFO);
        let result: GenerateHostInfoReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
    }

    async fn get_databases(&mut self) -> Result<GetDatabasesReply, TreatyError> {
        let url = self.get_http_url(GET_DATABASES);
        let result = self.get_http_result(url, String::from("")).await;
        Ok(result)
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
        let url = self.get_http_url(COOPERATIVE_WRITE_SQL_AT_HOST);
        let result: ExecuteCooperativeWriteReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
    }
    async fn view_pending_contracts(&mut self) -> Result<Vec<Contract>, TreatyError> {
        let url = self.get_http_url(VIEW_PENDING_CONTRACTS);
        let result: ViewPendingContractsReply = self.get_http_result(url, String::from("")).await;

        Ok(result.contracts)
    }

    async fn accept_pending_contract(&mut self, host_alias: &str) -> Result<bool, TreatyError> {
        let request = AcceptPendingContractRequest {
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
        let request = SendParticipantContractRequest {
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
        let request = GenerateContractRequest {
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
        let request = HasTableRequest {
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
        let request = GetLogicalStoragePolicyRequest {
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
        let request = SetLogicalStoragePolicyRequest {
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
        let request = ExecuteWriteRequest {
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
        let request = ExecuteWriteRequest {
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
        let request = TryAuthAtParticipantRequest {
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
        let request = ExecuteReadRequest {
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
        let request = ExecuteReadRequest {
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
        let request = EnableCoooperativeFeaturesRequest {
            database_name: db_name.to_string(),
        };

        let url = self.get_http_url(ENABLE_COOPERATIVE_FEATURES);
        let result: EnableCoooperativeFeaturesReply = self.get_http_result(url, request).await;

        Ok(result.is_successful)
    }

    async fn create_user_database(&mut self, db_name: &str) -> Result<bool, TreatyError> {
        let request = CreateUserDatabaseRequest {
            database_name: db_name.to_string(),
        };

        let url = self.get_http_url(NEW_DATABASE);
        let result: CreateUserDatabaseReply = self.get_http_result(url, request).await;

        Ok(result.is_created)
    }

    async fn drop_database_forcefully(
        &mut self,
        db_name: &str,
    ) -> Result<DeleteUserDatabaseReply, TreatyError> {
        let request = DeleteUserDatabaseRequest {
            database_name: db_name.to_string(),
        };

        let url = self.get_http_url(DELETE_DATABASE_FORCE);
        let result: DeleteUserDatabaseReply = self.get_http_result(url, request).await;

        Ok(result)
    }

    fn debug(&self) -> String {
        let message = format!("{self:?}");
        message
    }
}
