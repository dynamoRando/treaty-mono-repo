use std::net::SocketAddr;

use async_trait::async_trait;
use chrono::Utc;
use stdext::function_name;
use tonic::transport::Server;
use tracing::{debug, error, trace};
use triggered::Listener;
use treaty_types::enums::{ContractStatus};

use crate::{
    db_interface::dbi_actions::DbiActions,
    defaults,
    error::TreatyDbError,
    models::CoopDatabaseParticipant,
    settings::Settings,
    treaty_proto::{
        data_service_server::DataServiceServer, AuthRequest, AuthResult,
        CreatePartialDatabaseRequest, CreatePartialDatabaseResult, CreateTableRequest,
        CreateTableResult, DeleteDataRequest, DeleteDataResult, GetRowFromPartialDatabaseRequest,
        GetRowFromPartialDatabaseResult, InsertDataRequest, InsertDataResult,
        NotifyHostOfRemovedRowRequest, NotifyHostOfRemovedRowResult,
        ParticipantAcceptsContractRequest, ParticipantAcceptsContractResult, Row,
        SaveContractRequest, SaveContractResult, TestReply, TestRequest, TreatyError,
        TryAuthRequest, TryAuthResult, UpdateDataRequest, UpdateDataResult,
        UpdateRowDataHashForHostRequest, UpdateRowDataHashForHostResult,
    },
};

use self::{
    data_service_handler_actions::DataServiceHandlerActions,
    io::{delete::handle_delete_command_into_table, update::handle_update_command_into_table},
};

pub mod data_service_handler_actions;
pub mod grpc;
pub mod io;

#[derive(Debug, Clone)]
pub struct DataServiceHandler<D: DbiActions + Clone + Send + Sync + 'static> {
    db: D,
    settings: Settings,
}

#[async_trait]
impl<D: DbiActions + Clone + Send + Sync> DataServiceHandlerActions for DataServiceHandler<D> {
    async fn delete_command_into_table(&self, request: DeleteDataRequest) -> DeleteDataResult {
        self.delete_command_into_table(request).await
    }

    async fn get_row_from_partial_database(
        &self,
        request: GetRowFromPartialDatabaseRequest,
    ) -> GetRowFromPartialDatabaseResult {
        self.get_row_from_partial_database(request).await
    }

    async fn update_command_into_table(&self, request: UpdateDataRequest) -> UpdateDataResult {
        self.update_command_into_table(request).await
    }
    async fn save_contract(&self, request: SaveContractRequest) -> SaveContractResult {
        self.save_contract(request).await
    }
    async fn insert_command_into_table(&self, request: InsertDataRequest) -> InsertDataResult {
        self.insert_command_into_table(request).await
    }
    async fn create_table_in_database(&self, request: CreateTableRequest) -> CreateTableResult {
        self.create_table_in_database(request).await
    }
    async fn create_partial_database(
        &self,
        request: CreatePartialDatabaseRequest,
    ) -> CreatePartialDatabaseResult {
        self.create_partial_database(request).await
    }
    async fn accept_contract(
        &self,
        request: ParticipantAcceptsContractRequest,
    ) -> ParticipantAcceptsContractResult {
        self.accept_contract(request).await
    }
    async fn update_row_data_hash_for_host(
        &self,
        request: UpdateRowDataHashForHostRequest,
    ) -> UpdateRowDataHashForHostResult {
        self.update_row_data_hash_for_host(request).await
    }
    async fn notify_host_of_removed_row(
        &self,
        request: NotifyHostOfRemovedRowRequest,
    ) -> NotifyHostOfRemovedRowResult {
        self.notify_host_of_removed_row(request).await
    }
    async fn try_auth(&self, request: TryAuthRequest) -> TryAuthResult {
        self.try_auth(request).await
    }
    async fn is_online(&self, request: TestRequest) -> TestReply {
        self.is_online(request).await
    }
}

impl<D: DbiActions + Clone + Send + Sync> DataServiceHandler<D> {
    pub fn new(db: D, settings: Settings) -> Self {
        Self { db, settings }
    }

    fn authenticate_host(&self, authentication: &AuthRequest) -> Result<AuthResult, TreatyDbError> {
        let mut is_authenticated = false;

        let host_id = &authentication.user_name;
        let host_token = &authentication.token;

        if self.db.verify_host_by_id(host_id, host_token.to_vec())? {
            is_authenticated = true;
        }

        if self.db.verify_host_by_name(host_id, host_token.to_vec())? {
            is_authenticated = true;
        }

        Ok(AuthResult {
            is_authenticated,
            message: None,
        })
    }

    fn authenticate_participant(
        &self,
        authentication: &AuthRequest,
        db_name: &str,
    ) -> Result<AuthResult, TreatyDbError> {
        let host_id = &authentication.user_name;
        let host_token = &authentication.token;
        let participant = self.db.get_participant_by_alias(db_name, host_id)?;

        match participant {
            Some(p) => {
                let is_auth = do_vecs_match(&p.token, host_token);
                Ok(AuthResult {
                    is_authenticated: is_auth,
                    message: None,
                })
            }
            None => Ok(AuthResult {
                is_authenticated: false,
                message: None,
            }),
        }
    }

    #[tokio::main]
    pub async fn start_grpc(&self, shutdown: Listener) -> Result<(), Box<dyn std::error::Error>> {
        trace!("{}", self.settings.grpc_data_service_addr_port);
        let addr: SocketAddr = self.settings.grpc_data_service_addr_port.parse().unwrap();

        let data_client_service = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        debug!("data service gprc listening on {addr}");

        Server::builder()
            .add_service(DataServiceServer::new(self.clone()))
            .add_service(data_client_service) // Add this
            .serve_with_shutdown(addr, shutdown)
            .await?;

        Ok(())
    }

    pub async fn delete_command_into_table(&self, request: DeleteDataRequest) -> DeleteDataResult {
        let result = self.authenticate_host(request.authentication.as_ref().unwrap());
        match result {
            Ok(login) => handle_delete_command_into_table(&self.db, &request, &login).await,
            Err(e) => DeleteDataResult {
                authentication_result: None,
                is_successful: false,
                message: String::from(""),
                rows: Vec::new(),
                is_error: true,
                error: Some(e.into()),
            },
        }
    }

    pub async fn get_row_from_partial_database(
        &self,
        request: GetRowFromPartialDatabaseRequest,
    ) -> GetRowFromPartialDatabaseResult {
        let result = self.authenticate_host(&request.authentication.as_ref().unwrap().clone());
        match result {
            Ok(login) => {
                let mut is_successful = false;
                let mut row: Option<Row> = None;
                let mut error: Option<TreatyError> = None;

                trace!("[{}]: {request:?}", function_name!());

                if login.is_authenticated {
                    let db_name = request.row_address.as_ref().unwrap().database_name.clone();
                    let table_name = request.row_address.as_ref().unwrap().table_name.clone();
                    let row_id = request.row_address.as_ref().unwrap().row_id;

                    let result =
                        self.db
                            .get_row_from_partial_database(&db_name, &table_name, row_id);
                    match result {
                        Ok(_row) => {
                            trace!("[{}]: {_row:?}", function_name!());
                            is_successful = true;
                            row = Some(_row);
                        }
                        Err(e) => error = Some(e.into()),
                    }
                }

                {
                    GetRowFromPartialDatabaseResult {
                        authentication_result: Some(login),
                        is_successful,
                        result_message: String::from(""),
                        row,
                        error,
                    }
                }
            }
            Err(e) => GetRowFromPartialDatabaseResult {
                authentication_result: None,
                is_successful: false,
                result_message: String::from(""),
                row: None,
                error: Some(e.into()),
            },
        }
    }

    pub async fn update_command_into_table(&self, request: UpdateDataRequest) -> UpdateDataResult {
        let result = self.authenticate_host(request.authentication.as_ref().unwrap());
        match result {
            Ok(login) => handle_update_command_into_table(&self.db, &request, &login).await,
            Err(e) => UpdateDataResult {
                authentication_result: None,
                is_successful: false,
                message: String::from(""),
                rows: Vec::new(),
                update_status: 0,
                is_error: true,
                error: Some(e.into()),
            },
        }
    }

    pub async fn save_contract(&self, request: SaveContractRequest) -> SaveContractResult {
        trace!("[{}]: {request:?}", function_name!());
        let contract = request.contract.unwrap();
        let result = self.db.save_contract(contract);
        match result {
            Ok(save_result) => {
                trace!("[{}]: {save_result:?}", function_name!());
                let status = ContractStatus::to_u32(save_result.contract_status);
                SaveContractResult {
                    is_saved: save_result.is_successful,
                    contract_status: status,
                    participant_info: save_result.participant_information,
                    is_error: false,
                    error: None,
                }
            }
            Err(e) => SaveContractResult {
                is_saved: false,
                contract_status: 0,
                participant_info: None,
                is_error: true,
                error: Some(e.into()),
            },
        }
    }

    pub async fn insert_command_into_table(&self, request: InsertDataRequest) -> InsertDataResult {
        let result = self.authenticate_host(&request.authentication.unwrap());
        match result {
            Ok(login) => {
                let db_name = request.database_name;
                let table_name = request.table_name;
                let cmd = &request.cmd;

                let mut error: Option<TreatyError> = None;
                let mut row_id: u32 = 0;
                let mut data_hash: u64 = 0;
                let mut is_successful = false;

                if login.is_authenticated {
                    let result = self
                        .db
                        .insert_data_into_partial_db(&db_name, &table_name, cmd);
                    match result {
                        Ok(data_result) => {
                            trace!("[{}]: {data_result:?}", function_name!());
                            is_successful = data_result.is_successful;
                            row_id = data_result.row_id;
                            if let Some(hash) = data_result.data_hash {
                                data_hash = hash;
                            }
                        }
                        Err(e) => error = Some(e.into()),
                    }
                }

                InsertDataResult {
                    authentication_result: Some(login),
                    is_successful,
                    data_hash,
                    message: String::from(""),
                    row_id,
                    is_error: error.is_some(),
                    error,
                }
            }
            Err(e) => InsertDataResult {
                authentication_result: None,
                is_successful: false,
                data_hash: 0,
                message: String::from(""),
                row_id: 0,
                is_error: true,
                error: Some(e.into()),
            },
        }
    }

    pub async fn create_table_in_database(&self, request: CreateTableRequest) -> CreateTableResult {
        let result = self.authenticate_host(&request.authentication.unwrap());
        match result {
            Ok(login) => {
                let mut error: Option<TreatyError> = None;
                let db_name = request.database_name;
                let table_name = request.table_name;
                let table_schema = request.columns;

                let mut table_id = String::from("");
                let mut db_id = String::from("");
                let mut is_successful = false;

                if login.is_authenticated {
                    let result = self.db.create_table_in_partial_database(
                        &db_name,
                        &table_name,
                        table_schema,
                    );
                    match result {
                        Ok(_is_success) => {
                            if _is_success {
                                let result = self.db.get_table_id(&db_name, &table_name);
                                match result {
                                    Ok(_table_id) => {
                                        let result = self.db.get_db_id(&db_name);
                                        match result {
                                            Ok(_db_id) => {
                                                is_successful = _is_success;
                                                db_id = _db_id;
                                                table_id = _table_id;
                                            }
                                            Err(e) => error = Some(e.into()),
                                        }
                                    }
                                    Err(e) => error = Some(e.into()),
                                }
                            }
                        }
                        Err(e) => error = Some(e.into()),
                    }
                }

                CreateTableResult {
                    authentication_result: Some(login),
                    is_successful,
                    database_name: db_name,
                    result_message: String::from(""),
                    database_id: db_id,
                    table_name,
                    table_id,
                    is_error: error.is_some(),
                    error,
                }
            }
            Err(e) => CreateTableResult {
                authentication_result: None,
                is_successful: false,
                database_name: String::from(""),
                result_message: String::from(""),
                database_id: String::from(""),
                table_name: String::from(""),
                table_id: String::from(""),
                is_error: true,
                error: Some(e.into()),
            },
        }
    }

    pub async fn create_partial_database(
        &self,
        request: CreatePartialDatabaseRequest,
    ) -> CreatePartialDatabaseResult {
        let result = self.authenticate_host(&request.authentication.unwrap());

        match result {
            Ok(login) => {
                let mut error: Option<TreatyError> = None;
                let db_name = request.database_name;
                let mut db_id = String::from("");
                let mut is_successful = false;

                if login.is_authenticated {
                    let result = self.db.create_partial_database(&db_name);
                    match result {
                        Ok(_result) => {
                            if _result {
                                let result = self.db.get_db_id(&db_name);
                                match result {
                                    Ok(_db_id) => {
                                        db_id = _db_id;
                                        is_successful = true;
                                    }
                                    Err(e) => error = Some(e.into()),
                                }
                            }
                        }
                        Err(e) => error = Some(e.into()),
                    }
                }

                CreatePartialDatabaseResult {
                    authentication_result: Some(login),
                    is_successful,
                    database_name: db_name,
                    database_id: db_id,
                    is_error: error.is_some(),
                    error,
                }
            }
            Err(e) => CreatePartialDatabaseResult {
                authentication_result: None,
                is_successful: false,
                database_name: String::from(""),
                database_id: String::from(""),
                is_error: true,
                error: Some(e.into()),
            },
        }
    }

    pub async fn accept_contract(
        &self,
        request: ParticipantAcceptsContractRequest,
    ) -> ParticipantAcceptsContractResult {
        let debug_message_info = &request.telemetry.as_ref().unwrap().clone();

        trace!("{debug_message_info:?}");
        trace!("{request:?}");

        let participant_message = request.participant.as_ref().unwrap().clone();

        let coop_db_participant: CoopDatabaseParticipant;

        let accepted_participant = self
            .db
            .get_participant_by_alias(
                &request.database_name,
                &request.participant.as_ref().unwrap().alias,
            )
            .unwrap();

        if accepted_participant.is_none() {
            let _participant = self
                .db
                .get_participant_by_id(
                    &request.database_name,
                    &request.participant.as_ref().unwrap().participant_guid,
                )
                .unwrap();

            if _participant.is_some() {
                trace!(
                    "found participant: {:?}",
                    _participant.as_ref().unwrap().clone()
                );
                coop_db_participant = _participant.unwrap();
            } else {
                error!("Could not find participant by alias or id, this should not happen.");
                coop_db_participant = CoopDatabaseParticipant::default();
            }
        } else {
            trace!(
                "found participant: {:?}",
                accepted_participant.as_ref().unwrap().clone()
            );
            coop_db_participant = accepted_participant.unwrap();
        }

        let is_successful = self
            .db
            .update_participant_accepts_contract(
                &request.database_name,
                coop_db_participant,
                participant_message,
                &request.contract_version_guid,
            )
            .unwrap();

        ParticipantAcceptsContractResult {
            contract_acceptance_is_acknowledged: is_successful,
            is_error: false,
            error: None,
        }
    }

    pub async fn update_row_data_hash_for_host(
        &self,
        request: UpdateRowDataHashForHostRequest,
    ) -> UpdateRowDataHashForHostResult {
        let mut is_successful = false;

        let authentication = request.authentication.unwrap();
        let user_name = authentication.user_name.clone();

        let result = self.authenticate_participant(&authentication, &request.database_name);
        match result {
            Ok(login) => {
                let mut error: Option<TreatyError> = None;
                if login.is_authenticated {
                    let db_name = request.database_name.clone();
                    let table_name = request.table_name.clone();
                    let row_id = request.row_id;
                    let hash = request.updated_hash_value;

                    let result = self.db.get_participant_by_alias(&db_name, &user_name);
                    match result {
                        Ok(opt_participant) => match opt_participant {
                            Some(participant) => {
                                let result = self.db.update_metadata_in_host_db(
                                    &db_name,
                                    &table_name,
                                    row_id,
                                    hash,
                                    &participant.internal_id.to_string(),
                                );
                                match result {
                                    Ok(is_updated) => {
                                        is_successful = is_updated;
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

                UpdateRowDataHashForHostResult {
                    authentication_result: Some(login),
                    is_successful,
                    error,
                }
            }
            Err(e) => UpdateRowDataHashForHostResult {
                authentication_result: None,
                is_successful,
                error: Some(e.into()),
            },
        }
    }

    pub async fn notify_host_of_removed_row(
        &self,
        request: NotifyHostOfRemovedRowRequest,
    ) -> NotifyHostOfRemovedRowResult {
        let result =
            self.authenticate_participant(&request.authentication.unwrap(), &request.database_name);

        match result {
            Ok(login) => {
                let mut is_successful = false;
                let mut error: Option<TreatyError> = None;

                if login.is_authenticated {
                    let db_name = request.database_name.clone();
                    let table_name = request.table_name.clone();
                    let row_id = request.row_id;

                    let result = self.db.remove_remote_row_reference_from_host(
                        &db_name,
                        &table_name,
                        row_id,
                    );
                    match result {
                        Ok(_is_removed) => {
                            is_successful = _is_removed;
                        }
                        Err(e) => error = Some(e.into()),
                    }
                }

                NotifyHostOfRemovedRowResult {
                    authentication_result: Some(login),
                    is_successful,
                    error,
                }
            }
            Err(e) => NotifyHostOfRemovedRowResult {
                authentication_result: None,
                is_successful: false,
                error: Some(e.into()),
            },
        }
    }

    pub async fn try_auth(&self, request: TryAuthRequest) -> TryAuthResult {
        let result = self.authenticate_host(&request.authentication.unwrap());

        match result {
            Ok(login) => TryAuthResult {
                authentication_result: Some(login),
            },
            Err(_) => TryAuthResult {
                authentication_result: None,
            },
        }
    }

    pub async fn is_online(&self, request: TestRequest) -> TestReply {
        let item = request.request_echo_message;

        TestReply {
            reply_time_utc: Utc::now().to_rfc2822(),
            reply_echo_message: item,
            treaty_version: defaults::VERSION.to_string(),
        }
    }
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
