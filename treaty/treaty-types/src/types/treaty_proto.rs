/// A message describing an error in the system. This usually refers to a SQL database error.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct TreatyError {
    /// A description of what went wrong.
    
    pub message: String,
    /// An optional description of how to fix the error.
    
    pub help: ::core::option::Option<String>,
    /// Not used: A numerical value tied to the error.
    
    pub number: u32,
}
/// A message describing a potential problem in the system.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct TreatyWarning {
    /// A description of a problem.
    
    pub message: String,
    /// An optional description of how to fix the error.
    
    pub help: ::core::option::Option<String>,
    /// Not used: A numerical value tied to the error.
    
    pub number: u32,
}
/// A log entry within Treaty.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct TreatyLogEntry {
    /// The local datetime of the log entry.
    
    pub dt: String,
    /// The UTC datetime of the log entry.
    
    pub dt_utc: String,
    /// The logging level. In order of severity: Error, Warn, Info, Debug, Trace.
    
    pub level: String,
    /// The actual log message.
    
    pub message: String,
}
/// Requests Treaty to return the last X number of logs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetLogsByLastNumberRequest {
    /// Authentication.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The last number of logs to fetch.
    
    pub number_of_logs: u32,
}
/// Responds with the total requested number of logs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetLogsByLastNumberReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// A list of log entries.
    
    pub logs: Vec<TreatyLogEntry>,
    /// An error if Treaty was unable to fetch logs.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the current settings from Treaty. These are the values from the "Settings.toml" file.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetSettingsRequest {
    /// Authentication.
    
    pub authentication: ::core::option::Option<AuthRequest>,
}
/// Responds with the current Settings.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetSettingsReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The settings in JSON format.
    
    pub settings_json: ::core::option::Option<String>,
    /// An error if Treaty was unable to return the settings.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests a list of hosts that this Treaty instance is cooperating with.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetCooperativeHostsRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
}
/// Responds with a list of hosts that this Treaty instance is cooperating with.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetCooperativeHostsReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The list of hosts this Treaty instance is cooperating with.
    
    pub hosts: Vec<HostInfoStatus>,
    /// An error if Treaty was unable to return the list of Hosts.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the current "DeletesToHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetDeletesToHostBehaviorRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
}
/// Responds with the current "DeletesToHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetDeletesToHostBehaviorReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The current behavior for the requested database and table.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub behavior: ::core::option::Option<u32>,
    /// An error if Treaty was unable to return the current behavior.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the current "DeletesFromHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetDeletesFromHostBehaviorRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
}
/// Responds with the current "DeletesFromHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetDeletesFromHostBehaviorReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The current behavior for the requested database and table.
    /// This value is defined in the /treaty/treaty-types/enum.rs file.
    
    pub behavior: ::core::option::Option<u32>,
    /// An error if Treaty was unable to return the current behavior.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the current "UpdatesToHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetUpdatesToHostBehaviorRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
}
/// Responds with the current "UpdatesToHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetUpdatesToHostBehaviorReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The current behavior for the for the requested database and table.
    /// This value is defined in the /treaty/treaty-types/enum.rs file.
    
    pub behavior: ::core::option::Option<u32>,
    /// An error if Treaty was unable to return the current behavior.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the current "UpdatesFromHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetUpdatesFromHostBehaviorRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
}
/// Responds with the current "UpdatesFromHost" behavior for the specified database and table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetUpdatesFromHostBehaviorReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The current behavior for the requested database and table.
    /// This value is defined in the /treaty/treaty-types/enum.rs file.
    
    pub behavior: ::core::option::Option<u32>,
    /// An error if Treaty was unable to return the current behavior.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Replies with the current version of Treaty at this instance,.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct VersionReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The version of Treaty.
    
    pub versions: ::core::option::Option<Versions>,
    /// An error if Treaty was unable to return the current version numbers.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// The version of Treaty.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct Versions {
    
    pub treaty: ::core::option::Option<String>,
}
/// Replies with the current Host Info at this Treaty instance.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct HostInfoReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The host information.
    
    pub host_info: ::core::option::Option<Host>,
    /// An error if Treaty was unable to return the current host information.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Replies with the result of attempting to revoke the current Json Web Token.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct RevokeReply {
    
    pub is_successful: bool,
}
/// Replies with an issued Json Web Token.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct TokenReply {
    
    pub is_successful: bool,
    
    pub expiration_utc: String,
    
    pub jwt: String,
}
/// Requests the current Active Contract for the specified database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetActiveContractRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
}
/// Replies with the active contract for the specified database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetActiveContractReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The active database contract.
    
    pub contract: ::core::option::Option<Contract>,
    /// An error if Treaty was unable to return the Active Contract for the specified database.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests a list of participants for the specified database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetParticipantsRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
}
/// Replies with the list of Participants for the specified database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetParticipantsReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// A list of participants for the specified database.
    
    pub participants: Vec<ParticipantStatus>,
    /// If the request has an error.
    
    pub is_error: bool,
    /// An error if Treaty was unable to return the list of participants for the specified database.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests a list of the databases at Treaty.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetDatabasesRequest {
    
    pub authentication: ::core::option::Option<AuthRequest>,
}
/// Replies with the list of databses at Tretay.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetDatabasesReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The databases hosted at this Treaty instance.
    
    pub databases: Vec<DatabaseSchema>,
    /// An error if Treaty was unable to return the list of databases.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests to accept a pending action at a Participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct AcceptPendingActionRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The row id.
    
    pub row_id: u32,
}
/// Replies with the result of accepting a pending action at a Participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct AcceptPendingActionReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the acceptance of the action is successful.
    
    pub is_successful: bool,
    /// An error if Treaty was unable to accept the action.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests a list of pending actions at a Participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetPendingActionsRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The type of action we are interested in (UPDATE or DELETE)
    
    pub action: String,
}
/// Replies with a list of pending actions (statements).
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetPendingActionsReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// A list of pending statements to be executed.
    
    pub pending_statements: Vec<PendingStatement>,
    /// An error if Treaty was unable to get the list of pending actions.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// A statement that is queued to be executed at a Treaty instance.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct PendingStatement {
    /// The row id being affected
    
    pub row_id: u32,
    /// The UPDATE or DELETE statement
    
    pub statement: String,
    /// The time in UTC the request was made
    
    pub requested_ts_utc: String,
    /// The host id requesting the action
    
    pub host_id: String,
    /// The actual SQL statement being executed
    
    pub action: String,
}
/// Requests that a data log be enabled for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct SetDataLogTableStatusRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// If a data log should be enabled.
    
    pub use_data_log: bool,
}
/// Replies with the result of configuring a data log.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct SetDataLogTableStatusReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the request was successful or not.
    
    pub is_successful: bool,
    /// An error if Treaty was unable to set the requested status of data logging.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the status of data logging for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetDataLogTableStatusRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
}
/// Replies with the status of data logging for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetDataLogTableStatusReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If data logging was configured or not.
    
    pub use_data_log: bool,
    /// An error if Treaty was unable to get the status of data logging.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the row ids for the specified WHERE clause.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetReadRowIdsRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The WHERE clause.
    
    pub where_clause: String,
}
/// Replies with a list of row ids for the request.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetReadRowIdsReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The list of row ids.
    
    pub row_ids: Vec<u32>,
    /// An error if Treaty was unable to get the list of affected row ids.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests the saved data hash for the specified row id.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetDataHashRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The row id.
    
    pub row_id: u32,
}
/// Returns the requested data hash.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetDataHashReply {
    /// The authentiation result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The requested data hash.
    
    pub data_hash: u64,
    /// An error if Treaty was unable to get the requested data hash.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to change the "DeletesToHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ChangeDeletesToHostBehaviorRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The "DeletesToHost" before setting.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub behavior: u32,
}
/// Replies with the result of the request to change the "DeletesToHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ChangeDeletesToHostBehaviorReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the request was successful.
    
    pub is_successful: bool,
    /// A message if any additional information is needed. This value can be empty.
    
    pub message: String,
    /// An error if Treaty was unable to get the requested data hash.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// A request to change the "UpdatesToHostBehavior".
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ChangeUpdatesToHostBehaviorRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The behavior to change to.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub behavior: u32,
}
/// Replies with the result of attempting to change the "UpdatesToHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ChangeUpdatesToHostBehaviorReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the request was successful.
    
    pub is_successful: bool,
    /// A message if any additional information is needed. This value can be empty.
    
    pub message: String,
    /// An error if Treaty was unable to set the behavior.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to change the "DeletesFromHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ChangeDeletesFromHostBehaviorRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The behavior to change to.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub behavior: u32,
}
/// Replies with the result of changing the "DeletesFromHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ChangeDeletesFromHostBehaviorReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the request was successful or not.
    
    pub is_successful: bool,
    /// A message if any additional information is available. This value can be empty.
    
    pub message: String,
    /// An error if Treaty was unable to change the behavior.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to change the "UpdatesFromHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ChangeUpdatesFromHostBehaviorRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The behavior to change to.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub behavior: u32,
}
/// Replies with the result of changing the "UpdatesFromHost" behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ChangesUpdatesFromHostBehaviorReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the request was successful or not.
    
    pub is_successful: bool,
    /// A message with any additional information. This value can be empty.
    
    pub message: String,
    /// An error if Treaty was unable to change the behavior.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// A request to attempt to authenticate at the specified Participant. This tests to make sure that we
/// have not been rejected by the specified participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct TryAuthAtParticipantRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The participant id.
    
    pub participant_id: String,
    /// The participant alias.
    
    pub participant_alias: String,
    /// The database name.
    
    pub db_name: String,
}
/// Replies with the result of attempting to autenticate at the specified Participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct TryAuthAtPartipantReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the result was successful.
    
    pub is_successful: bool,
    /// A message with any additional information. This value can be empty.
    
    pub message: String,
    /// An error if Treaty was unable to attempt authentication.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to change the status of a Host to ALLOW/DENY.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ChangeHostStatusRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The host alias.
    
    pub host_alias: String,
    /// The host id.
    
    pub host_id: String,
    /// The status to change for the host.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub status: u32,
}
/// Replies with the result of changing the host status.
/// This can return "false" if the host id or alias was not found in the database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ChangeHostStatusReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the request was successful.
    
    pub is_successful: bool,
    /// The status the value was changed to. This echoes what was sent.
    
    pub status: u32,
    /// An error if Treaty was unable to change the host status.
    
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
#[derive(Clone, PartialEq)]
pub struct GenerateHostInfoRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The friendly host name to use.
    
    pub host_name: String,
}
/// Replies with the result of attempting to generate host information.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GenerateHostInfoReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If creating host information was successful or not.
    
    pub is_successful: bool,
    /// An error if Treaty was unable to generate host information.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests to send the active database contract ot the specified participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct SendParticipantContractRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The name of the database.
    
    pub database_name: String,
    /// The alias of the participant.
    
    pub participant_alias: String,
}
/// Replies with the result of sending the active contract to the participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct SendParticipantContractReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the contract was sent.
    
    pub is_sent: bool,
    /// The current status of the contract at the participant.
    /// This is an echo of what the Participant thinks the contract status is.
    
    pub contract_status: u32,
    /// An error if Treaty was unable to send the active contract to the Participant.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// A message representing the results of a SQL query.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct StatementResultset {
    /// The total number of rows affected, if applicable.
    
    pub number_of_rows_affected: u64,
    /// A list of Row items.
    
    pub rows: Vec<Row>,
    /// An error if Treaty was unable to provide results.
    
    pub error: ::core::option::Option<TreatyError>,
    /// A warning if there is a data mis-match.
    
    pub warning: ::core::option::Option<TreatyWarning>,
}
/// Requests to create a database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct CreateUserDatabaseRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
}
/// Delete user database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct DeleteUserDatabaseRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
}
/// Replies with the result of deleting a database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct DeleteUserDatabaseReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the database was deleted.
    
    pub is_deleted: bool,
    /// A message describing any details if needed. This field can be blank.
    
    pub message: String,
    /// An error if Treaty was unable to delete the requested database.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Replies with the result of creating a database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct CreateUserDatabaseReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the database was created.
    
    pub is_created: bool,
    /// A message describing any details if needed. This field can be blank.
    
    pub message: String,
    /// An error if Treaty was unable to create the requested database.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests to execute the specified SELECT statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ExecuteReadRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The SELECT SQL statement.
    
    pub sql_statement: String,
    /// The datababase type (Sqlite, MySQL, Postgres)
    
    pub database_type: u32,
}
/// Replies with the result of the SELECT statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ExecuteReadReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The total number of result-sets.
    
    pub total_resultsets: u64,
    /// The results of the query.
    
    pub results: Vec<StatementResultset>,
    /// Denotes if there was an error executing the query.
    
    pub is_error: bool,
    /// An error if Treaty was unable to execute the SELECT statement provided.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests to execute the provided INSERT/UPDATE/DELETE statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ExecuteWriteRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The INSERT/UPDATE/DELETE statement to execute.
    
    pub sql_statement: String,
    /// The database type (Sqlite, MySQL, Postgres).
    
    pub database_type: u32,
    /// The WHERE clause of the statement, if applicable.
    /// ℹ️ Note: If the "sql_statement" includes a WHERE clause, duplicate the contents here. Otherwise, leave the string empty.
    /// This is needed because of a limitation with Treaty's implementation of Antlr. In the future, hopefully this field will not be needed.
    
    pub where_clause: String,
}
/// Replies with the results of the provided INSERT/UPDATE/DELETE statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ExecuteWriteReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the statement executed without error.
    
    pub is_successful: bool,
    /// The total number of rows the statement affected, if applicable.
    
    pub total_rows_affected: u32,
    /// Denotes if there was an error executing the statement.
    
    pub is_error: bool,
    /// An error if Treaty was uanble to execute the INSERT/UPDATE/DELETE statement provided.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests to find out if the specified table exists.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct HasTableRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
}
/// Replies if the specified table exists.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct HasTableReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the table exists or not.
    
    pub has_table: bool,
    /// An error if Treaty was uanble to determine if the specified table exists or not.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// A request to generate a contract for the specified database.
/// If there is already a database contract for this database, it will be marked as inactive and a new one
/// will be generated.
/// ❗ Note: You will need to ensure that each table in your database has a Logical Storage Policy set before
/// generating a contract, otherwise this call will fail.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GenerateContractRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// A host name to identify this Treaty instance to others.
    
    pub host_name: String,
    /// A general description for the contract.
    /// This will be made visible to Participants.
    
    pub description: String,
    /// The name of the database this contract is for.
    
    pub database_name: String,
    /// The Remote Delete Behavior for this Host for this contract.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub remote_delete_behavior: u32,
}
/// Replies with the status of generating a contract for the specified database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GenerateContractReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If contract generation was successful.
    
    pub is_successful: bool,
    /// A message providing any additional details. This value can be empty.
    
    pub message: String,
    /// An error if Treaty was unable to generate the contract.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests Treaty to set the specified Logical Storage Policy for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct SetLogicalStoragePolicyRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The policy to set the table to.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub policy_mode: u32,
}
/// Replies with the result of setting the Logical Storage Policy for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct SetLogicalStoragePolicyReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the request was successful.
    
    pub is_successful: bool,
    /// A message providing any additional information. This value can be empty.
    
    pub message: String,
    /// An error if Treaty was unable to generate the contract.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Request the current Logical Storage Policy for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetLogicalStoragePolicyRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
}
/// Replies with the current Logical Storage Policy for the specified table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetLogicalStoragePolicyReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// The current Logical Storage policy for the requested table.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub policy_mode: u32,
    /// An error if Treaty was unable to get the Logical Storage Policy for the specified table.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests Treaty to execute the specified INSERT/UPDATE/DELETE statement both at the
/// Host and the Participant.
/// This attempts to execute the action at the Participant and if successful
/// will keep metadata about the action at the Host.
/// For more information, see the README.md or the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ExecuteCooperativeWriteRequest {
    /// The authentaction request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The name of the database.
    
    pub database_name: String,
    /// The INSERT/UPDATE/DELETE statement to execute at the Participant.
    
    pub sql_statement: String,
    /// The type of database: Sqlite, MySQL, Postgres.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub database_type: u32,
    /// The participant alias this statement is for.
    
    pub alias: String,
    /// The participant id this statement is for.
    
    pub participant_id: String,
    /// The WHERE clause of the INSERT/UPDATE/STATEMENT. For technical reasons this needs to be the same as in the "sql_statement" field
    /// if applicable. This field can be empty.
    
    pub where_clause: String,
}
/// Replies with the result of Cooperative Write.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ExecuteCooperativeWriteReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the result was successful.
    
    pub is_successful: bool,
    /// The total number of rows affected by the INSERT/UPDATE/DELETE statement.
    
    pub total_rows_affected: u32,
    /// An error if Treaty was unable to execute the Cooperative Write.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to add the Participant to the specified database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct AddParticipantRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// An alias for this participant.
    
    pub alias: String,
    /// The IP address for this Participant, in IP 4 format.
    
    pub ip4_address: String,
    /// The database port number for this Participant.
    
    pub port: u32,
    /// The HTTP address for this Participant.
    
    pub http_addr: String,
    /// The HTTP port for this Participant.
    
    pub http_port: u32,
    /// The Host Id for this participant. This field is optional. This is used if Treaty is being hosted by a
    /// `treaty-proxy` instance, where multiple Treaty instances are hosted and you need to uniquely identify which Treaty instance
    /// you are attempting to talk to.
    
    pub id: ::core::option::Option<String>,
}
/// Replies with the result of adding a Participant.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct AddParticipantReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If adding the Participant was successful.
    
    pub is_successful: bool,
    /// A message describing any additional details if needed. This field can be empty.
    
    pub message: String,
    /// An error if Treaty was unable to add the Participant.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests to view a list of all contracts Treaty has that are in a Pending state,.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ViewPendingContractsRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
}
/// Replies with a list of pending contracts.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ViewPendingContractsReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// A list of contracts that are in a pending state. This list may be empty.
    
    pub contracts: Vec<Contract>,
    /// An error if Treaty was unable to get the list of pending contracts.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests Treaty to accept the pending contract from the specified Host,.
/// This will send a message back to the host that we are ready to accept data
/// and will create additional meta-data structures to support the contract.
/// For more information, see the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct AcceptPendingContractRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The host that has sent us the pending contract.
    
    pub host_alias: String,
}
/// Replies with the result of accepting a pending contract,.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct AcceptPendingContractReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the request was successful.
    
    pub is_successful: bool,
    /// A message with any additional information. This field may be blank.
    
    pub message: String,
    /// An error if Treaty was unable to accept the pending contract.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests that Treaty reject the pending contract from the specified host,.
/// This sends a message back to the Host that we are not interested in this contract. No database changes are made.
/// For more information, see the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct RejectPendingContractRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The alias of the host.
    
    pub host_alias: String,
}
/// Replies with the result of rejecting a pending contract.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct RejectPendingContractReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the rejection was successful.
    
    pub is_successful: bool,
    /// A message with any additional information. This field may be blank.
    
    pub message: String,
    /// An error if Treaty was unable to reject the pending contract.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Requests that Treaty enable cooperative features for a database, if authentiated.
/// This modifies the database with additional structures to support adding Participants and other related actions.
/// For more information, see the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct EnableCoooperativeFeaturesRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name to enable cooperative features.
    
    pub database_name: String,
}
/// Replies with the result of enabling cooperative features,.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct EnableCoooperativeFeaturesReply {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If enabling cooperative features was successful.
    
    pub is_successful: bool,
    /// A message containing any additional details. This field may be blank.
    
    pub message: String,
    /// An error if Treaty was unable to enable cooperative features on the specified database.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// A request to validate that we have access to this Treaty instance.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct TryAuthRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
}
/// Replies with the authentication result.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct TryAuthResult {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
}
/// A message for creating a table in a database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct CreateTableRequest {
    /// The user requesting the table creation.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database in which to create the table.
    
    pub database_name: String,
    /// The database GUID in which to create the table.
    
    pub database_guid: String,
    /// The name of the table to create.
    
    pub table_name: String,
    /// A list of columns for the table.
    
    pub columns: Vec<ColumnSchema>,
}
/// A message for describing the result of a CreateTableRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct CreateTableResult {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the table was created.
    
    pub is_successful: bool,
    /// The name of the database the table was created in.
    
    pub database_name: String,
    /// Any additional information if needed. This field can be blank.
    
    pub result_message: String,
    /// The database id the table was created in.
    
    pub database_id: String,
    /// The table name that was created. This should line up with the request made and is intended for confirmation.
    
    pub table_name: String,
    /// The table id that was created.
    
    pub table_id: String,
    /// If the request failed in any manner.
    
    pub is_error: bool,
    /// An error detailing if the request failed in any manner.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// A message describing the details of a row in a partial database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct RowInfo {
    /// The name of the database the row is in.
    
    pub database_name: String,
    /// The table name the row is in.
    
    pub table_name: String,
    /// The row id.
    
    pub row_id: u32,
    /// The data hash of the row.
    
    pub data_hash: u64,
}
/// A request for Treaty to execute the specified INSERT statement.
/// ❗ Warning: At the moment, Treaty can only handle simple INSERT statements for a single row.
/// For more information, see the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct InsertDataRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The actual INSERT statement.
    /// Note: while this is duplicative, at the moment the contents of this INSERT statement must match the database and table name.
    
    pub cmd: String,
}
/// A result of executing an INSERT statement against a partial database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct InsertDataResult {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the result was successful.
    
    pub is_successful: bool,
    /// A hash of the data inserted.
    
    pub data_hash: u64,
    /// An additional message if needed. This field can be blank.
    
    pub message: String,
    /// The row id of the record inserted.
    
    pub row_id: u32,
    /// If there was an error executing the INSERT statement.
    
    pub is_error: bool,
    /// An error detailing if the request failed.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// A request for Treaty to execute the specified UPDATE statement if authentiated.
/// ❗ Warning: At the moment, Treaty can only handle simple UPDATE statements.
/// For more information, see the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct UpdateDataRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The actual UPDATE statement.
    
    pub cmd: String,
    
    pub where_clause: String,
}
/// Replies with the result of executing the provided UPDATE statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct UpdateDataResult {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the UPDATE statement was successful.
    
    pub is_successful: bool,
    /// A message describing any additional details. This field can be blank.
    
    pub message: String,
    /// A copy of the rows that were affected.
    
    pub rows: Vec<RowInfo>,
    /// The status of the actual update. Values are:
    /// 0 - unknown
    /// 1 - success (overwrite or overwrite with log)
    /// 2 - pending (queue for review)
    /// 3 - ignored (ignore)
    /// Note: These values are defined in the /treaty/treaty-types/enums.rs file.
    
    pub update_status: u32,
    /// If there was an error executing the UPDATE statement.
    
    pub is_error: bool,
    /// Any details if there was an error executing the UPDATE statement.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// A request for Treaty to execute the provided DELETE statement.
/// ❗ Warning: At the moment, Treaty can only handle simple UPDATE statements.
/// For more information, see the manual.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct DeleteDataRequest {
    /// The authentication requestl
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The actual DELETE statement.
    /// Note: This DELETE statement needs to match the field specified prior.
    
    pub cmd: String,
    /// ❗ The WHERE clause of the delete statement. This field needs to match the WHERE clause if there is one in the prior field.
    /// Otherwise, this field can be left blank.
    
    pub where_clause: String,
}
/// Describes the result of Treaty executing the specified DELETE statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct DeleteDataResult {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the command was successfully executed.
    
    pub is_successful: bool,
    /// A message providing further details if needed. This field can be blank.
    
    pub message: String,
    /// A message describing details of the rows impacted.
    
    pub rows: Vec<RowInfo>,
    /// Denotes if there was an error executing the DELETE statement.
    
    pub is_error: bool,
    /// An error describing details if needed.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// A request to get a specified row from a partial database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetRowFromPartialDatabaseRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The row which to get.
    
    pub row_address: ::core::option::Option<RowParticipantAddress>,
    /// Additional details for debugging purposes.
    
    pub telemetry: ::core::option::Option<Telemetry>,
}
/// A response containing the specified row requested,.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct GetRowFromPartialDatabaseResult {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the request was successful.
    
    pub is_successful: bool,
    /// Any additional details if needed. This field can be blank.
    
    pub result_message: String,
    /// The actual row requested.
    
    pub row: ::core::option::Option<Row>,
    /// An error if Treaty was unable to get the specified row.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// A message from a host to a participant to save a contract.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct SaveContractRequest {
    /// A contract to save.
    
    pub contract: ::core::option::Option<Contract>,
    /// Any additional debugging details.
    
    pub telemetry: ::core::option::Option<Telemetry>,
    /// The host id of the targeted Treaty instance. This is usually used if `treaty` is being hosted by a `treaty-proxy` instance.
    
    pub id: ::core::option::Option<String>,
}
/// A message describing the results of saving a contract.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct SaveContractResult {
    /// If the contract was saved.
    
    pub is_saved: bool,
    /// A message confirming the Participant's status of the contract (Accepted/Rejected/Pending)
    
    pub contract_status: u32,
    /// If the Participant wishes to confirm their information back to the Host. This is useful
    /// If the Host and the Participant are out of sync with the contract status.
    
    pub participant_info: ::core::option::Option<Participant>,
    /// If there was an error saving the contract.
    
    pub is_error: bool,
    /// Any details if Treaty was unable to save the contract.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to accept a particular contract.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ParticipantAcceptsContractRequest {
    /// The participant accepting the contract. This is used as a way to identify the Participant.
    
    pub participant: ::core::option::Option<Participant>,
    /// The GUID/UUID of the contract.
    
    pub contract_guid: String,
    /// The GUID/UUID version of the contract.
    /// Contracts can be updated, and so with each change of a contract the version must be changed.
    
    pub contract_version_guid: String,
    /// The database name.
    
    pub database_name: String,
    /// Any additional debugging information.
    
    pub telemetry: ::core::option::Option<Telemetry>,
    /// The host id of the targeted Treaty instance. This is usually used if `treaty` is being hosted by a `treaty-proxy` instance.
    
    pub id: ::core::option::Option<String>,
}
/// Describes the result of notifying that a Participant has accepted a contract.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ParticipantAcceptsContractResult {
    /// If the result is acknowledged.
    
    pub contract_acceptance_is_acknowledged: bool,
    /// If there was an error notifying acceptance of the contract.
    
    pub is_error: bool,
    /// An error describing details for the action, if appliable.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Request from a Participant to a Host to update the data hash.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct UpdateRowDataHashForHostRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// Additional telemetry for debugging.
    
    pub telemetry: ::core::option::Option<Telemetry>,
    /// The host information this data hash came from (from the perspective of the Host, this is the Participant's information).
    
    pub host_info: ::core::option::Option<Host>,
    /// The database name.
    
    pub database_name: String,
    /// The database id.
    
    pub database_id: String,
    /// The table name.
    
    pub table_name: String,
    /// The table id.
    
    pub table_id: u32,
    /// The row id.
    
    pub row_id: u32,
    /// The new hash value for the row.
    
    pub updated_hash_value: u64,
    /// If the row is deleted.
    
    pub is_deleted_at_participant: bool,
}
/// Replies with the result of the update data hash request.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct UpdateRowDataHashForHostResult {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the message was successful.
    
    pub is_successful: bool,
    /// An error if the updated data hash could not be sent.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// Request to notify the upstream Host that a row has been deleted.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct NotifyHostOfRemovedRowRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// Debugging information about the sender of this message.
    
    pub telemetry: ::core::option::Option<Telemetry>,
    /// The host information. From an upstream Host's perspective, this is the Participant.
    
    pub host_info: ::core::option::Option<Host>,
    /// The database name.
    
    pub database_name: String,
    /// The database id.
    
    pub database_id: String,
    /// The table name.
    
    pub table_name: String,
    /// The table id.
    
    pub table_id: u32,
    /// The row id.
    
    pub row_id: u32,
}
/// The result of notifying the upstream Host that a row has been deleted.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct NotifyHostOfRemovedRowResult {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the notification was successful.
    
    pub is_successful: bool,
    /// An error if Treaty was not able to notify the upstream Host.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// A message for basic online testing.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct TestRequest {
    /// The time the request was sent in UTC (RFC3339)
    
    pub request_time_utc: String,
    /// The origin URL, if applicable.
    
    pub request_origin_url: String,
    /// The origin IP4 address.
    
    pub request_origin_ip4: String,
    /// The oring IP6 address.
    
    pub request_origin_ip6: String,
    /// The origin port number.
    
    pub request_port_number: u32,
    /// A test message that should be echo'd back.
    
    pub request_echo_message: String,
}
/// A message for basic online testing.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct TestReply {
    /// The time the reply was generated in UTC (RFC3339)
    
    pub reply_time_utc: String,
    /// The message to echo back.
    
    pub reply_echo_message: String,
    /// The sender's Treaty version.
    
    pub treaty_version: String,
}
/// A message for general information.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct Telemetry {
    /// Endian-ness of the Treaty instance.
    
    pub is_little_endian: bool,
    /// A list of IP addresses for this sender.
    
    pub message_addresses: Vec<String>,
    /// The time the message was generated in UTC (RFC3339)
    
    pub message_generated_time_utc: String,
    /// A unique ID for this message.
    
    pub message_guid: String,
}
/// Credentials to authenticate against Treaty.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct AuthRequest {
    /// The name of the user.
    
    pub user_name: String,
    /// The pw of the user.
    
    pub pw: String,
    /// A hash of the pw of the user.
    
    pub pw_hash: Vec<u8>,
    /// A generated token of the pw of the user.
    
    pub token: Vec<u8>,
    /// A Json Web Token in place of credentials.
    
    pub jwt: String,
    /// An optional Host Id of the Treaty instance. This is used when talking to a `treaty-proxy` instance.
    
    pub id: ::core::option::Option<String>,
}
/// A message describing the results of an authentication attempt.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct AuthResult {
    /// .
    
    pub is_authenticated: bool,
    /// An optional message for any additional information.
    
    pub message: ::core::option::Option<String>,
}
/// A message for creating a partial database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct CreatePartialDatabaseRequest {
    /// The authentication request.
    
    pub authentication: ::core::option::Option<AuthRequest>,
    /// Additional debugging information.
    
    pub telemetry: ::core::option::Option<Telemetry>,
    /// The database name.
    
    pub database_name: String,
}
/// A message describing the results of a CreateDatabaseRequest.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct CreatePartialDatabaseResult {
    /// The authentication result.
    
    pub authentication_result: ::core::option::Option<AuthResult>,
    /// If the partial database creation was successful.
    
    pub is_successful: bool,
    /// The name of the database.
    
    pub database_name: String,
    /// The id of the database.
    
    pub database_id: String,
    /// If there was an error creating the database.
    
    pub is_error: bool,
    /// An error describing what happened during the request, if applicable.
    
    pub error: ::core::option::Option<TreatyError>,
}
/// An object for representing a row in a table. Used for returning data.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct Row {
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The row id.
    
    pub row_id: u32,
    /// A list of values held by the row.
    
    pub values: Vec<RowValue>,
    /// Deprecated. This will be deleted.
    
    pub is_remoteable: bool,
    /// A description about the row such as if the data is out of sync between a Host and a Participant.
    
    pub remote_metadata: ::core::option::Option<RowRemoteMetadata>,
    /// A hash of the row's data.
    
    pub hash: Vec<u8>,
}
/// An object for storing values for a row in a table. Used for returning data.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct RowValue {
    /// The column of the value.
    
    pub column: ::core::option::Option<ColumnSchema>,
    /// If the value is NULL.
    
    pub is_null_value: bool,
    /// The binary value.
    
    pub value: Vec<u8>,
    /// A string representation of the value.
    
    pub string_value: String,
}
/// Describes the data status of the host in relation to the Participant.
/// Example the data hash between the host and the participant do not match
/// or if the row was deleted at the participant, but the reference at the host is not.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct RowRemoteMetadata {
    
    pub is_remote_out_of_sync_with_host: bool,
    
    pub is_hash_out_of_sync_with_host: bool,
    
    pub is_remote_deleted: bool,
    
    pub is_local_deleted: bool,
}
/// A message for describing schema information of a column in a database table.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ColumnSchema {
    /// The column name.
    
    pub column_name: String,
    /// The column type.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub column_type: u32,
    /// The max or fixed length of the column, if applicable.
    
    pub column_length: u32,
    /// If the column is nullable or not.
    
    pub is_nullable: bool,
    /// The ordinal value of the column, i.e. the order in which the column appears in the table.
    
    pub ordinal: u32,
    /// Empty string in a request, populated in a response with the table GUID the column is attached to.
    
    pub table_id: String,
    /// Empty string in a request, populated in a response with the column GUID value.
    
    pub column_id: String,
    /// If the column is the primary key of the table. If this is part of a list of columns, it is implied to be a composite primary key.
    
    pub is_primary_key: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct Contract {
    /// the unique contract id
    
    pub contract_guid: String,
    /// a description of the rights in the contract
    
    pub description: String,
    /// the schema of the entire database
    
    pub schema: ::core::option::Option<DatabaseSchema>,
    /// a GUID representing the version of the contract
    
    pub contract_version: String,
    /// The host for the contract.
    
    pub host_info: ::core::option::Option<Host>,
    /// the status of the contract, if applicable
    
    pub status: u32,
}
/// A message representing information about a Participant in the system.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct Participant {
    /// The public GUID/UUID that a Participant identifies itself with.
    
    pub participant_guid: String,
    /// A friendly alias.
    
    pub alias: String,
    /// The IP4 address.
    
    pub ip4_address: String,
    /// The IP6 address.
    
    pub ip6_address: String,
    /// The database port number.
    
    pub database_port_number: u32,
    /// A token used for authentication.
    
    pub token: Vec<u8>,
    /// An internal generated GUID/UUID for the Participant.
    
    pub internal_participant_guid: String,
    /// The HTTP address.
    
    pub http_addr: String,
    /// The HTTP port number.
    
    pub http_port: u32,
}
/// The status of a Participant at a Host.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ParticipantStatus {
    /// The participant details.
    
    pub participant: ::core::option::Option<Participant>,
    /// The contract status.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub contract_status: u32,
}
/// A Host in Treaty.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct Host {
    /// The public GUID/UUID the Host identifies itself with.
    
    pub host_guid: String,
    /// A friendly name for the host.
    
    pub host_name: String,
    /// A token used for authentication.
    
    pub token: Vec<u8>,
    /// Network settings for the Host.
    
    pub network: ::core::option::Option<HostNetwork>,
}
/// A Host's network settings.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct HostNetwork {
    /// The IP4 Address.
    
    pub ip4_address: ::core::option::Option<String>,
    /// The IP6 address.
    
    pub ip6_address: ::core::option::Option<String>,
    /// The database port number.
    
    pub database_port_number: ::core::option::Option<u32>,
    /// The HTTP address.
    
    pub http_addr: ::core::option::Option<String>,
    /// The HTTP port.
    
    pub http_port: ::core::option::Option<u32>,
}
/// A message describing the latest status of a Host.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct HostInfoStatus {
    /// Host information.
    
    pub host: ::core::option::Option<Host>,
    /// The last time a message was seen from this host in UTC (RFC3339)
    
    pub last_communcation_utc: String,
    /// The current HostStatus.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub status: u32,
}
/// A message for describing the schema of a database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct DatabaseSchema {
    /// The database name.
    
    pub database_name: String,
    /// The database id.
    
    pub database_id: String,
    /// The tables of the database.
    
    pub tables: Vec<TableSchema>,
    /// The type of database: Sqlite, Postgres, or MySQL.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub database_type: u32,
    /// The type of Treaty database; i.e. A Host, Partial, or internal Treaty system database.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    
    pub treaty_database_type: u32,
    /// If the database has cooperative features.
    
    pub cooperation_enabled: bool,
    /// If the database has any participants.
    
    pub has_participants: bool,
}
/// A message for describing the schema information of a table in a database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct TableSchema {
    /// The table name.
    
    pub table_name: String,
    /// The table id.
    
    pub table_id: String,
    /// The database name this table belongs to.
    
    pub database_name: String,
    /// The database id this table belongs to.
    
    pub database_id: String,
    /// The columns of the table.
    
    pub columns: Vec<ColumnSchema>,
    /// The Logical Storage Policy for this table.
    /// This value is defined in the /treaty/treaty-types/enums.rs file.
    /// For more information, see the manual.
    
    pub logical_storage_policy: u32,
}
/// A message for identifying the location of a row in a partial database.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct RowParticipantAddress {
    /// The database name.
    
    pub database_name: String,
    /// The table name.
    
    pub table_name: String,
    /// The row id.
    
    pub row_id: u32,
}
