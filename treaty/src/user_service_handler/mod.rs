// natively handle the gRPC implementation of the service
// this module just passses call from the gRPC implementation back to this module
pub(crate) mod grpc;

// module for handing CRUD operations (Create, Read, Update, Delete)
pub(crate) mod io;

use std::net::SocketAddr;

use crate::{
    db_interface::dbi_actions::DbiActions,
    defaults,
    error::TreatyDbError,
    models::DataInfo,
    query_parser::{determine_dml_type, get_table_name},
    remote::remote_actions::RemoteActions,
    settings::Settings,
    treaty_proto::{
        user_service_server::UserServiceServer, AcceptPendingActionReply,
        AcceptPendingActionRequest, AcceptPendingContractReply, AcceptPendingContractRequest,
        AddParticipantReply, AddParticipantRequest, AuthRequestBasic, AuthRequestWebToken,
        AuthResult, ChangeDeletesFromHostBehaviorReply, ChangeDeletesFromHostBehaviorRequest,
        ChangeDeletesToHostBehaviorReply, ChangeDeletesToHostBehaviorRequest,
        ChangeHostStatusReply, ChangeHostStatusRequest, ChangeUpdatesFromHostBehaviorRequest,
        ChangeUpdatesToHostBehaviorReply, ChangeUpdatesToHostBehaviorRequest,
        ChangesUpdatesFromHostBehaviorReply, Contract, CreateUserDatabaseReply,
        CreateUserDatabaseRequest, DatabaseSchema, DeleteUserDatabaseReply,
        DeleteUserDatabaseRequest, EnableCoooperativeFeaturesReply,
        EnableCoooperativeFeaturesRequest, ExecuteCooperativeWriteReply,
        ExecuteCooperativeWriteRequest, ExecuteReadReply, ExecuteReadRequest, ExecuteWriteReply,
        ExecuteWriteRequest, GenerateContractReply, GenerateContractRequest, GenerateHostInfoReply,
        GenerateHostInfoRequest, GetActiveContractReply, GetActiveContractRequest,
        GetBackingDatabaseConfigReply, GetCooperativeHostsReply, GetDataHashReply,
        GetDataHashRequest, GetDatabasesReply, GetDeletesFromHostBehaviorReply,
        GetDeletesFromHostBehaviorRequest, GetDeletesToHostBehaviorReply,
        GetDeletesToHostBehaviorRequest, GetLogicalStoragePolicyReply,
        GetLogicalStoragePolicyRequest, GetParticipantsReply, GetParticipantsRequest,
        GetPendingActionsReply, GetPendingActionsRequest, GetReadRowIdsReply, GetReadRowIdsRequest,
        GetSettingsReply, GetUpdatesFromHostBehaviorReply, GetUpdatesFromHostBehaviorRequest,
        GetUpdatesToHostBehaviorReply, GetUpdatesToHostBehaviorRequest, HasTableReply,
        HasTableRequest, Host, HostInfoReply, HostInfoStatus, HostNetwork, ParticipantStatus,
        PendingStatement, RevokeReply, SendParticipantContractReply,
        SendParticipantContractRequest, SetLogicalStoragePolicyReply,
        SetLogicalStoragePolicyRequest, StatementResultset, TestReply, TestRequest, TokenReply,
        TreatyError, TryAuthAtParticipantRequest, TryAuthAtPartipantReply,
        ViewPendingContractsReply,
    },
    user_service_handler::io::db::handle_execute_read_at_host,
};
use async_trait::async_trait;
use chrono::Utc;
use stdext::function_name;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic_async_interceptor::AsyncInterceptedService;
use tracing::{debug, error, trace, warn};
use treaty_types::{auth::AuthRequestData, enums::*};
use triggered::Listener;

use self::{
    authentication::Authenticator,
    io::part_db::{handle_delete_write_at_participant, handle_update_write_at_participant},
    user_service_handler_actions::UserServiceHandlerActions,
};

const AUTO_UPDATE_PARTICIPANT_STATUS: bool = true;

pub mod authentication;
pub mod user_service_handler_actions;

#[derive(Debug, Clone)]
pub struct UserServiceHandler<
    D: DbiActions + Clone + Send + Sync + 'static,
    R: RemoteActions + Clone + Send + Sync + 'static,
> {
    db: D,
    remote: R,
    settings: Settings,
    authenticator: Authenticator<D>,
}

#[async_trait]
impl<D: DbiActions + Clone + Send + Sync, R: RemoteActions + Clone + Send + Sync>
    UserServiceHandlerActions for UserServiceHandler<D, R>
{
    async fn authenticate_request(&self, request: &AuthRequestData) -> bool {
        self.authenticator.authenticate(request).await
    }

    async fn change_host_status(&self, request: ChangeHostStatusRequest) -> ChangeHostStatusReply {
        self.change_host_status(request).await
    }
    async fn is_online(&self, request: TestRequest) -> TestReply {
        self.is_online(request).await
    }

    async fn get_backing_database_config(&self) -> GetBackingDatabaseConfigReply {
        self.get_backing_database_config().await
    }

    async fn get_cooperative_hosts(&self) -> GetCooperativeHostsReply {
        self.get_cooperative_hosts().await
    }
    async fn get_settings(&self) -> GetSettingsReply {
        self.get_settings().await
    }
    async fn get_deletes_from_host_behavior(
        &self,
        request: GetDeletesFromHostBehaviorRequest,
    ) -> GetDeletesFromHostBehaviorReply {
        self.get_deletes_from_host_behavior(request).await
    }
    async fn get_deletes_to_host_behavior(
        &self,
        request: GetDeletesToHostBehaviorRequest,
    ) -> GetDeletesToHostBehaviorReply {
        self.get_deletes_to_host_behavior(request).await
    }
    async fn get_updates_from_host_behavior(
        &self,
        request: GetUpdatesFromHostBehaviorRequest,
    ) -> GetUpdatesFromHostBehaviorReply {
        self.get_updates_from_host_behavior(request).await
    }
    async fn execute_write_at_host(&self, request: ExecuteWriteRequest) -> ExecuteWriteReply {
        self.execute_write_at_host(request).await
    }
    async fn execute_read_at_host(&self, request: ExecuteReadRequest) -> ExecuteReadReply {
        self.execute_read_at_host(request).await
    }
    async fn execute_read_at_participant(&self, request: ExecuteReadRequest) -> ExecuteReadReply {
        self.execute_read_at_participant(request).await
    }
    async fn enable_coooperative_features(
        &self,
        request: EnableCoooperativeFeaturesRequest,
    ) -> EnableCoooperativeFeaturesReply {
        self.enable_coooperative_features(request).await
    }
    async fn create_user_database(
        &self,
        request: CreateUserDatabaseRequest,
    ) -> CreateUserDatabaseReply {
        self.create_user_database(request).await
    }
    async fn delete_user_database(
        &self,
        request: DeleteUserDatabaseRequest,
    ) -> DeleteUserDatabaseReply {
        self.delete_user_database(request).await
    }

    async fn delete_user_database_forcefully(
        &self,
        request: DeleteUserDatabaseRequest,
    ) -> DeleteUserDatabaseReply {
        self.delete_user_database_destructively(request).await
    }
    async fn generate_host_info(&self, request: GenerateHostInfoRequest) -> GenerateHostInfoReply {
        self.generate_host_info(request).await
    }
    async fn get_active_contract(
        &self,
        request: GetActiveContractRequest,
    ) -> GetActiveContractReply {
        self.get_active_contract(request).await
    }
    async fn accept_pending_contract(
        &self,
        request: AcceptPendingContractRequest,
    ) -> AcceptPendingContractReply {
        self.accept_pending_contract(request).await
    }
    async fn get_data_hash_at_host(&self, request: GetDataHashRequest) -> GetDataHashReply {
        self.get_data_hash_at_host(request).await
    }
    async fn get_data_hash_at_participant(&self, request: GetDataHashRequest) -> GetDataHashReply {
        self.get_data_hash_at_participant(request).await
    }
    async fn read_row_id_at_participant(
        &self,
        request: GetReadRowIdsRequest,
    ) -> GetReadRowIdsReply {
        self.read_row_id_at_participant(request).await
    }
    async fn change_deletes_to_host_behavior(
        &self,
        request: ChangeDeletesToHostBehaviorRequest,
    ) -> ChangeDeletesToHostBehaviorReply {
        self.change_deletes_to_host_behavior(request).await
    }
    async fn change_updates_to_host_behavior(
        &self,
        request: ChangeUpdatesToHostBehaviorRequest,
    ) -> ChangeUpdatesToHostBehaviorReply {
        self.change_updates_to_host_behavior(request).await
    }
    async fn change_deletes_from_host_behavior(
        &self,
        request: ChangeDeletesFromHostBehaviorRequest,
    ) -> ChangeDeletesFromHostBehaviorReply {
        self.change_deletes_from_host_behavior(request).await
    }
    async fn change_updates_from_host_behavior(
        &self,
        request: ChangeUpdatesFromHostBehaviorRequest,
    ) -> ChangesUpdatesFromHostBehaviorReply {
        self.change_updates_from_host_behavior(request).await
    }
    async fn review_pending_contracts(&self) -> ViewPendingContractsReply {
        self.review_pending_contracts().await
    }
    async fn send_participant_contract(
        &self,
        request: SendParticipantContractRequest,
    ) -> SendParticipantContractReply {
        self.send_participant_contract(request).await
    }
    async fn add_participant(&self, request: AddParticipantRequest) -> AddParticipantReply {
        self.add_participant(request).await
    }
    async fn get_databases(&self) -> GetDatabasesReply {
        self.get_databases().await
    }
    async fn get_pending_actions_at_participant(
        &self,
        request: GetPendingActionsRequest,
    ) -> GetPendingActionsReply {
        self.get_pending_updates_at_participant(request).await
    }
    async fn accept_pending_action_at_participant(
        &self,
        request: AcceptPendingActionRequest,
    ) -> AcceptPendingActionReply {
        self.accept_pending_action_at_participant(request).await
    }
    async fn get_participants(&self, request: GetParticipantsRequest) -> GetParticipantsReply {
        self.get_participants(request).await
    }
    async fn execute_write_at_participant(
        &self,
        request: ExecuteWriteRequest,
    ) -> ExecuteWriteReply {
        self.execute_write_at_participant(request).await
    }
    async fn get_updates_to_host_behavior(
        &self,
        request: GetUpdatesToHostBehaviorRequest,
    ) -> GetUpdatesToHostBehaviorReply {
        self.get_updates_to_host_behavior(request).await
    }
    async fn revoke_token(&self, request: AuthRequestWebToken) -> RevokeReply {
        self.revoke_token(request).await
    }
    async fn auth_for_token(&self, request: AuthRequestBasic) -> TokenReply {
        self.auth_for_token(request, Some(self.settings.jwt_valid_time_in_minutes))
            .await
    }
    async fn set_logical_storage_policy(
        &self,
        request: SetLogicalStoragePolicyRequest,
    ) -> SetLogicalStoragePolicyReply {
        self.set_logical_storage_policy(request).await
    }
    async fn get_logical_storage_policy(
        &self,
        request: GetLogicalStoragePolicyRequest,
    ) -> GetLogicalStoragePolicyReply {
        self.get_logical_storage_policy(request).await
    }
    async fn try_auth_at_participant(
        &self,
        request: TryAuthAtParticipantRequest,
    ) -> TryAuthAtPartipantReply {
        self.try_auth_at_participant(request).await
    }
    async fn get_host_info(&self) -> HostInfoReply {
        self.get_host_info().await
    }
    async fn generate_contract(&self, request: GenerateContractRequest) -> GenerateContractReply {
        self.generate_contract(request).await
    }
    async fn has_table(&self, request: HasTableRequest) -> HasTableReply {
        self.has_table(request).await
    }
    async fn execute_cooperative_write_at_host(
        &self,
        request: ExecuteCooperativeWriteRequest,
    ) -> ExecuteCooperativeWriteReply {
        self.execute_cooperative_write_at_host(request).await
    }

    fn debug_settings(&self) -> Settings {
        todo!()
    }

    fn debug_root_dir(&self) -> Option<String> {
        todo!()
    }
}

impl<D: DbiActions + Clone + Send + Sync, R: RemoteActions + Clone + Send + Sync>
    UserServiceHandler<D, R>
{
    pub fn new(db: D, remote: R, settings: Settings) -> Self {
        trace!("[{}]: Making new authenticator", function_name!());

        let authenticator = Authenticator::new(db.clone());
        Self {
            db: db,
            remote,
            settings,
            authenticator,
        }
    }

    #[tokio::main]
    pub async fn start_grpc(&self, shutdown: Listener) -> Result<(), Box<dyn std::error::Error>> {
        trace!("{}", self.settings.grpc_user_service_addr_port);
        let addr: SocketAddr = self.settings.grpc_user_service_addr_port.parse().unwrap();
        debug!("user service gprc listening on {addr}");

        let authenticator = Authenticator::new(self.db.clone());

        let is = AsyncInterceptedService::new(UserServiceServer::new(self.clone()), authenticator);

        if self.settings.use_tls {
            let settings = self.settings.tls_options.as_ref().unwrap();
            trace!(
                "[{}]: cert: {}",
                function_name!(),
                settings.tls_cert_path.as_ref().unwrap()
            );

            trace!(
                "[{}]: key: {}",
                function_name!(),
                settings.tls_key_path.as_ref().unwrap()
            );

            let cert = std::fs::read_to_string(
                settings
                    .tls_cert_path
                    .as_ref()
                    .expect("Cert path was not provided in Settings.toml"),
            )?;
            let key = std::fs::read_to_string(
                settings
                    .tls_key_path
                    .as_ref()
                    .expect("Key path was not provided in Settings.toml"),
            )?;
            let identity = Identity::from_pem(cert, key);

            let router = Server::builder()
                .tls_config(ServerTlsConfig::new().identity(identity))?
                .add_service(is);
            router.serve_with_shutdown(addr, shutdown).await?;
        } else {
            let router = Server::builder().add_service(is);
            router.serve_with_shutdown(addr, shutdown).await?;
        }

        Ok(())
    }

    pub async fn is_online(&self, request: TestRequest) -> TestReply {
        let item = request.request_echo_message;

        trace!("is_online, requested echo: {item}");

        TestReply {
            reply_time_utc: Utc::now().to_rfc2822(),
            reply_echo_message: item,
            treaty_version: defaults::VERSION.to_string(),
        }
    }

    pub async fn get_cooperative_hosts(&self) -> GetCooperativeHostsReply {
        let mut hosts: Vec<HostInfoStatus> = Vec::new();
        let mut error: Option<TreatyError> = None;

        let result = self.db.get_cooperative_hosts().await;
        match result {
            Ok(result) => match result {
                Some(_hosts) => {
                    for host in &_hosts {
                        let n = HostNetwork {
                            ip4_address: Some(host.ip4.clone()),
                            ip6_address: Some(host.ip6.clone()),
                            database_port_number: Some(host.db_port),
                            http_addr: Some(host.http_addr.clone()),
                            http_port: Some(host.db_port),
                            info_port_number: Some(host.info_port),
                        };

                        let h = Host {
                            host_guid: host.host_id.clone(),
                            host_name: host.host_name.clone(),
                            token: Vec::new(),
                            network: Some(n),
                        };

                        let i = HostInfoStatus {
                            host: Some(h),
                            last_communcation_utc: host.last_comm_utc.clone(),
                            status: num::ToPrimitive::to_u32(&host.status).unwrap_or(0),
                        };

                        hosts.push(i);
                    }
                }
                None => todo!(),
            },
            Err(e) => error = Some(e.into()),
        }

        GetCooperativeHostsReply { hosts, error }
    }

    pub async fn get_settings(&self) -> GetSettingsReply {
        let mut settings_json: Option<String> = None;
        let mut error: Option<TreatyError> = None;

        let result = serde_json::to_string(&self.settings);
        match result {
            Ok(_settings) => settings_json = Some(_settings),
            Err(e) => error = Some(e.into()),
        }

        GetSettingsReply {
            settings_json,
            error,
        }
    }

    pub async fn get_backing_database_config(&self) -> GetBackingDatabaseConfigReply {
        let db_type = DatabaseType::to_u32(self.settings.database_type);

        let mut use_schema = false;

        if self.settings.database_type != DatabaseType::Sqlite {
            use_schema = self
                .settings
                .postgres_settings
                .as_ref()
                .unwrap()
                .use_treaty_schema;
        }

        GetBackingDatabaseConfigReply {
            database_type: db_type,
            use_schema,
            error: None,
        }
    }

    pub async fn get_deletes_from_host_behavior(
        &self,
        request: GetDeletesFromHostBehaviorRequest,
    ) -> GetDeletesFromHostBehaviorReply {
        let mut error: Option<TreatyError> = None;
        let mut behavior: Option<u32> = None;

        let db = request.database_name;
        let table = request.table_name;
        let result = self.db.get_deletes_from_host_behavior(&db, &table).await;
        match result {
            Ok(_behavior) => {
                behavior = Some(num::ToPrimitive::to_u32(&_behavior).unwrap());
            }
            Err(e) => {
                error = Some(e.into());
            }
        }

        GetDeletesFromHostBehaviorReply { behavior, error }
    }

    pub async fn get_deletes_to_host_behavior(
        &self,
        request: GetDeletesToHostBehaviorRequest,
    ) -> GetDeletesToHostBehaviorReply {
        let mut error: Option<TreatyError> = None;
        let mut behavior: Option<u32> = None;

        let db = request.database_name;
        let table = request.table_name;
        let result = self.db.get_deletes_to_host_behavior(&db, &table).await;
        match result {
            Ok(_behavior) => {
                behavior = Some(num::ToPrimitive::to_u32(&_behavior).unwrap());
            }
            Err(e) => {
                error = Some(e.into());
            }
        }

        GetDeletesToHostBehaviorReply { behavior, error }
    }

    pub async fn get_updates_from_host_behavior(
        &self,
        request: GetUpdatesFromHostBehaviorRequest,
    ) -> GetUpdatesFromHostBehaviorReply {
        let mut error: Option<TreatyError> = None;
        let mut behavior: Option<u32> = None;

        let db = request.database_name;
        let table = request.table_name;
        let result = self.db.get_updates_from_host_behavior(&db, &table).await;
        match result {
            Ok(_behavior) => {
                behavior = Some(UpdatesFromHostBehavior::to_u32(_behavior));
            }
            Err(e) => {
                error = Some(e.into());
            }
        }

        GetUpdatesFromHostBehaviorReply { behavior, error }
    }

    pub async fn execute_write_at_host(&self, request: ExecuteWriteRequest) -> ExecuteWriteReply {
        let mut rows_affected: u32 = 0;
        let mut error: Option<TreatyError> = None;

        let db_name = request.database_name;
        let statement = request.sql_statement;
        let mut is_sql_successful: bool = false;

        let result = self.db.execute_write_at_host(&db_name, &statement).await;
        match result {
            Ok(num) => {
                // If we execute a SQL statement against a table that has a logical storage policy enabled, we need to
                // check to make sure that we haven't made an error (i.e) forgot to send data to a participant
                // or alternatively if the Logical Storage Policy is Mirror or Shared, we need to replay the action
                // at the participant database
                let result = self.db.has_cooperative_tables(&db_name, &statement).await;
                match result {
                    Ok(has_cooperative_tables) => {
                        if has_cooperative_tables {
                            // we need to check to see if any of the cooperative tables are Shared or Mirrored
                            trace!("[{}]: execute_write_at_host has cooperative tables, checking to see if we should send participant a copy of the execute statement.", function_name!());
                            let result = self.db.get_cooperative_tables(&db_name, &statement).await;
                            match result {
                                Ok(opt_cooperative_tables) => {
                                    match opt_cooperative_tables {
                                        Some(tables) => {
                                            trace!("[{}]: cooperative tables found, getting LSP for each table", function_name!());
                                            for table_name in &tables {
                                                let result = self
                                                    .db
                                                    .get_logical_storage_policy(
                                                        &db_name, table_name,
                                                    )
                                                    .await;
                                                match result {
                                                    Ok(logical_storage_policy) => {
                                                        match logical_storage_policy {
                                                            LogicalStoragePolicy::None => {},
                                                            LogicalStoragePolicy::HostOnly => {},
                                                            LogicalStoragePolicy::ParticipantOwned => {
                                                                error!("[{}]: Statement has cooperative table that is Participant Owned but not executed at participant, statement: {}",
                                                                function_name!(),
                                                                &statement);
                                                            },
                                                            LogicalStoragePolicy::Shared | LogicalStoragePolicy::Mirror => {
                                                                trace!("[{}]: LogicalStoragePolicy is {logical_storage_policy:?}", function_name!());

                                                                let dml_type = determine_dml_type(&statement, self.db.db_type().await);
                                                                trace!("[{}]: DmlType is {dml_type:?}", function_name!());

                                                                let opt_host_info = self.db.treaty_get_host_info().await.expect("no host info is set");

                                                                let cmd_table_name = get_table_name(&statement, self.db.db_type().await);

                                                                if let Some(host_info) = opt_host_info {
                                                                    let opt_participants = self.db.get_participants_for_database(&db_name).await.unwrap();
                                                                    trace!("[{}]: Getting participants for database: {opt_participants:?}", function_name!());

                                                                    if let Some(participants) = opt_participants {
                                                                        for participant in &participants {
                                                                            trace!("[{}]: Replaying command at participant: {participant:?}", function_name!());

                                                                            if let Some(p) = &participant.participant {
                                                                                match dml_type {
                                                                                    DmlType::Unknown => todo!(),
                                                                                    DmlType::Insert => {
                                                                                        let insert_result = self.remote.insert_row_at_participant(
                                                                                            p.clone().try_into().unwrap(),
                                                                                            &host_info,
                                                                                            &db_name,
                                                                                            &cmd_table_name,
                                                                                            &statement,
                                                                                        )
                                                                                        .await;
        
                                                                                        trace!("[{}]: Insert Result for participant: {insert_result:?}", function_name!());
        
                                                                                        if insert_result.is_error {
                                                                                            error = insert_result.error
                                                                                        } else {
                                                                                            is_sql_successful = true;
                                                                                            rows_affected = num as u32;
                                                                                        }
                                                                                    },
                                                                                    DmlType::Update => {
                                                                                        let update_result = self.remote.update_row_at_participant(
                                                                                            p.clone().try_into().unwrap(),
                                                                                            &host_info,
                                                                                            &db_name,
                                                                                            &cmd_table_name,
                                                                                            &statement,
                                                                                            &request.where_clause,
                                                                                        )
                                                                                        .await;
        
                                                                                        trace!("[{}]: Update Result for participant: {update_result:?}", function_name!());
        
                                                                                        if update_result.is_error {
                                                                                            error = update_result.error
                                                                                        } else {
                                                                                            is_sql_successful = true;
                                                                                            rows_affected = num as u32;
                                                                                        }
                                                                                    },
                                                                                    DmlType::Delete => {
                                                                                        let delete_result = self.remote.remove_row_at_participant(
                                                                                            p.clone().try_into().unwrap(),
                                                                                            &host_info,
                                                                                            &db_name,
                                                                                            &cmd_table_name,
                                                                                            &statement,
                                                                                            &request.where_clause,
                                                                                        )
                                                                                        .await;
        
                                                                                        trace!("[{}]: Delete Result for participant: {delete_result:?}", function_name!());
        
                                                                                        if delete_result.is_error {
                                                                                            error = delete_result.error
                                                                                        } else {
                                                                                            is_sql_successful = true;
                                                                                            rows_affected = num as u32;
                                                                                        }
                                                                                    },
                                                                                    DmlType::Select => {},
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }

                                                                
                                                            },
                                                        }
                                                    },
                                                    Err(e) => error = Some(e.into()),
                                                }
                                            }
                                        }
                                        None => {
                                            warn!("[{}]: We have cooperative tables but none actually found", function_name!());
                                        }
                                    }
                                }
                                Err(e) => error = Some(e.into()),
                            }
                        } else {
                            rows_affected = num as u32;
                            is_sql_successful = true;
                        }
                    }
                    Err(e) => error = Some(e.into()),
                }
            }
            Err(e) => error = Some(e.into()),
        }

        ExecuteWriteReply {
            is_successful: is_sql_successful,
            total_rows_affected: rows_affected,
            is_error: error.is_some(),
            error,
        }
    }

    pub async fn execute_read_at_host(&self, request: ExecuteReadRequest) -> ExecuteReadReply {
        let mut error: Option<TreatyError> = None;

        let mut total_resultsets: u64 = 0;
        let mut statement_result_set = StatementResultset::default();

        let read_result = handle_execute_read_at_host(&self.db, &self.remote, &request).await;
        match read_result {
            Ok(_result) => {
                statement_result_set = _result.into();
                total_resultsets = 1;
            }
            Err(e) => {
                error = Some(e.clone().into());
                statement_result_set.error = Some(e.into());
            }
        }

        let statement_results = vec![statement_result_set];

        ExecuteReadReply {
            total_resultsets,
            results: statement_results,
            is_error: error.is_some(),
            error,
        }
    }

    pub async fn execute_read_at_participant(
        &self,
        request: ExecuteReadRequest,
    ) -> ExecuteReadReply {
        let mut statement_result_set = StatementResultset::default();
        let mut error: Option<TreatyError> = None;

        let result = self
            .db
            .execute_read_at_participant(&request.database_name, &request.sql_statement)
            .await;

        match result {
            Ok(query_result) => statement_result_set = query_result.into(),
            Err(e) => error = Some(e.into()),
        }

        let statement_results = vec![statement_result_set];

        ExecuteReadReply {
            total_resultsets: 1,
            results: statement_results,
            is_error: error.is_some(),
            error,
        }
    }

    pub async fn enable_coooperative_features(
        &self,
        request: EnableCoooperativeFeaturesRequest,
    ) -> EnableCoooperativeFeaturesReply {
        let mut is_successful = false;
        let mut error: Option<TreatyError> = None;

        let result = self
            .db
            .enable_coooperative_features(&request.database_name)
            .await;
        match result {
            Ok(is_enabled) => {
                trace!("[{}]: Cooperative Features Enabled", function_name!());
                is_successful = is_enabled;
            }
            Err(e) => error = Some(e.into()),
        }

        EnableCoooperativeFeaturesReply {
            is_successful,
            message: "".to_string(),
            error,
        }
    }

    pub async fn delete_user_database(
        &self,
        request: DeleteUserDatabaseRequest,
    ) -> DeleteUserDatabaseReply {
        let mut is_database_deleted = false;
        let mut error: Option<TreatyError> = None;

        let result = self.db.delete_database(&request.database_name).await;
        match result {
            Ok(_) => is_database_deleted = true,
            Err(e) => error = Some(e.into()),
        }

        DeleteUserDatabaseReply {
            is_deleted: is_database_deleted,
            message: "".to_string(),
            error,
        }
    }

    pub async fn delete_user_database_destructively(
        &self,
        request: DeleteUserDatabaseRequest,
    ) -> DeleteUserDatabaseReply {
        let mut is_database_deleted = false;
        let mut error: Option<TreatyError> = None;

        let result = self
            .db
            .delete_database_forcefully(&request.database_name)
            .await;
        match result {
            Ok(_) => is_database_deleted = true,
            Err(e) => error = Some(e.into()),
        }

        DeleteUserDatabaseReply {
            is_deleted: is_database_deleted,
            message: "".to_string(),
            error,
        }
    }

    pub async fn create_user_database(
        &self,
        request: CreateUserDatabaseRequest,
    ) -> CreateUserDatabaseReply {
        let mut is_database_created = false;
        let mut error: Option<TreatyError> = None;

        let result = self.db.create_database(&request.database_name).await;
        match result {
            Ok(_) => is_database_created = true,
            Err(e) => error = Some(e.into()),
        }

        CreateUserDatabaseReply {
            is_created: is_database_created,
            message: "".to_string(),
            error,
        }
    }

    pub async fn generate_host_info(
        &self,
        request: GenerateHostInfoRequest,
    ) -> GenerateHostInfoReply {
        let mut is_generate_successful = false;
        let host_name = request.host_name.clone();
        let mut error: Option<TreatyError> = None;

        let result = self.db.treaty_generate_host_info(&host_name).await;
        match result {
            Ok(_) => is_generate_successful = true,
            Err(e) => error = Some(e.into()),
        }

        GenerateHostInfoReply {
            is_successful: is_generate_successful,
            error,
        }
    }

    pub async fn get_active_contract(
        &self,
        request: GetActiveContractRequest,
    ) -> GetActiveContractReply {
        let mut contract: Option<Contract> = None;
        let mut error: Option<TreatyError> = None;

        let result = self
            .db
            .get_active_contract_proto(&request.database_name)
            .await;
        match result {
            Ok(opt_contract) => {
                if let Some(_contract) = opt_contract {
                    contract = Some(_contract);
                }
            }
            Err(e) => error = Some(e.into()),
        }

        GetActiveContractReply { contract, error }
    }

    pub async fn accept_pending_contract(
        &self,
        request: AcceptPendingContractRequest,
    ) -> AcceptPendingContractReply {
        let mut is_accepted = false;
        let mut error: Option<TreatyError> = None;
        let mut return_message = String::from("");

        let result = self.db.get_pending_contracts().await;
        match result {
            Ok(opt_contracts) => match opt_contracts {
                Some(_contracts) => {
                    debug!(
                        "[{}]: get_pending_contracts: {_contracts:#?}",
                        function_name!()
                    );
                    debug!(
                        "[{}]: requested host_alias: {}",
                        function_name!(),
                        request.host_alias
                    );

                    let pending_contract = _contracts
                        .iter()
                        .enumerate()
                        .filter(|&(_, c)| {
                            c.host_info.as_ref().unwrap().host_name == request.host_alias
                        })
                        .map(|(_, c)| c);

                    let param_contract = pending_contract.last().unwrap().clone();

                    // 1 - accept the contract
                    let result = self.db.accept_pending_contract(&request.host_alias).await;
                    match result {
                        Ok(is_contract_updated) => {
                            if is_contract_updated {
                                // 2 - create the database with the properties of the contract
                                // make the database
                                let result = self
                                    .db
                                    .create_partial_database_from_contract(&param_contract)
                                    .await;

                                match result {
                                    Ok(db_is_created) => {
                                        if db_is_created {
                                            let self_host_info = self
                                                .db
                                                .treaty_get_host_info()
                                                .await
                                                .expect("no host info is set")
                                                .unwrap();

                                            // 3 - notify the host that we've accepted the contract
                                            let is_host_notified = self
                                                .remote
                                                .notify_host_of_acceptance_of_contract(
                                                    &param_contract,
                                                    &self_host_info,
                                                )
                                                .await;

                                            if is_contract_updated
                                                && db_is_created
                                                && is_host_notified
                                            {
                                                is_accepted = true;
                                                return_message =
                                                    String::from("accepted contract successfuly")
                                            } else if !is_contract_updated {
                                                return_message = String::from(
                                                    "failed to update contract in treaty db",
                                                )
                                            } else if !db_is_created {
                                                return_message = String::from(
                                                    "failed to to create partial db from contract",
                                                )
                                            } else if !is_host_notified {
                                                return_message =
                                                    String::from("failed to notify host of acceptance of contract");
                                            }
                                        } else {
                                            return_message = String::from("The partial database was not created for an unknown reason")
                                        }
                                    }
                                    Err(e) => error = Some(e.into()),
                                }
                            } else {
                                return_message = String::from(
                                    "The contract was not accepted for an unknown reason",
                                )
                            }
                        }
                        Err(e) => error = Some(e.into()),
                    }
                }
                None => return_message = String::from("There are no pending contracts"),
            },
            Err(e) => error = Some(e.into()),
        }

        AcceptPendingContractReply {
            is_successful: is_accepted,
            message: return_message,
            error,
        }
    }

    pub async fn get_data_hash_at_host(&self, request: GetDataHashRequest) -> GetDataHashReply {
        let db_name = request.database_name;
        let table_name = request.table_name;
        let requested_row_id = request.row_id;
        let mut row_hash: u64 = 0;
        let mut error: Option<TreatyError> = None;

        let result = self
            .db
            .get_data_hash_at_host(&db_name, &table_name, requested_row_id)
            .await;
        match result {
            Ok(opt_hash) => match opt_hash {
                Some(hash) => row_hash = hash,
                None => {
                    error = Some(TreatyError {
                        message: String::from("No hash found"),
                        help: None,
                        number: 0,
                    })
                }
            },
            Err(e) => error = Some(e.into()),
        }

        GetDataHashReply {
            data_hash: row_hash,
            error,
        }
    }

    pub async fn get_data_hash_at_participant(
        &self,
        request: GetDataHashRequest,
    ) -> GetDataHashReply {
        let db_name = request.database_name;
        let table_name = request.table_name;
        let requested_row_id = request.row_id;
        let mut row_hash: u64 = 0;
        let mut error: Option<TreatyError> = None;

        let result = self
            .db
            .get_data_hash_at_participant(&db_name, &table_name, requested_row_id)
            .await;
        match result {
            Ok(opt_hash) => match opt_hash {
                Some(_hash) => row_hash = _hash,
                None => {
                    error = Some(TreatyError {
                        message: String::from("No row hash was found"),
                        help: None,
                        number: 0,
                    })
                }
            },
            Err(e) => error = Some(e.into()),
        }

        GetDataHashReply {
            data_hash: row_hash,
            error,
        }
    }

    pub async fn read_row_id_at_participant(
        &self,
        request: GetReadRowIdsRequest,
    ) -> GetReadRowIdsReply {
        let db_name = request.database_name;
        let table_name = request.table_name;
        let where_clause = request.where_clause;
        let mut error: Option<TreatyError> = None;
        let mut row_ids: Vec<u32> = Vec::new();

        let result = self
            .db
            .read_row_id_from_part_db(&db_name, &table_name, &where_clause)
            .await;
        match result {
            Ok(_row_id) => {
                if _row_id == 0 {
                    warn!("[{}]: row id returned was 0", function_name!());
                }

                row_ids.push(_row_id)
            }
            Err(e) => error = Some(e.into()),
        }

        GetReadRowIdsReply { row_ids, error }
    }

    pub async fn change_deletes_to_host_behavior(
        &self,
        request: ChangeDeletesToHostBehaviorRequest,
    ) -> ChangeDeletesToHostBehaviorReply {
        let db_name = request.database_name;
        let table_name = request.table_name;
        let behavior = request.behavior;
        let mut is_successful = false;
        let mut error: Option<TreatyError> = None;

        let result = self
            .db
            .change_deletes_to_host_behavior(&db_name, &table_name, behavior)
            .await;
        match result {
            Ok(_is_successful) => is_successful = _is_successful,
            Err(e) => error = Some(e.into()),
        }

        ChangeDeletesToHostBehaviorReply {
            is_successful,
            message: String::from(""),
            error,
        }
    }

    pub async fn change_updates_to_host_behavior(
        &self,
        request: ChangeUpdatesToHostBehaviorRequest,
    ) -> ChangeUpdatesToHostBehaviorReply {
        let db_name = request.database_name;
        let table_name = request.table_name;
        let behavior = request.behavior;
        let mut is_successful = false;
        let mut error: Option<TreatyError> = None;

        let result = self
            .db
            .change_updates_to_host_behavior(&db_name, &table_name, behavior)
            .await;
        match result {
            Ok(_is_successful) => is_successful = _is_successful,
            Err(e) => error = Some(e.into()),
        }

        ChangeUpdatesToHostBehaviorReply {
            is_successful,
            message: String::from(""),
            error,
        }
    }

    pub async fn change_deletes_from_host_behavior(
        &self,
        request: ChangeDeletesFromHostBehaviorRequest,
    ) -> ChangeDeletesFromHostBehaviorReply {
        let db_name = request.database_name;
        let table_name = request.table_name;
        let behavior = request.behavior;
        let mut is_successful = false;
        let mut error: Option<TreatyError> = None;

        let result = self
            .db
            .change_deletes_from_host_behavior(&db_name, &table_name, behavior)
            .await;
        match result {
            Ok(_is_successful) => {
                is_successful = _is_successful;
            }
            Err(e) => error = Some(e.into()),
        }

        ChangeDeletesFromHostBehaviorReply {
            is_successful,
            message: String::from(""),
            error,
        }
    }

    pub async fn change_updates_from_host_behavior(
        &self,
        request: ChangeUpdatesFromHostBehaviorRequest,
    ) -> ChangesUpdatesFromHostBehaviorReply {
        let db_name = request.database_name;
        let table_name = request.table_name;
        let behavior = request.behavior;
        let mut is_successful = false;
        let mut error: Option<TreatyError> = None;

        let result = self
            .db
            .change_updates_from_host_behavior(&db_name, &table_name, behavior)
            .await;
        match result {
            Ok(_is_successful) => {
                is_successful = _is_successful;
            }
            Err(e) => error = Some(e.into()),
        }

        ChangesUpdatesFromHostBehaviorReply {
            is_successful,
            message: String::from(""),
            error,
        }
    }

    pub async fn change_host_status(
        &self,
        request: ChangeHostStatusRequest,
    ) -> ChangeHostStatusReply {
        let host_name = request.host_alias.clone();
        let host_id = request.host_id.clone();
        let status = request.status;

        let mut name_result = false;
        let mut id_result = false;
        let mut error: Option<TreatyError> = None;

        let result = self.db.change_host_status_by_name(&host_name, status).await;
        match result {
            Ok(_is_successful) => {
                if !_is_successful {
                    let result = self.db.change_host_status_by_id(&host_id, status).await;
                    match result {
                        Ok(is_successful) => {
                            id_result = is_successful;
                        }
                        Err(e) => error = Some(e.into()),
                    }
                } else {
                    name_result = _is_successful;
                }
            }
            Err(e) => error = Some(e.into()),
        }

        ChangeHostStatusReply {
            is_successful: name_result || id_result,
            status,
            error,
        }
    }

    pub async fn review_pending_contracts(&self) -> ViewPendingContractsReply {
        let mut contracts: Vec<Contract> = Vec::new();
        let mut error: Option<TreatyError> = None;

        let result = self.db.get_pending_contracts().await;
        match result {
            Ok(opt_contracts) => {
                if let Some(_contracts) = opt_contracts {
                    contracts = _contracts;
                }
            }
            Err(e) => error = Some(e.into()),
        }

        ViewPendingContractsReply { contracts, error }
    }

    pub async fn send_participant_contract(
        &self,
        request: SendParticipantContractRequest,
    ) -> SendParticipantContractReply {
        let db_name = request.database_name;
        let participant_alias = request.participant_alias;

        let mut is_successful = false;
        let mut contract_status: u32 = 0;
        let mut error: Option<TreatyError> = None;

        let result = self.db.has_participant(&db_name, &participant_alias).await;
        match result {
            Ok(has_participant) => {
                if has_participant {
                    let result = self
                        .db
                        .get_participant_by_alias(&db_name, &participant_alias)
                        .await;
                    match result {
                        Ok(opt_participant) => match opt_participant {
                            Some(participant) => {
                                let result = self.db.get_active_contract(&db_name).await;
                                match result {
                                    Ok(active_contract) => {
                                        let result = self.db.get_database_schema(&db_name).await;
                                        match result {
                                            Ok(db_schema) => {
                                                let host_info = self
                                                    .db
                                                    .treaty_get_host_info()
                                                    .await
                                                    .expect("no host info is set")
                                                    .unwrap();
                                                let result = self
                                                    .remote
                                                    .send_participant_contract(
                                                        participant.clone(),
                                                        host_info,
                                                        active_contract.clone(),
                                                        db_schema,
                                                    )
                                                    .await;

                                                trace!("[{}]: {result:?}", function_name!());

                                                is_successful = result.is_successful;
                                                contract_status = num::ToPrimitive::to_u32(
                                                    &result.contract_status,
                                                )
                                                .unwrap();

                                                let participant_contract_status =
                                                    ContractStatus::from_u32(contract_status);

                                                trace!("returned result was: {result:?}");

                                                if AUTO_UPDATE_PARTICIPANT_STATUS
                                                    && !is_successful
                                                    && (participant_contract_status
                                                        != ContractStatus::Pending)
                                                {
                                                    trace!("saving updated contract status for participant: {result:?}");

                                                    let _participant = participant.clone();

                                                    let mut p =
                                                        result.participant_information.unwrap();
                                                    p.ip4_address = _participant.ip4addr;
                                                    p.ip6_address = _participant.ip6addr;
                                                    p.http_addr = _participant.http_addr;
                                                    p.http_port = _participant.http_port as u32;

                                                    self.db
                                                        .update_participant_accepts_contract(
                                                            &db_name,
                                                            participant.clone(),
                                                            p,
                                                            &active_contract
                                                                .contract_id
                                                                .to_string(),
                                                        )
                                                        .await
                                                        .unwrap();
                                                }
                                            }
                                            Err(e) => error = Some(e.into()),
                                        }
                                    }
                                    Err(e) => error = Some(e.into()),
                                }
                            }
                            None => {
                                error = Some(TreatyError {
                                    message: "Participant was not found".to_string(),
                                    help: None,
                                    number: 0,
                                })
                            }
                        },
                        Err(e) => error = Some(e.into()),
                    }
                }
            }
            Err(e) => error = Some(e.into()),
        }

        SendParticipantContractReply {
            is_sent: is_successful,
            contract_status,
            error,
        }
    }

    pub async fn add_participant(&self, request: AddParticipantRequest) -> AddParticipantReply {
        let db_name = request.database_name;
        let alias = request.alias;
        let ip4addr = request.ip4_address;
        let db_port: Option<u32> = request.db_port;
        let info_port: u32 = request.info_port;
        let http_addr = request.http_addr;
        let http_port = request.http_port;
        let id = request.id;
        let mut error: Option<TreatyError> = None;

        let message = String::from("");
        let mut is_successful = false;

        // if the db port is empty, we will attempt to query the info endpoint to get the actual db endpoint
        if db_port.is_none() || (db_port.is_some() && db_port.unwrap() == 0) {
            trace!(
                "[{}]: attempting to connect to participant endpoint for data port number",
                function_name!()
            );

            let _result = self.remote.get_remote_ports(&ip4addr, info_port).await;

            todo!()
        }

        let result = self
            .db
            .add_participant(
                &db_name,
                &alias,
                &ip4addr,
                db_port,
                info_port,
                http_addr,
                http_port as u16,
                id,
            )
            .await;
        match result {
            Ok(_is_successful) => is_successful = _is_successful,
            Err(e) => error = Some(e.into()),
        }

        AddParticipantReply {
            is_successful,
            message,
            error,
        }
    }

    pub async fn get_databases(&self) -> GetDatabasesReply {
        let mut db_result: Vec<DatabaseSchema> = Vec::new();

        let mut error: Option<TreatyError> = None;

        let result = self.db.get_database_names().await;
        match result {
            Ok(opt_names) => {
                if let Some(db_names) = opt_names {
                    for name in &db_names {
                        let result = self.db.get_database_schema(name).await;
                        trace!("{result:?}");
                        match result {
                            Ok(schema) => db_result.push(schema),
                            Err(e) => error = Some(e.into()),
                        }
                    }
                }
            }
            Err(e) => error = Some(e.into()),
        }

        GetDatabasesReply {
            databases: db_result,
            error,
        }
    }

    pub async fn get_pending_updates_at_participant(
        &self,
        request: GetPendingActionsRequest,
    ) -> GetPendingActionsReply {
        let db_name = &request.database_name;
        let table_name = &request.table_name;
        let action = &request.action;

        let mut error: Option<TreatyError> = None;
        let mut pending_statements: Vec<PendingStatement> = Vec::new();

        let result = self
            .db
            .get_pending_actions(db_name, table_name, action)
            .await;
        match result {
            Ok(opt_statements) => {
                if let Some(_statements) = opt_statements {
                    pending_statements = _statements;
                }
            }
            Err(e) => error = Some(e.into()),
        }

        GetPendingActionsReply {
            pending_statements,
            error,
        }
    }

    pub async fn accept_pending_action_at_participant(
        &self,
        request: AcceptPendingActionRequest,
    ) -> AcceptPendingActionReply {
        let mut is_local_update_successful = false;
        let mut is_remote_update_successful = false;

        let mut error: Option<TreatyError> = None;

        let db_name = &request.database_name;
        let table_name = &request.table_name;
        let row_id = request.row_id;

        let result = self
            .db
            .accept_pending_action_at_participant(db_name, table_name, row_id)
            .await;

        match result {
            Ok(data_result) => {
                if data_result.is_successful {
                    is_local_update_successful = true;

                    let result = self.db.get_cds_host_for_part_db(db_name).await;
                    match result {
                        Ok(opt_host) => {
                            if let Some(remote_host) = opt_host {
                                let result = self.db.treaty_get_host_info().await;

                                match result {
                                    Ok(opt_host_info) => match opt_host_info {
                                        Some(own_host_info) => {
                                            let hash = data_result.data_hash;

                                            let is_deleted = match data_result.action {
                                                Some(action) => match action {
                                                    PartialDataResultAction::Unknown => false,
                                                    PartialDataResultAction::Insert => false,
                                                    PartialDataResultAction::Update => false,
                                                    PartialDataResultAction::Delete => true,
                                                },
                                                None => false,
                                            };

                                            let data_info = DataInfo {
                                                db_name: db_name.to_string(),
                                                table_name: table_name.to_string(),
                                                row_id,
                                                hash,
                                                is_deleted,
                                            };

                                            let notify_is_successful = self
                                                .remote
                                                .notify_host_of_updated_hash(
                                                    &remote_host,
                                                    &own_host_info,
                                                    &data_info,
                                                )
                                                .await;

                                            trace!("notify_is_successful: {notify_is_successful}");

                                            if notify_is_successful {
                                                is_remote_update_successful = true;
                                            }
                                        }
                                        None => {
                                            let _error = TreatyError {
                                                                message: "No local host info is set to identify ourselves to cooperator".to_string(),
                                                                help: Some("Set local host info before enabling cooperative functions".to_string()),
                                                                number: 0,
                                                            };

                                            error = Some(_error);
                                        }
                                    },
                                    Err(e) => error = Some(e.into()),
                                }
                            }
                        }
                        Err(e) => error = Some(e.into()),
                    }
                }
            }
            Err(e) => error = Some(e.into()),
        }

        AcceptPendingActionReply {
            is_successful: is_local_update_successful
                && is_remote_update_successful
                && error.is_none(),
            error,
        }
    }

    pub async fn get_participants(&self, request: GetParticipantsRequest) -> GetParticipantsReply {
        let mut error: Option<TreatyError> = None;
        let mut participants: Vec<ParticipantStatus> = Vec::new();

        let result = self
            .db
            .get_participants_for_database(&request.database_name)
            .await;
        match result {
            Ok(opt_participants) => match opt_participants {
                Some(_participants) => participants = _participants,
                None => {
                    let message = "No participants found - Are cooperative functions enabled on this database?".to_string();
                    warn!("{}", message);
                    error = Some(TreatyError {
                        number: 0,
                        message,
                        help: Some(
                            "Are cooperative functions enabled on this database?".to_string(),
                        ),
                    })
                }
            },
            Err(e) => error = Some(e.into()),
        }

        GetParticipantsReply {
            participants,
            is_error: error.is_some(),
            error,
        }
    }

    pub async fn execute_write_at_participant(
        &self,
        request: ExecuteWriteRequest,
    ) -> ExecuteWriteReply {
        let mut rows_affected: u32 = 0;
        let mut is_overall_successful = false;
        let mut error: Option<TreatyError> = None;

        let db_name = &request.database_name;
        let statement = &request.sql_statement;

        let db_type = self.db.db_type().await;
        let treaty_db_type = self
            .db
            .get_treaty_db_type(db_name)
            .await
            .unwrap_or(TreatyDatabaseType::Unknown);

        trace!("[{}]: {db_name:?}", function_name!());

        let result = self.db.get_cds_host_for_part_db(db_name).await;

        match result {
            Ok(opt_known_host) => {
                match opt_known_host {
                    Some(known_host) => {
                        if treaty_db_type == TreatyDatabaseType::Partial {
                            let statement_type = determine_dml_type(statement, db_type);

                            match statement_type {
                                DmlType::Unknown => todo!(),
                                DmlType::Insert => todo!(),
                                DmlType::Update => {
                                    let result = handle_update_write_at_participant(
                                        &self.db,
                                        &self.remote,
                                        &request,
                                        &known_host,
                                    )
                                    .await;

                                    match result {
                                        Ok(io_result) => {
                                            is_overall_successful = io_result.is_successful;
                                            rows_affected = io_result.rows_affected;
                                        }
                                        Err(e) => error = Some(e.into()),
                                    }
                                }
                                DmlType::Delete => {
                                    let result = handle_delete_write_at_participant(
                                        &self.db,
                                        &self.remote,
                                        &request,
                                        &known_host,
                                    )
                                    .await;

                                    match result {
                                        Ok(io_result) => {
                                            is_overall_successful = io_result.is_successful;
                                            rows_affected = io_result.rows_affected;
                                        }
                                        Err(e) => error = Some(e.into()),
                                    }
                                }
                                DmlType::Select => todo!(),
                            }

                            // we need to determine the statement type (INSERT/UPDATE/DELETE)
                            // and check to see if we need to communicate changes upstream to the host
                            // we do this by looking at the CDS_CONTRACTS_TABLES and checking
                            // the UPDATES_TO_HOST_BEHAVIOR and/or the DELETES_TO_HOST_BEHAVIOR
                            // and responding accordingly
                        }
                    }
                    None => {
                        // check again to see if the db name is failing because of the partial database extension
                        let db_name = db_name.replace(".dbpart", ".db");
                        trace!("[{}]: {db_name:?}", function_name!());
                        let result = self.db.get_cds_host_for_part_db(&db_name).await;

                        if let Ok(opt_host) = result {
                            if let Some(known_host) = opt_host {
                                if treaty_db_type == TreatyDatabaseType::Partial {
                                    let statement_type = determine_dml_type(statement, db_type);

                                    match statement_type {
                                        DmlType::Unknown => todo!(),
                                        DmlType::Insert => todo!(),
                                        DmlType::Update => {
                                            let result = handle_update_write_at_participant(
                                                &self.db,
                                                &self.remote,
                                                &request,
                                                &known_host,
                                            )
                                            .await;

                                            match result {
                                                Ok(io_result) => {
                                                    is_overall_successful = io_result.is_successful;
                                                    rows_affected = io_result.rows_affected;
                                                }
                                                Err(e) => error = Some(e.into()),
                                            }
                                        }
                                        DmlType::Delete => {
                                            let result = handle_delete_write_at_participant(
                                                &self.db,
                                                &self.remote,
                                                &request,
                                                &known_host,
                                            )
                                            .await;

                                            match result {
                                                Ok(io_result) => {
                                                    is_overall_successful = io_result.is_successful;
                                                    rows_affected = io_result.rows_affected;
                                                }
                                                Err(e) => error = Some(e.into()),
                                            }
                                        }
                                        DmlType::Select => todo!(),
                                    }

                                    // we need to determine the statement type (INSERT/UPDATE/DELETE)
                                    // and check to see if we need to communicate changes upstream to the host
                                    // we do this by looking at the CDS_CONTRACTS_TABLES and checking
                                    // the UPDATES_TO_HOST_BEHAVIOR and/or the DELETES_TO_HOST_BEHAVIOR
                                    // and responding accordingly
                                }
                            }
                        } else {
                            let message = format!("No host found for {db_name:?}");
                            error = Some(TreatyError {
                                message,
                                help: None,
                                number: 0,
                            })
                        }
                    }
                }
            }
            Err(e) => error = Some(e.into()),
        }

        ExecuteWriteReply {
            is_successful: is_overall_successful,
            total_rows_affected: rows_affected,
            is_error: error.is_some(),
            error,
        }
    }

    pub async fn get_updates_to_host_behavior(
        &self,
        request: GetUpdatesToHostBehaviorRequest,
    ) -> GetUpdatesToHostBehaviorReply {
        let mut error: Option<TreatyError> = None;
        let mut behavior: Option<u32> = None;

        let db = request.database_name;
        let table = request.table_name;
        let result = self.db.get_updates_to_host_behavior(&db, &table).await;
        match result {
            Ok(_behavior) => {
                behavior = Some(num::ToPrimitive::to_u32(&_behavior).unwrap());
            }
            Err(e) => {
                error = Some(e.into());
            }
        }

        GetUpdatesToHostBehaviorReply { behavior, error }
    }

    pub async fn revoke_token(&self, request: AuthRequestWebToken) -> RevokeReply {
        let result = self.verify_token(&request).await;
        let mut is_successful = false;

        if let Ok(login) = result {
            if login.is_authenticated {
                let result = self.db.revoke_token(&request.jwt).await;
                if let Ok(is_revoked) = result {
                    is_successful = is_revoked;
                }
            }
        }

        RevokeReply { is_successful }
    }

    pub async fn auth_for_token(
        &self,
        request: AuthRequestBasic,
        timeout_in_minutes: Option<u32>,
    ) -> TokenReply {
        let result = self.verify_login_basic(&request).await;
        match result {
            Ok(login) => {
                if login.is_authenticated {
                    let result = self
                        .db
                        .create_token_for_login(&request.user_name, timeout_in_minutes)
                        .await;

                    if let Ok(token_data) = result {
                        let jwt = token_data.0;
                        let expiration = token_data.1.to_rfc3339();
                        debug!("created treaty token for {}", &request.user_name);
                        return TokenReply {
                            is_successful: true,
                            expiration_utc: expiration,
                            jwt,
                        };
                    }
                }

                TokenReply {
                    is_successful: false,
                    expiration_utc: "".to_string(),
                    jwt: "".to_string(),
                }
            }
            Err(_) => TokenReply {
                is_successful: false,
                expiration_utc: "".to_string(),
                jwt: "".to_string(),
            },
        }
    }

    pub async fn set_logical_storage_policy(
        &self,
        request: SetLogicalStoragePolicyRequest,
    ) -> SetLogicalStoragePolicyReply {
        let mut error: Option<TreatyError> = None;

        let mut is_successful = false;

        let policy = LogicalStoragePolicy::from_u32(request.policy_mode);
        let result = self
            .db
            .set_logical_storage_policy(&request.database_name, &request.table_name, policy)
            .await;
        match result {
            Ok(_is_successful) => is_successful = _is_successful,
            Err(e) => error = Some(e.into()),
        }

        SetLogicalStoragePolicyReply {
            is_successful,
            message: "".to_string(),
            error,
        }
    }

    pub async fn get_logical_storage_policy(
        &self,
        request: GetLogicalStoragePolicyRequest,
    ) -> GetLogicalStoragePolicyReply {
        let mut policy = LogicalStoragePolicy::None;
        let mut error: Option<TreatyError> = None;

        let result = self
            .db
            .get_logical_storage_policy(&request.database_name, &request.table_name)
            .await;
        match result {
            Ok(_policy) => {
                policy = _policy;
            }
            Err(e) => error = Some(e.into()),
        }

        GetLogicalStoragePolicyReply {
            policy_mode: LogicalStoragePolicy::to_u32(policy),
            error,
        }
    }

    pub async fn try_auth_at_participant(
        &self,
        request: TryAuthAtParticipantRequest,
    ) -> TryAuthAtPartipantReply {
        let mut error: Option<TreatyError> = None;

        let result = self
            .db
            .get_participant_by_alias(&request.db_name, &request.participant_alias)
            .await;
        match result {
            Ok(opt_participant) => {
                match opt_participant {
                    Some(participant) => {
                        let result = self.db.treaty_get_host_info().await;
                        match result {
                            Ok(opt_host_info) => {
                                match opt_host_info {
                                    Some(host_info) => {
                                        let result = self
                                            .remote
                                            .try_auth_at_participant(participant, &host_info)
                                            .await;
                                        return TryAuthAtPartipantReply {
                                            is_successful: result,
                                            message: "".to_string(),
                                            error,
                                        };
                                    }
                                    None => {
                                        error!("[{}]: No self host info available to identify to participant.", function_name!());
                                        let _error =  TreatyError {
                                            message: "No self host info available to identify to participant".to_string(), 
                                        help: Some("Is host information set?".to_string()), 
                                        number: 0
                                    };
                                        error = Some(_error)
                                    }
                                }
                            }
                            Err(e) => error = Some(e.into()),
                        }
                    }
                    None => todo!(),
                }
            }
            Err(e) => error = Some(e.into()),
        }

        TryAuthAtPartipantReply {
            is_successful: false,
            message: "".to_string(),
            error,
        }
    }

    pub async fn get_host_info(&self) -> HostInfoReply {
        let mut host: Option<Host> = None;
        let mut error: Option<TreatyError> = None;

        let result = self.db.treaty_get_host_info().await;
        match result {
            Ok(result) => match result {
                Some(host_info) => {
                    host = Some(Host {
                        host_guid: host_info.id.clone(),
                        host_name: host_info.name,
                        token: Vec::new(),
                        network: None,
                    });
                }
                None => {
                    warn!("[{}]: No host info available", function_name!());
                }
            },
            Err(e) => error = Some(e.into()),
        }

        HostInfoReply {
            host_info: host,
            error,
        }
    }

    pub async fn generate_contract(
        &self,
        request: GenerateContractRequest,
    ) -> GenerateContractReply {
        let mut is_successful = false;

        let mut _error: Option<TreatyError> = None;
        let db_name = request.database_name;
        let desc = request.description;
        let remote_delete_behavior = RemoteDeleteBehavior::from_u32(request.remote_delete_behavior);
        let host_name = request.host_name;

        let result = self
            .db
            .generate_contract(&db_name, &host_name, &desc, remote_delete_behavior)
            .await;
        match result {
            Ok(_is_successful) => {
                is_successful = _is_successful;
            }
            Err(e) => _error = Some(e.into()),
        }

        GenerateContractReply {
            is_successful,
            message: String::from(""),
            error: _error,
        }
    }

    pub async fn has_table(&self, request: HasTableRequest) -> HasTableReply {
        let mut db_has_table: bool = false;

        let db_name = request.database_name;
        let table_name = request.table_name;
        let mut error_item: Option<TreatyError> = None;

        let result = self.db.has_table(&db_name, &table_name).await;
        match result {
            Ok(_has_table) => db_has_table = _has_table,
            Err(e) => error_item = Some(e.into()),
        }

        HasTableReply {
            error: error_item,
            has_table: db_has_table,
        }
    }

    pub async fn execute_cooperative_write_at_host(
        &self,
        request: ExecuteCooperativeWriteRequest,
    ) -> ExecuteCooperativeWriteReply {
        trace!("[{}]: {request:?}", function_name!());

        let mut error: Option<TreatyError> = None;
        let mut is_remote_action_successful = false;
        let mut is_local_action_successful = false;
        let db_name = request.database_name;
        let statement = request.sql_statement;
        let total_rows_affected = 0;

        let result = self.db.has_participant(&db_name, &request.alias).await;
        match result {
            Ok(has_participant) => {
                if has_participant {
                    let result = self
                        .db
                        .get_participant_by_alias(&db_name, &request.alias)
                        .await;
                    match result {
                        Ok(opt_participant) => {
                            match opt_participant {
                                Some(db_participant) => {
                                    let opt_host_info = self
                                        .db
                                        .treaty_get_host_info()
                                        .await
                                        .expect("no host info is set");
                                    match opt_host_info {
                                        Some(host_info) => {
                                            let cmd_table_name =
                                                get_table_name(&statement, self.db.db_type().await);
                                            let where_clause = request.where_clause.clone();
                                            let db_participant_reference = db_participant.clone();
                                            let dml_type = determine_dml_type(
                                                &statement,
                                                self.db.db_type().await,
                                            );

                                            match dml_type {
                                                DmlType::Unknown => {
                                                    error!("Unknown Dml Type");
                                                }
                                                DmlType::Insert => {
                                                    let remote_insert_result = self
                                                        .remote
                                                        .insert_row_at_participant(
                                                            db_participant,
                                                            &host_info,
                                                            &db_name,
                                                            &cmd_table_name,
                                                            &statement,
                                                        )
                                                        .await;

                                                    if remote_insert_result.is_successful {
                                                        // we need to add the data hash and row id here
                                                        let data_hash =
                                                            remote_insert_result.data_hash;
                                                        let row_id = remote_insert_result.row_id;

                                                        if row_id == 0 {
                                                            warn!("potentially invalid row_id 0");
                                                        }

                                                        let internal_participant_id =
                                                            db_participant_reference
                                                                .internal_id
                                                                .to_string();

                                                        let result = self
                                                            .db
                                                            .insert_metadata_into_host_db(
                                                                &db_name,
                                                                &cmd_table_name,
                                                                row_id,
                                                                data_hash,
                                                                &internal_participant_id,
                                                            )
                                                            .await;

                                                        match result {
                                                            Ok(local_insert_is_successful) => {
                                                                if local_insert_is_successful {
                                                                    is_remote_action_successful =
                                                                        true;
                                                                    is_local_action_successful =
                                                                        local_insert_is_successful;
                                                                }
                                                            }
                                                            Err(e) => error = Some(e.into()),
                                                        }
                                                    } else {
                                                        warn!("[{}]: remote insert was not successful: {remote_insert_result:?}", function_name!());
                                                    }
                                                }
                                                DmlType::Update => {
                                                    let remote_update_result = self
                                                        .remote
                                                        .update_row_at_participant(
                                                            db_participant,
                                                            &host_info,
                                                            &db_name,
                                                            &cmd_table_name,
                                                            &statement,
                                                            &where_clause,
                                                        )
                                                        .await;

                                                    trace!(
                                                        "[{}]: {remote_update_result:?}",
                                                        function_name!()
                                                    );

                                                    if remote_update_result.is_successful {
                                                        let data_hash: u64;
                                                        let row_id: u32;

                                                        let update_result =
                                                            PartialDataStatus::from_u32(
                                                                remote_update_result.update_status,
                                                            );

                                                        match update_result {
                                                                    PartialDataStatus::Unknown => todo!(),
                                                                    PartialDataStatus::SucessOverwriteOrLog => {
                                                                        data_hash = remote_update_result.rows.first().unwrap().data_hash;
                                                                        row_id = remote_update_result.rows.first().unwrap().row_id;
                                                                        let internal_participant_id =
                                                                            db_participant_reference.internal_id.to_string();
                                            
                                                                        let result = self.db.update_metadata_in_host_db(
                                                                            &db_name,
                                                                            &cmd_table_name,
                                                                            row_id,
                                                                            data_hash,
                                                                            &internal_participant_id,
                                                                        ).await;

                                                                        match result {
                                                                            Ok(local_update_is_successful) => {
                                                                                trace!("local update is successful: {local_update_is_successful}");
                                            
                                                                                if local_update_is_successful {
                                                                                    is_remote_action_successful = true;
                                                                                    is_local_action_successful = local_update_is_successful;
                                                                                }
                                                                            },
                                                                            Err(e) => error = Some(e.into())
                                                                        }
                                                                    }
                                                                    PartialDataStatus::Pending => {
                                                                        is_remote_action_successful = true;
                                                                        is_local_action_successful = true;
                                                                    }
                                                                    PartialDataStatus::Ignored => todo!(),
                                                                }
                                                    }
                                                }
                                                DmlType::Delete => {
                                                    let remote_delete_result = self
                                                        .remote
                                                        .remove_row_at_participant(
                                                            db_participant,
                                                            &host_info,
                                                            &db_name,
                                                            &cmd_table_name,
                                                            &statement,
                                                            &where_clause,
                                                        )
                                                        .await;

                                                    trace!(
                                                                "[{}]: remote_delete_result: {remote_delete_result:?}",
                                                                function_name!()
                                                            );

                                                    if remote_delete_result.is_successful {
                                                        let row_id: u32 =
                                                            if remote_delete_result.rows.is_empty()
                                                            {
                                                                0
                                                            } else {
                                                                remote_delete_result
                                                                    .rows
                                                                    .first()
                                                                    .unwrap()
                                                                    .row_id
                                                            };

                                                        let internal_participant_id =
                                                            db_participant_reference
                                                                .internal_id
                                                                .to_string();

                                                        let result = self
                                                            .db
                                                            .delete_metadata_in_host_db(
                                                                &db_name,
                                                                &cmd_table_name,
                                                                row_id,
                                                                &internal_participant_id,
                                                            )
                                                            .await;

                                                        match result {
                                                            Ok(local_delete_is_successful) => {
                                                                trace!(
                                                                            "[{}]: local_delete_is_successful: {local_delete_is_successful:?}",
                                                                            function_name!()
                                                                        );

                                                                if local_delete_is_successful {
                                                                    is_remote_action_successful =
                                                                        true;
                                                                    is_local_action_successful =
                                                                        local_delete_is_successful;
                                                                }
                                                            }
                                                            Err(e) => error = Some(e.into()),
                                                        }
                                                    } else {
                                                        warn!("remote delete was not successful");
                                                    }
                                                }
                                                DmlType::Select => {
                                                    error!("[{}]: A read statement was executed for a cooperative write, this should not happen.", function_name!());
                                                }
                                            };
                                        }
                                        None => {
                                            error = Some(TreatyError::message_help(
                                                "No host info is set",
                                                "Set host info before any cooperative functions",
                                            ))
                                        }
                                    }
                                }
                                None => {
                                    error = Some(TreatyError::message("Participant was not found"))
                                }
                            }
                        }
                        Err(e) => error = Some(e.into()),
                    }
                } else {
                    error = Some(TreatyError::message("Participant was not found"))
                }
            }
            Err(e) => error = Some(e.into()),
        }

        trace!(
            "[{}]: is_local_action_successful: {}",
            function_name!(),
            is_local_action_successful
        );
        trace!(
            "[{}]: is_remote_action_successful: {}",
            function_name!(),
            is_remote_action_successful
        );

        ExecuteCooperativeWriteReply {
            is_successful: is_local_action_successful && is_remote_action_successful,
            total_rows_affected,
            error,
        }
    }

    async fn verify_login_basic(
        &self,
        request: &AuthRequestBasic,
    ) -> Result<AuthResult, TreatyDbError> {
        let is_authenticated = self
            .db
            .verify_login(&request.user_name, &request.pw)
            .await?;

        Ok(AuthResult {
            is_authenticated,
            message: None,
        })
    }

    async fn verify_token(
        &self,
        request: &AuthRequestWebToken,
    ) -> Result<AuthResult, TreatyDbError> {
        let is_authenticated = self.db.verify_token(&request.jwt).await?;

        Ok(AuthResult {
            is_authenticated,
            message: None,
        })
    }
}

pub fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
