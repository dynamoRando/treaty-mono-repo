use crate::TreatyProxy;
use stdext::function_name;
use tonic::{Request, Response, Status};
use tracing::{debug, error, trace, warn};
use treaty::treaty_proto::{info_service_server::InfoService, *};

use treaty::info_service_handler::info_service_handler_actions::InfoServiceHandlerActions;
use treaty_http_endpoints::headers::TREATY_AUTH_HEADER_METADATA_BIN;

#[allow(dead_code)]
/// Implements the `InfoServiceHandler` definition from the protobuf file
pub struct ProxyInfoServiceHandlerGrpc {
    root_folder: String,
    database_name: String,
    data_addr_port: String,
    info_addr_port: String,
    own_db_addr_port: String,
    proxy: TreatyProxy,
}

impl ProxyInfoServiceHandlerGrpc {
    #[allow(dead_code, unused_variables)]
    pub fn new(
        root_folder: String,
        database_name: String,
        data_addr_port: String,
        info_addr_port: String,
        own_db_addr_port: String,
        proxy: TreatyProxy,
    ) -> Self {
        Self {
            root_folder,
            database_name,
            data_addr_port,
            info_addr_port,
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
    #[allow(dead_code, unused_variables)]
    async fn validate_auth_request(
        &self,
        request: &tonic::metadata::MetadataMap,
    ) -> Result<Box<dyn InfoServiceHandlerActions + Send>, AuthResult> {
        trace!("[{}]: metadata: {request:?}", function_name!());
        if request.contains_key(TREATY_AUTH_HEADER_METADATA_BIN) {
            return self.handle_auth(request).await;
        }

        warn!("[{}]: No metadata provided", function_name!());

        Err(AuthResult {
            is_authenticated: false,
            message: Some("No metadata provided".to_string()),
        })
    }

    async fn handle_auth(
        &self,
        request: &tonic::metadata::MetadataMap,
    ) -> Result<Box<dyn InfoServiceHandlerActions + Send>, AuthResult> {
        let kv = request.get_bin(TREATY_AUTH_HEADER_METADATA_BIN);
        if let Some(key_value) = kv {
            let bytes = key_value.to_bytes().unwrap();
            let metadata: AuthRequestMetadata = bincode::deserialize(&bytes).unwrap();

            if metadata.id.is_some() {
                let id = metadata.id.unwrap();
                let result_has_core = self.proxy.get_treaty_info_handler(&id).await;

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
            message: Some("No header metadata provided provided".to_string()),
        })
    }
}

#[tonic::async_trait]
impl InfoService for ProxyInfoServiceHandlerGrpc {
    async fn is_online(
        &self,
        _request: tonic::Request<TestRequest>,
    ) -> Result<tonic::Response<TestReply>, tonic::Status> {
        todo!()
    }

    async fn try_auth_web_token(
        &self,
        request: tonic::Request<AuthRequestWebToken>,
    ) -> Result<Response<TryAuthResult>, tonic::Status> {
        debug!("Request from {:?}", request.remote_addr());
        trace!("[{}]: request: {request:?}", function_name!());
        let auth_result = self.validate_auth_request(request.metadata()).await;

        match auth_result {
            Ok(core) => {
                let request = request.into_inner();
                let response = core.try_auth_web_token(request).await;
                return Ok(Response::new(response));
            }
            Err(e) => {
                error!("{e:?}");
                let reply = TryAuthResult {
                    is_authenticated: false,
                };
                return Ok(Response::new(reply));
            }
        }
    }

    async fn auth_for_token(
        &self,
        request: tonic::Request<AuthRequestBasic>,
    ) -> Result<Response<TokenReply>, tonic::Status> {
        debug!("Request from {:?}", request.remote_addr());
        trace!("[{}]: request: {request:?}", function_name!());
        let auth_result = self.validate_auth_request(request.metadata()).await;

        match auth_result {
            Ok(core) => {
                let request = request.into_inner();
                let response = core.auth_for_token(request).await;
                return Ok(Response::new(response));
            }
            Err(e) => {
                error!("{e:?}");
                let reply = TokenReply {
                    is_successful: false,
                    expiration_utc: String::from(""),
                    jwt: String::from(""),
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
        trace!("[{}]: request: {request:?}", function_name!());
        let auth_result = self.validate_auth_request(request.metadata()).await;

        match auth_result {
            Ok(core) => {
                let request = request.into_inner();
                let response = core.save_contract(request).await;
                return Ok(Response::new(response));
            }
            Err(e) => {
                error!("{e:?}");
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
        let auth_result = self.validate_auth_request(&request.metadata()).await;
        let request = request.into_inner().clone();

        match auth_result {
            Ok(core) => {
                let response = core.accept_contract(request).await;
                return Ok(Response::new(response));
            }
            Err(e) => {
                error!("[{}]: {}", function_name!(), e.message());
                let reply = ParticipantAcceptsContractResult {
                    contract_acceptance_is_acknowledged: false,
                    is_error: true,
                    error: None,
                };

                return Ok(Response::new(reply));
            }
        }
    }

    async fn ports_available(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<TreatyPorts>, tonic::Status> {
        todo!()
    }
}
