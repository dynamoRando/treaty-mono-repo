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
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use treaty_types::enums::*;

/// An abstraction over the database type: Sqlite or Postgres. These are the database actions
/// that Treaty can support.
#[async_trait]
pub trait DbiActions {
    async fn verify_token(&self, token: &str) -> Result<bool, TreatyDbError>;
    async fn get_cooperative_hosts(&self) -> Result<Option<Vec<CdsHosts>>, TreatyDbError>;
    async fn save_token(
        &self,
        login: &str,
        token: &str,
        expiration: DateTime<Utc>,
    ) -> Result<(), TreatyDbError>;
    async fn auth_for_token(
        &self,
        login: &str,
        pw: &str,
        timeout_in_minutes: Option<u32>,
    ) -> Result<TokenReply, TreatyDbError>;
    async fn login_has_token(&self, login: &str) -> Result<bool, TreatyDbError>;
    async fn revoke_token(&self, token: &str) -> Result<bool, TreatyDbError>;
    async fn delete_expired_tokens(&self) -> Result<(), TreatyDbError>;
    async fn get_last_log_entries(
        &self,
        number_of_entries: u32,
    ) -> Result<Vec<LogEntry>, TreatyDbError>;
    async fn create_token_for_login(
        &self,
        login: &str,
        timeout_in_minutes: Option<u32>,
    ) -> Result<(String, DateTime<Utc>), TreatyDbError>;
    async fn accept_pending_action_at_participant(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<PartialDataResult, TreatyDbError>;
    async fn get_pending_actions(
        &self,
        db_name: &str,
        table_name: &str,
        action: &str,
    ) -> Result<Option<Vec<PendingStatement>>, TreatyDbError>;
    async fn get_data_hash_at_host(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Option<u64>, TreatyDbError>;
    async fn get_data_hash_at_participant(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Option<u64>, TreatyDbError>;
    async fn read_row_id_from_part_db(
        &self,
        db_name: &str,
        table_name: &str,
        where_clause: &str,
    ) -> Result<u32, TreatyDbError>;
    async fn remove_remote_row_reference_from_host(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<bool, TreatyDbError>;
    async fn get_cds_host_for_part_db(
        &self,
        db_name: &str,
    ) -> Result<Option<CdsHosts>, TreatyDbError>;
    async fn get_treaty_db_type(&self, db_name: &str) -> Result<TreatyDatabaseType, TreatyDbError>;
    async fn db_type(&self) -> DatabaseType;
    async fn get_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<UpdatesToHostBehavior, TreatyDbError>;
    async fn get_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<DeletesToHostBehavior, TreatyDbError>;
    async fn get_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<DeletesFromHostBehavior, TreatyDbError>;
    async fn get_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<UpdatesFromHostBehavior, TreatyDbError>;
    async fn change_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError>;
    async fn change_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError>;
    async fn change_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError>;
    async fn change_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError>;
    async fn get_row_from_partial_database(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Row, TreatyDbError>;
    async fn change_host_status_by_id(
        &self,
        host_id: &str,
        status: u32,
    ) -> Result<bool, TreatyDbError>;
    async fn change_host_status_by_name(
        &self,
        host_name: &str,
        status: u32,
    ) -> Result<bool, TreatyDbError>;
    async fn verify_host_by_id(&self, host_id: &str, token: Vec<u8>)
        -> Result<bool, TreatyDbError>;
    async fn verify_host_by_name(
        &self,
        host_name: &str,
        token: Vec<u8>,
    ) -> Result<bool, TreatyDbError>;
    async fn delete_metadata_in_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError>;
    async fn update_metadata_in_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError>;

    async fn insert_metadata_into_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError>;
    async fn delete_data_in_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host_id: &str,
    ) -> Result<PartialDataResult, TreatyDbError>;
    async fn update_data_into_partial_db_queue(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host: &CdsHosts,
    ) -> Result<PartialDataResult, TreatyDbError>;
    async fn update_data_into_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        host_id: &str,
        where_clause: &str,
    ) -> Result<PartialDataResult, TreatyDbError>;
    async fn insert_data_into_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
    ) -> Result<PartialDataResult, TreatyDbError>;
    async fn update_participant_accepts_contract(
        &self,
        db_name: &str,
        participant: CoopDatabaseParticipant,
        participant_message: Participant,
        accepted_contract_id: &str,
    ) -> Result<bool, TreatyDbError>;
    async fn create_partial_database_from_contract(
        &self,
        contract: &Contract,
    ) -> Result<bool, TreatyDbError>;
    async fn accept_pending_contract(&self, host_name: &str) -> Result<bool, TreatyDbError>;
    async fn get_pending_contracts(&self) -> Result<Option<Vec<Contract>>, TreatyDbError>;
    async fn get_accepted_contracts(&self) -> Result<Option<Vec<Contract>>, TreatyDbError>;
    async fn save_contract(
        &self,
        contract: Contract,
    ) -> Result<TreatySaveContractResult, TreatyDbError>;
    async fn get_table_id(&self, db_name: &str, table_name: &str) -> Result<String, TreatyDbError>;
    async fn create_table_in_partial_database(
        &self,
        db_name: &str,
        table_name: &str,
        schema: Vec<ColumnSchema>,
    ) -> Result<bool, TreatyDbError>;
    async fn get_db_id(&self, db_name: &str) -> Result<String, TreatyDbError>;
    async fn create_partial_database(&self, db_name: &str) -> Result<bool, TreatyDbError>;
    async fn has_role_name(&self, role_name: &str) -> Result<bool, TreatyDbError>;
    async fn add_login_to_role(&self, login: &str, role_name: &str) -> Result<bool, TreatyDbError>;
    async fn login_is_in_role(&self, login: &str, role_name: &str) -> Result<bool, TreatyDbError>;
    async fn create_login(&self, login: &str, pw: &str) -> Result<bool, TreatyDbError>;
    async fn get_database_names(&self) -> Result<Option<Vec<String>>, TreatyDbError>;
    async fn has_login(&self, login: &str) -> Result<bool, TreatyDbError>;
    async fn add_participant(
        &self,
        db_name: &str,
        alias: &str,
        ip4addr: &str,
        db_port: Option<u32>,
        info_port: u32,
        http_addr: String,
        http_port: u16,
        id: Option<String>,
    ) -> Result<bool, TreatyDbError>;
    async fn get_database_schema(&self, db_name: &str) -> Result<DatabaseSchema, TreatyDbError>;
    async fn get_participant_by_alias(
        &self,
        db_name: &str,
        participant_alias: &str,
    ) -> Result<Option<CoopDatabaseParticipant>, TreatyDbError>;
    async fn get_participant_by_id(
        &self,
        db_name: &str,
        participant_id: &str,
    ) -> Result<Option<CoopDatabaseParticipant>, TreatyDbError>;
    async fn has_participant(
        &self,
        db_name: &str,
        participant_alias: &str,
    ) -> Result<bool, TreatyDbError>;
    async fn get_active_contract(
        &self,
        db_name: &str,
    ) -> Result<CoopDatabaseContract, TreatyDbError>;
    async fn get_logical_storage_policy(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<LogicalStoragePolicy, TreatyDbError>;
    async fn set_logical_storage_policy(
        &self,
        db_name: &str,
        table_name: &str,
        policy: LogicalStoragePolicy,
    ) -> Result<bool, TreatyDbError>;
    async fn has_table(&self, db_name: &str, table_name: &str) -> Result<bool, TreatyDbError>;
    async fn execute_write_at_host(&self, db_name: &str, cmd: &str)
        -> Result<usize, TreatyDbError>;
    async fn execute_write_at_partipant(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<usize, TreatyDbError>;
    async fn execute_read_at_participant(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<Table, TreatyDbError>;
    async fn execute_read_at_host(&self, db_name: &str, cmd: &str) -> Result<Table, TreatyDbError>;
    async fn has_cooperative_tables(&self, db_name: &str, cmd: &str)
        -> Result<bool, TreatyDbError>;
    async fn get_participants_for_table(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<Option<Vec<CoopDatabaseParticipantData>>, TreatyDbError>;
    async fn get_active_contract_proto(
        &self,
        db_name: &str,
    ) -> Result<Option<Contract>, TreatyDbError>;
    async fn get_participants_for_database(
        &self,
        db_name: &str,
    ) -> Result<Option<Vec<ParticipantStatus>>, TreatyDbError>;
    async fn get_cooperative_tables(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<Option<Vec<String>>, TreatyDbError>;
    async fn create_database(&self, db_name: &str) -> Result<(), TreatyDbError>;
    async fn delete_database(&self, db_name: &str) -> Result<(), TreatyDbError>;
    async fn delete_database_forcefully(&self, db_name: &str) -> Result<(), TreatyDbError>;
    async fn enable_coooperative_features(&self, db_name: &str) -> Result<bool, TreatyDbError>;
    async fn generate_contract(
        &self,
        db_name: &str,
        host_name: &str,
        desc: &str,
        remote_delete_behavior: RemoteDeleteBehavior,
    ) -> Result<bool, TreatyGenerateContractError>;
    async fn treaty_get_host_info(&self) -> Result<Option<HostInfo>, TreatyDbError>;
    async fn treaty_generate_host_info(&self, host_name: &str) -> Result<(), TreatyDbError>;
    async fn if_treaty_host_info_exists(&self) -> Result<bool, TreatyDbError>;
    async fn generate_and_get_host_info(&self, host_name: &str) -> Result<HostInfo, TreatyDbError>;
    async fn configure_admin_hash(&self, login: &str, hash: Vec<u8>) -> Result<(), TreatyDbError>;
    async fn configure_admin(&self, login: &str, pw: &str) -> Result<(), TreatyDbError>;
    async fn verify_login(&self, login: &str, pw: &str) -> Result<bool, TreatyDbError>;
    async fn configure_treaty_db(&self) -> Result<(), TreatyDbError>;
}
