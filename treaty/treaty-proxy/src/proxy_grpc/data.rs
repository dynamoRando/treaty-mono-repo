use crate::TreatyProxy;
use chrono::Utc;
use stdext::function_name;
use tonic::{Request, Response, Status};
use tracing::{debug, trace, warn};
use treaty::{
    data_service_handler::data_service_handler_actions::DataServiceHandlerActions,
    defaults,
    treaty_proto::{data_service_server::DataService, *},
};
use treaty_http_endpoints::headers::TREATY_AUTH_HEADER_METADATA_BIN;

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
    async fn validate_auth_request(
        &self,
        request: &tonic::metadata::MetadataMap,
    ) -> Result<Box<dyn DataServiceHandlerActions + Send>, AuthResult> {
        trace!("[{}]: metadata: {request:?}", function_name!());

        if request.contains_key(TREATY_AUTH_HEADER_METADATA_BIN) {
            let kv = request.get_bin(TREATY_AUTH_HEADER_METADATA_BIN);
            if let Some(key_value) = kv {
                let bytes = key_value.to_bytes().unwrap();
                let metadata: AuthRequestMetadata = bincode::deserialize(&bytes).unwrap();
                trace! {"[{}]: {metadata:?}", function_name!()};

                if metadata.id.is_some() {
                    let id = metadata.id.unwrap();
                    trace!("[{}]: request_id: {id:?}", function_name!());
                    let result_has_core = self.proxy.get_treaty_data_handler(&id).await;

                    if result_has_core.is_ok() {
                        return Ok(result_has_core.unwrap());
                    } else {
                        warn!(
                            "[{}]: Host Id was not found in at this proxy instance",
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
                        "[{}]: No Host Id was not found in request",
                        function_name!()
                    );
                    return Err(AuthResult {
                        is_authenticated: false,
                        message: Some("No Host Id provided for treaty-proxy instance".to_string()),
                    });
                }
            }
        }

        warn!("[{}]: No authentication provided", function_name!());
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
        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();
        match auth_result {
            Ok(core) => {
                let response = core.create_partial_database(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = CreatePartialDatabaseResult {
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
        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.create_table_in_database(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = CreateTableResult {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.insert_command_into_table(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = InsertDataResult {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.update_command_into_table(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = UpdateDataResult {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.delete_command_into_table(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = DeleteDataResult {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.get_row_from_partial_database(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = GetRowFromPartialDatabaseResult {
                    is_successful: false,
                    row: None,
                    result_message: "".to_string(),
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.update_row_data_hash_for_host(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = UpdateRowDataHashForHostResult {
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

        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.notify_host_of_removed_row(request).await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = NotifyHostOfRemovedRowResult {
                    is_successful: false,
                    error: None,
                };

                warn!("notify_host_of_removed_row: {reply:?}");

                return Ok(Response::new(reply));
            }
        }
    }

    async fn try_auth(&self, request: Request<()>) -> Result<Response<TryAuthResult>, Status> {
        debug!("Request from {:?}", request.remote_addr());

        let auth_result = self.validate_auth_request(&request.metadata()).await;

        match auth_result {
            Ok(core) => {
                let response = core.try_auth().await;
                return Ok(Response::new(response));
            }
            Err(_) => {
                let reply = TryAuthResult {
                    is_authenticated: false,
                };

                return Ok(Response::new(reply));
            }
        }
    }
}
