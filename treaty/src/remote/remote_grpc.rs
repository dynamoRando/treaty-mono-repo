use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use endianness::{read_i32, ByteOrder};
use guid_create::GUID;
use stdext::function_name;
use tonic::transport::Channel;
use tracing::{debug, info, trace};
use treaty_types::enums::*;
use crate::{
  
    error::LogErrorActions,
    models::{
        CdsHosts, CoopDatabaseContract, CoopDatabaseParticipant, CoopDatabaseParticipantData,
        DataInfo, HostInfo, TreatySaveContractResult,
    },
    treaty_proto::{
        data_service_client::DataServiceClient, AuthRequest, Contract, DatabaseSchema,
        DeleteDataRequest, DeleteDataResult, GetRowFromPartialDatabaseRequest,
        GetRowFromPartialDatabaseResult, Host, InsertDataRequest, InsertDataResult,
        NotifyHostOfRemovedRowRequest, Participant, ParticipantAcceptsContractRequest,
        RowParticipantAddress, SaveContractRequest, Telemetry, TryAuthRequest, UpdateDataRequest,
        UpdateDataResult, UpdateRowDataHashForHostRequest,
    },
};

use super::remote_actions::RemoteActions;

#[derive(Debug, Clone)]
pub struct RemoteGrpc {
    db_addr_port: String,
    timeout_in_seconds: u32,
}

impl RemoteGrpc {
    pub fn new(db_addr_port: &str, timeout_in_seconds: u32) -> Self {
        Self {
            db_addr_port: db_addr_port.to_string(),
            timeout_in_seconds,
        }
    }

    pub fn addr(&self) -> String {
        let parts = self.db_addr_port.split(':');
        let parts = parts.collect::<Vec<&str>>();
        return parts.first().unwrap().to_string();
    }

    pub fn port(&self) -> u32 {
        let parts = self.db_addr_port.split(':');
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
        let auth = get_auth_request(own_host_info, Some(participant.id.to_string()));
        let request = TryAuthRequest {
            authentication: Some(auth),
        };

        let mut client = get_client(participant, self.timeout_in_seconds).await;
        let response = client.try_auth(request).await;
        let result = response.unwrap().into_inner();

        match result.authentication_result {
            Some(login) => return login.is_authenticated,
            None => return false,
        }
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
            self.port(),
        );

        let request = tonic::Request::new(SaveContractRequest {
            contract: Some(contract),
            telemetry: Some(telemetry),
            id: Some(participant.id.to_string()),
        });

        trace!("send_participant_contract: {request:?}");

        let mut addr_port = format!("{}{}", participant.ip4addr, participant.db_port);

        if !addr_port.contains(':') {
            addr_port = format!("{}:{}", participant.ip4addr, participant.db_port);
            debug!("reformatted - sending request to Treaty at: {}", addr_port);
        }

        trace!("sending request to treaty at: {}", addr_port);

        let mut client = get_client(participant, self.timeout_in_seconds).await;
        let response = client.save_contract(request).await.unwrap().into_inner();
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
        let auth = get_auth_request(own_host_info, Some(host.host_id.clone()));

        trace!("[{}]: {auth:?}", function_name!());
        trace!("[{}]: {host:?}", function_name!());

        let message_info = get_telemetry(own_host_info, self.db_addr_port.clone());

        let chost = Host {
            host_guid: own_host_info.id.clone(),
            host_name: own_host_info.name.clone(),
            token: own_host_info.token.clone(),
            network: None,
        };

        let request = NotifyHostOfRemovedRowRequest {
            authentication: Some(auth),
            telemetry: Some(message_info),
            host_info: Some(chost),
            database_name: db_name.to_string(),
            database_id: String::from(""),
            table_name: table_name.to_string(),
            table_id: 0,
            row_id,
        };

        trace!("[{}]: {request:?}", function_name!());

        let client = get_client_from_cds_host(host);
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
        let auth = get_auth_request(own_host_info, Some(participant.id.to_string()));

        let request = DeleteDataRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            cmd: sql.to_string(),
            where_clause: where_clause.to_string(),
        };

        let client = get_client(participant, self.timeout_in_seconds);
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
        let auth = get_auth_request(own_host_info, Some(participant.id.to_string()));

        let request = UpdateDataRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            cmd: sql.to_string(),
            where_clause: where_clause.to_string(),
        };

        let client = get_client(participant, self.timeout_in_seconds);
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
        let auth = get_auth_request(own_host_info, Some(participant.id.to_string()));

        let request = InsertDataRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            cmd: sql.to_string(),
        };

        let client = get_client(participant, self.timeout_in_seconds);
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
        let auth = get_auth_request(&own_host_info, Some(participant.participant.id.to_string()));

        let row_address = RowParticipantAddress {
            database_name: participant.db_name.clone(),
            table_name: participant.table_name.clone(),
            row_id,
        };

        let request = GetRowFromPartialDatabaseRequest {
            authentication: Some(auth),
            row_address: Some(row_address),
            telemetry: Some(telemetry),
        };

        let participant_info = participant.participant.clone();

        let client = get_client(participant_info, self.timeout_in_seconds);
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
        let auth = get_auth_request(own_host_info, Some(host.host_id.clone()));

        debug!("notify_host_of_updated_hash::auth: {auth:?}");

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
                authentication: Some(auth),
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

            let client = get_client_from_cds_host(host);
            let response = client.await.update_row_data_hash_for_host(request).await;
            let result = response.unwrap().into_inner();
            result.log_any_err(function_name!());
            result.is_successful
        } else {
            let request = NotifyHostOfRemovedRowRequest {
                authentication: Some(auth),
                telemetry: Some(telemetry),
                host_info: Some(chost),
                database_name: data_info.db_name.to_string(),
                database_id: String::from(""),
                table_name: data_info.table_name.to_string(),
                table_id: 0,
                row_id: data_info.row_id,
            };

            let client = get_client_from_cds_host(host);
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
            token: own_host_info.token.clone(),
            internal_participant_guid: "".to_string(),
            http_addr: "".to_string(),
            http_port: 0,
        };

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
            id: Some(host_info.host_guid),
        };

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
                .database_port_number
                .unwrap()
        );

        let message = format!("sending request to treaty at: {}", &ip4);
        trace!("[{}]: {}", function_name!(), message);

        let client = get_client_with_addr_port(ip4, self.timeout_in_seconds);
        let response = client.await.accept_contract(request).await.unwrap();

        let result = response.into_inner();
        result.log_any_err(function_name!());

        result.contract_acceptance_is_acknowledged
    }
}

fn is_little_endian() -> bool {
    let v = vec![0, 128, 128, 0];

    match read_i32(&v, ByteOrder::LittleEndian) {
        Ok(_n) => true,
        Err(_err) => false,
    }
}

async fn get_client_from_cds_host(host: &CdsHosts) -> DataServiceClient<Channel> {
    let addr_port = if !host.ip4.contains(':') {
        format!("{}:{}", host.ip4, host.port.to_string())
    } else {
        host.ip4.clone()
    };

    // let addr_port = format!("{}{}", host.ip4, host.port.to_string());
    // let addr_port = host.ip4.clone();

    let http_addr_port = format!("{}{}", String::from("http://"), addr_port);
    trace!("configuring to connect to treaty from cds host at: {addr_port}");

    trace!("{http_addr_port}");

    let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap());
    let channel = endpoint.connect().await.unwrap();

    DataServiceClient::new(channel)
}

fn get_auth_request(own_host_info: &HostInfo, id: Option<String>) -> AuthRequest {
    AuthRequest {
        user_name: own_host_info.name.clone(),
        pw: String::from(""),
        pw_hash: Vec::new(),
        token: own_host_info.token.clone(),
        jwt: String::from(""),
        id,
    }
}

async fn get_client(
    participant: CoopDatabaseParticipant,
    timeout_in_seconds: u32,
) -> DataServiceClient<Channel> {
    let mut addr_port = format!("{}{}", participant.ip4addr, participant.db_port);

    if !addr_port.contains(':') {
        addr_port = format!("{}:{}", participant.ip4addr, participant.db_port);
        debug!("reformatted addr_port: {}", addr_port);
    }

    let http_addr_port = format!("{}{}", String::from("http://"), addr_port);
    trace!(
        "[{}]: configuring to connect to treaty at: {}",
        function_name!(),
        addr_port
    );
    trace!("[{}]: {http_addr_port}", function_name!());

    let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap())
        .timeout(Duration::from_secs(timeout_in_seconds.into()));
    let channel = endpoint.connect().await.unwrap();

    DataServiceClient::new(channel)
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
) -> DataServiceClient<Channel> {
    let http_addr_port = format!("{}{}", String::from("http://"), addr_port);
    let message = format!("configuring to connect to treaty at: {addr_port}");
    info!("{}", message);

    let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap())
        .timeout(Duration::from_secs(timeout_in_seconds.into()));
    let channel = endpoint.connect().await.unwrap();

    DataServiceClient::new(channel)
}
