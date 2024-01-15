use super::remote_actions::RemoteActions;
use crate::{
    error::LogErrorActions,
    models::{
        CdsHosts, CoopDatabaseContract, CoopDatabaseParticipant, CoopDatabaseParticipantData,
        DataInfo, HostInfo, TreatySaveContractResult,
    },
    settings::TlsServerClientSettings,
    treaty_proto::{
        data_service_client::DataServiceClient, info_service_client::InfoServiceClient,
        AuthRequestAuthor, AuthRequestBinary, AuthRequestMetadata, Contract, DatabaseSchema,
        DeleteDataRequest, DeleteDataResult, GetRowFromPartialDatabaseRequest,
        GetRowFromPartialDatabaseResult, Host, InsertDataRequest, InsertDataResult,
        NotifyHostOfRemovedRowRequest, Participant, ParticipantAcceptsContractRequest,
        RowParticipantAddress, SaveContractRequest, Telemetry, TreatyPorts, UpdateDataRequest,
        UpdateDataResult, UpdateRowDataHashForHostRequest,
    },
};
use async_trait::async_trait;
use chrono::Utc;
use endianness::{read_i32, ByteOrder};
use guid_create::GUID;
use std::time::Duration;
use stdext::function_name;
use tonic::Request;
use tonic::Status;
use tonic::{
    codegen::InterceptedService,
    transport::{Certificate, ClientTlsConfig},
};
use tonic::{metadata::MetadataValue, transport::Channel};
use tracing::{debug, error, trace};
use treaty_http_endpoints::headers::{
    TREATY_AUTH_HEADER_AUTHOR_BIN, TREATY_AUTH_HEADER_BIN, TREATY_AUTH_HEADER_METADATA_BIN,
};
use treaty_types::enums::*;

#[derive(Debug, Clone)]
pub struct RemoteGrpc {
    db_addr_port: String,
    info_addr_port: String,
    timeout_in_seconds: u32,
    tls_options: Option<TlsOptions>,
}

#[derive(Debug, Clone)]
pub struct RemoteGrpcTlsClientSettings {
    pub tls_client_cert_path: String,
    pub tls_client_domain: Option<String>,
}

impl From<TlsServerClientSettings> for RemoteGrpcTlsClientSettings {
    fn from(settings: TlsServerClientSettings) -> Self {
        let domain: Option<String> = if settings.tls_client_domain.is_empty() {
            None
        } else {
            Some(settings.tls_client_domain)
        };

        Self {
            tls_client_cert_path: settings.tls_client_cert_path.unwrap(),
            tls_client_domain: domain,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TlsOptions {
    pub tls_client: String,
    pub tls_client_domain: Option<String>,
}

impl From<RemoteGrpcTlsClientSettings> for TlsOptions {
    fn from(settings: RemoteGrpcTlsClientSettings) -> Self {
        let pem = std::fs::read_to_string(settings.tls_client_cert_path).unwrap();

        Self {
            tls_client: pem,
            tls_client_domain: settings.tls_client_domain,
        }
    }
}

impl From<TlsServerClientSettings> for TlsOptions {
    fn from(settings: TlsServerClientSettings) -> Self {
        let pem = std::fs::read_to_string(settings.tls_client_cert_path.unwrap()).unwrap();

        Self {
            tls_client: pem,
            tls_client_domain: Some(settings.tls_client_domain),
        }
    }
}

impl RemoteGrpc {
    pub fn new(
        db_addr_port: &str,
        info_addr_port: &str,
        timeout_in_seconds: u32,
        tls_options: Option<RemoteGrpcTlsClientSettings>,
    ) -> Self {
        match tls_options {
            Some(tls_settings) => {
                let pem = std::fs::read_to_string(tls_settings.tls_client_cert_path).unwrap();

                let tls_options = TlsOptions {
                    tls_client: pem,
                    tls_client_domain: tls_settings.tls_client_domain,
                };

                Self {
                    db_addr_port: db_addr_port.to_string(),
                    info_addr_port: info_addr_port.to_string(),
                    timeout_in_seconds,
                    tls_options: Some(tls_options),
                }
            }
            None => Self {
                db_addr_port: db_addr_port.to_string(),
                info_addr_port: info_addr_port.to_string(),
                timeout_in_seconds,
                tls_options: None,
            },
        }
    }

    pub fn addr(&self) -> String {
        let parts = self.db_addr_port.split(':');
        let parts = parts.collect::<Vec<&str>>();
        return parts.first().unwrap().to_string();
    }

    pub fn db_port(&self) -> u32 {
        let parts = self.db_addr_port.split(':');
        let parts = parts.collect::<Vec<&str>>();
        let port = parts.last().unwrap().to_string();
        port.parse::<u32>().unwrap()
    }

    pub fn info_port(&self) -> u32 {
        let parts = self.info_addr_port.split(':');
        let parts = parts.collect::<Vec<&str>>();
        let port = parts.last().unwrap().to_string();
        port.parse::<u32>().unwrap()
    }
}

#[async_trait]
impl RemoteActions for RemoteGrpc {
    async fn try_auth_at_participant(
        &self,
        participant: CoopDatabaseParticipant,
        own_host_info: &HostInfo,
    ) -> bool {
        let mut client = get_data_service_client(
            participant,
            self.timeout_in_seconds,
            own_host_info,
            None,
            false,
            self.tls_options.clone(),
        )
        .await;
        let response = client.try_auth(()).await;

        trace!("[{}]: {response:?}", function_name!());

        match response {
            Ok(response) => {
                let result = response.into_inner();
                return result.is_authenticated;
            }
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
            }
        }

        false
    }

    async fn send_participant_contract(
        &self,
        participant: CoopDatabaseParticipant,
        host_info: HostInfo,
        contract: CoopDatabaseContract,
        db_schema: DatabaseSchema,
    ) -> TreatySaveContractResult {
        let telemetry = get_telemetry(&host_info, self.db_addr_port.clone());

        let contract = contract.convert_to_protobuf(
            &host_info,
            db_schema,
            ContractStatus::Pending,
            &self.addr(),
            self.db_port(),
            self.info_port(),
        );

        let request = tonic::Request::new(SaveContractRequest {
            contract: Some(contract),
            telemetry: Some(telemetry),
            id: Some(participant.id.to_string()),
        });

        trace!("send_participant_contract: {request:?}");

        let mut addr_port = format!("{}{}", participant.ip4addr, participant.info_port);

        if !addr_port.contains(':') {
            addr_port = format!("{}:{}", participant.ip4addr, participant.info_port);
            debug!("reformatted - sending request to Treaty at: {}", addr_port);
        }

        trace!("sending request to treaty at: {}", addr_port);

        let mut info_client = get_info_service_client(
            participant,
            self.timeout_in_seconds,
            self.tls_options.clone(),
        )
        .await;
        let response = info_client
            .save_contract(request)
            .await
            .unwrap()
            .into_inner();
        response.log_any_err(function_name!());

        let is_saved = response.is_saved;
        let contract_status = response.contract_status;
        let participant_info = response.participant_info;

        TreatySaveContractResult {
            is_successful: is_saved,
            contract_status: ContractStatus::from_u32(contract_status),
            participant_information: participant_info,
        }
    }

    async fn notify_host_of_removed_row(
        &self,
        host: &CdsHosts,
        own_host_info: &HostInfo,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> bool {
        trace!("[{}]: {host:?}", function_name!());

        let message_info = get_telemetry(own_host_info, self.db_addr_port.clone());

        let chost = Host {
            host_guid: own_host_info.id.clone(),
            host_name: own_host_info.name.clone(),
            token: own_host_info.token.clone(),
            network: None,
        };

        let request = NotifyHostOfRemovedRowRequest {
            telemetry: Some(message_info),
            host_info: Some(chost),
            database_name: db_name.to_string(),
            database_id: String::from(""),
            table_name: table_name.to_string(),
            table_id: 0,
            row_id,
        };

        trace!("[{}]: {request:?}", function_name!());

        let client = get_data_client_from_cds_host(
            host,
            Some(own_host_info.clone()),
            Some(host.host_id.clone()),
            Some(db_name.to_owned()),
            true,
            self.tls_options.clone(),
        );
        let response = client.await.notify_host_of_removed_row(request).await;
        let result = response.unwrap().into_inner();
        result.log_any_err(function_name!());
        result.is_successful
    }

    async fn remove_row_at_participant(
        &self,
        participant: CoopDatabaseParticipant,
        own_host_info: &HostInfo,
        db_name: &str,
        table_name: &str,
        sql: &str,
        where_clause: &str,
    ) -> DeleteDataResult {
        let request = DeleteDataRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            cmd: sql.to_string(),
            where_clause: where_clause.to_string(),
        };

        let client = get_data_service_client(
            participant,
            self.timeout_in_seconds,
            own_host_info,
            Some(db_name.to_owned()),
            false,
            self.tls_options.clone(),
        );
        let response = client
            .await
            .delete_command_into_table(request)
            .await
            .unwrap();

        let result = response.into_inner();
        result.log_any_err(function_name!());
        result
    }

    async fn update_row_at_participant(
        &self,
        participant: CoopDatabaseParticipant,
        own_host_info: &HostInfo,
        db_name: &str,
        table_name: &str,
        sql: &str,
        where_clause: &str,
    ) -> UpdateDataResult {
        let request = UpdateDataRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            cmd: sql.to_string(),
            where_clause: where_clause.to_string(),
        };

        let client = get_data_service_client(
            participant,
            self.timeout_in_seconds,
            own_host_info,
            Some(db_name.to_owned()),
            false,
            self.tls_options.clone(),
        );
        let response = client
            .await
            .update_command_into_table(request)
            .await
            .unwrap();

        trace!("[{}]: {:?}", function_name!(), response);

        let result = response.into_inner();
        result.log_any_err(function_name!());
        result
    }

    async fn insert_row_at_participant(
        &self,
        participant: CoopDatabaseParticipant,
        own_host_info: &HostInfo,
        db_name: &str,
        table_name: &str,
        sql: &str,
    ) -> InsertDataResult {
        let request = InsertDataRequest {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            cmd: sql.to_string(),
        };

        let client = get_data_service_client(
            participant,
            self.timeout_in_seconds,
            own_host_info,
            Some(db_name.to_owned()),
            false,
            self.tls_options.clone(),
        );
        let response = client
            .await
            .insert_command_into_table(request)
            .await
            .unwrap();

        let result = response.into_inner();
        result.log_any_err(function_name!());
        result
    }

    async fn get_row_from_participant(
        &self,
        participant: CoopDatabaseParticipantData,
        own_host_info: HostInfo,
        row_id: u32,
    ) -> GetRowFromPartialDatabaseResult {
        let telemetry = get_telemetry(&own_host_info, self.db_addr_port.clone());

        let row_address = RowParticipantAddress {
            database_name: participant.db_name.clone(),
            table_name: participant.table_name.clone(),
            row_id,
        };

        let request = GetRowFromPartialDatabaseRequest {
            row_address: Some(row_address),
            telemetry: Some(telemetry),
        };

        let participant_info = participant.participant.clone();

        let client = get_data_service_client(
            participant_info,
            self.timeout_in_seconds,
            &own_host_info,
            Some(participant.db_name.clone()),
            false,
            self.tls_options.clone(),
        );
        let response = client
            .await
            .get_row_from_partial_database(request)
            .await
            .unwrap();

        let result = response.into_inner();
        result.log_any_err(function_name!());
        result
    }

    async fn notify_host_of_updated_hash(
        &self,
        host: &CdsHosts,
        own_host_info: &HostInfo,
        data_info: &DataInfo,
    ) -> bool {
        // let auth = get_auth_request(own_host_info, Some(host.host_id.clone()));
        // debug!("notify_host_of_updated_hash::auth: {auth:?}");

        let telemetry = get_telemetry(own_host_info, self.db_addr_port.clone());

        let chost = Host {
            host_guid: own_host_info.id.clone(),
            host_name: own_host_info.name.clone(),
            token: own_host_info.token.clone(),
            network: None,
        };

        let hash_val = match data_info.hash {
            Some(_) => data_info.hash.unwrap(),
            None => 0,
        };

        if !data_info.is_deleted {
            let request = UpdateRowDataHashForHostRequest {
                telemetry: Some(telemetry),
                host_info: Some(chost),
                database_name: data_info.db_name.to_string(),
                database_id: String::from(""),
                table_name: data_info.table_name.to_string(),
                table_id: 0,
                row_id: data_info.row_id,
                updated_hash_value: hash_val,
                is_deleted_at_participant: data_info.is_deleted,
            };

            let client = get_data_client_from_cds_host(
                host,
                Some(own_host_info.clone()),
                Some(host.host_id.clone()),
                Some(data_info.db_name.clone()),
                true,
                self.tls_options.clone(),
            );

            let response = client.await.update_row_data_hash_for_host(request).await;
            let result = response.unwrap().into_inner();
            result.log_any_err(function_name!());
            result.is_successful
        } else {
            let request = NotifyHostOfRemovedRowRequest {
                telemetry: Some(telemetry),
                host_info: Some(chost),
                database_name: data_info.db_name.to_string(),
                database_id: String::from(""),
                table_name: data_info.table_name.to_string(),
                table_id: 0,
                row_id: data_info.row_id,
            };

            let client = get_data_client_from_cds_host(
                host,
                Some(own_host_info.clone()),
                Some(host.host_id.clone()),
                Some(data_info.db_name.to_string()),
                true,
                self.tls_options.clone(),
            );
            let response = client.await.notify_host_of_removed_row(request).await;
            let result = response.unwrap().into_inner();
            result.log_any_err(function_name!());
            result.is_successful
        }
    }

    async fn notify_host_of_acceptance_of_contract(
        &self,
        accepted_contract: &Contract,
        own_host_info: &HostInfo,
    ) -> bool {
        trace!("[{}]: {accepted_contract:?}", function_name!());

        let telemetry = get_telemetry(own_host_info, self.db_addr_port.clone());
        let host_info = accepted_contract.host_info.as_ref().unwrap().clone();

        if host_info.network.is_none() {
            return false;
        }

        let participant = Participant {
            participant_guid: own_host_info.id.clone(),
            alias: own_host_info.name.clone(),
            ip4_address: self.db_addr_port.clone(),
            ip6_address: String::from(""),
            database_port_number: 0,
            info_port_number: 0,
            token: own_host_info.token.clone(),
            internal_participant_guid: "".to_string(),
            http_addr: "".to_string(),
            http_port: 0,
        };

        let host_info = host_info.clone();

        let request = ParticipantAcceptsContractRequest {
            participant: Some(participant),
            contract_guid: accepted_contract.contract_guid.clone(),
            contract_version_guid: accepted_contract.contract_version.clone(),
            database_name: accepted_contract
                .schema
                .as_ref()
                .unwrap()
                .database_name
                .clone(),
            telemetry: Some(telemetry),
            id: Some(host_info.host_guid.clone()),
        };

        let host_info = host_info.clone();

        let ip4 = host_info
            .network
            .as_ref()
            .unwrap()
            .ip4_address
            .as_ref()
            .unwrap()
            .clone();
        let ip4 = format!(
            "{}:{}",
            ip4,
            &host_info
                .network
                .as_ref()
                .unwrap()
                .info_port_number
                .unwrap()
        );

        let message = format!("sending request to treaty at: {}", &ip4);
        trace!("[{}]: {}", function_name!(), message);

        let client = get_info_service_client_by_ip(
            ip4,
            self.timeout_in_seconds,
            own_host_info,
            Some(host_info.host_guid),
            self.tls_options.clone(),
        );
        // this used to be an unguarded call; we will need to move this call to the info service.
        let response = client.await.accept_contract(request).await.unwrap();

        let result = response.into_inner();
        result.log_any_err(function_name!());

        result.contract_acceptance_is_acknowledged
    }

    async fn get_remote_ports(&self, ip4addr: &str, info_port: u32) -> TreatyPorts {
        todo!("get_remote_ports")
    }
}

fn is_little_endian() -> bool {
    let v = vec![0, 128, 128, 0];

    match read_i32(&v, ByteOrder::LittleEndian) {
        Ok(_n) => true,
        Err(_err) => false,
    }
}

async fn get_data_client_from_cds_host(
    host: &CdsHosts,
    opt_own_host_info: Option<HostInfo>,
    host_id: Option<String>,
    db_name: Option<String>,
    send_as_participant: bool,
    tls_options: Option<TlsOptions>,
) -> DataServiceClient<
    InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
> {
    let addr_port = if !host.ip4.contains(':') {
        format!("{}:{}", host.ip4, host.db_port)
    } else {
        host.ip4.clone()
    };

    let http_addr_port = match tls_options {
        Some(_) => format!("{}{}", String::from("https://"), addr_port),
        None => format!("{}{}", String::from("http://"), addr_port),
    };

    trace!("configuring to connect to treaty from cds host at: {addr_port}");
    trace!("{http_addr_port}");

    let channel = match tls_options {
        Some(settings) => {
            let ca = Certificate::from_pem(settings.tls_client);

            let tls = match settings.tls_client_domain {
                Some(domain) => ClientTlsConfig::new()
                    .ca_certificate(ca)
                    .domain_name(domain),
                None => ClientTlsConfig::new().ca_certificate(ca),
            };

            Channel::builder(http_addr_port.parse().unwrap())
                .tls_config(tls)
                .unwrap()
                .connect()
                .await
                .unwrap()
        }
        None => {
            let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap());
            let channel = endpoint.connect().await.unwrap();
            channel
        }
    };

    let db_name = db_name.clone();

    let client = DataServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        if let Some(info) = opt_own_host_info.clone() {
            let db_name = db_name.clone();
            let auth_bin = get_auth_request_binary(&info, host_id.clone(), db_name.clone());
            let auth_meta = get_auth_request_metadata(host_id.clone(), db_name.clone());
            let author = get_author_binary(send_as_participant);

            let bin_author = bincode::serialize(&author.clone()).unwrap();
            let bin_author = MetadataValue::from_bytes(&bin_author);
            req.metadata_mut()
                .append_bin(TREATY_AUTH_HEADER_AUTHOR_BIN, bin_author);

            let bin_auth = bincode::serialize(&auth_bin.clone()).unwrap();
            let bin = MetadataValue::from_bytes(&bin_auth);
            req.metadata_mut().append_bin(TREATY_AUTH_HEADER_BIN, bin);

            let bin_meta = bincode::serialize(&auth_meta.clone()).unwrap();
            let bin = MetadataValue::from_bytes(&bin_meta);
            req.metadata_mut()
                .append_bin(TREATY_AUTH_HEADER_METADATA_BIN, bin);
        }

        Ok(req)
    });

    client
}

fn get_auth_request_metadata(id: Option<String>, db_name: Option<String>) -> AuthRequestMetadata {
    AuthRequestMetadata { id, db_name }
}

fn get_auth_request_binary(
    own_host_info: &HostInfo,
    id: Option<String>,
    db_name: Option<String>,
) -> AuthRequestBinary {
    AuthRequestBinary {
        user_name: own_host_info.name.clone(),
        token: own_host_info.token.clone(),
    }
}

fn get_author_binary(request_as_participant: bool) -> AuthRequestAuthor {
    if request_as_participant {
        return AuthRequestAuthor { author_type: 3 };
    }

    AuthRequestAuthor { author_type: 2 }
}

async fn get_data_service_client(
    participant: CoopDatabaseParticipant,
    timeout_in_seconds: u32,
    own_host_info: &HostInfo,
    db_name: Option<String>,
    send_as_participant: bool,
    tls_options: Option<TlsOptions>,
) -> DataServiceClient<
    InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
> {
    trace!("[{}]: {participant:?}", function_name!());
    trace!("[{}]: {own_host_info:?}", function_name!());

    let mut addr_port = format!("{}{}", participant.ip4addr, participant.db_port);

    if !addr_port.contains(':') {
        addr_port = format!("{}:{}", participant.ip4addr, participant.db_port);
        debug!("reformatted addr_port: {}", addr_port);
    }

    let http_addr_port = match tls_options {
        Some(_) => format!("{}{}", String::from("https://"), addr_port),
        None => format!("{}{}", String::from("http://"), addr_port),
    };

    trace!(
        "[{}]: configuring to connect to treaty at data endpoint: {}",
        function_name!(),
        addr_port
    );
    trace!("[{}]: {http_addr_port}", function_name!());

    let channel = match tls_options {
        Some(settings) => {
            let ca = Certificate::from_pem(settings.tls_client);

            let tls = match settings.tls_client_domain {
                Some(domain) => ClientTlsConfig::new()
                    .ca_certificate(ca)
                    .domain_name(domain),
                None => ClientTlsConfig::new().ca_certificate(ca),
            };

            Channel::builder(http_addr_port.parse().unwrap())
                .tls_config(tls)
                .unwrap()
                .timeout(Duration::from_secs(timeout_in_seconds.into()))
                .connect()
                .await
                .unwrap()
        }
        None => {
            let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap())
                .timeout(Duration::from_secs(timeout_in_seconds.into()));
            let channel = endpoint.connect().await.unwrap();
            channel
        }
    };

    let hi = own_host_info.clone();
    let db_name = db_name.clone();

    let client = DataServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        let hi = hi.clone();

        let author = get_author_binary(send_as_participant);
        let bin_author = bincode::serialize(&author.clone()).unwrap();
        let bin_author = MetadataValue::from_bytes(&bin_author);
        req.metadata_mut()
            .append_bin(TREATY_AUTH_HEADER_AUTHOR_BIN, bin_author);

        let db_name = db_name.clone();
        let auth_bin = get_auth_request_binary(
            &hi,
            Some(participant.id.to_string().clone()),
            db_name.clone(),
        );
        let auth_meta =
            get_auth_request_metadata(Some(participant.id.to_string()), db_name.clone());
        let bin_auth = bincode::serialize(&auth_bin.clone()).unwrap();
        let bin = MetadataValue::from_bytes(&bin_auth);
        req.metadata_mut().append_bin(TREATY_AUTH_HEADER_BIN, bin);

        let bin_meta = bincode::serialize(&auth_meta.clone()).unwrap();
        let bin = MetadataValue::from_bytes(&bin_meta);
        req.metadata_mut()
            .append_bin(TREATY_AUTH_HEADER_METADATA_BIN, bin);

        Ok(req)
    });

    client
}

// This is also going to need the TLS settings, same as treaty-client
async fn get_info_service_client(
    participant: CoopDatabaseParticipant,
    timeout_in_seconds: u32,
    tls_options: Option<TlsOptions>,
) -> InfoServiceClient<
    InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
> {
    trace!(
        "[{}]: CoopDatabaseParticipant: {participant:?}",
        function_name!()
    );

    let mut addr_port = format!("{}{}", participant.ip4addr, participant.info_port);

    if !addr_port.contains(':') {
        addr_port = format!("{}:{}", participant.ip4addr, participant.info_port);
        debug!("reformatted addr_port: {}", addr_port);
    }

    let http_addr_port = match tls_options {
        Some(_) => format!("{}{}", String::from("https://"), addr_port),
        None => format!("{}{}", String::from("http://"), addr_port),
    };

    trace!("configuring to connect to treaty from cds host at: {addr_port}");
    trace!("{http_addr_port}");

    let channel = match tls_options {
        Some(settings) => {
            let ca = Certificate::from_pem(settings.tls_client);

            let tls = match settings.tls_client_domain {
                Some(domain) => ClientTlsConfig::new()
                    .ca_certificate(ca)
                    .domain_name(domain),
                None => ClientTlsConfig::new().ca_certificate(ca),
            };

            Channel::builder(http_addr_port.parse().unwrap())
                .tls_config(tls)
                .unwrap()
                .timeout(Duration::from_secs(timeout_in_seconds.into()))
                .connect()
                .await
                .unwrap()
        }
        None => {
            let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap())
                .timeout(Duration::from_secs(timeout_in_seconds.into()));
            let channel = endpoint.connect().await.unwrap();
            channel
        }
    };

    let client = InfoServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        let auth = get_auth_request_metadata(Some(participant.id.to_string()), None);
        let bin = bincode::serialize(&auth.clone()).unwrap();
        let bin = MetadataValue::from_bytes(&bin);
        req.metadata_mut()
            .append_bin(TREATY_AUTH_HEADER_METADATA_BIN, bin);

        Ok(req)
    });

    client
}

async fn get_info_service_client_by_ip(
    addr_port: String,
    timeout_in_seconds: u32,
    own_host_info: &HostInfo,
    destination_host_id: Option<String>,
    tls_options: Option<TlsOptions>,
) -> InfoServiceClient<
    InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
> {
    let http_addr_port = match tls_options {
        Some(_) => format!("{}{}", String::from("https://"), addr_port),
        None => format!("{}{}", String::from("http://"), addr_port),
    };

    trace!("configuring to connect to treaty from info host at: {addr_port}");
    trace!("{http_addr_port}");

    let channel = match tls_options {
        Some(settings) => {
            let ca = Certificate::from_pem(settings.tls_client);

            let tls = match settings.tls_client_domain {
                Some(domain) => ClientTlsConfig::new()
                    .ca_certificate(ca)
                    .domain_name(domain),
                None => ClientTlsConfig::new().ca_certificate(ca),
            };

            Channel::builder(http_addr_port.parse().unwrap())
                .tls_config(tls)
                .unwrap()
                .timeout(Duration::from_secs(timeout_in_seconds.into()))
                .connect()
                .await
                .unwrap()
        }
        None => {
            let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap())
                .timeout(Duration::from_secs(timeout_in_seconds.into()));
            let channel = endpoint.connect().await.unwrap();
            channel
        }
    };

    let id = destination_host_id.clone();

    let client = InfoServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        let id = id.clone();
        let auth = get_auth_request_metadata(id, None);
        let bin = bincode::serialize(&auth.clone()).unwrap();
        let bin = MetadataValue::from_bytes(&bin);
        req.metadata_mut()
            .append_bin(TREATY_AUTH_HEADER_METADATA_BIN, bin);

        Ok(req)
    });

    client
}

fn get_telemetry(host_info: &HostInfo, own_db_addr_port: String) -> Telemetry {
    let addresses: Vec<String> = vec![
        host_info.id.clone(),
        host_info.name.clone(),
        own_db_addr_port,
    ];

    let is_little_endian = is_little_endian();

    Telemetry {
        is_little_endian,
        message_addresses: addresses,
        message_generated_time_utc: Utc::now().to_string(),
        message_guid: GUID::rand().to_string(),
    }
}

async fn get_client_with_addr_port(
    addr_port: String,
    timeout_in_seconds: u32,
    own_host_info: &HostInfo,
    db_name: Option<String>,
    tls_options: Option<TlsOptions>,
) -> DataServiceClient<
    InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
> {
    let http_addr_port = match tls_options {
        Some(_) => format!("{}{}", String::from("https://"), addr_port),
        None => format!("{}{}", String::from("http://"), addr_port),
    };

    trace!("configuring to connect to treaty at: {addr_port}");
    trace!("{http_addr_port}");

    let channel = match tls_options {
        Some(settings) => {
            let ca = Certificate::from_pem(settings.tls_client);

            let tls = match settings.tls_client_domain {
                Some(domain) => ClientTlsConfig::new()
                    .ca_certificate(ca)
                    .domain_name(domain),
                None => ClientTlsConfig::new().ca_certificate(ca),
            };

            Channel::builder(http_addr_port.parse().unwrap())
                .tls_config(tls)
                .unwrap()
                .timeout(Duration::from_secs(timeout_in_seconds.into()))
                .connect()
                .await
                .unwrap()
        }
        None => {
            let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap());
            let channel = endpoint.connect().await.unwrap();
            channel
        }
    };

    let hi = own_host_info.clone();
    let db_name = db_name.clone();

    let client = DataServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        let hi = hi.clone();
        let db_name = db_name.clone();
        let auth = get_auth_request_binary(&hi, Some(hi.id.clone()), db_name);
        let bin = bincode::serialize(&auth.clone()).unwrap();
        let bin = MetadataValue::from_bytes(&bin);
        req.metadata_mut().append_bin(TREATY_AUTH_HEADER_BIN, bin);

        Ok(req)
    });

    client
}
