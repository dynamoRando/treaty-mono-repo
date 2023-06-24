use chrono::{DateTime, Utc};
use treaty_types::enums::*;
use crate::{
   
    error::{TreatyDbError, TreatyGenerateContractError},
    models::{
        CdsHosts, CoopDatabaseContract, CoopDatabaseParticipant, CoopDatabaseParticipantData,
        HostInfo, LogEntry, PartialDataResult, Table, TreatySaveContractResult,
    },
    treaty_proto::{
        ColumnSchema, Contract, DatabaseSchema, Participant, ParticipantStatus, PendingStatement,
        Row, TokenReply,
    },
};

/// An abstraction over the database type: Sqlite, MySql, Postgres, SQL Server. These are the database actions
/// that Treaty can support.
pub trait DbiActions {
    fn verify_token(&self, token: &str) -> Result<bool, TreatyDbError>;
    fn get_cooperative_hosts(&self) -> Result<Option<Vec<CdsHosts>>, TreatyDbError>;
    fn save_token(
        &self,
        login: &str,
        token: &str,
        expiration: DateTime<Utc>,
    ) -> Result<(), TreatyDbError>;
    fn auth_for_token(&self, login: &str, pw: &str) -> Result<TokenReply, TreatyDbError>;
    fn login_has_token(&self, login: &str) -> Result<bool, TreatyDbError>;
    fn revoke_token(&self, token: &str) -> Result<bool, TreatyDbError>;
    fn delete_expired_tokens(&self) -> Result<(), TreatyDbError>;
    fn get_last_log_entries(&self, number_of_entries: u32) -> Result<Vec<LogEntry>, TreatyDbError>;
    fn create_token_for_login(&self, login: &str)
        -> Result<(String, DateTime<Utc>), TreatyDbError>;
    fn accept_pending_action_at_participant(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<PartialDataResult, TreatyDbError>;
    fn get_pending_actions(
        &self,
        db_name: &str,
        table_name: &str,
        action: &str,
    ) -> Result<Option<Vec<PendingStatement>>, TreatyDbError>;
    fn get_data_hash_at_host(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Option<u64>, TreatyDbError>;
    fn get_data_hash_at_participant(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Option<u64>, TreatyDbError>;
    fn read_row_id_from_part_db(
        &self,
        db_name: &str,
        table_name: &str,
        where_clause: &str,
    ) -> Result<u32, TreatyDbError>;
    fn remove_remote_row_reference_from_host(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<bool, TreatyDbError>;
    fn get_cds_host_for_part_db(&self, db_name: &str) -> Result<Option<CdsHosts>, TreatyDbError>;
    fn get_treaty_db_type(&self, db_name: &str) -> Result<TreatyDatabaseType, TreatyDbError>;
    fn db_type(&self) -> DatabaseType;
    fn get_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<UpdatesToHostBehavior, TreatyDbError>;
    fn get_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<DeletesToHostBehavior, TreatyDbError>;
    fn get_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<DeletesFromHostBehavior, TreatyDbError>;
    fn get_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<UpdatesFromHostBehavior, TreatyDbError>;
    fn change_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError>;
    fn change_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError>;
    fn change_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError>;
    fn change_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError>;
    fn get_row_from_partial_database(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Row, TreatyDbError>;
    fn change_host_status_by_id(&self, host_id: &str, status: u32) -> Result<bool, TreatyDbError>;
    fn change_host_status_by_name(
        &self,
        host_name: &str,
        status: u32,
    ) -> Result<bool, TreatyDbError>;
    fn verify_host_by_id(&self, host_id: &str, token: Vec<u8>) -> Result<bool, TreatyDbError>;
    fn verify_host_by_name(&self, host_name: &str, token: Vec<u8>) -> Result<bool, TreatyDbError>;
    fn delete_metadata_in_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError>;
    fn update_metadata_in_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError>;

    fn insert_metadata_into_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError>;
    fn delete_data_in_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host_id: &str,
    ) -> Result<PartialDataResult, TreatyDbError>;
    fn update_data_into_partial_db_queue(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host: &CdsHosts,
    ) -> Result<PartialDataResult, TreatyDbError>;
    fn update_data_into_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        host_id: &str,
        where_clause: &str,
    ) -> Result<PartialDataResult, TreatyDbError>;
    fn insert_data_into_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
    ) -> Result<PartialDataResult, TreatyDbError>;
    fn update_participant_accepts_contract(
        &self,
        db_name: &str,
        participant: CoopDatabaseParticipant,
        participant_message: Participant,
        accepted_contract_id: &str,
    ) -> Result<bool, TreatyDbError>;
    fn create_partial_database_from_contract(
        &self,
        contract: &Contract,
    ) -> Result<bool, TreatyDbError>;
    fn accept_pending_contract(&self, host_name: &str) -> Result<bool, TreatyDbError>;
    fn get_pending_contracts(&self) -> Result<Option<Vec<Contract>>, TreatyDbError>;
    fn get_accepted_contracts(&self) -> Result<Option<Vec<Contract>>, TreatyDbError>;
    fn save_contract(&self, contract: Contract) -> Result<TreatySaveContractResult, TreatyDbError>;
    fn get_table_id(&self, db_name: &str, table_name: &str) -> Result<String, TreatyDbError>;
    fn create_table_in_partial_database(
        &self,
        db_name: &str,
        table_name: &str,
        schema: Vec<ColumnSchema>,
    ) -> Result<bool, TreatyDbError>;
    fn get_db_id(&self, db_name: &str) -> Result<String, TreatyDbError>;
    fn create_partial_database(&self, db_name: &str) -> Result<bool, TreatyDbError>;
    fn has_role_name(&self, role_name: &str) -> Result<bool, TreatyDbError>;
    fn add_login_to_role(&self, login: &str, role_name: &str) -> Result<bool, TreatyDbError>;
    fn login_is_in_role(&self, login: &str, role_name: &str) -> Result<bool, TreatyDbError>;
    fn create_login(&self, login: &str, pw: &str) -> Result<bool, TreatyDbError>;
    fn get_database_names(&self) -> Result<Option<Vec<String>>, TreatyDbError>;
    fn has_login(&self, login: &str) -> Result<bool, TreatyDbError>;
    fn add_participant(
        &self,
        db_name: &str,
        alias: &str,
        ip4addr: &str,
        db_port: u32,
        http_addr: String,
        http_port: u16,
        id: Option<String>,
    ) -> Result<bool, TreatyDbError>;
    fn get_database_schema(&self, db_name: &str) -> Result<DatabaseSchema, TreatyDbError>;
    fn get_participant_by_alias(
        &self,
        db_name: &str,
        participant_alias: &str,
    ) -> Result<Option<CoopDatabaseParticipant>, TreatyDbError>;
    fn get_participant_by_id(
        &self,
        db_name: &str,
        participant_id: &str,
    ) -> Result<Option<CoopDatabaseParticipant>, TreatyDbError>;
    fn has_participant(
        &self,
        db_name: &str,
        participant_alias: &str,
    ) -> Result<bool, TreatyDbError>;
    fn get_active_contract(&self, db_name: &str) -> Result<CoopDatabaseContract, TreatyDbError>;
    fn get_logical_storage_policy(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<LogicalStoragePolicy, TreatyDbError>;
    fn set_logical_storage_policy(
        &self,
        db_name: &str,
        table_name: &str,
        policy: LogicalStoragePolicy,
    ) -> Result<bool, TreatyDbError>;
    fn has_table(&self, db_name: &str, table_name: &str) -> Result<bool, TreatyDbError>;
    fn execute_write_at_host(&self, db_name: &str, cmd: &str) -> Result<usize, TreatyDbError>;
    fn execute_write_at_partipant(&self, db_name: &str, cmd: &str) -> Result<usize, TreatyDbError>;
    fn execute_read_at_participant(&self, db_name: &str, cmd: &str)
        -> Result<Table, TreatyDbError>;
    fn execute_read_at_host(&self, db_name: &str, cmd: &str) -> Result<Table, TreatyDbError>;
    fn has_cooperative_tables(&self, db_name: &str, cmd: &str) -> Result<bool, TreatyDbError>;
    fn get_participants_for_table(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<Option<Vec<CoopDatabaseParticipantData>>, TreatyDbError>;
    fn get_active_contract_proto(&self, db_name: &str) -> Result<Option<Contract>, TreatyDbError>;
    fn get_participants_for_database(
        &self,
        db_name: &str,
    ) -> Result<Option<Vec<ParticipantStatus>>, TreatyDbError>;
    fn get_cooperative_tables(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<Option<Vec<String>>, TreatyDbError>;
    fn create_database(&self, db_name: &str) -> Result<(), TreatyDbError>;
    fn delete_database(&self, db_name: &str) -> Result<(), TreatyDbError>;
    fn enable_coooperative_features(&self, db_name: &str) -> Result<bool, TreatyDbError>;
    fn generate_contract(
        &self,
        db_name: &str,
        host_name: &str,
        desc: &str,
        remote_delete_behavior: RemoteDeleteBehavior,
    ) -> Result<bool, TreatyGenerateContractError>;
    fn treaty_get_host_info(&self) -> Result<Option<HostInfo>, TreatyDbError>;
    fn treaty_generate_host_info(&self, host_name: &str) -> Result<(), TreatyDbError>;
    fn if_treaty_host_info_exists(&self) -> Result<bool, TreatyDbError>;
    fn generate_and_get_host_info(&self, host_name: &str) -> Result<HostInfo, TreatyDbError>;
    fn configure_admin_hash(&self, login: &str, hash: Vec<u8>) -> Result<(), TreatyDbError>;
    fn configure_admin(&self, login: &str, pw: &str) -> Result<(), TreatyDbError>;
    fn verify_login(&self, login: &str, pw: &str) -> Result<bool, TreatyDbError>;
    fn configure_treaty_db(&self) -> Result<(), TreatyDbError>;
}
