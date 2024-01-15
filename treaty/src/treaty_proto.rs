/// Responds with information about the backing database technology used at this Treaty instance.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackingDatabaseConfigReply {
    /// The backing database type. This corresponds to the DatabaseType enum.
    #[prost(uint32, tag = "1")]
    pub database_type: u32,
    /// Specifies if the database technology is using a schema for Treaty settings instead of a
    /// seperate database, if applicable. Default is false.
    #[prost(bool, tag = "2")]
    pub use_schema: bool,
    /// An error if Treaty was unable to return the database information.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// A message describing an error in the system. This usually refers to a SQL database error.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TreatyError {
    /// A description of what went wrong.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// An optional description of how to fix the error.
    #[prost(string, optional, tag = "2")]
    pub help: ::core::option::Option<::prost::alloc::string::String>,
    /// Not used: A numerical value tied to the error.
    #[prost(uint32, tag = "3")]
    pub number: u32,
}
/// A message describing a potential problem in the system.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TreatyWarning {
    /// A description of a problem.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// An optional description of how to fix the error.
    #[prost(string, optional, tag = "2")]
    pub help: ::core::option::Option<::prost::alloc::string::String>,
    /// Not used: A numerical value tied to the error.
    #[prost(uint32, tag = "3")]
    pub number: u32,
}
/// A log entry within Treaty.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TreatyLogEntry {
    /// The local datetime of the log entry.
    #[prost(string, tag = "1")]
    pub dt: ::prost::alloc::string::String,
    /// The UTC datetime of the log entry.
    #[prost(string, tag = "2")]
    pub dt_utc: ::prost::alloc::string::String,
    /// The logging level. In order of severity: Error, Warn, Info, Debug, Trace.
    #[prost(string, tag = "3")]
    pub level: ::prost::alloc::string::String,
    /// The actual log message.
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
}
/// Requests Treaty to return the last X number of logs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogsByLastNumberRequest {
    /// The last number of logs to fetch.
    #[prost(uint32, tag = "1")]
    pub number_of_logs: u32,
}
/// Responds with the total requested number of logs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogsByLastNumberReply {
    /// A list of log entries.
    #[prost(message, repeated, tag = "1")]
    pub logs: ::prost::alloc::vec::Vec<TreatyLogEntry>,
    /// An error if Treaty was unable to fetch logs.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Responds with the current Settings.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingsReply {
    /// The settings in JSON format.
    #[prost(string, optional, tag = "1")]
    pub settings_json: ::core::option::Option<::prost::alloc::string::String>,
    /// An error if Treaty was unable to return the settings.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Responds with a list of hosts that this Treaty instance is cooperating with.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCooperativeHostsReply {
    /// The list of hosts this Treaty instance is cooperating with.
    #[prost(message, repeated, tag = "1")]
    pub hosts: ::prost::alloc::vec::Vec<HostInfoStatus>,
    /// An error if Treaty was unable to return the list of Hosts.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the current "DeletesToHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeletesToHostBehaviorRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
}
/// Responds with the current "DeletesToHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeletesToHostBehaviorReply {
    /// The current behavior for the requested database and table.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, optional, tag = "1")]
    pub behavior: ::core::option::Option<u32>,
    /// An error if Treaty was unable to return the current behavior.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the current "DeletesFromHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeletesFromHostBehaviorRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
}
/// Responds with the current "DeletesFromHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeletesFromHostBehaviorReply {
    /// The current behavior for the requested database and table.
    /// This value is defined in the /treaty/treaty-types/enum.rs file.
    #[prost(uint32, optional, tag = "1")]
    pub behavior: ::core::option::Option<u32>,
    /// An error if Treaty was unable to return the current behavior.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the current "UpdatesToHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesToHostBehaviorRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
}
/// Responds with the current "UpdatesToHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesToHostBehaviorReply {
    /// The current behavior for the for the requested database and table.
    /// This value is defined in the /treaty/treaty-types/enum.rs file.
    #[prost(uint32, optional, tag = "1")]
    pub behavior: ::core::option::Option<u32>,
    /// An error if Treaty was unable to return the current behavior.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the current "UpdatesFromHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesFromHostBehaviorRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
}
/// Responds with the current "UpdatesFromHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesFromHostBehaviorReply {
    /// The current behavior for the requested database and table.
    /// This value is defined in the /treaty/treaty-types/enum.rs file.
    #[prost(uint32, optional, tag = "1")]
    pub behavior: ::core::option::Option<u32>,
    /// An error if Treaty was unable to return the current behavior.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Replies with the current version of Treaty at this instance,.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionReply {
    /// The version of Treaty.
    #[prost(message, optional, tag = "1")]
    pub versions: ::core::option::Option<Versions>,
    /// An error if Treaty was unable to return the current version numbers.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// The version of Treaty.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Versions {
    #[prost(string, optional, tag = "1")]
    pub treaty: ::core::option::Option<::prost::alloc::string::String>,
}
/// Replies with the current Host Info at this Treaty instance.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostInfoReply {
    /// The host information.
    #[prost(message, optional, tag = "1")]
    pub host_info: ::core::option::Option<Host>,
    /// An error if Treaty was unable to return the current host information.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Replies with the result of attempting to revoke the current Json Web Token.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeReply {
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
}
/// Replies with an issued Json Web Token.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenReply {
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    #[prost(string, tag = "2")]
    pub expiration_utc: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub jwt: ::prost::alloc::string::String,
}
/// Requests the current Active Contract for the specified database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActiveContractRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
}
/// Replies with the active contract for the specified database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActiveContractReply {
    /// The active database contract.
    #[prost(message, optional, tag = "1")]
    pub contract: ::core::option::Option<Contract>,
    /// An error if Treaty was unable to return the Active Contract for the specified database.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests a list of participants for the specified database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParticipantsRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
}
/// Replies with the list of Participants for the specified database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParticipantsReply {
    /// A list of participants for the specified database.
    #[prost(message, repeated, tag = "1")]
    pub participants: ::prost::alloc::vec::Vec<ParticipantStatus>,
    /// If the request has an error.
    #[prost(bool, tag = "2")]
    pub is_error: bool,
    /// An error if Treaty was unable to return the list of participants for the specified database.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Replies with the list of databses at Tretay.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabasesReply {
    /// The databases hosted at this Treaty instance.
    #[prost(message, repeated, tag = "1")]
    pub databases: ::prost::alloc::vec::Vec<DatabaseSchema>,
    /// An error if Treaty was unable to return the list of databases.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests to accept a pending action at a Participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptPendingActionRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The row id.
    #[prost(uint32, tag = "3")]
    pub row_id: u32,
}
/// Replies with the result of accepting a pending action at a Participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptPendingActionReply {
    /// If the acceptance of the action is successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// An error if Treaty was unable to accept the action.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests a list of pending actions at a Participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPendingActionsRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The type of action we are interested in (UPDATE or DELETE)
    #[prost(string, tag = "3")]
    pub action: ::prost::alloc::string::String,
}
/// Replies with a list of pending actions (statements).
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPendingActionsReply {
    /// A list of pending statements to be executed.
    #[prost(message, repeated, tag = "1")]
    pub pending_statements: ::prost::alloc::vec::Vec<PendingStatement>,
    /// An error if Treaty was unable to get the list of pending actions.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// A statement that is queued to be executed at a Treaty instance.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingStatement {
    /// The row id being affected
    #[prost(uint32, tag = "1")]
    pub row_id: u32,
    /// The UPDATE or DELETE statement
    #[prost(string, tag = "2")]
    pub statement: ::prost::alloc::string::String,
    /// The time in UTC the request was made
    #[prost(string, tag = "3")]
    pub requested_ts_utc: ::prost::alloc::string::String,
    /// The host id requesting the action
    #[prost(string, tag = "4")]
    pub host_id: ::prost::alloc::string::String,
    /// The actual SQL statement being executed
    #[prost(string, tag = "5")]
    pub action: ::prost::alloc::string::String,
}
/// Requests that a data log be enabled for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataLogTableStatusRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// If a data log should be enabled.
    #[prost(bool, tag = "3")]
    pub use_data_log: bool,
}
/// Replies with the result of configuring a data log.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataLogTableStatusReply {
    /// If the request was successful or not.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// An error if Treaty was unable to set the requested status of data logging.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the status of data logging for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataLogTableStatusRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
}
/// Replies with the status of data logging for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataLogTableStatusReply {
    /// If data logging was configured or not.
    #[prost(bool, tag = "1")]
    pub use_data_log: bool,
    /// An error if Treaty was unable to get the status of data logging.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the row ids for the specified WHERE clause.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReadRowIdsRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The WHERE clause.
    #[prost(string, tag = "3")]
    pub where_clause: ::prost::alloc::string::String,
}
/// Replies with a list of row ids for the request.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReadRowIdsReply {
    /// The list of row ids.
    #[prost(uint32, repeated, tag = "1")]
    pub row_ids: ::prost::alloc::vec::Vec<u32>,
    /// An error if Treaty was unable to get the list of affected row ids.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the saved data hash for the specified row id.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataHashRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The row id.
    #[prost(uint32, tag = "3")]
    pub row_id: u32,
}
/// Returns the requested data hash.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataHashReply {
    /// The requested data hash.
    #[prost(uint64, tag = "1")]
    pub data_hash: u64,
    /// An error if Treaty was unable to get the requested data hash.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to change the "DeletesToHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDeletesToHostBehaviorRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The "DeletesToHost" before setting.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "3")]
    pub behavior: u32,
}
/// Replies with the result of the request to change the "DeletesToHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDeletesToHostBehaviorReply {
    /// If the request was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A message if any additional information is needed. This value can be empty.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// An error if Treaty was unable to get the requested data hash.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// A request to change the "UpdatesToHostBehavior".
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeUpdatesToHostBehaviorRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The behavior to change to.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "3")]
    pub behavior: u32,
}
/// Replies with the result of attempting to change the "UpdatesToHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeUpdatesToHostBehaviorReply {
    /// If the request was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A message if any additional information is needed. This value can be empty.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// An error if Treaty was unable to set the behavior.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to change the "DeletesFromHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDeletesFromHostBehaviorRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The behavior to change to.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "3")]
    pub behavior: u32,
}
/// Replies with the result of changing the "DeletesFromHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDeletesFromHostBehaviorReply {
    /// If the request was successful or not.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A message if any additional information is available. This value can be empty.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// An error if Treaty was unable to change the behavior.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to change the "UpdatesFromHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeUpdatesFromHostBehaviorRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The behavior to change to.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "3")]
    pub behavior: u32,
}
/// Replies with the result of changing the "UpdatesFromHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangesUpdatesFromHostBehaviorReply {
    /// If the request was successful or not.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A message with any additional information. This value can be empty.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// An error if Treaty was unable to change the behavior.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// A request to attempt to authenticate at the specified Participant. This tests to make sure that we
/// have not been rejected by the specified participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryAuthAtParticipantRequest {
    /// The participant id.
    #[prost(string, tag = "1")]
    pub participant_id: ::prost::alloc::string::String,
    /// The participant alias.
    #[prost(string, tag = "2")]
    pub participant_alias: ::prost::alloc::string::String,
    /// The database name.
    #[prost(string, tag = "3")]
    pub db_name: ::prost::alloc::string::String,
}
/// Replies with the result of attempting to autenticate at the specified Participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryAuthAtPartipantReply {
    /// If the result was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A message with any additional information. This value can be empty.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// An error if Treaty was unable to attempt authentication.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to change the status of a Host to ALLOW/DENY.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHostStatusRequest {
    /// The host alias.
    #[prost(string, tag = "1")]
    pub host_alias: ::prost::alloc::string::String,
    /// The host id.
    #[prost(string, tag = "2")]
    pub host_id: ::prost::alloc::string::String,
    /// The status to change for the host.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "3")]
    pub status: u32,
}
/// Replies with the result of changing the host status.
/// This can return "false" if the host id or alias was not found in the database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHostStatusReply {
    /// If the request was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// The status the value was changed to. This echoes what was sent.
    #[prost(uint32, tag = "2")]
    pub status: u32,
    /// An error if Treaty was unable to change the host status.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to generate host information for Treaty.
/// This will assign our Treaty instance with the specified host name and an auto
/// generated UUID if not already set. Otherwise, it will update the host name and token.
///
/// ❗ WARNING: If calling this request, this will re-generate the host token which is used to identify this
/// Treaty instance to other Treaty instances.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateHostInfoRequest {
    /// The friendly host name to use.
    #[prost(string, tag = "1")]
    pub host_name: ::prost::alloc::string::String,
}
/// Replies with the result of attempting to generate host information.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateHostInfoReply {
    /// If creating host information was successful or not.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// An error if Treaty was unable to generate host information.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests to send the active database contract ot the specified participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendParticipantContractRequest {
    /// The name of the database.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The alias of the participant.
    #[prost(string, tag = "2")]
    pub participant_alias: ::prost::alloc::string::String,
}
/// Replies with the result of sending the active contract to the participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendParticipantContractReply {
    /// If the contract was sent.
    #[prost(bool, tag = "1")]
    pub is_sent: bool,
    /// The current status of the contract at the participant.
    /// This is an echo of what the Participant thinks the contract status is.
    #[prost(uint32, tag = "2")]
    pub contract_status: u32,
    /// An error if Treaty was unable to send the active contract to the Participant.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// A message representing the results of a SQL query.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatementResultset {
    /// The total number of rows affected, if applicable.
    #[prost(uint64, tag = "1")]
    pub number_of_rows_affected: u64,
    /// A list of Row items.
    #[prost(message, repeated, tag = "2")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
    /// An error if Treaty was unable to provide results.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
    /// A warning if there is a data mis-match.
    #[prost(message, optional, tag = "4")]
    pub warning: ::core::option::Option<TreatyWarning>,
}
/// Requests to create a database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserDatabaseRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
}
/// Delete user database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserDatabaseRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
}
/// Replies with the result of deleting a database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserDatabaseReply {
    /// If the database was deleted.
    #[prost(bool, tag = "1")]
    pub is_deleted: bool,
    /// A message describing any details if needed. This field can be blank.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// An error if Treaty was unable to delete the requested database.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Replies with the result of creating a database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserDatabaseReply {
    /// If the database was created.
    #[prost(bool, tag = "1")]
    pub is_created: bool,
    /// A message describing any details if needed. This field can be blank.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// An error if Treaty was unable to create the requested database.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests to execute the specified SELECT statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteReadRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The SELECT SQL statement.
    #[prost(string, tag = "2")]
    pub sql_statement: ::prost::alloc::string::String,
    /// The datababase type (Sqlite, Postgres)
    #[prost(uint32, tag = "3")]
    pub database_type: u32,
}
/// Replies with the result of the SELECT statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteReadReply {
    /// The total number of result-sets.
    #[prost(uint64, tag = "1")]
    pub total_resultsets: u64,
    /// The results of the query.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<StatementResultset>,
    /// Denotes if there was an error executing the query.
    #[prost(bool, tag = "3")]
    pub is_error: bool,
    /// An error if Treaty was unable to execute the SELECT statement provided.
    #[prost(message, optional, tag = "4")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests to execute the provided INSERT/UPDATE/DELETE statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteWriteRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The INSERT/UPDATE/DELETE statement to execute.
    #[prost(string, tag = "2")]
    pub sql_statement: ::prost::alloc::string::String,
    /// The database type (Sqlite, Postgres).
    #[prost(uint32, tag = "3")]
    pub database_type: u32,
    /// The WHERE clause of the statement, if applicable.
    /// ℹ️ Note: If the "sql_statement" includes a WHERE clause, duplicate the contents here. Otherwise, leave the string empty.
    /// This is needed because of a limitation with Treaty's implementation of Antlr. In the future, hopefully this field will not be needed.
    #[prost(string, tag = "4")]
    pub where_clause: ::prost::alloc::string::String,
}
/// Replies with the results of the provided INSERT/UPDATE/DELETE statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteWriteReply {
    /// If the statement executed without error.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// The total number of rows the statement affected, if applicable.
    #[prost(uint32, tag = "2")]
    pub total_rows_affected: u32,
    /// Denotes if there was an error executing the statement.
    #[prost(bool, tag = "3")]
    pub is_error: bool,
    /// An error if Treaty was uanble to execute the INSERT/UPDATE/DELETE statement provided.
    #[prost(message, optional, tag = "4")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests to find out if the specified table exists.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HasTableRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
}
/// Replies if the specified table exists.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HasTableReply {
    /// If the table exists or not.
    #[prost(bool, tag = "1")]
    pub has_table: bool,
    /// An error if Treaty was uanble to determine if the specified table exists or not.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// A request to generate a contract for the specified database.
/// If there is already a database contract for this database, it will be marked as inactive and a new one
/// will be generated.
/// ❗ Note: You will need to ensure that each table in your database has a Logical Storage Policy set before
/// generating a contract, otherwise this call will fail.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateContractRequest {
    /// A host name to identify this Treaty instance to others.
    #[prost(string, tag = "1")]
    pub host_name: ::prost::alloc::string::String,
    /// A general description for the contract.
    /// This will be made visible to Participants.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// The name of the database this contract is for.
    #[prost(string, tag = "3")]
    pub database_name: ::prost::alloc::string::String,
    /// The Remote Delete Behavior for this Host for this contract.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "4")]
    pub remote_delete_behavior: u32,
}
/// Replies with the status of generating a contract for the specified database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateContractReply {
    /// If contract generation was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A message providing any additional details. This value can be empty.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// An error if Treaty was unable to generate the contract.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests Treaty to set the specified Logical Storage Policy for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLogicalStoragePolicyRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The policy to set the table to.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "3")]
    pub policy_mode: u32,
}
/// Replies with the result of setting the Logical Storage Policy for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLogicalStoragePolicyReply {
    /// If the request was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A message providing any additional information. This value can be empty.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// An error if Treaty was unable to generate the contract.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Request the current Logical Storage Policy for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogicalStoragePolicyRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
}
/// Replies with the current Logical Storage Policy for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogicalStoragePolicyReply {
    /// The current Logical Storage policy for the requested table.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "1")]
    pub policy_mode: u32,
    /// An error if Treaty was unable to get the Logical Storage Policy for the specified table.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests Treaty to execute the specified INSERT/UPDATE/DELETE statement both at the
/// Host and the Participant.
/// This attempts to execute the action at the Participant and if successful
/// will keep metadata about the action at the Host.
/// For more information, see the README.md or the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteCooperativeWriteRequest {
    /// The name of the database.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The INSERT/UPDATE/DELETE statement to execute at the Participant.
    #[prost(string, tag = "2")]
    pub sql_statement: ::prost::alloc::string::String,
    /// The type of database: Sqlite, Postgres.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "3")]
    pub database_type: u32,
    /// The participant alias this statement is for.
    #[prost(string, tag = "4")]
    pub alias: ::prost::alloc::string::String,
    /// The participant id this statement is for.
    #[prost(string, tag = "5")]
    pub participant_id: ::prost::alloc::string::String,
    /// The WHERE clause of the INSERT/UPDATE/STATEMENT. For technical reasons this needs to be the same as in the "sql_statement" field
    /// if applicable. This field can be empty.
    #[prost(string, tag = "6")]
    pub where_clause: ::prost::alloc::string::String,
}
/// Replies with the result of Cooperative Write.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteCooperativeWriteReply {
    /// If the result was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// The total number of rows affected by the INSERT/UPDATE/DELETE statement.
    #[prost(uint32, tag = "2")]
    pub total_rows_affected: u32,
    /// An error if Treaty was unable to execute the Cooperative Write.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to add the Participant to the specified database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddParticipantRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// An alias for this participant.
    #[prost(string, tag = "2")]
    pub alias: ::prost::alloc::string::String,
    /// The IP address for this Participant, in IP 4 format.
    #[prost(string, tag = "3")]
    pub ip4_address: ::prost::alloc::string::String,
    /// The database port number for this Participant.
    #[prost(uint32, optional, tag = "4")]
    pub db_port: ::core::option::Option<u32>,
    /// The info port number for this Participant.
    #[prost(uint32, tag = "5")]
    pub info_port: u32,
    /// The HTTP address for this Participant.
    #[prost(string, tag = "6")]
    pub http_addr: ::prost::alloc::string::String,
    /// The HTTP port for this Participant.
    #[prost(uint32, tag = "7")]
    pub http_port: u32,
    /// The Host Id for this participant. This field is optional. This is used if Treaty is being hosted by a
    /// `treaty-proxy` instance, where multiple Treaty instances are hosted and you need to uniquely identify which Treaty instance
    /// you are attempting to talk to.
    #[prost(string, optional, tag = "8")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Replies with the result of adding a Participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddParticipantReply {
    /// If adding the Participant was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A message describing any additional details if needed. This field can be empty.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// An error if Treaty was unable to add the Participant.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Replies with a list of pending contracts.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewPendingContractsReply {
    /// A list of contracts that are in a pending state. This list may be empty.
    #[prost(message, repeated, tag = "1")]
    pub contracts: ::prost::alloc::vec::Vec<Contract>,
    /// An error if Treaty was unable to get the list of pending contracts.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests Treaty to accept the pending contract from the specified Host,.
/// This will send a message back to the host that we are ready to accept data
/// and will create additional meta-data structures to support the contract.
/// For more information, see the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptPendingContractRequest {
    /// The host that has sent us the pending contract.
    #[prost(string, tag = "1")]
    pub host_alias: ::prost::alloc::string::String,
}
/// Replies with the result of accepting a pending contract,.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptPendingContractReply {
    /// If the request was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A message with any additional information. This field may be blank.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// An error if Treaty was unable to accept the pending contract.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests that Treaty reject the pending contract from the specified host,.
/// This sends a message back to the Host that we are not interested in this contract. No database changes are made.
/// For more information, see the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejectPendingContractRequest {
    /// The alias of the host.
    #[prost(string, tag = "1")]
    pub host_alias: ::prost::alloc::string::String,
}
/// Replies with the result of rejecting a pending contract.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejectPendingContractReply {
    /// If the rejection was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A message with any additional information. This field may be blank.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// An error if Treaty was unable to reject the pending contract.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests that Treaty enable cooperative features for a database, if authentiated.
/// This modifies the database with additional structures to support adding Participants and other related actions.
/// For more information, see the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableCoooperativeFeaturesRequest {
    /// The database name to enable cooperative features.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
}
/// Replies with the result of enabling cooperative features,.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableCoooperativeFeaturesReply {
    /// If enabling cooperative features was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A message containing any additional details. This field may be blank.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// An error if Treaty was unable to enable cooperative features on the specified database.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Replies with the authentication result.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryAuthResult {
    /// returns if the authentication is valid
    #[prost(bool, tag = "1")]
    pub is_authenticated: bool,
}
/// A message for creating a table in a database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableRequest {
    /// The database in which to create the table.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The database GUID in which to create the table.
    #[prost(string, tag = "2")]
    pub database_guid: ::prost::alloc::string::String,
    /// The name of the table to create.
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    /// A list of columns for the table.
    #[prost(message, repeated, tag = "4")]
    pub columns: ::prost::alloc::vec::Vec<ColumnSchema>,
}
/// A message for describing the result of a CreateTableRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableResult {
    /// If the table was created.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// The name of the database the table was created in.
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    /// Any additional information if needed. This field can be blank.
    #[prost(string, tag = "3")]
    pub result_message: ::prost::alloc::string::String,
    /// The database id the table was created in.
    #[prost(string, tag = "4")]
    pub database_id: ::prost::alloc::string::String,
    /// The table name that was created. This should line up with the request made and is intended for confirmation.
    #[prost(string, tag = "5")]
    pub table_name: ::prost::alloc::string::String,
    /// The table id that was created.
    #[prost(string, tag = "6")]
    pub table_id: ::prost::alloc::string::String,
    /// If the request failed in any manner.
    #[prost(bool, tag = "7")]
    pub is_error: bool,
    /// An error detailing if the request failed in any manner.
    #[prost(message, optional, tag = "8")]
    pub error: ::core::option::Option<TreatyError>,
}
/// A message describing the details of a row in a partial database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowInfo {
    /// The name of the database the row is in.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name the row is in.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The row id.
    #[prost(uint32, tag = "3")]
    pub row_id: u32,
    /// The data hash of the row.
    #[prost(uint64, tag = "4")]
    pub data_hash: u64,
}
/// A request for Treaty to execute the specified INSERT statement.
/// ❗ Warning: At the moment, Treaty can only handle simple INSERT statements for a single row.
/// For more information, see the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertDataRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The actual INSERT statement.
    /// Note: while this is duplicative, at the moment the contents of this INSERT statement must match the database and table name.
    #[prost(string, tag = "3")]
    pub cmd: ::prost::alloc::string::String,
}
/// A result of executing an INSERT statement against a partial database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertDataResult {
    /// If the result was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A hash of the data inserted.
    #[prost(uint64, tag = "2")]
    pub data_hash: u64,
    /// An additional message if needed. This field can be blank.
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    /// The row id of the record inserted.
    #[prost(uint32, tag = "4")]
    pub row_id: u32,
    /// If there was an error executing the INSERT statement.
    #[prost(bool, tag = "5")]
    pub is_error: bool,
    /// An error detailing if the request failed.
    #[prost(message, optional, tag = "6")]
    pub error: ::core::option::Option<TreatyError>,
}
/// A request for Treaty to execute the specified UPDATE statement if authentiated.
/// ❗ Warning: At the moment, Treaty can only handle simple UPDATE statements.
/// For more information, see the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The actual UPDATE statement.
    #[prost(string, tag = "3")]
    pub cmd: ::prost::alloc::string::String,
    /// The where clause
    #[prost(string, tag = "4")]
    pub where_clause: ::prost::alloc::string::String,
}
/// Replies with the result of executing the provided UPDATE statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataResult {
    /// If the UPDATE statement was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A message describing any additional details. This field can be blank.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// A copy of the rows that were affected.
    #[prost(message, repeated, tag = "3")]
    pub rows: ::prost::alloc::vec::Vec<RowInfo>,
    /// The status of the actual update. Values are:
    /// 0 - unknown
    /// 1 - success (overwrite or overwrite with log)
    /// 2 - pending (queue for review)
    /// 3 - ignored (ignore)
    /// Note: These values are defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "4")]
    pub update_status: u32,
    /// If there was an error executing the UPDATE statement.
    #[prost(bool, tag = "5")]
    pub is_error: bool,
    /// Any details if there was an error executing the UPDATE statement.
    #[prost(message, optional, tag = "6")]
    pub error: ::core::option::Option<TreatyError>,
}
/// A request for Treaty to execute the provided DELETE statement.
/// ❗ Warning: At the moment, Treaty can only handle simple UPDATE statements.
/// For more information, see the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataRequest {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The actual DELETE statement.
    /// Note: This DELETE statement needs to match the field specified prior.
    #[prost(string, tag = "3")]
    pub cmd: ::prost::alloc::string::String,
    /// ❗ The WHERE clause of the delete statement. This field needs to match the WHERE clause if there is one in the prior field.
    /// Otherwise, this field can be left blank.
    #[prost(string, tag = "4")]
    pub where_clause: ::prost::alloc::string::String,
}
/// Describes the result of Treaty executing the specified DELETE statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataResult {
    /// If the command was successfully executed.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// A message providing further details if needed. This field can be blank.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// A message describing details of the rows impacted.
    #[prost(message, repeated, tag = "3")]
    pub rows: ::prost::alloc::vec::Vec<RowInfo>,
    /// Denotes if there was an error executing the DELETE statement.
    #[prost(bool, tag = "4")]
    pub is_error: bool,
    /// An error describing details if needed.
    #[prost(message, optional, tag = "5")]
    pub error: ::core::option::Option<TreatyError>,
}
/// A request to get a specified row from a partial database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowFromPartialDatabaseRequest {
    /// The row which to get.
    #[prost(message, optional, tag = "1")]
    pub row_address: ::core::option::Option<RowParticipantAddress>,
    /// Additional details for debugging purposes.
    #[prost(message, optional, tag = "2")]
    pub telemetry: ::core::option::Option<Telemetry>,
}
/// A response containing the specified row requested,.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowFromPartialDatabaseResult {
    /// If the request was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// Any additional details if needed. This field can be blank.
    #[prost(string, tag = "2")]
    pub result_message: ::prost::alloc::string::String,
    /// The actual row requested.
    #[prost(message, optional, tag = "3")]
    pub row: ::core::option::Option<Row>,
    /// An error if Treaty was unable to get the specified row.
    #[prost(message, optional, tag = "4")]
    pub error: ::core::option::Option<TreatyError>,
}
/// A message from a host to a participant to save a contract.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveContractRequest {
    /// A contract to save.
    #[prost(message, optional, tag = "1")]
    pub contract: ::core::option::Option<Contract>,
    /// Any additional debugging details.
    #[prost(message, optional, tag = "2")]
    pub telemetry: ::core::option::Option<Telemetry>,
    /// The host id of the targeted Treaty instance. This is usually used if `treaty` is being hosted by a `treaty-proxy` instance.
    #[prost(string, optional, tag = "3")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
/// A message describing the results of saving a contract.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveContractResult {
    /// If the contract was saved.
    #[prost(bool, tag = "1")]
    pub is_saved: bool,
    /// A message confirming the Participant's status of the contract (Accepted/Rejected/Pending)
    #[prost(uint32, tag = "2")]
    pub contract_status: u32,
    /// If the Participant wishes to confirm their information back to the Host. This is useful
    /// If the Host and the Participant are out of sync with the contract status.
    #[prost(message, optional, tag = "3")]
    pub participant_info: ::core::option::Option<Participant>,
    /// If there was an error saving the contract.
    #[prost(bool, tag = "4")]
    pub is_error: bool,
    /// Any details if Treaty was unable to save the contract.
    #[prost(message, optional, tag = "5")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to accept a particular contract.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantAcceptsContractRequest {
    /// The participant accepting the contract. This is used as a way to identify the Participant.
    #[prost(message, optional, tag = "1")]
    pub participant: ::core::option::Option<Participant>,
    /// The GUID/UUID of the contract.
    #[prost(string, tag = "2")]
    pub contract_guid: ::prost::alloc::string::String,
    /// The GUID/UUID version of the contract.
    /// Contracts can be updated, and so with each change of a contract the version must be changed.
    #[prost(string, tag = "3")]
    pub contract_version_guid: ::prost::alloc::string::String,
    /// The database name.
    #[prost(string, tag = "4")]
    pub database_name: ::prost::alloc::string::String,
    /// Any additional debugging information.
    #[prost(message, optional, tag = "5")]
    pub telemetry: ::core::option::Option<Telemetry>,
    /// The host id of the targeted Treaty instance. This is usually used if `treaty` is being hosted by a `treaty-proxy` instance.
    #[prost(string, optional, tag = "6")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Describes the result of notifying that a Participant has accepted a contract.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantAcceptsContractResult {
    /// If the result is acknowledged.
    #[prost(bool, tag = "1")]
    pub contract_acceptance_is_acknowledged: bool,
    /// If there was an error notifying acceptance of the contract.
    #[prost(bool, tag = "2")]
    pub is_error: bool,
    /// An error describing details for the action, if appliable.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Request from a Participant to a Host to update the data hash.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRowDataHashForHostRequest {
    /// Additional telemetry for debugging.
    #[prost(message, optional, tag = "1")]
    pub telemetry: ::core::option::Option<Telemetry>,
    /// The host information this data hash came from (from the perspective of the Host, this is the Participant's information).
    #[prost(message, optional, tag = "2")]
    pub host_info: ::core::option::Option<Host>,
    /// The database name.
    #[prost(string, tag = "3")]
    pub database_name: ::prost::alloc::string::String,
    /// The database id.
    #[prost(string, tag = "4")]
    pub database_id: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "5")]
    pub table_name: ::prost::alloc::string::String,
    /// The table id.
    #[prost(uint32, tag = "6")]
    pub table_id: u32,
    /// The row id.
    #[prost(uint32, tag = "7")]
    pub row_id: u32,
    /// The new hash value for the row.
    #[prost(uint64, tag = "8")]
    pub updated_hash_value: u64,
    /// If the row is deleted.
    #[prost(bool, tag = "9")]
    pub is_deleted_at_participant: bool,
}
/// Replies with the result of the update data hash request.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRowDataHashForHostResult {
    /// If the message was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// An error if the updated data hash could not be sent.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to notify the upstream Host that a row has been deleted.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyHostOfRemovedRowRequest {
    /// Debugging information about the sender of this message.
    #[prost(message, optional, tag = "1")]
    pub telemetry: ::core::option::Option<Telemetry>,
    /// The host information. From an upstream Host's perspective, this is the Participant.
    #[prost(message, optional, tag = "2")]
    pub host_info: ::core::option::Option<Host>,
    /// The database name.
    #[prost(string, tag = "3")]
    pub database_name: ::prost::alloc::string::String,
    /// The database id.
    #[prost(string, tag = "4")]
    pub database_id: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "5")]
    pub table_name: ::prost::alloc::string::String,
    /// The table id.
    #[prost(uint32, tag = "6")]
    pub table_id: u32,
    /// The row id.
    #[prost(uint32, tag = "7")]
    pub row_id: u32,
}
/// The result of notifying the upstream Host that a row has been deleted.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyHostOfRemovedRowResult {
    /// If the notification was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// An error if Treaty was not able to notify the upstream Host.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<TreatyError>,
}
/// A message for basic online testing.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestRequest {
    /// The time the request was sent in UTC (RFC3339)
    #[prost(string, tag = "1")]
    pub request_time_utc: ::prost::alloc::string::String,
    /// The origin URL, if applicable.
    #[prost(string, tag = "2")]
    pub request_origin_url: ::prost::alloc::string::String,
    /// The origin IP4 address.
    #[prost(string, tag = "3")]
    pub request_origin_ip4: ::prost::alloc::string::String,
    /// The oring IP6 address.
    #[prost(string, tag = "4")]
    pub request_origin_ip6: ::prost::alloc::string::String,
    /// The origin port number.
    #[prost(uint32, tag = "5")]
    pub request_port_number: u32,
    /// A test message that should be echo'd back.
    #[prost(string, tag = "6")]
    pub request_echo_message: ::prost::alloc::string::String,
}
/// A message for basic online testing.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestReply {
    /// The time the reply was generated in UTC (RFC3339)
    #[prost(string, tag = "1")]
    pub reply_time_utc: ::prost::alloc::string::String,
    /// The message to echo back.
    #[prost(string, tag = "2")]
    pub reply_echo_message: ::prost::alloc::string::String,
    /// The sender's Treaty version.
    #[prost(string, tag = "3")]
    pub treaty_version: ::prost::alloc::string::String,
}
/// A message for general information.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Telemetry {
    /// Endian-ness of the Treaty instance.
    #[prost(bool, tag = "1")]
    pub is_little_endian: bool,
    /// A list of IP addresses for this sender.
    #[prost(string, repeated, tag = "2")]
    pub message_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The time the message was generated in UTC (RFC3339)
    #[prost(string, tag = "3")]
    pub message_generated_time_utc: ::prost::alloc::string::String,
    /// A unique ID for this message.
    #[prost(string, tag = "4")]
    pub message_guid: ::prost::alloc::string::String,
}
/// Credentials to authenticate against Treaty.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequestBasic {
    /// The name of the user.
    #[prost(string, tag = "1")]
    pub user_name: ::prost::alloc::string::String,
    /// The pw of the user.
    #[prost(string, tag = "2")]
    pub pw: ::prost::alloc::string::String,
}
/// Credentials to authenticate against Treaty.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequestWebToken {
    /// A Json Web Token in place of credentials.
    #[prost(string, tag = "1")]
    pub jwt: ::prost::alloc::string::String,
}
/// Credentials to authenticate against Treaty.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequestBinary {
    /// The name of the user.
    #[prost(string, tag = "1")]
    pub user_name: ::prost::alloc::string::String,
    /// A generated token of the pw of the user.
    #[prost(bytes = "vec", tag = "2")]
    pub token: ::prost::alloc::vec::Vec<u8>,
}
/// Additional metadata to support authorization actions.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequestMetadata {
    /// An optional Host Id of the Treaty instance. This is used when talking to a `treaty-proxy` instance.
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of the database to verify authorization, if applicable. This is usually in
    /// the case of Participants sending messages back to the Host.
    #[prost(string, optional, tag = "2")]
    pub db_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// A message explaining the author of the request. Maps to the treaty-types enum of the same name.
/// Values are:
/// - 0 - Unknown
/// - 1 - User
/// - 2 - Data
/// - 3 - Participant
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequestAuthor {
    /// The type who is making the request: a user or a type of
    /// Treaty instance.
    #[prost(uint32, tag = "1")]
    pub author_type: u32,
}
/// A message describing the results of an authentication attempt.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthResult {
    /// If the authentication attempt was successful.
    #[prost(bool, tag = "1")]
    pub is_authenticated: bool,
    /// An optional message for any additional information.
    #[prost(string, optional, tag = "2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// A message for creating a partial database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePartialDatabaseRequest {
    /// Additional debugging information.
    #[prost(message, optional, tag = "1")]
    pub telemetry: ::core::option::Option<Telemetry>,
    /// The database name.
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
}
/// A message describing the results of a CreateDatabaseRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePartialDatabaseResult {
    /// If the partial database creation was successful.
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    /// The name of the database.
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    /// The id of the database.
    #[prost(string, tag = "3")]
    pub database_id: ::prost::alloc::string::String,
    /// If there was an error creating the database.
    #[prost(bool, tag = "4")]
    pub is_error: bool,
    /// An error describing what happened during the request, if applicable.
    #[prost(message, optional, tag = "5")]
    pub error: ::core::option::Option<TreatyError>,
}
/// An object for representing a row in a table. Used for returning data.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The row id.
    #[prost(uint32, tag = "3")]
    pub row_id: u32,
    /// A list of values held by the row.
    #[prost(message, repeated, tag = "4")]
    pub values: ::prost::alloc::vec::Vec<RowValue>,
    /// Deprecated. This will be deleted.
    #[prost(bool, tag = "5")]
    pub is_remoteable: bool,
    /// A description about the row such as if the data is out of sync between a Host and a Participant.
    #[prost(message, optional, tag = "6")]
    pub remote_metadata: ::core::option::Option<RowRemoteMetadata>,
    /// A hash of the row's data.
    #[prost(bytes = "vec", tag = "7")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// An object for storing values for a row in a table. Used for returning data.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowValue {
    /// The column of the value.
    #[prost(message, optional, tag = "1")]
    pub column: ::core::option::Option<ColumnSchema>,
    /// If the value is NULL.
    #[prost(bool, tag = "2")]
    pub is_null_value: bool,
    /// The binary value.
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// A string representation of the value.
    #[prost(string, tag = "4")]
    pub string_value: ::prost::alloc::string::String,
}
/// Describes the data status of the host in relation to the Participant.
/// Example the data hash between the host and the participant do not match
/// or if the row was deleted at the participant, but the reference at the host is not.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowRemoteMetadata {
    #[prost(bool, tag = "1")]
    pub is_remote_out_of_sync_with_host: bool,
    #[prost(bool, tag = "2")]
    pub is_hash_out_of_sync_with_host: bool,
    #[prost(bool, tag = "3")]
    pub is_remote_deleted: bool,
    #[prost(bool, tag = "4")]
    pub is_local_deleted: bool,
}
/// A message for describing schema information of a column in a database table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnSchema {
    /// The column name.
    #[prost(string, tag = "1")]
    pub column_name: ::prost::alloc::string::String,
    /// The column type.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "2")]
    pub column_type: u32,
    /// The max or fixed length of the column, if applicable.
    #[prost(uint32, tag = "3")]
    pub column_length: u32,
    /// If the column is nullable or not.
    #[prost(bool, tag = "4")]
    pub is_nullable: bool,
    /// The ordinal value of the column, i.e. the order in which the column appears in the table.
    #[prost(uint32, tag = "5")]
    pub ordinal: u32,
    /// Empty string in a request, populated in a response with the table GUID the column is attached to.
    #[prost(string, tag = "6")]
    pub table_id: ::prost::alloc::string::String,
    /// Empty string in a request, populated in a response with the column GUID value.
    #[prost(string, tag = "7")]
    pub column_id: ::prost::alloc::string::String,
    /// If the column is the primary key of the table. If this is part of a list of columns, it is implied to be a composite primary key.
    #[prost(bool, tag = "8")]
    pub is_primary_key: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    /// the unique contract id
    #[prost(string, tag = "1")]
    pub contract_guid: ::prost::alloc::string::String,
    /// a description of the rights in the contract
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// the schema of the entire database
    #[prost(message, optional, tag = "3")]
    pub schema: ::core::option::Option<DatabaseSchema>,
    /// a GUID representing the version of the contract
    #[prost(string, tag = "4")]
    pub contract_version: ::prost::alloc::string::String,
    /// The host for the contract.
    #[prost(message, optional, tag = "5")]
    pub host_info: ::core::option::Option<Host>,
    /// the status of the contract, if applicable
    #[prost(uint32, tag = "6")]
    pub status: u32,
}
/// A message representing information about a Participant in the system.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Participant {
    /// The public GUID/UUID that a Participant identifies itself with.
    #[prost(string, tag = "1")]
    pub participant_guid: ::prost::alloc::string::String,
    /// A friendly alias.
    #[prost(string, tag = "2")]
    pub alias: ::prost::alloc::string::String,
    /// The IP4 address.
    #[prost(string, tag = "3")]
    pub ip4_address: ::prost::alloc::string::String,
    /// The IP6 address.
    #[prost(string, tag = "4")]
    pub ip6_address: ::prost::alloc::string::String,
    /// The database port number.
    #[prost(uint32, tag = "5")]
    pub database_port_number: u32,
    /// The info port number
    #[prost(uint32, tag = "6")]
    pub info_port_number: u32,
    /// A token used for authentication.
    #[prost(bytes = "vec", tag = "7")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    /// An internal generated GUID/UUID for the Participant.
    #[prost(string, tag = "8")]
    pub internal_participant_guid: ::prost::alloc::string::String,
    /// The HTTP address.
    #[prost(string, tag = "9")]
    pub http_addr: ::prost::alloc::string::String,
    /// The HTTP port number.
    #[prost(uint32, tag = "10")]
    pub http_port: u32,
}
/// The status of a Participant at a Host.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantStatus {
    /// The participant details.
    #[prost(message, optional, tag = "1")]
    pub participant: ::core::option::Option<Participant>,
    /// The contract status.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "2")]
    pub contract_status: u32,
}
/// A Host in Treaty.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Host {
    /// The public GUID/UUID the Host identifies itself with.
    #[prost(string, tag = "1")]
    pub host_guid: ::prost::alloc::string::String,
    /// A friendly name for the host.
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    /// A token used for authentication.
    #[prost(bytes = "vec", tag = "3")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    /// Network settings for the Host.
    #[prost(message, optional, tag = "4")]
    pub network: ::core::option::Option<HostNetwork>,
}
/// A Host's network settings.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostNetwork {
    /// The IP4 Address.
    #[prost(string, optional, tag = "1")]
    pub ip4_address: ::core::option::Option<::prost::alloc::string::String>,
    /// The IP6 address.
    #[prost(string, optional, tag = "4")]
    pub ip6_address: ::core::option::Option<::prost::alloc::string::String>,
    /// The database port number.
    #[prost(uint32, optional, tag = "5")]
    pub database_port_number: ::core::option::Option<u32>,
    /// The HTTP address.
    #[prost(string, optional, tag = "7")]
    pub http_addr: ::core::option::Option<::prost::alloc::string::String>,
    /// The HTTP port.
    #[prost(uint32, optional, tag = "8")]
    pub http_port: ::core::option::Option<u32>,
    /// The information port number
    #[prost(uint32, optional, tag = "9")]
    pub info_port_number: ::core::option::Option<u32>,
}
/// A message describing the latest status of a Host.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostInfoStatus {
    /// Host information.
    #[prost(message, optional, tag = "1")]
    pub host: ::core::option::Option<Host>,
    /// The last time a message was seen from this host in UTC (RFC3339)
    #[prost(string, tag = "2")]
    pub last_communcation_utc: ::prost::alloc::string::String,
    /// The current HostStatus.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "3")]
    pub status: u32,
}
/// A message for describing the schema of a database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseSchema {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The database id.
    #[prost(string, tag = "2")]
    pub database_id: ::prost::alloc::string::String,
    /// The tables of the database.
    #[prost(message, repeated, tag = "3")]
    pub tables: ::prost::alloc::vec::Vec<TableSchema>,
    /// The type of database: Sqlite or Postgres.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "4")]
    pub database_type: u32,
    /// The type of Treaty database; i.e. A Host, Partial, or internal Treaty system database.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    #[prost(uint32, tag = "5")]
    pub treaty_database_type: u32,
    /// If the database has cooperative features.
    #[prost(bool, tag = "6")]
    pub cooperation_enabled: bool,
    /// If the database has any participants.
    #[prost(bool, tag = "7")]
    pub has_participants: bool,
}
/// A message for describing the schema information of a table in a database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableSchema {
    /// The table name.
    #[prost(string, tag = "1")]
    pub table_name: ::prost::alloc::string::String,
    /// The table id.
    #[prost(string, tag = "2")]
    pub table_id: ::prost::alloc::string::String,
    /// The database name this table belongs to.
    #[prost(string, tag = "3")]
    pub database_name: ::prost::alloc::string::String,
    /// The database id this table belongs to.
    #[prost(string, tag = "4")]
    pub database_id: ::prost::alloc::string::String,
    /// The columns of the table.
    #[prost(message, repeated, tag = "5")]
    pub columns: ::prost::alloc::vec::Vec<ColumnSchema>,
    /// The Logical Storage Policy for this table.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    /// For more information, see the manual.
    #[prost(uint32, tag = "6")]
    pub logical_storage_policy: u32,
}
/// A message for identifying the location of a row in a partial database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowParticipantAddress {
    /// The database name.
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// The table name.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// The row id.
    #[prost(uint32, tag = "3")]
    pub row_id: u32,
}
/// A message with ports available. These values can be empty depending on what the Treaty host has configured.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TreatyPorts {
    /// The public info port used to provide general public information
    #[prost(uint32, optional, tag = "1")]
    pub info_port: ::core::option::Option<u32>,
    /// The data port used for Treaty to Treaty operations
    #[prost(uint32, optional, tag = "2")]
    pub data_port: ::core::option::Option<u32>,
    /// The client port, used for application developer or admin user operations
    #[prost(uint32, optional, tag = "3")]
    pub user_port: ::core::option::Option<u32>,
}
/// Generated client implementations.
pub mod user_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// *
    /// A service by which application developers can talk to a Treaty instance.
    /// Generally defaults to port 50051. See the "Settings.toml" file for configuration.
    /// 🔐 These calls require authentication.
    #[derive(Debug, Clone)]
    pub struct UserServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> UserServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UserServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            UserServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Denotes if the instance is online.
        pub async fn is_online(
            &mut self,
            request: impl tonic::IntoRequest<super::TestRequest>,
        ) -> std::result::Result<tonic::Response<super::TestReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/IsOnline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.UserService", "IsOnline"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates a database.
        /// ℹ️ This will return an error if the database already exists.
        pub async fn create_user_database(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserDatabaseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserDatabaseReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/CreateUserDatabase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.UserService", "CreateUserDatabase"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a database in a cleanup oriented manner, cleaning up references, etc.
        pub async fn delete_user_database(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserDatabaseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteUserDatabaseReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/DeleteUserDatabase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.UserService", "DeleteUserDatabase"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the database off of disk or drops the database forcefully, without removing references. This can cause un-intended consequences.
        pub async fn delete_user_database_destructively(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserDatabaseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteUserDatabaseReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/DeleteUserDatabaseDestructively",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "DeleteUserDatabaseDestructively",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Instructs Treaty to create needed meta-data tables.
        pub async fn enable_coooperative_features(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableCoooperativeFeaturesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EnableCoooperativeFeaturesReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/EnableCoooperativeFeatures",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "EnableCoooperativeFeatures",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Executes the specified SELECT SQL query against a Host database.
        pub async fn execute_read_at_host(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteReadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteReadReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/ExecuteReadAtHost",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.UserService", "ExecuteReadAtHost"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Executes the specified INSERT/UPDATE/DELETE SQL statement against a Host database.
        pub async fn execute_write_at_host(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteWriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteWriteReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/ExecuteWriteAtHost",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.UserService", "ExecuteWriteAtHost"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Executes the specified INSERT/UPDATE/DELETE SQL statement at the Participant and saves the meta-data at the Host.
        pub async fn execute_cooperative_write_at_host(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteCooperativeWriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteCooperativeWriteReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/ExecuteCooperativeWriteAtHost",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "ExecuteCooperativeWriteAtHost",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Executes the specified SELECT SQL query against a Partial database.
        pub async fn execute_read_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteReadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteReadReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/ExecuteReadAtParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "ExecuteReadAtParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Executes the specified INSERT/UPDATE/DELETE SQL statment against a Partial database.
        pub async fn execute_write_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteWriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteWriteReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/ExecuteWriteAtParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "ExecuteWriteAtParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Checks if the specified table exists in the specified database.
        pub async fn has_table(
            &mut self,
            request: impl tonic::IntoRequest<super::HasTableRequest>,
        ) -> std::result::Result<tonic::Response<super::HasTableReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/HasTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.UserService", "HasTable"));
            self.inner.unary(req, path, codec).await
        }
        /// Sets the Logical Storage Policy for the specified table in the specified database.
        pub async fn set_logical_storage_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLogicalStoragePolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetLogicalStoragePolicyReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/SetLogicalStoragePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "SetLogicalStoragePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the Logical Storage Policy for the specified table in the specified database.
        pub async fn get_logical_storage_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLogicalStoragePolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLogicalStoragePolicyReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetLogicalStoragePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "GetLogicalStoragePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates a database contract for the specified database.
        /// ℹ️ INFORMATION: For this to work, you must set a Logical Storage Policy ahead of time on all database tables.
        /// See the manual for more information.
        pub async fn generate_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateContractReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GenerateContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.UserService", "GenerateContract"));
            self.inner.unary(req, path, codec).await
        }
        /// Adds a participant with the specified attributes to the specified database.
        pub async fn add_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::AddParticipantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddParticipantReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/AddParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.UserService", "AddParticipant"));
            self.inner.unary(req, path, codec).await
        }
        /// Sends a copy of the active database contract to the specified Participant.
        pub async fn send_participant_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::SendParticipantContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendParticipantContractReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/SendParticipantContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "SendParticipantContract",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a list of pending contracts at our Treaty instance.
        pub async fn review_pending_contracts(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::ViewPendingContractsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/ReviewPendingContracts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.UserService", "ReviewPendingContracts"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Accepts the specified database contract. This creates the needed partial database and supporting database structures.
        pub async fn accept_pending_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::AcceptPendingContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceptPendingContractReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/AcceptPendingContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.UserService", "AcceptPendingContract"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Rejects the specified database contract. This informs the Host that we do not agree to cooperate.
        pub async fn reject_pending_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::RejectPendingContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RejectPendingContractReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/RejectPendingContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.UserService", "RejectPendingContract"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates our host info with the specified host name.
        /// ❗ WARNING: Calling this may overwrite any existing authentication token you have
        /// used to identify your Treaty instance to others. See the manual for more information.
        pub async fn generate_host_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateHostInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateHostInfoReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GenerateHostInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.UserService", "GenerateHostInfo"));
            self.inner.unary(req, path, codec).await
        }
        /// Change the status for the specified Host. This configures if a Host is allowed to talk to our Treaty instance.
        pub async fn change_host_status(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeHostStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeHostStatusReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/ChangeHostStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.UserService", "ChangeHostStatus"));
            self.inner.unary(req, path, codec).await
        }
        /// Attempt authentication at the specified host.
        pub async fn try_auth_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::TryAuthAtParticipantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TryAuthAtPartipantReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/TryAuthAtParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.UserService", "TryAuthAtParticipant"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Changes the UpdatesFromHost behavior.
        pub async fn change_updates_from_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeUpdatesFromHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangesUpdatesFromHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/ChangeUpdatesFromHostBehavior",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "ChangeUpdatesFromHostBehavior",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Changes the DeletesFromHost behavior.
        pub async fn change_deletes_from_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeDeletesFromHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeDeletesFromHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/ChangeDeletesFromHostBehavior",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "ChangeDeletesFromHostBehavior",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Changes the UpdatesToHost behavior.
        pub async fn change_updates_to_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeUpdatesToHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeUpdatesToHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/ChangeUpdatesToHostBehavior",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "ChangeUpdatesToHostBehavior",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Changes the DeletesToHost behavior.
        pub async fn change_deletes_to_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeDeletesToHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeDeletesToHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/ChangeDeletesToHostBehavior",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "ChangeDeletesToHostBehavior",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the data hash at the specified Host database for the specified row.
        pub async fn get_data_hash_at_host(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataHashRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDataHashReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetDataHashAtHost",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.UserService", "GetDataHashAtHost"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the data hash at the specified Partial database for the specified row.
        pub async fn get_data_hash_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataHashRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDataHashReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetDataHashAtParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "GetDataHashAtParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the Row ID at the specified Partial database for the specified WHERE clause.
        pub async fn read_row_id_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReadRowIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetReadRowIdsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/ReadRowIdAtParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.UserService", "ReadRowIdAtParticipant"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the status of our Log table at the Partial database.
        pub async fn get_data_log_table_status_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataLogTableStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDataLogTableStatusReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetDataLogTableStatusAtParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "GetDataLogTableStatusAtParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the status of our Log table at the Partial database.
        pub async fn set_data_log_table_status_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDataLogTableStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetDataLogTableStatusReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/SetDataLogTableStatusAtParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "SetDataLogTableStatusAtParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a list of pending actions at the Partial database.
        pub async fn get_pending_actions_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPendingActionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPendingActionsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetPendingActionsAtParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "GetPendingActionsAtParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Accepts the pending database action at the Partial database.
        pub async fn accept_pending_action_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::AcceptPendingActionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceptPendingActionReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/AcceptPendingActionAtParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "AcceptPendingActionAtParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a list of databases at our Treaty instance.
        pub async fn get_databases(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatabasesReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetDatabases",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.UserService", "GetDatabases"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets a list of Participants at our Treaty instance.
        pub async fn get_participants(
            &mut self,
            request: impl tonic::IntoRequest<super::GetParticipantsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetParticipantsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetParticipants",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.UserService", "GetParticipants"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the active database contract for the specified database.
        pub async fn get_active_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::GetActiveContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetActiveContractReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetActiveContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.UserService", "GetActiveContract"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Requests Treaty to generate a Json Web Token for the credentials provided.
        pub async fn auth_for_token(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRequestBasic>,
        ) -> std::result::Result<tonic::Response<super::TokenReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/AuthForToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.UserService", "AuthForToken"));
            self.inner.unary(req, path, codec).await
        }
        /// Requests Treaty to revoke the Json Web Token for the credentials provided.
        pub async fn revoke_token(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRequestWebToken>,
        ) -> std::result::Result<tonic::Response<super::RevokeReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/RevokeToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.UserService", "RevokeToken"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets our Host Information.
        pub async fn get_host_info(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::HostInfoReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetHostInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.UserService", "GetHostInfo"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the versions of Treaty assemblies.
        pub async fn get_versions(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::VersionReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.UserService", "GetVersions"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the current configured UpdatesFromHostBehavior.
        pub async fn get_updates_from_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUpdatesFromHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUpdatesFromHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetUpdatesFromHostBehavior",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "GetUpdatesFromHostBehavior",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the current configured UpdatesToHostBehavior.
        pub async fn get_updates_to_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUpdatesToHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUpdatesToHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetUpdatesToHostBehavior",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "GetUpdatesToHostBehavior",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the current configured DeletesFromHostBehavior.
        pub async fn get_deletes_from_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeletesFromHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDeletesFromHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetDeletesFromHostBehavior",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "GetDeletesFromHostBehavior",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the current configured DeletesToHostBehavior.
        pub async fn get_deletes_to_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeletesToHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDeletesToHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetDeletesToHostBehavior",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "GetDeletesToHostBehavior",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a list of Hosts that we are cooperating with. These are all the Treaty instances that we have accepted contracts from.
        pub async fn get_cooperative_hosts(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetCooperativeHostsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetCooperativeHosts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.UserService", "GetCooperativeHosts"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the current configured settings from the Settings.toml file.
        pub async fn get_settings(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetSettingsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.UserService", "GetSettings"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the last X number of log entries.
        pub async fn get_logs_by_last_number(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLogsByLastNumberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLogsByLastNumberReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetLogsByLastNumber",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.UserService", "GetLogsByLastNumber"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the backing database technology used at this Treaty instance.
        pub async fn get_backing_database_config(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetBackingDatabaseConfigReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.UserService/GetBackingDatabaseConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.UserService",
                        "GetBackingDatabaseConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod data_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// *
    /// A service that a Treaty instance can talk to other Treaty instances.
    /// Generally defaults to port 50052. See the "Settings.toml" file for configuration.
    /// 🔐 These calls require authentication.
    #[derive(Debug, Clone)]
    pub struct DataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DataServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DataServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DataServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DataServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// A call to see if the service is available.
        pub async fn is_online(
            &mut self,
            request: impl tonic::IntoRequest<super::TestRequest>,
        ) -> std::result::Result<tonic::Response<super::TestReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.DataService/IsOnline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.DataService", "IsOnline"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates a partial database.
        pub async fn create_partial_database(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePartialDatabaseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePartialDatabaseResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.DataService/CreatePartialDatabase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.DataService", "CreatePartialDatabase"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a table in a partial database.
        pub async fn create_table_in_database(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTableResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.DataService/CreateTableInDatabase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.DataService", "CreateTableInDatabase"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Executes the provided INSERT statement against a partial database.
        pub async fn insert_command_into_table(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InsertDataResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.DataService/InsertCommandIntoTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.DataService", "InsertCommandIntoTable"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Executes the provided UPDATE statement against a partial database.
        pub async fn update_command_into_table(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateDataResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.DataService/UpdateCommandIntoTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.DataService", "UpdateCommandIntoTable"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Executes the provided DELETE statement againts a partial database.
        pub async fn delete_command_into_table(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteDataResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.DataService/DeleteCommandIntoTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.DataService", "DeleteCommandIntoTable"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Requests a specific row from a partial database.
        pub async fn get_row_from_partial_database(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRowFromPartialDatabaseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRowFromPartialDatabaseResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.DataService/GetRowFromPartialDatabase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.DataService",
                        "GetRowFromPartialDatabase",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Notification that a data hash has changed at a Participant.
        pub async fn update_row_data_hash_for_host(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRowDataHashForHostRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRowDataHashForHostResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.DataService/UpdateRowDataHashForHost",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "treaty_proto.DataService",
                        "UpdateRowDataHashForHost",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Notification that a row in a partial database has been removed at a Participant.
        pub async fn notify_host_of_removed_row(
            &mut self,
            request: impl tonic::IntoRequest<super::NotifyHostOfRemovedRowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NotifyHostOfRemovedRowResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.DataService/NotifyHostOfRemovedRow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("treaty_proto.DataService", "NotifyHostOfRemovedRow"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Check if we can authenticate at this Treaty instance.
        pub async fn try_auth(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::TryAuthResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.DataService/TryAuth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.DataService", "TryAuth"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod info_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// *
    /// A service that can be queried for general or unauthenticated activities.
    /// It can also provide authentication as needed.
    /// Generally defaults to port 50059. See the "Settings.toml" file for configuration.
    /// 🔓 These calls generally do not require authentication, unless explicitly seeking to generate an authentication token.
    #[derive(Debug, Clone)]
    pub struct InfoServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InfoServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> InfoServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InfoServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            InfoServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Denotes if the instance is online.
        pub async fn is_online(
            &mut self,
            request: impl tonic::IntoRequest<super::TestRequest>,
        ) -> std::result::Result<tonic::Response<super::TestReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.InfoService/IsOnline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.InfoService", "IsOnline"));
            self.inner.unary(req, path, codec).await
        }
        /// Request to save a Contract; usually to be later Accepted or Rejected.
        pub async fn save_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::SaveContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SaveContractResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.InfoService/SaveContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.InfoService", "SaveContract"));
            self.inner.unary(req, path, codec).await
        }
        /// Request to get the public available ports on this instance
        pub async fn ports_available(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::TreatyPorts>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.InfoService/PortsAvailable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.InfoService", "PortsAvailable"));
            self.inner.unary(req, path, codec).await
        }
        /// Notification that a Participant has accepted a contract.
        pub async fn accept_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::ParticipantAcceptsContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ParticipantAcceptsContractResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.InfoService/AcceptContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.InfoService", "AcceptContract"));
            self.inner.unary(req, path, codec).await
        }
        /// Attempts to see if the supplied token is valid
        pub async fn try_auth_web_token(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRequestWebToken>,
        ) -> std::result::Result<tonic::Response<super::TryAuthResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.InfoService/TryAuthWebToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.InfoService", "TryAuthWebToken"));
            self.inner.unary(req, path, codec).await
        }
        /// Requests Treaty to generate a Json Web Token for the credentials provided.
        /// Note: This call is the same as the one on the User service.
        pub async fn auth_for_token(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRequestBasic>,
        ) -> std::result::Result<tonic::Response<super::TokenReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/treaty_proto.InfoService/AuthForToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("treaty_proto.InfoService", "AuthForToken"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod user_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UserServiceServer.
    #[async_trait]
    pub trait UserService: Send + Sync + 'static {
        /// Denotes if the instance is online.
        async fn is_online(
            &self,
            request: tonic::Request<super::TestRequest>,
        ) -> std::result::Result<tonic::Response<super::TestReply>, tonic::Status>;
        /// Creates a database.
        /// ℹ️ This will return an error if the database already exists.
        async fn create_user_database(
            &self,
            request: tonic::Request<super::CreateUserDatabaseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserDatabaseReply>,
            tonic::Status,
        >;
        /// Deletes a database in a cleanup oriented manner, cleaning up references, etc.
        async fn delete_user_database(
            &self,
            request: tonic::Request<super::DeleteUserDatabaseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteUserDatabaseReply>,
            tonic::Status,
        >;
        /// Deletes the database off of disk or drops the database forcefully, without removing references. This can cause un-intended consequences.
        async fn delete_user_database_destructively(
            &self,
            request: tonic::Request<super::DeleteUserDatabaseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteUserDatabaseReply>,
            tonic::Status,
        >;
        /// Instructs Treaty to create needed meta-data tables.
        async fn enable_coooperative_features(
            &self,
            request: tonic::Request<super::EnableCoooperativeFeaturesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EnableCoooperativeFeaturesReply>,
            tonic::Status,
        >;
        /// Executes the specified SELECT SQL query against a Host database.
        async fn execute_read_at_host(
            &self,
            request: tonic::Request<super::ExecuteReadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteReadReply>,
            tonic::Status,
        >;
        /// Executes the specified INSERT/UPDATE/DELETE SQL statement against a Host database.
        async fn execute_write_at_host(
            &self,
            request: tonic::Request<super::ExecuteWriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteWriteReply>,
            tonic::Status,
        >;
        /// Executes the specified INSERT/UPDATE/DELETE SQL statement at the Participant and saves the meta-data at the Host.
        async fn execute_cooperative_write_at_host(
            &self,
            request: tonic::Request<super::ExecuteCooperativeWriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteCooperativeWriteReply>,
            tonic::Status,
        >;
        /// Executes the specified SELECT SQL query against a Partial database.
        async fn execute_read_at_participant(
            &self,
            request: tonic::Request<super::ExecuteReadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteReadReply>,
            tonic::Status,
        >;
        /// Executes the specified INSERT/UPDATE/DELETE SQL statment against a Partial database.
        async fn execute_write_at_participant(
            &self,
            request: tonic::Request<super::ExecuteWriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteWriteReply>,
            tonic::Status,
        >;
        /// Checks if the specified table exists in the specified database.
        async fn has_table(
            &self,
            request: tonic::Request<super::HasTableRequest>,
        ) -> std::result::Result<tonic::Response<super::HasTableReply>, tonic::Status>;
        /// Sets the Logical Storage Policy for the specified table in the specified database.
        async fn set_logical_storage_policy(
            &self,
            request: tonic::Request<super::SetLogicalStoragePolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetLogicalStoragePolicyReply>,
            tonic::Status,
        >;
        /// Gets the Logical Storage Policy for the specified table in the specified database.
        async fn get_logical_storage_policy(
            &self,
            request: tonic::Request<super::GetLogicalStoragePolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLogicalStoragePolicyReply>,
            tonic::Status,
        >;
        /// Generates a database contract for the specified database.
        /// ℹ️ INFORMATION: For this to work, you must set a Logical Storage Policy ahead of time on all database tables.
        /// See the manual for more information.
        async fn generate_contract(
            &self,
            request: tonic::Request<super::GenerateContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateContractReply>,
            tonic::Status,
        >;
        /// Adds a participant with the specified attributes to the specified database.
        async fn add_participant(
            &self,
            request: tonic::Request<super::AddParticipantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddParticipantReply>,
            tonic::Status,
        >;
        /// Sends a copy of the active database contract to the specified Participant.
        async fn send_participant_contract(
            &self,
            request: tonic::Request<super::SendParticipantContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendParticipantContractReply>,
            tonic::Status,
        >;
        /// Gets a list of pending contracts at our Treaty instance.
        async fn review_pending_contracts(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::ViewPendingContractsReply>,
            tonic::Status,
        >;
        /// Accepts the specified database contract. This creates the needed partial database and supporting database structures.
        async fn accept_pending_contract(
            &self,
            request: tonic::Request<super::AcceptPendingContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceptPendingContractReply>,
            tonic::Status,
        >;
        /// Rejects the specified database contract. This informs the Host that we do not agree to cooperate.
        async fn reject_pending_contract(
            &self,
            request: tonic::Request<super::RejectPendingContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RejectPendingContractReply>,
            tonic::Status,
        >;
        /// Generates our host info with the specified host name.
        /// ❗ WARNING: Calling this may overwrite any existing authentication token you have
        /// used to identify your Treaty instance to others. See the manual for more information.
        async fn generate_host_info(
            &self,
            request: tonic::Request<super::GenerateHostInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateHostInfoReply>,
            tonic::Status,
        >;
        /// Change the status for the specified Host. This configures if a Host is allowed to talk to our Treaty instance.
        async fn change_host_status(
            &self,
            request: tonic::Request<super::ChangeHostStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeHostStatusReply>,
            tonic::Status,
        >;
        /// Attempt authentication at the specified host.
        async fn try_auth_at_participant(
            &self,
            request: tonic::Request<super::TryAuthAtParticipantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TryAuthAtPartipantReply>,
            tonic::Status,
        >;
        /// Changes the UpdatesFromHost behavior.
        async fn change_updates_from_host_behavior(
            &self,
            request: tonic::Request<super::ChangeUpdatesFromHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangesUpdatesFromHostBehaviorReply>,
            tonic::Status,
        >;
        /// Changes the DeletesFromHost behavior.
        async fn change_deletes_from_host_behavior(
            &self,
            request: tonic::Request<super::ChangeDeletesFromHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeDeletesFromHostBehaviorReply>,
            tonic::Status,
        >;
        /// Changes the UpdatesToHost behavior.
        async fn change_updates_to_host_behavior(
            &self,
            request: tonic::Request<super::ChangeUpdatesToHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeUpdatesToHostBehaviorReply>,
            tonic::Status,
        >;
        /// Changes the DeletesToHost behavior.
        async fn change_deletes_to_host_behavior(
            &self,
            request: tonic::Request<super::ChangeDeletesToHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeDeletesToHostBehaviorReply>,
            tonic::Status,
        >;
        /// Gets the data hash at the specified Host database for the specified row.
        async fn get_data_hash_at_host(
            &self,
            request: tonic::Request<super::GetDataHashRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDataHashReply>,
            tonic::Status,
        >;
        /// Gets the data hash at the specified Partial database for the specified row.
        async fn get_data_hash_at_participant(
            &self,
            request: tonic::Request<super::GetDataHashRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDataHashReply>,
            tonic::Status,
        >;
        /// Gets the Row ID at the specified Partial database for the specified WHERE clause.
        async fn read_row_id_at_participant(
            &self,
            request: tonic::Request<super::GetReadRowIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetReadRowIdsReply>,
            tonic::Status,
        >;
        /// Gets the status of our Log table at the Partial database.
        async fn get_data_log_table_status_at_participant(
            &self,
            request: tonic::Request<super::GetDataLogTableStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDataLogTableStatusReply>,
            tonic::Status,
        >;
        /// Sets the status of our Log table at the Partial database.
        async fn set_data_log_table_status_at_participant(
            &self,
            request: tonic::Request<super::SetDataLogTableStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetDataLogTableStatusReply>,
            tonic::Status,
        >;
        /// Gets a list of pending actions at the Partial database.
        async fn get_pending_actions_at_participant(
            &self,
            request: tonic::Request<super::GetPendingActionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPendingActionsReply>,
            tonic::Status,
        >;
        /// Accepts the pending database action at the Partial database.
        async fn accept_pending_action_at_participant(
            &self,
            request: tonic::Request<super::AcceptPendingActionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceptPendingActionReply>,
            tonic::Status,
        >;
        /// Gets a list of databases at our Treaty instance.
        async fn get_databases(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatabasesReply>,
            tonic::Status,
        >;
        /// Gets a list of Participants at our Treaty instance.
        async fn get_participants(
            &self,
            request: tonic::Request<super::GetParticipantsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetParticipantsReply>,
            tonic::Status,
        >;
        /// Gets the active database contract for the specified database.
        async fn get_active_contract(
            &self,
            request: tonic::Request<super::GetActiveContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetActiveContractReply>,
            tonic::Status,
        >;
        /// Requests Treaty to generate a Json Web Token for the credentials provided.
        async fn auth_for_token(
            &self,
            request: tonic::Request<super::AuthRequestBasic>,
        ) -> std::result::Result<tonic::Response<super::TokenReply>, tonic::Status>;
        /// Requests Treaty to revoke the Json Web Token for the credentials provided.
        async fn revoke_token(
            &self,
            request: tonic::Request<super::AuthRequestWebToken>,
        ) -> std::result::Result<tonic::Response<super::RevokeReply>, tonic::Status>;
        /// Gets our Host Information.
        async fn get_host_info(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::HostInfoReply>, tonic::Status>;
        /// Gets the versions of Treaty assemblies.
        async fn get_versions(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::VersionReply>, tonic::Status>;
        /// Gets the current configured UpdatesFromHostBehavior.
        async fn get_updates_from_host_behavior(
            &self,
            request: tonic::Request<super::GetUpdatesFromHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUpdatesFromHostBehaviorReply>,
            tonic::Status,
        >;
        /// Gets the current configured UpdatesToHostBehavior.
        async fn get_updates_to_host_behavior(
            &self,
            request: tonic::Request<super::GetUpdatesToHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUpdatesToHostBehaviorReply>,
            tonic::Status,
        >;
        /// Gets the current configured DeletesFromHostBehavior.
        async fn get_deletes_from_host_behavior(
            &self,
            request: tonic::Request<super::GetDeletesFromHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDeletesFromHostBehaviorReply>,
            tonic::Status,
        >;
        /// Gets the current configured DeletesToHostBehavior.
        async fn get_deletes_to_host_behavior(
            &self,
            request: tonic::Request<super::GetDeletesToHostBehaviorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDeletesToHostBehaviorReply>,
            tonic::Status,
        >;
        /// Gets a list of Hosts that we are cooperating with. These are all the Treaty instances that we have accepted contracts from.
        async fn get_cooperative_hosts(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetCooperativeHostsReply>,
            tonic::Status,
        >;
        /// Gets the current configured settings from the Settings.toml file.
        async fn get_settings(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetSettingsReply>,
            tonic::Status,
        >;
        /// Gets the last X number of log entries.
        async fn get_logs_by_last_number(
            &self,
            request: tonic::Request<super::GetLogsByLastNumberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLogsByLastNumberReply>,
            tonic::Status,
        >;
        /// Gets the backing database technology used at this Treaty instance.
        async fn get_backing_database_config(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetBackingDatabaseConfigReply>,
            tonic::Status,
        >;
    }
    /// *
    /// A service by which application developers can talk to a Treaty instance.
    /// Generally defaults to port 50051. See the "Settings.toml" file for configuration.
    /// 🔐 These calls require authentication.
    #[derive(Debug)]
    pub struct UserServiceServer<T: UserService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UserService> UserServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UserServiceServer<T>
    where
        T: UserService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/treaty_proto.UserService/IsOnline" => {
                    #[allow(non_camel_case_types)]
                    struct IsOnlineSvc<T: UserService>(pub Arc<T>);
                    impl<T: UserService> tonic::server::UnaryService<super::TestRequest>
                    for IsOnlineSvc<T> {
                        type Response = super::TestReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TestRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::is_online(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsOnlineSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/CreateUserDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUserDatabaseSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::CreateUserDatabaseRequest>
                    for CreateUserDatabaseSvc<T> {
                        type Response = super::CreateUserDatabaseReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateUserDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::create_user_database(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateUserDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/DeleteUserDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteUserDatabaseSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::DeleteUserDatabaseRequest>
                    for DeleteUserDatabaseSvc<T> {
                        type Response = super::DeleteUserDatabaseReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteUserDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::delete_user_database(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteUserDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/DeleteUserDatabaseDestructively" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteUserDatabaseDestructivelySvc<T: UserService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::DeleteUserDatabaseRequest>
                    for DeleteUserDatabaseDestructivelySvc<T> {
                        type Response = super::DeleteUserDatabaseReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteUserDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::delete_user_database_destructively(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteUserDatabaseDestructivelySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/EnableCoooperativeFeatures" => {
                    #[allow(non_camel_case_types)]
                    struct EnableCoooperativeFeaturesSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<
                        super::EnableCoooperativeFeaturesRequest,
                    > for EnableCoooperativeFeaturesSvc<T> {
                        type Response = super::EnableCoooperativeFeaturesReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::EnableCoooperativeFeaturesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::enable_coooperative_features(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EnableCoooperativeFeaturesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/ExecuteReadAtHost" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteReadAtHostSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::ExecuteReadRequest>
                    for ExecuteReadAtHostSvc<T> {
                        type Response = super::ExecuteReadReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteReadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::execute_read_at_host(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteReadAtHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/ExecuteWriteAtHost" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteWriteAtHostSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::ExecuteWriteRequest>
                    for ExecuteWriteAtHostSvc<T> {
                        type Response = super::ExecuteWriteReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteWriteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::execute_write_at_host(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteWriteAtHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/ExecuteCooperativeWriteAtHost" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteCooperativeWriteAtHostSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::ExecuteCooperativeWriteRequest>
                    for ExecuteCooperativeWriteAtHostSvc<T> {
                        type Response = super::ExecuteCooperativeWriteReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ExecuteCooperativeWriteRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::execute_cooperative_write_at_host(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteCooperativeWriteAtHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/ExecuteReadAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteReadAtParticipantSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::ExecuteReadRequest>
                    for ExecuteReadAtParticipantSvc<T> {
                        type Response = super::ExecuteReadReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteReadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::execute_read_at_participant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteReadAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/ExecuteWriteAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteWriteAtParticipantSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::ExecuteWriteRequest>
                    for ExecuteWriteAtParticipantSvc<T> {
                        type Response = super::ExecuteWriteReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteWriteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::execute_write_at_participant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteWriteAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/HasTable" => {
                    #[allow(non_camel_case_types)]
                    struct HasTableSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::HasTableRequest>
                    for HasTableSvc<T> {
                        type Response = super::HasTableReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HasTableRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::has_table(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HasTableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/SetLogicalStoragePolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetLogicalStoragePolicySvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::SetLogicalStoragePolicyRequest>
                    for SetLogicalStoragePolicySvc<T> {
                        type Response = super::SetLogicalStoragePolicyReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SetLogicalStoragePolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::set_logical_storage_policy(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetLogicalStoragePolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetLogicalStoragePolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetLogicalStoragePolicySvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetLogicalStoragePolicyRequest>
                    for GetLogicalStoragePolicySvc<T> {
                        type Response = super::GetLogicalStoragePolicyReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetLogicalStoragePolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_logical_storage_policy(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLogicalStoragePolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GenerateContract" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateContractSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GenerateContractRequest>
                    for GenerateContractSvc<T> {
                        type Response = super::GenerateContractReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateContractRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::generate_contract(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GenerateContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/AddParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct AddParticipantSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::AddParticipantRequest>
                    for AddParticipantSvc<T> {
                        type Response = super::AddParticipantReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddParticipantRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::add_participant(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/SendParticipantContract" => {
                    #[allow(non_camel_case_types)]
                    struct SendParticipantContractSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::SendParticipantContractRequest>
                    for SendParticipantContractSvc<T> {
                        type Response = super::SendParticipantContractReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SendParticipantContractRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::send_participant_contract(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendParticipantContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/ReviewPendingContracts" => {
                    #[allow(non_camel_case_types)]
                    struct ReviewPendingContractsSvc<T: UserService>(pub Arc<T>);
                    impl<T: UserService> tonic::server::UnaryService<()>
                    for ReviewPendingContractsSvc<T> {
                        type Response = super::ViewPendingContractsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::review_pending_contracts(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReviewPendingContractsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/AcceptPendingContract" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptPendingContractSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::AcceptPendingContractRequest>
                    for AcceptPendingContractSvc<T> {
                        type Response = super::AcceptPendingContractReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AcceptPendingContractRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::accept_pending_contract(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AcceptPendingContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/RejectPendingContract" => {
                    #[allow(non_camel_case_types)]
                    struct RejectPendingContractSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::RejectPendingContractRequest>
                    for RejectPendingContractSvc<T> {
                        type Response = super::RejectPendingContractReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RejectPendingContractRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::reject_pending_contract(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RejectPendingContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GenerateHostInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateHostInfoSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GenerateHostInfoRequest>
                    for GenerateHostInfoSvc<T> {
                        type Response = super::GenerateHostInfoReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateHostInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::generate_host_info(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GenerateHostInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/ChangeHostStatus" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeHostStatusSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::ChangeHostStatusRequest>
                    for ChangeHostStatusSvc<T> {
                        type Response = super::ChangeHostStatusReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeHostStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::change_host_status(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeHostStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/TryAuthAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct TryAuthAtParticipantSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::TryAuthAtParticipantRequest>
                    for TryAuthAtParticipantSvc<T> {
                        type Response = super::TryAuthAtPartipantReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TryAuthAtParticipantRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::try_auth_at_participant(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TryAuthAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/ChangeUpdatesFromHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeUpdatesFromHostBehaviorSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<
                        super::ChangeUpdatesFromHostBehaviorRequest,
                    > for ChangeUpdatesFromHostBehaviorSvc<T> {
                        type Response = super::ChangesUpdatesFromHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ChangeUpdatesFromHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::change_updates_from_host_behavior(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeUpdatesFromHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/ChangeDeletesFromHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeDeletesFromHostBehaviorSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<
                        super::ChangeDeletesFromHostBehaviorRequest,
                    > for ChangeDeletesFromHostBehaviorSvc<T> {
                        type Response = super::ChangeDeletesFromHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ChangeDeletesFromHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::change_deletes_from_host_behavior(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeDeletesFromHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/ChangeUpdatesToHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeUpdatesToHostBehaviorSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<
                        super::ChangeUpdatesToHostBehaviorRequest,
                    > for ChangeUpdatesToHostBehaviorSvc<T> {
                        type Response = super::ChangeUpdatesToHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ChangeUpdatesToHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::change_updates_to_host_behavior(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeUpdatesToHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/ChangeDeletesToHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeDeletesToHostBehaviorSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<
                        super::ChangeDeletesToHostBehaviorRequest,
                    > for ChangeDeletesToHostBehaviorSvc<T> {
                        type Response = super::ChangeDeletesToHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ChangeDeletesToHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::change_deletes_to_host_behavior(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeDeletesToHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetDataHashAtHost" => {
                    #[allow(non_camel_case_types)]
                    struct GetDataHashAtHostSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetDataHashRequest>
                    for GetDataHashAtHostSvc<T> {
                        type Response = super::GetDataHashReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDataHashRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_data_hash_at_host(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDataHashAtHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetDataHashAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct GetDataHashAtParticipantSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetDataHashRequest>
                    for GetDataHashAtParticipantSvc<T> {
                        type Response = super::GetDataHashReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDataHashRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_data_hash_at_participant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDataHashAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/ReadRowIdAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct ReadRowIdAtParticipantSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetReadRowIdsRequest>
                    for ReadRowIdAtParticipantSvc<T> {
                        type Response = super::GetReadRowIdsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetReadRowIdsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::read_row_id_at_participant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadRowIdAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetDataLogTableStatusAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct GetDataLogTableStatusAtParticipantSvc<T: UserService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetDataLogTableStatusRequest>
                    for GetDataLogTableStatusAtParticipantSvc<T> {
                        type Response = super::GetDataLogTableStatusReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDataLogTableStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_data_log_table_status_at_participant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDataLogTableStatusAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/SetDataLogTableStatusAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct SetDataLogTableStatusAtParticipantSvc<T: UserService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::SetDataLogTableStatusRequest>
                    for SetDataLogTableStatusAtParticipantSvc<T> {
                        type Response = super::SetDataLogTableStatusReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetDataLogTableStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::set_data_log_table_status_at_participant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetDataLogTableStatusAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetPendingActionsAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct GetPendingActionsAtParticipantSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetPendingActionsRequest>
                    for GetPendingActionsAtParticipantSvc<T> {
                        type Response = super::GetPendingActionsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPendingActionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_pending_actions_at_participant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPendingActionsAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/AcceptPendingActionAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptPendingActionAtParticipantSvc<T: UserService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::AcceptPendingActionRequest>
                    for AcceptPendingActionAtParticipantSvc<T> {
                        type Response = super::AcceptPendingActionReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AcceptPendingActionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::accept_pending_action_at_participant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AcceptPendingActionAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetDatabases" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatabasesSvc<T: UserService>(pub Arc<T>);
                    impl<T: UserService> tonic::server::UnaryService<()>
                    for GetDatabasesSvc<T> {
                        type Response = super::GetDatabasesReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_databases(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDatabasesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetParticipants" => {
                    #[allow(non_camel_case_types)]
                    struct GetParticipantsSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetParticipantsRequest>
                    for GetParticipantsSvc<T> {
                        type Response = super::GetParticipantsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetParticipantsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_participants(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetParticipantsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetActiveContract" => {
                    #[allow(non_camel_case_types)]
                    struct GetActiveContractSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetActiveContractRequest>
                    for GetActiveContractSvc<T> {
                        type Response = super::GetActiveContractReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetActiveContractRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_active_contract(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetActiveContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/AuthForToken" => {
                    #[allow(non_camel_case_types)]
                    struct AuthForTokenSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::AuthRequestBasic>
                    for AuthForTokenSvc<T> {
                        type Response = super::TokenReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthRequestBasic>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::auth_for_token(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthForTokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/RevokeToken" => {
                    #[allow(non_camel_case_types)]
                    struct RevokeTokenSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::AuthRequestWebToken>
                    for RevokeTokenSvc<T> {
                        type Response = super::RevokeReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthRequestWebToken>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::revoke_token(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RevokeTokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetHostInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetHostInfoSvc<T: UserService>(pub Arc<T>);
                    impl<T: UserService> tonic::server::UnaryService<()>
                    for GetHostInfoSvc<T> {
                        type Response = super::HostInfoReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_host_info(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetHostInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetVersions" => {
                    #[allow(non_camel_case_types)]
                    struct GetVersionsSvc<T: UserService>(pub Arc<T>);
                    impl<T: UserService> tonic::server::UnaryService<()>
                    for GetVersionsSvc<T> {
                        type Response = super::VersionReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_versions(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetVersionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetUpdatesFromHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct GetUpdatesFromHostBehaviorSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<
                        super::GetUpdatesFromHostBehaviorRequest,
                    > for GetUpdatesFromHostBehaviorSvc<T> {
                        type Response = super::GetUpdatesFromHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetUpdatesFromHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_updates_from_host_behavior(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUpdatesFromHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetUpdatesToHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct GetUpdatesToHostBehaviorSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetUpdatesToHostBehaviorRequest>
                    for GetUpdatesToHostBehaviorSvc<T> {
                        type Response = super::GetUpdatesToHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetUpdatesToHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_updates_to_host_behavior(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUpdatesToHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetDeletesFromHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct GetDeletesFromHostBehaviorSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<
                        super::GetDeletesFromHostBehaviorRequest,
                    > for GetDeletesFromHostBehaviorSvc<T> {
                        type Response = super::GetDeletesFromHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetDeletesFromHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_deletes_from_host_behavior(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDeletesFromHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetDeletesToHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct GetDeletesToHostBehaviorSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetDeletesToHostBehaviorRequest>
                    for GetDeletesToHostBehaviorSvc<T> {
                        type Response = super::GetDeletesToHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetDeletesToHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_deletes_to_host_behavior(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDeletesToHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetCooperativeHosts" => {
                    #[allow(non_camel_case_types)]
                    struct GetCooperativeHostsSvc<T: UserService>(pub Arc<T>);
                    impl<T: UserService> tonic::server::UnaryService<()>
                    for GetCooperativeHostsSvc<T> {
                        type Response = super::GetCooperativeHostsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_cooperative_hosts(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCooperativeHostsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetSettings" => {
                    #[allow(non_camel_case_types)]
                    struct GetSettingsSvc<T: UserService>(pub Arc<T>);
                    impl<T: UserService> tonic::server::UnaryService<()>
                    for GetSettingsSvc<T> {
                        type Response = super::GetSettingsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_settings(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSettingsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetLogsByLastNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetLogsByLastNumberSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetLogsByLastNumberRequest>
                    for GetLogsByLastNumberSvc<T> {
                        type Response = super::GetLogsByLastNumberReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLogsByLastNumberRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_logs_by_last_number(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLogsByLastNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.UserService/GetBackingDatabaseConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetBackingDatabaseConfigSvc<T: UserService>(pub Arc<T>);
                    impl<T: UserService> tonic::server::UnaryService<()>
                    for GetBackingDatabaseConfigSvc<T> {
                        type Response = super::GetBackingDatabaseConfigReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_backing_database_config(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBackingDatabaseConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: UserService> Clone for UserServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: UserService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UserService> tonic::server::NamedService for UserServiceServer<T> {
        const NAME: &'static str = "treaty_proto.UserService";
    }
}
/// Generated server implementations.
pub mod data_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DataServiceServer.
    #[async_trait]
    pub trait DataService: Send + Sync + 'static {
        /// A call to see if the service is available.
        async fn is_online(
            &self,
            request: tonic::Request<super::TestRequest>,
        ) -> std::result::Result<tonic::Response<super::TestReply>, tonic::Status>;
        /// Creates a partial database.
        async fn create_partial_database(
            &self,
            request: tonic::Request<super::CreatePartialDatabaseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePartialDatabaseResult>,
            tonic::Status,
        >;
        /// Creates a table in a partial database.
        async fn create_table_in_database(
            &self,
            request: tonic::Request<super::CreateTableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTableResult>,
            tonic::Status,
        >;
        /// Executes the provided INSERT statement against a partial database.
        async fn insert_command_into_table(
            &self,
            request: tonic::Request<super::InsertDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InsertDataResult>,
            tonic::Status,
        >;
        /// Executes the provided UPDATE statement against a partial database.
        async fn update_command_into_table(
            &self,
            request: tonic::Request<super::UpdateDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateDataResult>,
            tonic::Status,
        >;
        /// Executes the provided DELETE statement againts a partial database.
        async fn delete_command_into_table(
            &self,
            request: tonic::Request<super::DeleteDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteDataResult>,
            tonic::Status,
        >;
        /// Requests a specific row from a partial database.
        async fn get_row_from_partial_database(
            &self,
            request: tonic::Request<super::GetRowFromPartialDatabaseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRowFromPartialDatabaseResult>,
            tonic::Status,
        >;
        /// Notification that a data hash has changed at a Participant.
        async fn update_row_data_hash_for_host(
            &self,
            request: tonic::Request<super::UpdateRowDataHashForHostRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRowDataHashForHostResult>,
            tonic::Status,
        >;
        /// Notification that a row in a partial database has been removed at a Participant.
        async fn notify_host_of_removed_row(
            &self,
            request: tonic::Request<super::NotifyHostOfRemovedRowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NotifyHostOfRemovedRowResult>,
            tonic::Status,
        >;
        /// Check if we can authenticate at this Treaty instance.
        async fn try_auth(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::TryAuthResult>, tonic::Status>;
    }
    /// *
    /// A service that a Treaty instance can talk to other Treaty instances.
    /// Generally defaults to port 50052. See the "Settings.toml" file for configuration.
    /// 🔐 These calls require authentication.
    #[derive(Debug)]
    pub struct DataServiceServer<T: DataService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DataService> DataServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DataServiceServer<T>
    where
        T: DataService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/treaty_proto.DataService/IsOnline" => {
                    #[allow(non_camel_case_types)]
                    struct IsOnlineSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<super::TestRequest>
                    for IsOnlineSvc<T> {
                        type Response = super::TestReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TestRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataService>::is_online(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsOnlineSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.DataService/CreatePartialDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePartialDatabaseSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::CreatePartialDatabaseRequest>
                    for CreatePartialDatabaseSvc<T> {
                        type Response = super::CreatePartialDatabaseResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePartialDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataService>::create_partial_database(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreatePartialDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.DataService/CreateTableInDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTableInDatabaseSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::CreateTableRequest>
                    for CreateTableInDatabaseSvc<T> {
                        type Response = super::CreateTableResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTableRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataService>::create_table_in_database(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTableInDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.DataService/InsertCommandIntoTable" => {
                    #[allow(non_camel_case_types)]
                    struct InsertCommandIntoTableSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::InsertDataRequest>
                    for InsertCommandIntoTableSvc<T> {
                        type Response = super::InsertDataResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InsertDataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataService>::insert_command_into_table(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InsertCommandIntoTableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.DataService/UpdateCommandIntoTable" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCommandIntoTableSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::UpdateDataRequest>
                    for UpdateCommandIntoTableSvc<T> {
                        type Response = super::UpdateDataResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataService>::update_command_into_table(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateCommandIntoTableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.DataService/DeleteCommandIntoTable" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCommandIntoTableSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::DeleteDataRequest>
                    for DeleteCommandIntoTableSvc<T> {
                        type Response = super::DeleteDataResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataService>::delete_command_into_table(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteCommandIntoTableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.DataService/GetRowFromPartialDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct GetRowFromPartialDatabaseSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::GetRowFromPartialDatabaseRequest,
                    > for GetRowFromPartialDatabaseSvc<T> {
                        type Response = super::GetRowFromPartialDatabaseResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetRowFromPartialDatabaseRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataService>::get_row_from_partial_database(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRowFromPartialDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.DataService/UpdateRowDataHashForHost" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRowDataHashForHostSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::UpdateRowDataHashForHostRequest>
                    for UpdateRowDataHashForHostSvc<T> {
                        type Response = super::UpdateRowDataHashForHostResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateRowDataHashForHostRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataService>::update_row_data_hash_for_host(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateRowDataHashForHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.DataService/NotifyHostOfRemovedRow" => {
                    #[allow(non_camel_case_types)]
                    struct NotifyHostOfRemovedRowSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::NotifyHostOfRemovedRowRequest>
                    for NotifyHostOfRemovedRowSvc<T> {
                        type Response = super::NotifyHostOfRemovedRowResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NotifyHostOfRemovedRowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataService>::notify_host_of_removed_row(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NotifyHostOfRemovedRowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.DataService/TryAuth" => {
                    #[allow(non_camel_case_types)]
                    struct TryAuthSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<()>
                    for TryAuthSvc<T> {
                        type Response = super::TryAuthResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataService>::try_auth(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TryAuthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: DataService> Clone for DataServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: DataService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DataService> tonic::server::NamedService for DataServiceServer<T> {
        const NAME: &'static str = "treaty_proto.DataService";
    }
}
/// Generated server implementations.
pub mod info_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InfoServiceServer.
    #[async_trait]
    pub trait InfoService: Send + Sync + 'static {
        /// Denotes if the instance is online.
        async fn is_online(
            &self,
            request: tonic::Request<super::TestRequest>,
        ) -> std::result::Result<tonic::Response<super::TestReply>, tonic::Status>;
        /// Request to save a Contract; usually to be later Accepted or Rejected.
        async fn save_contract(
            &self,
            request: tonic::Request<super::SaveContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SaveContractResult>,
            tonic::Status,
        >;
        /// Request to get the public available ports on this instance
        async fn ports_available(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::TreatyPorts>, tonic::Status>;
        /// Notification that a Participant has accepted a contract.
        async fn accept_contract(
            &self,
            request: tonic::Request<super::ParticipantAcceptsContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ParticipantAcceptsContractResult>,
            tonic::Status,
        >;
        /// Attempts to see if the supplied token is valid
        async fn try_auth_web_token(
            &self,
            request: tonic::Request<super::AuthRequestWebToken>,
        ) -> std::result::Result<tonic::Response<super::TryAuthResult>, tonic::Status>;
        /// Requests Treaty to generate a Json Web Token for the credentials provided.
        /// Note: This call is the same as the one on the User service.
        async fn auth_for_token(
            &self,
            request: tonic::Request<super::AuthRequestBasic>,
        ) -> std::result::Result<tonic::Response<super::TokenReply>, tonic::Status>;
    }
    /// *
    /// A service that can be queried for general or unauthenticated activities.
    /// It can also provide authentication as needed.
    /// Generally defaults to port 50059. See the "Settings.toml" file for configuration.
    /// 🔓 These calls generally do not require authentication, unless explicitly seeking to generate an authentication token.
    #[derive(Debug)]
    pub struct InfoServiceServer<T: InfoService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InfoService> InfoServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for InfoServiceServer<T>
    where
        T: InfoService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/treaty_proto.InfoService/IsOnline" => {
                    #[allow(non_camel_case_types)]
                    struct IsOnlineSvc<T: InfoService>(pub Arc<T>);
                    impl<T: InfoService> tonic::server::UnaryService<super::TestRequest>
                    for IsOnlineSvc<T> {
                        type Response = super::TestReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TestRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InfoService>::is_online(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsOnlineSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.InfoService/SaveContract" => {
                    #[allow(non_camel_case_types)]
                    struct SaveContractSvc<T: InfoService>(pub Arc<T>);
                    impl<
                        T: InfoService,
                    > tonic::server::UnaryService<super::SaveContractRequest>
                    for SaveContractSvc<T> {
                        type Response = super::SaveContractResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SaveContractRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InfoService>::save_contract(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.InfoService/PortsAvailable" => {
                    #[allow(non_camel_case_types)]
                    struct PortsAvailableSvc<T: InfoService>(pub Arc<T>);
                    impl<T: InfoService> tonic::server::UnaryService<()>
                    for PortsAvailableSvc<T> {
                        type Response = super::TreatyPorts;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InfoService>::ports_available(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PortsAvailableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.InfoService/AcceptContract" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptContractSvc<T: InfoService>(pub Arc<T>);
                    impl<
                        T: InfoService,
                    > tonic::server::UnaryService<
                        super::ParticipantAcceptsContractRequest,
                    > for AcceptContractSvc<T> {
                        type Response = super::ParticipantAcceptsContractResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ParticipantAcceptsContractRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InfoService>::accept_contract(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AcceptContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.InfoService/TryAuthWebToken" => {
                    #[allow(non_camel_case_types)]
                    struct TryAuthWebTokenSvc<T: InfoService>(pub Arc<T>);
                    impl<
                        T: InfoService,
                    > tonic::server::UnaryService<super::AuthRequestWebToken>
                    for TryAuthWebTokenSvc<T> {
                        type Response = super::TryAuthResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthRequestWebToken>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InfoService>::try_auth_web_token(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TryAuthWebTokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/treaty_proto.InfoService/AuthForToken" => {
                    #[allow(non_camel_case_types)]
                    struct AuthForTokenSvc<T: InfoService>(pub Arc<T>);
                    impl<
                        T: InfoService,
                    > tonic::server::UnaryService<super::AuthRequestBasic>
                    for AuthForTokenSvc<T> {
                        type Response = super::TokenReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthRequestBasic>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InfoService>::auth_for_token(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthForTokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: InfoService> Clone for InfoServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: InfoService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InfoService> tonic::server::NamedService for InfoServiceServer<T> {
        const NAME: &'static str = "treaty_proto.InfoService";
    }
}
