use crate::{
    defaults,
    treaty_proto::{
        ColumnSchema, Contract, DatabaseSchema, Host, HostNetwork, Participant, RowRemoteMetadata,
        RowValue,
    },
};
use chrono::{DateTime, Utc};
use guid_create::GUID;
use rocket::log::private::warn;
use treaty_types::enums::*;

/*
This module contains all the types that can be returned from a database interface
 */

#[derive(Clone, Debug)]
pub(crate) struct CdsContracts {
    pub host_id: String,
    pub contract_id: String,
    pub contract_version_id: String,
    pub database_name: String,
    pub database_id: String,
    pub description: String,
    pub generated_date: String,
    pub contract_status: ContractStatus,
}

#[derive(Clone, Debug)]
pub struct CdsContractsTables {
    pub database_id: String,
    pub database_name: String,
    pub table_id: String,
    pub table_name: String,
    pub logical_storage_policy: u32,
}

#[derive(Clone, Debug)]
pub struct CdsContractsTablesColumns {
    pub table_id: String,
    pub column_id: String,
    pub column_name: String,
    pub column_type: u32,
    pub column_length: u32,
    pub column_ordinal: u32,
    pub is_nullable: bool,
}

#[derive(Clone, Debug)]
pub struct CdsCoop {
    pub table_id: String,
    pub column_id: String,
    pub column_name: String,
    pub column_type: u32,
    pub column_length: u32,
    pub column_ordinal: u32,
    pub is_nullable: bool,
}

#[derive(Clone, Debug)]
pub struct CdsHosts {
    pub host_id: String,
    pub host_name: String,
    pub token: Vec<u8>,
    pub ip4: String,
    pub ip6: String,
    pub db_port: u32,
    pub info_port: u32,
    pub last_comm_utc: String,
    pub http_addr: String,
    pub http_port: u32,
    pub status: HostStatus,
}

#[derive(Debug, Clone)]
pub struct PartialDataResult {
    pub is_successful: bool,
    pub row_id: u32,
    pub data_hash: Option<u64>,
    pub partial_data_status: Option<u32>,
    pub action: Option<PartialDataResultAction>,
}

#[derive(Debug, Clone)]
pub struct DbiConfigSqlite {
    pub root_folder: String,
    pub treaty_db_name: String,
}

#[derive(Debug, Clone)]
pub struct DbiConfigPostgres {
    pub user_name: String,
    pub pw: String,
    pub connection_string: String,
    pub host: String,
    pub connect_options: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LogEntry {
    pub dt: String,
    pub dt_utc: String,
    pub level: String,
    pub message: String,
}

#[derive(Clone, Debug, Default)]
pub struct CoopDatabaseParticipant {
    pub internal_id: GUID,
    pub alias: String,
    pub ip4addr: String,
    pub ip6addr: String,
    pub db_port: u32,
    pub info_port: u32,
    pub contract_status: ContractStatus,
    pub accepted_contract_version: GUID,
    pub token: Vec<u8>,
    pub id: GUID,
    pub http_addr: String,
    pub http_port: u16,
}

impl TryFrom<Participant> for CoopDatabaseParticipant {
    type Error = String;

    fn try_from(participant: Participant) -> Result<Self, String> {
        let result = GUID::parse(&participant.internal_participant_guid);

        let iid: GUID;
        let pid: GUID;

        match result {
            Ok(id) => {
                iid = id;
            }
            Err(e) => return Err(e.to_string()),
        }

        let result = GUID::parse(&participant.participant_guid);
        match result {
            Ok(id) => {
                pid = id;
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(Self {
            internal_id: iid,
            alias: participant.alias,
            ip4addr: participant.ip4_address,
            ip6addr: participant.ip6_address,
            db_port: participant.database_port_number,
            info_port: participant.info_port_number,
            contract_status: ContractStatus::Unknown,
            accepted_contract_version: GUID::default(),
            token: participant.token,
            id: pid,
            http_addr: participant.http_addr,
            http_port: participant.http_port as u16,
        })
    }
}

#[derive(Clone, Debug)]
pub struct CoopDatabaseParticipantData {
    pub participant: CoopDatabaseParticipant,
    pub db_name: String,
    pub table_name: String,
    pub row_data: Vec<(u32, Vec<u8>)>,
}

#[derive(Clone, Default, Debug)]
/// Represents the information about an `treaty` instance. This data is used to identify a particular
/// `treaty` instances to others. From the perspective of *participants*, this is the *host*.
pub struct HostInfo {
    pub id: String,
    pub name: String,
    pub token: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct CoopDatabaseContract {
    pub contract_id: GUID,
    pub generated_date: DateTime<Utc>,
    pub description: String,
    pub retired_date: Option<DateTime<Utc>>,
    pub version_id: GUID,
    pub remote_delete_behavior: u32,
}

impl CoopDatabaseContract {
    pub fn convert_to_protobuf(
        &self,
        host_info: &HostInfo,
        db_schema: DatabaseSchema,
        status: ContractStatus,
        addr: &str,
        db_port: u32,
        info_port: u32,
    ) -> Contract {
        let mut network: Option<HostNetwork> = None;

        if !addr.is_empty() {
            network = Some(HostNetwork {
                ip4_address: Some(addr.into()),
                ip6_address: None,
                database_port_number: Some(db_port),
                info_port_number: Some(info_port),
                http_addr: Some(addr.into()),
                http_port: Some(db_port),
            });
        }

        let c_host_info = Host {
            host_guid: host_info.id.clone(),
            host_name: host_info.name.clone(),
            token: host_info.token.clone(),
            network,
        };

        Contract {
            contract_guid: self.contract_id.to_string(),
            description: self.description.clone(),
            contract_version: self.version_id.to_string(),
            host_info: Some(c_host_info),
            schema: Some(db_schema),
            status: ContractStatus::to_u32(status),
        }
    }

    /// Checks if the contract has a retired date or not
    pub fn is_retired(&self) -> bool {
        self.retired_date.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct TreatySaveContractResult {
    pub is_successful: bool,
    pub contract_status: ContractStatus,
    pub participant_information: Option<Participant>,
}

use stdext::function_name;
use substring::Substring;
use tracing::trace;

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub data_string: String,
    pub data_byte: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct Value {
    pub data: Option<Data>,
    pub col: Column,
}

impl Value {
    pub fn is_null(&self) -> bool {
        self.data.is_none()
    }
}

#[derive(Debug)]
pub struct Row {
    pub vals: Vec<Value>,
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

impl Row {
    pub fn new() -> Self {
        Self { vals: Vec::new() }
    }

    pub fn add_value(&mut self, value: Value) {
        self.vals.push(value);
    }
}

#[derive(Clone, Debug, Default)]
pub struct Column {
    pub name: String,
    pub is_nullable: bool,
    pub idx: usize,
    pub data_type: String,
    pub is_primary_key: bool,
}

impl Column {
    pub fn data_type_to_enum_u32(&self) -> u32 {
        let ct = ColumnType::try_parse(&self.data_type).unwrap_or_default();
        ColumnType::to_u32(ct)
    }

    pub fn column_type(&self) -> ColumnType {
        ColumnType::try_parse(&self.data_type).unwrap_or_default()
    }

    pub fn data_type_len(&self) -> u32 {
        let str_data_type: String = self.data_type.clone();

        trace!("[{}]: {str_data_type:?}", function_name!());

        let idx_first_paren = str_data_type.find('(');

        match idx_first_paren {
            Some(idx) => {
                let idx_first = idx + 1;
                let idx_last = str_data_type.find(')').unwrap_or_default();
                let str_length = str_data_type.substring(idx_first, idx_last).trim();

                trace!("[{}]: {str_length:?}", function_name!());

                let length: u32 = str_length.parse().unwrap_or_default();

                if length == 0 {
                    warn!("[{}]: length is zero", function_name!());
                }

                trace!("[{}]: {length:?}", function_name!());
                length
            }
            None => 0,
        }
    }
}

#[derive(Debug)]
pub struct Table {
    pub num_cols: usize,
    pub name: String,
    pub cols: Vec<Column>,
    pub rows: Vec<Row>,
    pub backing_database_type: u32,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    pub fn new() -> Self {
        let default_db_type = DatabaseType::Unknown;
        Self {
            num_cols: 0,
            name: String::from(""),
            cols: Vec::new(),
            rows: Vec::new(),
            backing_database_type: DatabaseType::to_u32(default_db_type),
        }
    }

    pub fn set_database_type(&mut self, db_type: DatabaseType) {
        self.backing_database_type = DatabaseType::to_u32(db_type);
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_num_cols(&mut self, total_cols: usize) {
        self.num_cols = total_cols;
    }

    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn add_column(&mut self, column: Column) {
        self.cols.push(column);
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn get_column_by_index(&self, idx: usize) -> Option<Column> {
        for col in &self.cols {
            if col.idx == idx {
                return Some(col.clone());
            }
        }
        None
    }

    pub fn debug(&self) {
        for row in &self.rows {
            for val in &row.vals {
                trace!(
                    "Col: {} Value {} ",
                    val.col.name,
                    &val.data.as_ref().unwrap().data_string
                );
            }
        }
    }

    pub fn convert_to_protobuf(&self) -> Vec<crate::treaty_proto::Row> {
        let mut result: Vec<crate::treaty_proto::Row> = Vec::new();

        for (idx, t_row) in self.rows.iter().enumerate() {
            let mut c_values: Vec<RowValue> = Vec::new();

            for t_val in &t_row.vals {
                let t_col_item = &t_val.col;

                let c_col_schema_item = ColumnSchema {
                    column_name: t_col_item.name.to_string(),
                    column_type: t_col_item.data_type_to_enum_u32(),
                    column_length: t_col_item.data_type_len(),
                    column_id: GUID::rand().to_string(),
                    is_nullable: t_col_item.is_nullable,
                    ordinal: t_col_item.idx as u32,
                    table_id: GUID::rand().to_string(),
                    is_primary_key: t_col_item.is_primary_key,
                };

                let mut c_bin_data = &t_val.data.as_ref().unwrap().data_byte;
                let c_str_data = &t_val.data.as_ref().unwrap().data_string;
                let c_str_bin_data = c_str_data.as_bytes().to_vec();

                if c_bin_data.is_empty() {
                    c_bin_data = &c_str_bin_data;
                }

                let c_bd = c_bin_data.to_vec();

                let c_val: RowValue = RowValue {
                    column: Some(c_col_schema_item),
                    is_null_value: c_bd.is_empty(),
                    value: c_bd,
                    string_value: c_str_data.clone(),
                };

                c_values.push(c_val);
            }

            let c_remote_data: RowRemoteMetadata = RowRemoteMetadata {
                is_hash_out_of_sync_with_host: false,
                is_local_deleted: false,
                is_remote_deleted: false,
                is_remote_out_of_sync_with_host: false,
            };

            let c_row = crate::treaty_proto::Row {
                row_id: idx as u32,
                table_name: self.name.clone(),
                database_name: String::from(""),
                values: c_values,
                is_remoteable: false,
                remote_metadata: Some(c_remote_data),
                hash: Vec::new(),
            };

            result.push(c_row);
        }

        result
    }
}

pub fn get_data_queue_table_name(table_name: &str) -> String {
    format!("{}{}", table_name, defaults::DATA_QUEUE_TABLE_SUFFIX)
}

pub fn get_metadata_table_name(table_name: &str) -> String {
    format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX)
}

pub fn get_data_log_table_name(table_name: &str) -> String {
    format!("{}{}", table_name, defaults::DATA_LOG_TABLE_SUFFIX)
}

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub hash: Vec<u8>,
}

/// Describes the details of a row in a database. Usually used in communication with a host or participant
#[derive(Debug, Clone)]
pub struct DataInfo {
    pub db_name: String,
    pub table_name: String,
    pub row_id: u32,
    pub hash: Option<u64>,
    pub is_deleted: bool,
}
