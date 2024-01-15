use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use stdext::function_name;
use tonic::service::Interceptor;
use tonic::transport::Channel;
use tonic::Status;
use tonic::{
    codegen::InterceptedService,
    transport::{Certificate, ClientTlsConfig},
};
use tonic::{metadata::MetadataValue, Request};
use tracing::{error, trace, warn};
use treaty::treaty_proto::{
    info_service_client::InfoServiceClient, AuthRequestBasic, AuthRequestMetadata,
    AuthRequestWebToken, TokenReply,
};
use treaty_http_endpoints::headers::{
    TREATY_AUTH_HEADER_AUTHOR_BIN, TREATY_AUTH_HEADER_BASIC_BIN, TREATY_AUTH_HEADER_METADATA_BIN,
    TREATY_AUTH_HEADER_WEB_TOKEN_BIN,
};
use treaty_types::types::treaty_proto::AuthRequestAuthor;

use super::TlsSettings;

/// Sends authentication with each call to the user service.
#[derive(Debug, Clone)]
pub(crate) struct AuthenticationInterceptor {
    basic: AuthRequestBasic,
    web_token: Option<AuthRequestWebToken>,
    info_address_port: String,
    timeout_in_seconds: u64,
    host_id: Option<String>,
    tls: Option<TlsSettings>,
}

/// Intercepts all outgoing gRPC calls and appends the appropriate authentication needed
impl AuthenticationInterceptor {
    /// Creates a new interceptor to send authentication with each call to the User service.
    ///
    /// Upon construction, it will save Basic credentials in memory and attempt to
    /// get a token back to use for further requests. If the token authentication fails,
    /// it will fall back to basic.
    ///
    /// To make this work, we will need the location of the Info service to manage authentication.
    pub async fn new(
        username: &str,
        pw: &str,
        info_address_port: &str,
        timeout_in_seconds: u64,
        host_id: Option<String>,
        tls: Option<TlsSettings>,
    ) -> Self {
        let request = AuthRequestBasic {
            user_name: username.to_string(),
            pw: pw.to_string(),
        };

        let option_token = Self::get_token(
            request.clone(),
            info_address_port,
            timeout_in_seconds,
            host_id.clone(),
            tls.clone(),
        )
        .await;

        match option_token {
            None => {}
            Some(token_reply) => {
                if token_reply.is_successful {
                    let web_token = AuthRequestWebToken {
                        jwt: token_reply.jwt,
                    };

                    return Self {
                        basic: request.clone(),
                        web_token: Some(web_token),
                        info_address_port: info_address_port.to_string(),
                        timeout_in_seconds: timeout_in_seconds,
                        host_id: host_id.clone(),
                        tls: tls.clone(),
                    };
                }
            }
        }

        Self {
            basic: request.clone(),
            web_token: None,
            info_address_port: info_address_port.to_string(),
            timeout_in_seconds: timeout_in_seconds,
            host_id: host_id.clone(),
            tls: tls.clone(),
        }
    }

    async fn get_token(
        auth_basic: AuthRequestBasic,
        info_address_port: &str,
        timeout_in_seconds: u64,
        host_id: Option<String>,
        tls: Option<TlsSettings>,
    ) -> Option<TokenReply> {
        let mut client =
            Self::get_info_client(info_address_port, timeout_in_seconds, host_id, tls).await;

        let result = client.auth_for_token(auth_basic.clone()).await;
        match result {
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
            }
            Ok(response) => {
                return Some(response.into_inner());
            }
        }

        None
    }

    async fn validate_token(
        web_token: Option<AuthRequestWebToken>,
        info_address_port: &str,
        timeout_in_seconds: u64,
        host_id: Option<String>,
        tls: Option<TlsSettings>,
    ) -> bool {
        let mut client =
            Self::get_info_client(info_address_port, timeout_in_seconds, host_id, tls).await;

        if let Some(web_token) = web_token {
            let result = client.try_auth_web_token(web_token.clone()).await;
            trace!("[{}]: {result:?}", function_name!());
            match result {
                Err(e) => {
                    error!("[{}]: {e:?}", function_name!());
                }
                Ok(response) => {
                    let auth_result = response.into_inner();
                    trace!("[{}]: {auth_result:?}", function_name!());
                    return auth_result.is_authenticated;
                }
            }
        }

        false
    }

    /// Returns a regular info client
    async fn get_info_client(
        info_addr_port: &str,
        timeout_in_seconds: u64,
        host_id: Option<String>,
        tls: Option<TlsSettings>,
    ) -> InfoServiceClient<
        InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
    > {
        let channel = match tls {
            Some(settings) => {
                let ca = Certificate::from_pem(settings.pem);

                let tls = match settings.domain {
                    Some(domain) => ClientTlsConfig::new()
                        .ca_certificate(ca)
                        .domain_name(domain),
                    None => ClientTlsConfig::new().ca_certificate(ca),
                };

                Channel::builder(info_addr_port.parse().unwrap())
                    .tls_config(tls)
                    .unwrap()
                    .timeout(Duration::from_secs(timeout_in_seconds.into()))
                    .connect()
                    .await
                    .unwrap()
            }
            None => {
                let endpoint = tonic::transport::Channel::builder(info_addr_port.parse().unwrap())
                    .timeout(Duration::from_secs(timeout_in_seconds.into()));
                let channel = endpoint.connect().await.unwrap();
                channel
            }
        };

        let host_id = host_id.clone();
        let client = InfoServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
            let metadata = AuthRequestMetadata {
                id: host_id.clone(),
                db_name: None,
            };

            let bin = bincode::serialize(&metadata.clone()).unwrap();
            let bin = MetadataValue::from_bytes(&bin);
            req.metadata_mut()
                .append_bin(TREATY_AUTH_HEADER_METADATA_BIN, bin);

            Ok(req)
        });

        client
    }

    /// Checks to see if the current web token is valid and
    /// if it isn't, it will attempt to fetch a new one and assign it
    /// to ourselves
    fn refresh_token(&mut self) {
        trace!("[{}]: Refreshing token...", function_name!());

        let auth_basic = self.basic.clone();
        let opt_web_token = self.web_token.clone();
        let address_port = self.info_address_port.clone();
        let timeout = self.timeout_in_seconds;

        let arc_token_reply: Option<TokenReply> = None;
        let arc_token_reply = Arc::new(Mutex::new(arc_token_reply));

        {
            let arc_token_reply = arc_token_reply.clone();
            let host_id = self.host_id.clone();
            let tls_ops = self.tls.clone();
            tokio::spawn(async move {
                if !Self::validate_token(
                    opt_web_token,
                    &address_port,
                    timeout,
                    host_id.clone(),
                    tls_ops.clone(),
                )
                .await
                {
                    let option_token_reply = Self::get_token(
                        auth_basic,
                        &address_port,
                        timeout,
                        host_id.clone(),
                        tls_ops.clone(),
                    )
                    .await;

                    *arc_token_reply.lock().unwrap() = option_token_reply;
                }
            });
        }

        if arc_token_reply.lock().unwrap().is_some() {
            let option_token_reply = arc_token_reply.lock().unwrap().clone();
            if let Some(token_reply) = option_token_reply {
                if token_reply.is_successful {
                    let web_token = AuthRequestWebToken {
                        jwt: token_reply.jwt.clone(),
                    };

                    trace!("[{}]: Setting new token...", function_name!());
                    self.web_token = Some(web_token);
                }
            }
        }
    }

    fn request_using_web(
        &self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, Status> {
        let bin_web_token = bincode::serialize(&self.web_token.as_ref().unwrap().clone()).unwrap();
        let bin_web_token = MetadataValue::from_bytes(&bin_web_token);

        let requestor = AuthRequestAuthor { author_type: 1 };

        let bin_author = bincode::serialize(&requestor).unwrap();
        let bin_author = MetadataValue::from_bytes(&bin_author);

        let metadata = AuthRequestMetadata {
            id: self.host_id.clone(),
            db_name: None,
        };

        let bin_metadata = bincode::serialize(&metadata).unwrap();
        let bin_metadata = MetadataValue::from_bytes(&bin_metadata);

        request
            .metadata_mut()
            .append_bin(TREATY_AUTH_HEADER_METADATA_BIN, bin_metadata);

        request
            .metadata_mut()
            .append_bin(TREATY_AUTH_HEADER_WEB_TOKEN_BIN, bin_web_token);

        request
            .metadata_mut()
            .append_bin(TREATY_AUTH_HEADER_AUTHOR_BIN, bin_author);

        trace!(
            "[{}]: Sending request using web token auth...",
            function_name!()
        );
        Ok(request)
    }

    fn request_using_basic(
        &self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, Status> {
        let bin_basic_token = bincode::serialize(&self.basic.clone()).unwrap();
        let bin_basic_token = MetadataValue::from_bytes(&bin_basic_token);

        let requestor = AuthRequestAuthor { author_type: 1 };

        let bin_author = bincode::serialize(&requestor).unwrap();
        let bin_author = MetadataValue::from_bytes(&bin_author);

        let metadata = AuthRequestMetadata {
            id: self.host_id.clone(),
            db_name: None,
        };

        let bin_metadata = bincode::serialize(&metadata).unwrap();
        let bin_metadata = MetadataValue::from_bytes(&bin_metadata);

        request
            .metadata_mut()
            .append_bin(TREATY_AUTH_HEADER_METADATA_BIN, bin_metadata);

        request
            .metadata_mut()
            .append_bin(TREATY_AUTH_HEADER_BASIC_BIN, bin_basic_token);

        request
            .metadata_mut()
            .append_bin(TREATY_AUTH_HEADER_AUTHOR_BIN, bin_author);

        trace!(
            "[{}]: Sending request using basic auth...",
            function_name!()
        );
        Ok(request)
    }

    pub fn basic_auth(&self) -> AuthRequestBasic {
        self.basic.clone()
    }

    pub fn web_token(&self) -> Option<AuthRequestWebToken> {
        self.web_token.clone()
    }
}

impl Interceptor for AuthenticationInterceptor {
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        self.refresh_token();
        trace!("[{}]: {request:?}", function_name!());

        if self.web_token.is_none() {
            warn!(
                "[{}]: Web Token is not available, falling back to basic auth",
                function_name!()
            );

            return self.request_using_basic(request);
        } else {
            return self.request_using_web(request);
        };
    }
}
