use std::net::SocketAddr;

use async_trait::async_trait;
use chrono::Utc;
use stdext::function_name;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic_async_interceptor::AsyncInterceptedService;
use tracing::{debug, trace};
use triggered::Listener;

use crate::{
    db_interface::dbi_actions::DbiActions,
    defaults,
    settings::Settings,
    treaty_proto::{
        data_service_server::DataServiceServer, CreatePartialDatabaseRequest,
        CreatePartialDatabaseResult, CreateTableRequest, CreateTableResult, DeleteDataRequest,
        DeleteDataResult, GetRowFromPartialDatabaseRequest, GetRowFromPartialDatabaseResult,
        InsertDataRequest, InsertDataResult, NotifyHostOfRemovedRowRequest,
        NotifyHostOfRemovedRowResult, Row, TestReply, TestRequest, TreatyError, TryAuthResult,
        UpdateDataRequest, UpdateDataResult, UpdateRowDataHashForHostRequest,
        UpdateRowDataHashForHostResult,
    },
    user_service_handler::authentication::Authenticator,
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
    async fn try_auth(&self) -> TryAuthResult {
        self.try_auth().await
    }
    async fn is_online(&self, request: TestRequest) -> TestReply {
        self.is_online(request).await
    }
}

impl<D: DbiActions + Clone + Send + Sync> DataServiceHandler<D> {
    pub fn new(db: D, settings: Settings) -> Self {
        Self { db: db, settings }
    }

    #[tokio::main]
    pub async fn start_grpc(&self, shutdown: Listener) -> Result<(), Box<dyn std::error::Error>> {
        trace!("{}", self.settings.grpc_data_service_addr_port);
        let addr: SocketAddr = self.settings.grpc_data_service_addr_port.parse().unwrap();
        debug!("data service gprc listening on {addr}");

        let authenticator = Authenticator::new(self.db.clone());

        let is = AsyncInterceptedService::new(DataServiceServer::new(self.clone()), authenticator);

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

    pub async fn delete_command_into_table(&self, request: DeleteDataRequest) -> DeleteDataResult {
        handle_delete_command_into_table(&self.db, &request).await
    }

    pub async fn get_row_from_partial_database(
        &self,
        request: GetRowFromPartialDatabaseRequest,
    ) -> GetRowFromPartialDatabaseResult {
        let mut is_successful = false;
        let mut row: Option<Row> = None;
        let mut error: Option<TreatyError> = None;

        trace!("[{}]: {request:?}", function_name!());

        let db_name = request.row_address.as_ref().unwrap().database_name.clone();
        let table_name = request.row_address.as_ref().unwrap().table_name.clone();
        let row_id = request.row_address.as_ref().unwrap().row_id;

        let result = self
            .db
            .get_row_from_partial_database(&db_name, &table_name, row_id)
            .await;
        match result {
            Ok(_row) => {
                trace!("[{}]: {_row:?}", function_name!());
                is_successful = true;
                row = Some(_row);
            }
            Err(e) => error = Some(e.into()),
        }

        GetRowFromPartialDatabaseResult {
            is_successful,
            result_message: String::from(""),
            row,
            error,
        }
    }

    pub async fn update_command_into_table(&self, request: UpdateDataRequest) -> UpdateDataResult {
        handle_update_command_into_table(&self.db, &request).await
    }

    pub async fn insert_command_into_table(&self, request: InsertDataRequest) -> InsertDataResult {
        let db_name = request.database_name;
        let table_name = request.table_name;
        let cmd = &request.cmd;

        let mut error: Option<TreatyError> = None;
        let mut row_id: u32 = 0;
        let mut data_hash: u64 = 0;
        let mut is_successful = false;

        let result = self
            .db
            .insert_data_into_partial_db(&db_name, &table_name, cmd)
            .await;
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

        InsertDataResult {
            is_successful,
            data_hash,
            message: String::from(""),
            row_id,
            is_error: error.is_some(),
            error,
        }
    }

    pub async fn create_table_in_database(&self, request: CreateTableRequest) -> CreateTableResult {
        let mut error: Option<TreatyError> = None;
        let db_name = request.database_name;
        let table_name = request.table_name;
        let table_schema = request.columns;

        let mut table_id = String::from("");
        let mut db_id = String::from("");
        let mut is_successful = false;

        let result = self
            .db
            .create_table_in_partial_database(&db_name, &table_name, table_schema)
            .await;
        match result {
            Ok(_is_success) => {
                if _is_success {
                    let result = self.db.get_table_id(&db_name, &table_name).await;
                    match result {
                        Ok(_table_id) => {
                            let result = self.db.get_db_id(&db_name).await;
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

        CreateTableResult {
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

    pub async fn create_partial_database(
        &self,
        request: CreatePartialDatabaseRequest,
    ) -> CreatePartialDatabaseResult {
        let mut error: Option<TreatyError> = None;
        let db_name = request.database_name;
        let mut db_id = String::from("");
        let mut is_successful = false;

        let result = self.db.create_partial_database(&db_name).await;
        match result {
            Ok(_result) => {
                if _result {
                    let result = self.db.get_db_id(&db_name).await;
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

        CreatePartialDatabaseResult {
            is_successful,
            database_name: db_name,
            database_id: db_id,
            is_error: error.is_some(),
            error,
        }
    }

    pub async fn update_row_data_hash_for_host(
        &self,
        request: UpdateRowDataHashForHostRequest,
    ) -> UpdateRowDataHashForHostResult {
        let mut is_successful = false;
        let user_name = request.host_info.unwrap_or_default().host_name;
        let mut error: Option<TreatyError> = None;

        let db_name = request.database_name.clone();
        let table_name = request.table_name.clone();
        let row_id = request.row_id;
        let hash = request.updated_hash_value;

        let result = self.db.get_participant_by_alias(&db_name, &user_name).await;
        match result {
            Ok(opt_participant) => match opt_participant {
                Some(participant) => {
                    let result = self
                        .db
                        .update_metadata_in_host_db(
                            &db_name,
                            &table_name,
                            row_id,
                            hash,
                            &participant.internal_id.to_string(),
                        )
                        .await;
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

        UpdateRowDataHashForHostResult {
            is_successful,
            error,
        }
    }

    pub async fn notify_host_of_removed_row(
        &self,
        request: NotifyHostOfRemovedRowRequest,
    ) -> NotifyHostOfRemovedRowResult {
        let mut is_successful = false;
        let mut error: Option<TreatyError> = None;

        let db_name = request.database_name.clone();
        let table_name = request.table_name.clone();
        let row_id = request.row_id;

        let result = self
            .db
            .remove_remote_row_reference_from_host(&db_name, &table_name, row_id)
            .await;
        match result {
            Ok(_is_removed) => {
                is_successful = _is_removed;
            }
            Err(e) => error = Some(e.into()),
        }

        NotifyHostOfRemovedRowResult {
            is_successful,
            error,
        }
    }

    pub async fn try_auth(&self) -> TryAuthResult {
        TryAuthResult {
            is_authenticated: false,
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

// fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
//     let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
//     matching == a.len() && matching == b.len()
// }
