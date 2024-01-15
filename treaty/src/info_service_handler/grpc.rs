use std::net::SocketAddr;

use super::InfoServiceHandler;
use crate::{
    db_interface::dbi_actions::DbiActions,
    defaults,
    models::CoopDatabaseParticipant,
    settings::Settings,
    treaty_proto::{
        info_service_server::{InfoService, InfoServiceServer},
        AuthRequestBasic, AuthRequestWebToken, ParticipantAcceptsContractRequest,
        ParticipantAcceptsContractResult, SaveContractRequest, SaveContractResult, TestReply,
        TestRequest, TokenReply, TreatyPorts, TryAuthResult,
    },
};
use async_trait::async_trait;
use chrono::Utc;
use stdext::function_name;
use tonic::{
    transport::{Identity, Server, ServerTlsConfig},
    Request, Response, Status,
};
use tracing::{debug, error, trace};
use treaty_types::enums::ContractStatus;
use triggered::Listener;

#[async_trait]
impl<D: DbiActions + Clone + Sync + Send + 'static> InfoService for InfoServiceHandler<D> {
    async fn is_online(
        &self,
        request: Request<TestRequest>,
    ) -> Result<Response<TestReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let item = request.into_inner().request_echo_message;

        trace!("is_online, requested echo: {item}");

        Ok(Response::new(TestReply {
            reply_time_utc: Utc::now().to_rfc2822(),
            reply_echo_message: item,
            treaty_version: defaults::VERSION.to_string(),
        }))
    }

    async fn try_auth_web_token(
        &self,
        request: Request<AuthRequestWebToken>,
    ) -> Result<Response<TryAuthResult>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.try_auth_web_token(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn auth_for_token(
        &self,
        request: Request<AuthRequestBasic>,
    ) -> Result<Response<TokenReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.auth_for_token(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn accept_contract(
        &self,
        request: Request<ParticipantAcceptsContractRequest>,
    ) -> Result<Response<ParticipantAcceptsContractResult>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.accept_contract(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn save_contract(
        &self,
        request: Request<SaveContractRequest>,
    ) -> Result<Response<SaveContractResult>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.save_contract(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn ports_available(&self, request: Request<()>) -> Result<Response<TreatyPorts>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.ports_available().await;
        Ok(Response::new(response))
    }
}

impl<D: DbiActions + Clone + Sync + Send> InfoServiceHandler<D> {
    pub fn new(db: D, settings: Settings) -> Self {
        Self { db: db, settings }
    }

    #[tokio::main]
    pub async fn start_grpc(&self, shutdown: Listener) -> Result<(), Box<dyn std::error::Error>> {
        trace!("{}", self.settings.grpc_info_service_addr_port);
        let addr: SocketAddr = self.settings.grpc_info_service_addr_port.parse().unwrap();

        // let info_client_service = tonic_reflection::server::Builder::configure()
        //     .build()
        //     .unwrap();

        debug!("info service gprc listening on {addr}");

        if self.settings.use_tls {
            let settings = self.settings.tls_options.as_ref().unwrap().clone();

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
                .add_service(InfoServiceServer::new(self.clone()));
            router.serve_with_shutdown(addr, shutdown).await?;
        } else {
            Server::builder()
                .add_service(InfoServiceServer::new(self.clone()))
                // .add_service(info_client_service) // Add this
                .serve_with_shutdown(addr, shutdown)
                .await?;
        }

        Ok(())
    }

    pub async fn try_auth_web_token(&self, request: AuthRequestWebToken) -> TryAuthResult {
        trace!("{request:?}");

        let result = self.db.verify_token(&request.jwt).await;

        match result {
            Ok(is_authenticated) => TryAuthResult { is_authenticated },
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                TryAuthResult {
                    is_authenticated: false,
                }
            }
        }
    }

    pub async fn auth_for_token(&self, request: AuthRequestBasic) -> TokenReply {
        trace!("{request:?}");

        if self
            .db
            .verify_login(&request.user_name, &request.pw)
            .await
            .unwrap()
        {
            let result = self
                .db
                .create_token_for_login(
                    &request.user_name,
                    Some(self.settings.jwt_valid_time_in_minutes),
                )
                .await;
            match result {
                Ok(res) => {
                    return TokenReply {
                        is_successful: true,
                        expiration_utc: res.1.to_rfc3339(),
                        jwt: res.0,
                    }
                }
                Err(e) => {
                    error!("[{}]: {e:?}", function_name!());
                }
            }
        }

        TokenReply {
            is_successful: false,
            expiration_utc: String::from(""),
            jwt: String::from(""),
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
            .await
            .unwrap();

        if accepted_participant.is_none() {
            let _participant = self
                .db
                .get_participant_by_id(
                    &request.database_name,
                    &request.participant.as_ref().unwrap().participant_guid,
                )
                .await
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
            .await
            .unwrap();

        ParticipantAcceptsContractResult {
            contract_acceptance_is_acknowledged: is_successful,
            is_error: false,
            error: None,
        }
    }

    pub async fn save_contract(&self, request: SaveContractRequest) -> SaveContractResult {
        trace!("[{}]: {request:?}", function_name!());
        let contract = request.contract.unwrap();
        let result = self.db.save_contract(contract).await;
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

    pub async fn ports_available(&self) -> TreatyPorts {
        let mut ports = TreatyPorts::default();

        if self.settings.send_data_port_number {
            let parts = self.settings.grpc_data_service_addr_port.split(':');
            let parts = parts.collect::<Vec<&str>>();
            let port = parts.last().unwrap().to_string();
            let port = port.parse::<u32>().unwrap();
            ports.data_port = Some(port);
        }

        if self.settings.send_user_port_number {
            let parts = self.settings.grpc_user_service_addr_port.split(':');
            let parts = parts.collect::<Vec<&str>>();
            let port = parts.last().unwrap().to_string();
            let port = port.parse::<u32>().unwrap();
            ports.user_port = Some(port);
        }

        if self.settings.send_info_port_number {
            let parts = self.settings.grpc_info_service_addr_port.split(':');
            let parts = parts.collect::<Vec<&str>>();
            let port = parts.last().unwrap().to_string();
            let port = port.parse::<u32>().unwrap();
            ports.info_port = Some(port);
        }

        ports
    }
}
