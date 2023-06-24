use std::fmt;

use num_derive::{FromPrimitive, ToPrimitive};
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, PartialEq, Copy, Clone, ToPrimitive)]
pub enum ContractStatus {
    Unknown = 0,
    NotSent = 1,
    Pending = 2,
    Accepted = 3,
    Rejected = 4,
}
impl Default for ContractStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

// https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
impl ContractStatus {
    pub fn from_i64(value: i64) -> ContractStatus {
        match value {
            0 => ContractStatus::Unknown,
            1 => ContractStatus::NotSent,
            2 => ContractStatus::Pending,
            3 => ContractStatus::Accepted,
            4 => ContractStatus::Rejected,
            _ => {
                error!("Unknown value: {value}");
                ContractStatus::Unknown
            }
        }
    }

    pub fn from_u32(value: u32) -> ContractStatus {
        match value {
            0 => ContractStatus::Unknown,
            1 => ContractStatus::NotSent,
            2 => ContractStatus::Pending,
            3 => ContractStatus::Accepted,
            4 => ContractStatus::Rejected,
            _ => {
                error!("Unknown value: {value}");
                ContractStatus::Unknown
            }
        }
    }

    pub fn to_u32(status: ContractStatus) -> u32 {
        match status {
            ContractStatus::Unknown => 0,
            ContractStatus::NotSent => 1,
            ContractStatus::Pending => 2,
            ContractStatus::Accepted => 3,
            ContractStatus::Rejected => 4,
        }
    }
}

/// Represents the type of backing database treaty is hosting
/// # Types
/// * 0 - Unknown
/// * 1 - Sqlite
/// * 2 - Mysql
/// * 3 - Postgres
/// * 4 - Sqlserver
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, Default)]
pub enum DatabaseType {
    #[default]
    Unknown = 0,
    Sqlite = 1,
    Mysql = 2,
    Postgres = 3,
    Sqlserver = 4,
}

// https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
impl DatabaseType {
    pub fn from_i64(value: i64) -> DatabaseType {
        match value {
            0 => DatabaseType::Unknown,
            1 => DatabaseType::Sqlite,
            2 => DatabaseType::Mysql,
            3 => DatabaseType::Postgres,
            4 => DatabaseType::Sqlserver,
            _ => {
                error!("Unknown value: {value}");
                DatabaseType::Unknown
            }
        }
    }

    pub fn from_u32(value: u32) -> DatabaseType {
        match value {
            0 => DatabaseType::Unknown,
            1 => DatabaseType::Sqlite,
            2 => DatabaseType::Mysql,
            3 => DatabaseType::Postgres,
            4 => DatabaseType::Sqlserver,
            _ => {
                error!("Unknown value: {value}");
                DatabaseType::Unknown
            }
        }
    }

    pub fn to_u32(db_type: DatabaseType) -> u32 {
        match db_type {
            DatabaseType::Unknown => 0,
            DatabaseType::Sqlite => 1,
            DatabaseType::Mysql => 2,
            DatabaseType::Postgres => 3,
            DatabaseType::Sqlserver => 4,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum DeletesFromHostBehavior {
    Unknown = 0,
    AllowRemoval = 1,
    QueueForReview = 2,
    DeleteWithLog = 3,
    Ignore = 4,
    QueueForReviewAndLog = 5,
}

impl fmt::Display for DeletesFromHostBehavior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeletesFromHostBehavior::Unknown => write!(f, "Unknown"),
            DeletesFromHostBehavior::AllowRemoval => write!(f, "AllowRemoval"),
            DeletesFromHostBehavior::QueueForReview => write!(f, "QueueForReview"),
            DeletesFromHostBehavior::Ignore => write!(f, "Ignore"),
            DeletesFromHostBehavior::QueueForReviewAndLog => write!(f, "QueueForReviewAndLog"),
            DeletesFromHostBehavior::DeleteWithLog => write!(f, "DeleteWithLog"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum DeletesToHostBehavior {
    Unknown = 0,
    SendNotification = 1,
    DoNothing = 2,
}

impl fmt::Display for DeletesToHostBehavior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeletesToHostBehavior::Unknown => write!(f, "Unknown"),
            DeletesToHostBehavior::SendNotification => write!(f, "SendNotification"),
            DeletesToHostBehavior::DoNothing => write!(f, "DoNothing"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum HostStatus {
    Unknown = 0,
    Allow = 1,
    Deny = 2,
}

impl HostStatus {
    pub fn from_u32(value: u32) -> HostStatus {
        match value {
            0 => HostStatus::Unknown,
            1 => HostStatus::Allow,
            2 => HostStatus::Deny,
            _ => {
                error!("Unknown value: {value}");
                HostStatus::Unknown
            }
        }
    }

    pub fn to_u32(value: HostStatus) -> u32 {
        match value {
            HostStatus::Unknown => 0,
            HostStatus::Allow => 1,
            HostStatus::Deny => 2,
        }
    }

    pub fn to_string(value: HostStatus) -> String {
        match value {
            HostStatus::Unknown => "Unknown".to_string(),
            HostStatus::Allow => "Allow".to_string(),
            HostStatus::Deny => "Deny".to_string(),
        }
    }

    pub fn from_str(value: &str) -> HostStatus {
        match value {
            "Unknown" => HostStatus::Unknown,
            "Allow" => HostStatus::Allow,
            "Deny" => HostStatus::Deny,
            _ => HostStatus::Unknown,
        }
    }

    pub fn as_string(self) -> String {
        HostStatus::to_string(self)
    }
}

/// Determines where data in table will be stored.
/// # Types
/// * 0 - None - This is the default and when a database has no participants. Data is kept at the host.
/// * 1 - HostOnly - Data is only kept at the host.
/// * 2 - ParticpantOwned - Data is kept at the participant. Hashes of the data are kept at the host. If the participant
/// changes the data, the hash will no longer match unless the host has configured the table to accept changes.
/// * 3 - Shared - Data is at the host, and changes are automatically pushed to the participant. This is essentially SQL replication.
/// If data is deleted at the host, it is not automatically deleted at the participant but rather a record marker (tombstone) showing
/// it's been deleted is sent to the participant, which the participant can act on or ignore (note: the marker will still exist in the meta-data table at the Partial database).
/// This is a "soft" delete at the participant. Changes at the participant are not communicated back to the host.
/// * 4 - Mirror - Data is at the host, and changes are automatically pushed to the participant. This is essentially SQL replication, with the
/// option of the Participant being able to override changes from the Host by configuring the "FromHostBehavior" enums. The difference
/// between "Mirror" and "Shared" is intent: Shared is intended to signal to the Participant that the Host would prefer to send "soft"
/// delete notices to the participant. "Mirror" is signaling to the participant that all actions will be "hard" deletes by default, unless
/// overriden by the Participant. Changes at the participant are not communicated back to the host.
///
/// Shared and Mirror modes are intended for situations where logically the data in the table combines items that belong
/// to both a Host and a Participant. For example, in an e-commerce database, a customer's order history logically contains
/// information that belong both to the Host and the Participant. A Host may want to make sure that a Participant always has a copy
/// of their order history for their own consumption, but for performance reasons needs to keep the data at the Host. These modes
/// (Shared and Mirror) are intended to account for this.
///
/// The difference between a Host wanting to signal hard versus soft deletes may be a function of infastructure limitations. If
/// `treaty` is being offered by the Host as a SaaS (via `treaty-proxy`) the Host may want to by default always "hard" delete changes, with the
/// option that if a Participant chooses to override this setting, any additional storage consumption may result in, for example, extra
/// infastructure charges for storage.
///
/// For more information, see the manual.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LogicalStoragePolicy {
    None = 0,
    HostOnly = 1,
    ParticpantOwned = 2,
    Shared = 3,
    Mirror = 4,
}

impl LogicalStoragePolicy {
    pub fn from_i64(value: i64) -> LogicalStoragePolicy {
        match value {
            0 => LogicalStoragePolicy::None,
            1 => LogicalStoragePolicy::HostOnly,
            2 => LogicalStoragePolicy::ParticpantOwned,
            3 => LogicalStoragePolicy::Shared,
            4 => LogicalStoragePolicy::Mirror,
            _ => {
                error!("Unknown value: {value}");
                LogicalStoragePolicy::None
            }
        }
    }

    pub fn from_u32(value: u32) -> LogicalStoragePolicy {
        match value {
            0 => LogicalStoragePolicy::None,
            1 => LogicalStoragePolicy::HostOnly,
            2 => LogicalStoragePolicy::ParticpantOwned,
            3 => LogicalStoragePolicy::Shared,
            4 => LogicalStoragePolicy::Mirror,
            _ => {
                error!("Unknown value: {value:?}");
                LogicalStoragePolicy::None
            }
        }
    }

    pub fn to_u32(policy: LogicalStoragePolicy) -> u32 {
        match policy {
            LogicalStoragePolicy::None => 0,
            LogicalStoragePolicy::HostOnly => 1,
            LogicalStoragePolicy::ParticpantOwned => 2,
            LogicalStoragePolicy::Shared => 3,
            LogicalStoragePolicy::Mirror => 4,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum PartialDataResultAction {
    Unknown = 0,
    Insert = 1,
    Update = 2,
    Delete = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromPrimitive, ToPrimitive)]
pub enum TreatyDatabaseType {
    Unknown = 0,
    System = 1,
    Host = 2,
    Partial = 3,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(u32)]
pub enum UpdatesToHostBehavior {
    Unknown = 0,
    SendDataHashChange = 1,
    DoNothing = 2,
}

impl fmt::Display for UpdatesToHostBehavior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UpdatesToHostBehavior::Unknown => write!(f, "Unknown"),
            UpdatesToHostBehavior::SendDataHashChange => write!(f, "SendDataHashChange"),
            UpdatesToHostBehavior::DoNothing => write!(f, "DoNothing"),
        }
    }
}

use substring::Substring;
use tracing::error;
use tracing::trace;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum ColumnType {
    #[default]
    Unknown = 0,
    Int = 1,
    Bit = 2,
    Char = 3,
    DateTime = 4,
    Decimal = 5,
    Varchar = 6,
    Binary = 7,
    Varbinary = 8,
    Text = 9,
}

impl ColumnType {
    pub fn data_type_as_string_sqlite(&self) -> String {
        match self {
            ColumnType::Unknown => {
                error!("Unknown data type");
                String::from("")
            }
            ColumnType::Int => String::from("INT"),
            ColumnType::Bit => String::from("TINYINT"),
            ColumnType::Char => String::from("CHAR"),
            ColumnType::DateTime => String::from("DATETIME"),
            ColumnType::Decimal => String::from("DECIMAL"),
            ColumnType::Varchar => String::from("VARCHAR"),
            ColumnType::Binary => String::from("BLOB"),
            ColumnType::Varbinary => String::from("BLOB"),
            ColumnType::Text => String::from("TEXT"),
        }
    }

    pub fn data_type_to_enum_u32(desc: String) -> u32 {
        trace!("{desc:?}");
        let ct = ColumnType::try_parse(&desc).unwrap();
        ColumnType::to_u32(ct)
    }

    pub fn data_type_len(desc: String) -> u32 {
        let idx_first_paren = desc.find('(');

        match idx_first_paren {
            Some(idx) => {
                let idx_first = idx;
                let idx_last = desc.find(')').unwrap();
                let str_length = desc.substring(idx_first + 1, idx_last);
                trace!("column string length: {str_length}");
                let str_length = str_length.trim();
                let length: u32 = str_length.parse().unwrap();
                length
            }
            None => 0,
        }
    }

    pub fn try_parse(desc: &str) -> Option<ColumnType> {
        let string_data_type = desc.to_lowercase();

        if string_data_type.is_empty() {
            return Some(ColumnType::Unknown);
        }

        if string_data_type.contains("int") {
            return Some(ColumnType::Int);
        }

        if string_data_type.contains("bit") {
            return Some(ColumnType::Bit);
        }

        if string_data_type.contains("varchar") {
            return Some(ColumnType::Varchar);
        }

        if string_data_type.contains("char") {
            return Some(ColumnType::Char);
        }

        if string_data_type.contains("datetime") {
            return Some(ColumnType::DateTime);
        }

        if string_data_type.contains("decimal") {
            return Some(ColumnType::Decimal);
        }

        if string_data_type.contains("varbinary") {
            return Some(ColumnType::Varbinary);
        }

        if string_data_type.contains("blob") {
            return Some(ColumnType::Varbinary);
        }

        if string_data_type.contains("binary") {
            return Some(ColumnType::Binary);
        }

        if string_data_type.contains("text") {
            return Some(ColumnType::Text);
        }

        None
    }

    pub fn from_u32(value: u32) -> ColumnType {
        match value {
            0 => ColumnType::Unknown,
            1 => ColumnType::Int,
            2 => ColumnType::Bit,
            3 => ColumnType::Char,
            4 => ColumnType::DateTime,
            5 => ColumnType::Decimal,
            6 => ColumnType::Varchar,
            7 => ColumnType::Binary,
            8 => ColumnType::Varbinary,
            9 => ColumnType::Text,
            _ => {
                error!("Unknown value: {value}");
                ColumnType::Unknown
            }
        }
    }

    pub fn to_u32(col_type: ColumnType) -> u32 {
        match col_type {
            ColumnType::Unknown => 0,
            ColumnType::Int => 1,
            ColumnType::Bit => 2,
            ColumnType::Char => 3,
            ColumnType::DateTime => 4,
            ColumnType::Decimal => 5,
            ColumnType::Varchar => 6,
            ColumnType::Binary => 7,
            ColumnType::Varbinary => 8,
            ColumnType::Text => 9,
        }
    }
}

/// Determines how a host will respond to a particpant's delete action.
/// # Types
/// * 0 - Unknown
/// * 1 - Ignore - If the host discovers that the particpant has deleted the row, it will take no action.
/// * 2 - AutoDelete - If the host discovers that the participant has deleted the row, it will update the reference row with
/// the delete data then logically delete the row.
/// * 3 - UpdateStatusOnly - If the host discovers that the particpant has deleted the row then update the reference row
/// with the delete data but do not perform a logical delete on the row (Note: The row can still be manually deleted
/// on the host side at a later time.)
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RemoteDeleteBehavior {
    Unknown = 0,
    Ignore = 1,
    AutoDelete = 2,
    UpdateStatusOnly = 3,
}

// https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
impl RemoteDeleteBehavior {
    pub fn from_i64(value: i64) -> RemoteDeleteBehavior {
        match value {
            0 => RemoteDeleteBehavior::Unknown,
            1 => RemoteDeleteBehavior::Ignore,
            2 => RemoteDeleteBehavior::AutoDelete,
            3 => RemoteDeleteBehavior::UpdateStatusOnly,
            _ => {
                error!("Unknown value: {value}");
                RemoteDeleteBehavior::Unknown
            }
        }
    }

    pub fn from_u32(value: u32) -> RemoteDeleteBehavior {
        match value {
            0 => RemoteDeleteBehavior::Unknown,
            1 => RemoteDeleteBehavior::Ignore,
            2 => RemoteDeleteBehavior::AutoDelete,
            3 => RemoteDeleteBehavior::UpdateStatusOnly,
            _ => {
                error!("Unknown value: {value}");
                RemoteDeleteBehavior::Unknown
            }
        }
    }

    pub fn to_u32(behavior: RemoteDeleteBehavior) -> u32 {
        match behavior {
            RemoteDeleteBehavior::Unknown => 0,
            RemoteDeleteBehavior::Ignore => 1,
            RemoteDeleteBehavior::AutoDelete => 2,
            RemoteDeleteBehavior::UpdateStatusOnly => 3,
        }
    }
}

/// From the perspective of a participant: if we get an `UPDATE` statement from a database host
/// we can define how we want to respond:
/// 1. Allow Overwrite - will execute the `UPDATE` statement
/// 2. Queue For Review  - will add a "Pending" flag on the row
/// 3. Overwrite With Log - will copy the row to _HISTORY table and then overwrite
/// 4. Ignore - will not update the row but respond to the host with FALSE on the success reply
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UpdatesFromHostBehavior {
    Unknown = 0,
    AllowOverwrite = 1,
    QueueForReview = 2,
    OverwriteWithLog = 3,
    Ignore = 4,
    QueueForReviewAndLog = 5,
}

impl UpdatesFromHostBehavior {
    pub fn from_u32(value: u32) -> UpdatesFromHostBehavior {
        match value {
            0 => UpdatesFromHostBehavior::Unknown,
            1 => UpdatesFromHostBehavior::AllowOverwrite,
            2 => UpdatesFromHostBehavior::QueueForReview,
            3 => UpdatesFromHostBehavior::OverwriteWithLog,
            4 => UpdatesFromHostBehavior::Ignore,
            5 => UpdatesFromHostBehavior::QueueForReviewAndLog,
            _ => {
                error!("Unknown value: {value}");
                UpdatesFromHostBehavior::Unknown
            }
        }
    }

    pub fn to_u32(behavior: UpdatesFromHostBehavior) -> u32 {
        match behavior {
            UpdatesFromHostBehavior::Unknown => 0,
            UpdatesFromHostBehavior::AllowOverwrite => 1,
            UpdatesFromHostBehavior::QueueForReview => 2,
            UpdatesFromHostBehavior::OverwriteWithLog => 3,
            UpdatesFromHostBehavior::Ignore => 4,
            UpdatesFromHostBehavior::QueueForReviewAndLog => 5,
        }
    }

    pub fn to_string(value: UpdatesFromHostBehavior) -> String {
        match value {
            UpdatesFromHostBehavior::Unknown => "Unknown".to_string(),
            UpdatesFromHostBehavior::AllowOverwrite => "AllowOverwrite".to_string(),
            UpdatesFromHostBehavior::QueueForReview => "QueueForReview".to_string(),
            UpdatesFromHostBehavior::OverwriteWithLog => "OverwriteWithLog".to_string(),
            UpdatesFromHostBehavior::Ignore => "Ignore".to_string(),
            UpdatesFromHostBehavior::QueueForReviewAndLog => "QueueForReviewAndLog".to_string(),
        }
    }

    pub fn from_str(value: &str) -> UpdatesFromHostBehavior {
        match value {
            "Unknown" => UpdatesFromHostBehavior::Unknown,
            "AllowOverwrite" => UpdatesFromHostBehavior::AllowOverwrite,
            "QueueForReview" => UpdatesFromHostBehavior::QueueForReview,
            "OverwriteWithLog" => UpdatesFromHostBehavior::OverwriteWithLog,
            "QueueForReviewAndLog" => UpdatesFromHostBehavior::QueueForReviewAndLog,
            _ => UpdatesFromHostBehavior::Unknown,
        }
    }

    pub fn as_string(self) -> String {
        UpdatesFromHostBehavior::to_string(self)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DmlType {
    Unknown = 0,
    Insert = 1,
    Update = 2,
    Delete = 3,
    Select = 4,
}

impl DmlType {
    pub fn from_u32(value: u32) -> DmlType {
        match value {
            0 => DmlType::Unknown,
            1 => DmlType::Insert,
            2 => DmlType::Update,
            3 => DmlType::Delete,
            4 => DmlType::Select,
            _ => {
                error!("Unknown value: {value}");
                DmlType::Unknown
            }
        }
    }

    pub fn to_u32(dml_type: DmlType) -> u32 {
        match dml_type {
            DmlType::Unknown => 0,
            DmlType::Insert => 1,
            DmlType::Update => 2,
            DmlType::Delete => 3,
            DmlType::Select => 4,
        }
    }
}

/// Specifies the UpdateStatus in a UpdateDataResult message
/// in treatyp.proto
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PartialDataStatus {
    Unknown = 0,
    SucessOverwriteOrLog = 1,
    Pending = 2,
    Ignored = 3,
}

impl PartialDataStatus {
    pub fn from_u32(value: u32) -> PartialDataStatus {
        match value {
            0 => PartialDataStatus::Unknown,
            1 => PartialDataStatus::SucessOverwriteOrLog,
            2 => PartialDataStatus::Pending,
            3 => PartialDataStatus::Ignored,
            _ => {
                error!("Unknown value: {value}");
                PartialDataStatus::Unknown
            }
        }
    }

    pub fn to_u32(value: PartialDataStatus) -> u32 {
        match value {
            PartialDataStatus::Unknown => 0,
            PartialDataStatus::SucessOverwriteOrLog => 1,
            PartialDataStatus::Pending => 2,
            PartialDataStatus::Ignored => 3,
        }
    }

    pub fn to_string(value: PartialDataStatus) -> String {
        match value {
            PartialDataStatus::Unknown => "Unknown".to_string(),
            PartialDataStatus::SucessOverwriteOrLog => "SucessOverwriteOrLog".to_string(),
            PartialDataStatus::Pending => "Pending".to_string(),
            PartialDataStatus::Ignored => "Ignored".to_string(),
        }
    }

    pub fn from_str(value: &str) -> PartialDataStatus {
        match value {
            "Unknown" => PartialDataStatus::Unknown,
            "SucessOverwriteOrLog" => PartialDataStatus::SucessOverwriteOrLog,
            "Pending" => PartialDataStatus::Pending,
            "Ignored" => PartialDataStatus::Ignored,
            _ => PartialDataStatus::Unknown,
        }
    }
}
