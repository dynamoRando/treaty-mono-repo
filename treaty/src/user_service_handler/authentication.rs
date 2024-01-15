use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use stdext::function_name;
use tonic::{Request, Status};

use tonic_async_interceptor::AsyncInterceptor;

use tower::Service;
use tracing::{trace, warn};
use treaty_http_endpoints::headers::{
    TREATY_AUTH_HEADER, TREATY_AUTH_HEADER_AUTHOR_BIN, TREATY_AUTH_HEADER_BASIC,
    TREATY_AUTH_HEADER_BASIC_BIN, TREATY_AUTH_HEADER_BIN, TREATY_AUTH_HEADER_METADATA_BIN,
    TREATY_AUTH_HEADER_WEB_TOKEN, TREATY_AUTH_HEADER_WEB_TOKEN_BIN,
};
use treaty_types::{
    auth::AuthRequestData,
    enums::{self, AuthRequestMethod},
    types::treaty_proto::{
        AuthRequestAuthor, AuthRequestBasic, AuthRequestBinary, AuthRequestMetadata,
        AuthRequestWebToken,
    },
};

use crate::{db_interface::dbi_actions::DbiActions, user_service_handler::do_vecs_match};

/// An authenticator for incoming gRPC or HTTP requests.
#[derive(Debug, Clone)]
pub struct Authenticator<D: DbiActions + Clone + Send + Sync> {
    db: D,
}

impl<D: DbiActions + Clone + Send + Sync> Authenticator<D> {
    pub fn new(db: D) -> Self {
        Self { db }
    }

    pub async fn validate(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        trace!("[{}]: metadata: {request:?}", function_name!());
        let request_data = get_auth_data_from_grpc_request(&request);
        let is_authenticated = self.authenticate(&request_data).await;

        if is_authenticated {
            return Ok(request);
        }

        Err(Status::unauthenticated(
            "Could not authenticate using Treaty native authentication",
        ))
    }

    /// Authenticates a Treaty Host
    pub async fn authenticate_host(&self, request: &AuthRequestBinary) -> bool {
        let mut is_authenticated = false;

        let host_id = &request.user_name;
        let host_token = &request.token;

        if self
            .db
            .verify_host_by_id(host_id, host_token.to_vec())
            .await
            .unwrap()
        {
            is_authenticated = true;
        }

        if self
            .db
            .verify_host_by_name(host_id, host_token.to_vec())
            .await
            .unwrap()
        {
            is_authenticated = true;
        }

        is_authenticated
    }

    /// Authenticates a participant who is reporting actions back to us such as a remote UPDATE or DELETE
    pub async fn authenticate_participant(
        &self,
        request: &AuthRequestBinary,
        metadata: &AuthRequestMetadata,
    ) -> bool {
        let host_id = &request.user_name;
        let host_token = &request.token;

        let opt_db = metadata.db_name.clone();

        trace!("[{}]: Participant username: {host_id:?}", function_name!());
        trace!("[{}]: Participant db_name: {opt_db:?}", function_name!());

        if let Some(db_name) = &metadata.db_name {
            let participant = self
                .db
                .get_participant_by_alias(db_name, host_id)
                .await
                .unwrap();

            match participant {
                Some(p) => {
                    let is_auth = do_vecs_match(&p.token, host_token);
                    return is_auth;
                }
                None => return false,
            }
        }

        trace!(
            "[{}]: No database name provided to verify against.",
            function_name!()
        );

        false
    }

    /// Helper function that inspects the AuthRequestData and returns the first
    /// authentication type passed
    pub fn authentication_type(request: &AuthRequestData) -> AuthRequestMethod {
        if request
            .headers
            .contains(&TREATY_AUTH_HEADER_BASIC_BIN.to_string())
            || request
                .headers
                .contains(&TREATY_AUTH_HEADER_BASIC.to_string())
        {
            trace!("[{}]: Found basic bin authentication.", function_name!());
            return AuthRequestMethod::BasicBinary;
        }

        if request
            .headers
            .contains(&TREATY_AUTH_HEADER_WEB_TOKEN_BIN.to_string())
            || request
                .headers
                .contains(&TREATY_AUTH_HEADER_WEB_TOKEN.to_string())
        {
            trace!(
                "[{}]: Found web token bin authentication.",
                function_name!()
            );
            return AuthRequestMethod::WebTokenBinary;
        }

        if request
            .headers
            .contains(&TREATY_AUTH_HEADER_BIN.to_string())
            || request.headers.contains(&TREATY_AUTH_HEADER.to_string())
        {
            trace!("[{}]: Found binary bin authentication.", function_name!());
            return AuthRequestMethod::BinaryTokenBinary;
        }

        warn!("[{}]: Unknown authentication.", function_name!());
        AuthRequestMethod::Unknown
    }

    pub async fn authenticate(&self, request_data: &AuthRequestData) -> bool {
        match Self::authentication_type(request_data) {
            AuthRequestMethod::Unknown => {
                warn!("[{}]: Could not identify author.", function_name!());
                return false;
            }
            // Usual type is from a User
            AuthRequestMethod::BasicBinary => {
                match enums::AuthRequestAuthor::from_u32(
                    request_data.author.as_ref().unwrap().author_type,
                ) {
                    enums::AuthRequestAuthor::Unknown => {
                        warn!("[{}]: Could not identify author.", function_name!());
                        return false;
                    }
                    enums::AuthRequestAuthor::User => {
                        let option_auth = &request_data.basic;
                        if let Some(auth) = option_auth {
                            if self
                                .db
                                .verify_login(&auth.user_name, &auth.pw)
                                .await
                                .unwrap()
                            {
                                trace!(
                                    "[{}]: User is authenticated using basic authentication.",
                                    function_name!()
                                );
                                return true;
                            }
                        }
                    }
                    enums::AuthRequestAuthor::Data => todo!(),
                    enums::AuthRequestAuthor::Participant => todo!(),
                }
            }
            // Usual type is from a User
            AuthRequestMethod::WebTokenBinary => {
                match enums::AuthRequestAuthor::from_u32(
                    request_data.author.as_ref().unwrap().author_type,
                ) {
                    enums::AuthRequestAuthor::Unknown => {
                        warn!("[{}]: Could not identify author.", function_name!());
                        return false;
                    }
                    enums::AuthRequestAuthor::User => {
                        trace!("[{}]: Trying user as author...", function_name!());
                        let option_auth = &request_data.web_token;
                        if let Some(auth) = option_auth {
                            if self.db.verify_token(&auth.jwt).await.unwrap() {
                                trace!(
                                    "[{}]: User is authenticated using web token authentication.",
                                    function_name!()
                                );
                                return true;
                            }
                        }
                    }
                    enums::AuthRequestAuthor::Data => todo!(),
                    enums::AuthRequestAuthor::Participant => todo!(),
                }
            }
            // Usual type is from either a Data (Host) service or a participant of a database.
            // We will need to inspet the author to determine if we should query the `Treaty` database
            // or one of the host databases for authentication purposes.
            AuthRequestMethod::BinaryTokenBinary => {
                match enums::AuthRequestAuthor::from_u32(
                    request_data.author.as_ref().unwrap().author_type,
                ) {
                    enums::AuthRequestAuthor::Unknown => {
                        warn!("[{}]: Could not identify author.", function_name!());
                        return false;
                    }
                    enums::AuthRequestAuthor::User => {
                        todo!()
                    }
                    enums::AuthRequestAuthor::Data => {
                        trace!("[{}]: Trying data as author...", function_name!());
                        let option_auth = request_data.binary.clone();
                        if let Some(auth) = option_auth {
                            if self.authenticate_host(&auth).await {
                                trace!(
                                    "[{}]: Data/Host is authenticated using bin authentication.",
                                    function_name!()
                                );
                                return true;
                            }
                        }
                    }
                    enums::AuthRequestAuthor::Participant => {
                        trace!("[{}]: Trying participant as author...", function_name!());
                        let option_auth = request_data.binary.clone();
                        if let Some(auth) = option_auth {
                            if let Some(metadata) = request_data.metadata.clone() {
                                if self.authenticate_participant(&auth, &metadata).await {
                                    trace!("[{}]: Participant is authenticated using bin authentication.", function_name!());
                                    return true;
                                }
                            } else {
                                trace!("[{}]: Participant has NO metadata.", function_name!());
                            }
                        } else {
                            trace!(
                                "[{}]: Participant has NO binary authentication.",
                                function_name!()
                            );
                        }
                    }
                }
            }
        }

        warn!(
            "[{}]: Could not authenticate using Treaty native authentication.",
            function_name!()
        );
        false
    }
}

/// Takes a gRPC request and creates a AuthRequestData struct from it
fn get_auth_data_from_grpc_request(request: &Request<()>) -> AuthRequestData {
    let mut request_data: AuthRequestData = AuthRequestData {
        headers: Vec::new(),
        basic: None,
        web_token: None,
        binary: None,
        metadata: None,
        author: None,
    };

    let kv = request.metadata().get_bin(TREATY_AUTH_HEADER_BASIC_BIN);
    if let Some(key_value) = kv {
        request_data
            .headers
            .push(TREATY_AUTH_HEADER_BASIC_BIN.to_string());
        let bytes = key_value.to_bytes().unwrap();
        let auth: AuthRequestBasic = bincode::deserialize(&bytes).unwrap();
        request_data.basic = Some(auth);
    } else {
        trace!("[{}]: No Basic Auth header found", function_name!());
    }

    let kv = request.metadata().get_bin(TREATY_AUTH_HEADER_WEB_TOKEN_BIN);
    if let Some(key_value) = kv {
        request_data
            .headers
            .push(TREATY_AUTH_HEADER_WEB_TOKEN_BIN.to_string());
        let bytes = key_value.to_bytes().unwrap();
        let auth: AuthRequestWebToken = bincode::deserialize(&bytes).unwrap();
        request_data.web_token = Some(auth);
    } else {
        trace!("[{}]: No Web Token header found", function_name!());
    }

    let kv = request.metadata().get_bin(TREATY_AUTH_HEADER_BIN);
    if let Some(key_value) = kv {
        request_data
            .headers
            .push(TREATY_AUTH_HEADER_BIN.to_string());
        let bytes = key_value.to_bytes().unwrap();
        let auth: AuthRequestBinary = bincode::deserialize(&bytes).unwrap();
        request_data.binary = Some(auth);
    } else {
        trace!("[{}]: No Bin header found", function_name!());
    }

    let kv = request.metadata().get_bin(TREATY_AUTH_HEADER_AUTHOR_BIN);
    if let Some(key_value) = kv {
        request_data
            .headers
            .push(TREATY_AUTH_HEADER_AUTHOR_BIN.to_string());
        let bytes = key_value.to_bytes().unwrap();
        let auth: AuthRequestAuthor = bincode::deserialize(&bytes).unwrap();
        request_data.author = Some(auth);
    } else {
        trace!("[{}]: No Author header found", function_name!());
    }

    let kv = request.metadata().get_bin(TREATY_AUTH_HEADER_METADATA_BIN);
    if let Some(key_value) = kv {
        request_data
            .headers
            .push(TREATY_AUTH_HEADER_METADATA_BIN.to_string());
        let bytes = key_value.to_bytes().unwrap();
        let auth: AuthRequestMetadata = bincode::deserialize(&bytes).unwrap();
        request_data.metadata = Some(auth);
    } else {
        trace!("[{}]: No Metadata header found", function_name!());
    }

    request_data
}

/*
impl<D: DbiActions + Clone + Send + Sync> Interceptor for Authenticator<D> {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        trace!("[{}]: metadata: {request:?}", function_name!());
        let request_data = get_auth_data_from_grpc_request(&request);
        let is_authenticated = self.authenticate(&request_data).await;

        if is_authenticated {
            return Ok(request);
        }

        Err(Status::unauthenticated(
            "Could not authenticate using Treaty native authentication",
        ))
    }
}
*/

impl<D: DbiActions + Clone + Send + Sync + 'static> AsyncInterceptor for Authenticator<D> {
    type Future = Pin<Box<dyn Future<Output = Result<Request<()>, Status>> + Send>>;
    // type Future = Pin<Box<dyn Future<Output = Result<Request<()>, Status>>>>;

    fn call(&mut self, request: Request<()>) -> Self::Future {
        trace!("[{}]: metadata: {request:?}", function_name!());

        let request_data = get_auth_data_from_grpc_request(&request);
        let auth = self.clone();

        let future = async move {
            let is_authenticated = auth.authenticate(&request_data).await;
            if is_authenticated {
                Ok(request)
            } else {
                Err(Status::unauthenticated(
                    "Could not authenticate using Treaty native authentication",
                ))
            }
        };

        Box::pin(future)
    }
}

impl<D: DbiActions + Clone + Send + Sync + 'static> Service<Request<()>> for Authenticator<D> {
    type Response = Request<()>;
    type Error = Status;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<()>) -> Self::Future {
        let request_data = get_auth_data_from_grpc_request(&request);
        let auth = self.clone();

        let future = async move {
            let is_authenticated = auth.authenticate(&request_data).await;
            if is_authenticated {
                Ok(request)
            } else {
                Err(Status::unauthenticated(
                    "Could not authenticate using Treaty native authentication",
                ))
            }
        };

        // Return the response as an immediate future
        Box::pin(future)
    }
}
