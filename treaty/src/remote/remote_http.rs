
use treaty_types::enums::*;

use crate::{
    
    error::LogErrorActions,
    models::{
        CdsHosts, CoopDatabaseContract, CoopDatabaseParticipant, CoopDatabaseParticipantData,
        DataInfo, HostInfo, TreatySaveContractResult,
    },
};
use chrono::Utc;
use endianness::{read_i32, ByteOrder};
use guid_create::GUID;
use stdext::function_name;
use tracing::trace;

use crate::treaty_proto::{
    AuthRequest, Contract, DatabaseSchema, DeleteDataRequest, DeleteDataResult,
    GetRowFromPartialDatabaseRequest, GetRowFromPartialDatabaseResult, Host, InsertDataRequest,
    InsertDataResult, NotifyHostOfRemovedRowRequest, NotifyHostOfRemovedRowResult, Participant,
    ParticipantAcceptsContractRequest, ParticipantAcceptsContractResult, RowParticipantAddress,
    SaveContractRequest, SaveContractResult, Telemetry, TryAuthRequest, TryAuthResult,
    UpdateDataRequest, UpdateDataResult, UpdateRowDataHashForHostRequest,
    UpdateRowDataHashForHostResult,
};

use treaty_http_endpoints::data::{
    GET_ROW_AT_PARTICIPANT, INSERT_ROW_AT_PARTICIPANT, NOTIFY_HOST_OF_REMOVED_ROW,
    NOTIFY_HOST_OF_UPDATED_HASH, PARTICIPANT_ACCEPTS_CONTRACT, REMOVE_ROW_AT_PARTICIPANT,
    SAVE_CONTRACT, TRY_AUTH, UPDATE_ROW_AT_PARTICIPANT,
};

use super::remote_actions::RemoteActions;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct RemoteHttp {
    pub own_http_addr: String,
    pub own_http_port: u32,
}

impl RemoteHttp {
    pub fn new(addr: &str, port: u32) -> Self {
        Self {
            own_http_addr: addr.into(),
            own_http_port: port,
        }
    }
}

#[async_trait]
impl RemoteActions for RemoteHttp {
    async fn notify_host_of_updated_hash(
        &self,
        host: &CdsHosts,
        own_host_info: &HostInfo,
        data_info: &DataInfo,
    ) -> bool {
        let auth = get_auth_request(own_host_info, Some(host.host_id.clone()));
        let telemetry = get_telemetry(own_host_info, "".to_string());

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

            let request_json = serde_json::to_string(&request).unwrap();

            let addr_port = format!("{}:{}", host.http_addr, host.http_port);

            trace!("sending request to treaty at: {}", addr_port);

            let url = format!("http://{addr_port}{NOTIFY_HOST_OF_UPDATED_HASH}");
            let result = send_message(request_json, url).await;
            let result: UpdateRowDataHashForHostResult = serde_json::from_str(&result).unwrap();
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

            let request_json = serde_json::to_string(&request).unwrap();

            let addr_port = format!("{}:{}", host.http_addr, host.http_port);

            trace!("sending request to treaty at: {}", addr_port);

            let url = format!("http://{addr_port}{NOTIFY_HOST_OF_REMOVED_ROW}");
            let result = send_message(request_json, url).await;
            let result: NotifyHostOfRemovedRowResult = serde_json::from_str(&result).unwrap();
            result.log_any_err(function_name!());
            result.is_successful
        }
    }

    async fn get_row_from_participant(
        &self,
        participant: CoopDatabaseParticipantData,
        own_host_info: HostInfo,
        row_id: u32,
    ) -> GetRowFromPartialDatabaseResult {
        let telemetry = get_telemetry(&own_host_info, "".to_string());
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

        let request_json = serde_json::to_string(&request).unwrap();

        let addr_port = format!(
            "{}:{}",
            participant.participant.http_addr, participant.participant.http_port
        );

        trace!("sending request to treaty at: {}", addr_port);

        let url = format!("http://{addr_port}{GET_ROW_AT_PARTICIPANT}");
        let result = send_message(request_json, url).await;
        let reply: GetRowFromPartialDatabaseResult = serde_json::from_str(&result).unwrap();
        reply.log_any_err(function_name!());
        reply
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

        let request_json = serde_json::to_string(&request).unwrap();

        let addr_port = format!("{}:{}", participant.http_addr, participant.http_port);

        trace!("sending request to treaty at: {}", addr_port);

        let url = format!("http://{addr_port}{INSERT_ROW_AT_PARTICIPANT}");
        let result = send_message(request_json, url).await;
        let reply: InsertDataResult = serde_json::from_str(&result).unwrap();
        reply.log_any_err(function_name!());
        reply
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

        let request_json = serde_json::to_string(&request).unwrap();

        let addr_port = format!("{}:{}", participant.http_addr, participant.http_port);

        trace!("sending request to treaty at: {}", addr_port);

        let url = format!("http://{addr_port}{UPDATE_ROW_AT_PARTICIPANT}");
        let result = send_message(request_json, url).await;
        let reply: UpdateDataResult = serde_json::from_str(&result).unwrap();
        reply.log_any_err(function_name!());
        reply
    }

    async fn try_auth_at_participant(
        &self,
        participant: CoopDatabaseParticipant,
        own_host_info: &HostInfo,
    ) -> bool {
        let auth = get_auth_request(own_host_info, Some(participant.id.to_string()));
        let request = TryAuthRequest {
            authentication: Some(auth),
        };

        let request_json = serde_json::to_string(&request).unwrap();

        let addr_port = format!("{}:{}", participant.http_addr, participant.http_port);

        trace!("sending request to treaty at: {}", addr_port);

        let url = format!("http://{addr_port}{TRY_AUTH}");
        let result = send_message(request_json, url).await;
        let reply: TryAuthResult = serde_json::from_str(&result).unwrap();

        reply.authentication_result.unwrap().is_authenticated
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
        let telemetry = get_telemetry(own_host_info, "".to_string());

        let chost = Host {
            host_guid: own_host_info.id.clone(),
            host_name: own_host_info.name.clone(),
            network: None,
            token: own_host_info.token.clone(),
        };

        let request = NotifyHostOfRemovedRowRequest {
            authentication: Some(auth),
            telemetry: Some(telemetry),
            host_info: Some(chost),
            database_name: db_name.to_string(),
            database_id: String::from(""),
            table_name: table_name.to_string(),
            table_id: 0,
            row_id,
        };

        let request_json = serde_json::to_string(&request).unwrap();

        let addr_port = format!("{}:{}", host.http_addr, host.http_port);

        trace!("sending request to treaty at: {}", addr_port);

        let url = format!("http://{addr_port}{NOTIFY_HOST_OF_REMOVED_ROW}");
        let result = send_message(request_json, url).await;
        let reply: NotifyHostOfRemovedRowResult = serde_json::from_str(&result).unwrap();
        reply.log_any_err(function_name!());
        reply.is_successful
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

        let request_json = serde_json::to_string(&request).unwrap();

        let addr_port = format!("{}:{}", participant.http_addr, participant.http_port);

        trace!("sending request to treaty at: {}", addr_port);

        let url = format!("http://{addr_port}{REMOVE_ROW_AT_PARTICIPANT}");
        let result = send_message(request_json, url).await;
        let reply: DeleteDataResult = serde_json::from_str(&result).unwrap();
        reply.log_any_err(function_name!());
        reply
    }

    async fn notify_host_of_acceptance_of_contract(
        &self,
        accepted_contract: &Contract,
        own_host_info: &HostInfo,
    ) -> bool {
        let telemetry = get_telemetry(own_host_info, "".to_string());
        let host_info = accepted_contract.host_info.as_ref().unwrap().clone();

        let participant = Participant {
            participant_guid: own_host_info.id.clone(),
            alias: own_host_info.name.clone(),
            ip4_address: self.own_http_addr.clone(),
            ip6_address: String::from(""),
            database_port_number: 0,
            token: own_host_info.token.clone(),
            internal_participant_guid: "".to_string(),
            http_addr: self.own_http_addr.clone(),
            http_port: self.own_http_port,
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
            id: Some(host_info.host_guid.clone()),
        };

        let request_json = serde_json::to_string(&request).unwrap();

        let addr = host_info
            .network
            .as_ref()
            .unwrap()
            .http_addr
            .as_ref()
            .unwrap()
            .clone();
        let port = host_info.network.as_ref().unwrap().http_port.unwrap();

        let addr_port = format!("{}:{}", addr, port);

        trace!("sending request to treaty at: {}", addr_port);

        let url = format!("http://{addr_port}{PARTICIPANT_ACCEPTS_CONTRACT}");
        let result = send_message(request_json, url).await;
        let reply: ParticipantAcceptsContractResult = serde_json::from_str(&result).unwrap();
        reply.log_any_err(function_name!());
        reply.contract_acceptance_is_acknowledged
    }

    async fn send_participant_contract(
        &self,
        participant: CoopDatabaseParticipant,
        host_info: HostInfo,
        contract: CoopDatabaseContract,
        db_schema: DatabaseSchema,
    ) -> TreatySaveContractResult {
        let telemetry = get_telemetry(&host_info, "".to_string());

        let contract = contract.convert_to_protobuf(
            &host_info,
            db_schema,
            ContractStatus::Pending,
            &self.own_http_addr,
            self.own_http_port,
        );

        let request = SaveContractRequest {
            contract: Some(contract),
            telemetry: Some(telemetry),
            id: Some(participant.id.to_string()),
        };

        let request_json = serde_json::to_string(&request).unwrap();
        let addr_port = format!("{}:{}", participant.http_addr, participant.http_port);

        trace!("sending request to treaty at: {}", addr_port);

        let url = format!("http://{addr_port}{SAVE_CONTRACT}");
        let result = send_message(request_json, url).await;
        let reply: SaveContractResult = serde_json::from_str(&result).unwrap();
        reply.log_any_err(function_name!());
        /*
        let http_response = reqwest::new(&url)
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
            */

        TreatySaveContractResult {
            is_successful: reply.is_saved,
            contract_status: ContractStatus::from_u32(reply.contract_status),
            participant_information: reply.participant_info,
        }
    }
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

fn is_little_endian() -> bool {
    let v = vec![0, 128, 128, 0];

    match read_i32(&v, ByteOrder::LittleEndian) {
        Ok(_n) => true,
        Err(_err) => false,
    }
}

async fn send_message(json_message: String, url: String) -> String {
    let client = reqwest::Client::new();

    trace!("{json_message}");
    trace!("{url}");

    client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_message)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
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
