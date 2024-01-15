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
use tracing::error;
use treaty_types::enums::*;

use super::dbi_actions::DbiActions;

#[derive(Debug, Clone)]
pub struct DbInterface<T: DbiActions + Clone + Send + Sync + 'static> {
    db: T,
}

impl<T: DbiActions + Clone + Send + Sync + 'static> DbInterface<T> {
    #[allow(dead_code)]
    pub fn new(db: T) -> Self {
        Self { db }
    }

    #[allow(dead_code)]
    pub async fn init(&self, un: &str, pw: &str) {
        if let Err(e) = self.configure_treaty_db().await {
            error!("{e:?}");
        }
        if let Err(e) = self.configure_admin(un, pw).await {
            error!("{e:?}");
        }
    }

    #[allow(dead_code)]
    pub async fn init_with_hash(&self, un: &str, hash: Vec<u8>) {
        if let Err(e) = self.configure_treaty_db().await {
            error!("{e:?}");
        }
        if let Err(e) = self.configure_admin_hash(un, hash).await {
            error!("{e:?}");
        }
    }
}

#[async_trait]
impl<T: DbiActions + Clone + Send + Sync + 'static> DbiActions for DbInterface<T> {
    async fn verify_token(&self, token: &str) -> Result<bool, TreatyDbError> {
        self.db.verify_token(token).await
    }

    async fn get_cooperative_hosts(&self) -> Result<Option<Vec<CdsHosts>>, TreatyDbError> {
        self.db.get_cooperative_hosts().await
    }

    async fn save_token(
        &self,
        login: &str,
        token: &str,
        expiration: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), TreatyDbError> {
        self.db.save_token(login, token, expiration).await
    }

    async fn auth_for_token(
        &self,
        login: &str,
        pw: &str,
        timeout_in_minutes: Option<u32>,
    ) -> Result<TokenReply, TreatyDbError> {
        self.db.auth_for_token(login, pw, timeout_in_minutes).await
    }

    async fn login_has_token(&self, login: &str) -> Result<bool, TreatyDbError> {
        self.db.login_has_token(login).await
    }

    async fn revoke_token(&self, token: &str) -> Result<bool, TreatyDbError> {
        self.db.revoke_token(token).await
    }

    async fn delete_expired_tokens(&self) -> Result<(), TreatyDbError> {
        self.db.delete_expired_tokens().await
    }

    async fn get_last_log_entries(
        &self,
        number_of_entries: u32,
    ) -> Result<Vec<LogEntry>, TreatyDbError> {
        self.db.get_last_log_entries(number_of_entries).await
    }

    async fn create_token_for_login(
        &self,
        login: &str,
        timeout_in_minutes: Option<u32>,
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), TreatyDbError> {
        self.db
            .create_token_for_login(login, timeout_in_minutes)
            .await
    }

    async fn accept_pending_action_at_participant(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.db
            .accept_pending_action_at_participant(db_name, table_name, row_id)
            .await
    }

    async fn get_pending_actions(
        &self,
        db_name: &str,
        table_name: &str,
        action: &str,
    ) -> Result<Option<Vec<PendingStatement>>, TreatyDbError> {
        self.db
            .get_pending_actions(db_name, table_name, action)
            .await
    }

    async fn get_data_hash_at_host(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Option<u64>, TreatyDbError> {
        self.db
            .get_data_hash_at_host(db_name, table_name, row_id)
            .await
    }

    async fn get_data_hash_at_participant(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Option<u64>, TreatyDbError> {
        self.db
            .get_data_hash_at_participant(db_name, table_name, row_id)
            .await
    }

    async fn read_row_id_from_part_db(
        &self,
        db_name: &str,
        table_name: &str,
        where_clause: &str,
    ) -> Result<u32, TreatyDbError> {
        self.db
            .read_row_id_from_part_db(db_name, table_name, where_clause)
            .await
    }

    async fn remove_remote_row_reference_from_host(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<bool, TreatyDbError> {
        self.db
            .remove_remote_row_reference_from_host(db_name, table_name, row_id)
            .await
    }

    async fn get_cds_host_for_part_db(
        &self,
        db_name: &str,
    ) -> Result<Option<CdsHosts>, TreatyDbError> {
        self.db.get_cds_host_for_part_db(db_name).await
    }

    async fn get_treaty_db_type(&self, db_name: &str) -> Result<TreatyDatabaseType, TreatyDbError> {
        self.db.get_treaty_db_type(db_name).await
    }

    async fn db_type(&self) -> DatabaseType {
        self.db.db_type().await
    }

    async fn get_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<UpdatesToHostBehavior, TreatyDbError> {
        self.db
            .get_updates_to_host_behavior(db_name, table_name)
            .await
    }

    async fn get_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<DeletesToHostBehavior, TreatyDbError> {
        self.db
            .get_deletes_to_host_behavior(db_name, table_name)
            .await
    }

    async fn get_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<DeletesFromHostBehavior, TreatyDbError> {
        self.db
            .get_deletes_from_host_behavior(db_name, table_name)
            .await
    }

    async fn get_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<UpdatesFromHostBehavior, TreatyDbError> {
        self.db
            .get_updates_from_host_behavior(db_name, table_name)
            .await
    }

    async fn change_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        self.db
            .change_updates_from_host_behavior(db_name, table_name, behavior)
            .await
    }

    async fn change_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        self.db
            .change_deletes_from_host_behavior(db_name, table_name, behavior)
            .await
    }

    async fn change_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        self.db
            .change_updates_to_host_behavior(db_name, table_name, behavior)
            .await
    }

    async fn change_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        self.db
            .change_deletes_to_host_behavior(db_name, table_name, behavior)
            .await
    }

    async fn get_row_from_partial_database(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Row, TreatyDbError> {
        self.db
            .get_row_from_partial_database(db_name, table_name, row_id)
            .await
    }

    async fn change_host_status_by_id(
        &self,
        host_id: &str,
        status: u32,
    ) -> Result<bool, TreatyDbError> {
        self.db.change_host_status_by_id(host_id, status).await
    }

    async fn change_host_status_by_name(
        &self,
        host_name: &str,
        status: u32,
    ) -> Result<bool, TreatyDbError> {
        self.db.change_host_status_by_name(host_name, status).await
    }

    async fn verify_host_by_id(
        &self,
        host_id: &str,
        token: Vec<u8>,
    ) -> Result<bool, TreatyDbError> {
        self.db.verify_host_by_id(host_id, token).await
    }

    async fn verify_host_by_name(
        &self,
        host_name: &str,
        token: Vec<u8>,
    ) -> Result<bool, TreatyDbError> {
        self.db.verify_host_by_name(host_name, token).await
    }

    async fn delete_metadata_in_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError> {
        self.db
            .delete_metadata_in_host_db(db_name, table_name, row_id, internal_participant_id)
            .await
    }

    async fn update_metadata_in_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError> {
        self.db
            .update_metadata_in_host_db(db_name, table_name, row_id, hash, internal_participant_id)
            .await
    }

    async fn insert_metadata_into_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError> {
        self.db
            .insert_metadata_into_host_db(
                db_name,
                table_name,
                row_id,
                hash,
                internal_participant_id,
            )
            .await
    }

    async fn delete_data_in_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host_id: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.db
            .delete_data_in_partial_db(part_db_name, table_name, cmd, where_clause, host_id)
            .await
    }

    async fn update_data_into_partial_db_queue(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host: &CdsHosts,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.db
            .update_data_into_partial_db_queue(part_db_name, table_name, cmd, where_clause, host)
            .await
    }

    async fn update_data_into_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        host_id: &str,
        where_clause: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.db
            .update_data_into_partial_db(part_db_name, table_name, cmd, host_id, where_clause)
            .await
    }

    async fn insert_data_into_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.db
            .insert_data_into_partial_db(part_db_name, table_name, cmd)
            .await
    }

    async fn update_participant_accepts_contract(
        &self,
        db_name: &str,
        participant: CoopDatabaseParticipant,
        participant_message: Participant,
        accepted_contract_id: &str,
    ) -> Result<bool, TreatyDbError> {
        self.db
            .update_participant_accepts_contract(
                db_name,
                participant,
                participant_message,
                accepted_contract_id,
            )
            .await
    }

    async fn create_partial_database_from_contract(
        &self,
        contract: &Contract,
    ) -> Result<bool, TreatyDbError> {
        self.db
            .create_partial_database_from_contract(contract)
            .await
    }

    async fn accept_pending_contract(&self, host_name: &str) -> Result<bool, TreatyDbError> {
        self.db.accept_pending_contract(host_name).await
    }

    async fn get_pending_contracts(&self) -> Result<Option<Vec<Contract>>, TreatyDbError> {
        self.db.get_pending_contracts().await
    }

    async fn get_accepted_contracts(&self) -> Result<Option<Vec<Contract>>, TreatyDbError> {
        self.db.get_accepted_contracts().await
    }

    async fn save_contract(
        &self,
        contract: Contract,
    ) -> Result<TreatySaveContractResult, TreatyDbError> {
        self.db.save_contract(contract).await
    }

    async fn get_table_id(&self, db_name: &str, table_name: &str) -> Result<String, TreatyDbError> {
        self.db.get_table_id(db_name, table_name).await
    }

    async fn create_table_in_partial_database(
        &self,
        db_name: &str,
        table_name: &str,
        schema: Vec<ColumnSchema>,
    ) -> Result<bool, TreatyDbError> {
        self.db
            .create_table_in_partial_database(db_name, table_name, schema)
            .await
    }

    async fn get_db_id(&self, db_name: &str) -> Result<String, TreatyDbError> {
        self.db.get_db_id(db_name).await
    }

    async fn create_partial_database(&self, db_name: &str) -> Result<bool, TreatyDbError> {
        self.db.create_partial_database(db_name).await
    }

    async fn has_role_name(&self, role_name: &str) -> Result<bool, TreatyDbError> {
        self.db.has_role_name(role_name).await
    }

    async fn add_login_to_role(&self, login: &str, role_name: &str) -> Result<bool, TreatyDbError> {
        self.db.add_login_to_role(login, role_name).await
    }

    async fn login_is_in_role(&self, login: &str, role_name: &str) -> Result<bool, TreatyDbError> {
        self.db.login_is_in_role(login, role_name).await
    }

    async fn create_login(&self, login: &str, pw: &str) -> Result<bool, TreatyDbError> {
        self.db.create_login(login, pw).await
    }

    async fn get_database_names(&self) -> Result<Option<Vec<String>>, TreatyDbError> {
        self.db.get_database_names().await
    }

    async fn has_login(&self, login: &str) -> Result<bool, TreatyDbError> {
        self.db.has_login(login).await
    }

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
    ) -> Result<bool, TreatyDbError> {
        self.db
            .add_participant(
                db_name, alias, ip4addr, db_port, info_port, http_addr, http_port, id,
            )
            .await
    }

    async fn get_database_schema(&self, db_name: &str) -> Result<DatabaseSchema, TreatyDbError> {
        self.db.get_database_schema(db_name).await
    }

    async fn get_participant_by_alias(
        &self,
        db_name: &str,
        participant_alias: &str,
    ) -> Result<Option<CoopDatabaseParticipant>, TreatyDbError> {
        self.db
            .get_participant_by_alias(db_name, participant_alias)
            .await
    }

    async fn get_participant_by_id(
        &self,
        db_name: &str,
        participant_id: &str,
    ) -> Result<Option<CoopDatabaseParticipant>, TreatyDbError> {
        self.db.get_participant_by_id(db_name, participant_id).await
    }

    async fn has_participant(
        &self,
        db_name: &str,
        participant_alias: &str,
    ) -> Result<bool, TreatyDbError> {
        self.db.has_participant(db_name, participant_alias).await
    }

    async fn get_active_contract(
        &self,
        db_name: &str,
    ) -> Result<CoopDatabaseContract, TreatyDbError> {
        self.db.get_active_contract(db_name).await
    }

    async fn get_logical_storage_policy(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<LogicalStoragePolicy, TreatyDbError> {
        self.db
            .get_logical_storage_policy(db_name, table_name)
            .await
    }

    async fn set_logical_storage_policy(
        &self,
        db_name: &str,
        table_name: &str,
        policy: LogicalStoragePolicy,
    ) -> Result<bool, TreatyDbError> {
        self.db
            .set_logical_storage_policy(db_name, table_name, policy)
            .await
    }

    async fn has_table(&self, db_name: &str, table_name: &str) -> Result<bool, TreatyDbError> {
        self.db.has_table(db_name, table_name).await
    }

    async fn execute_write_at_host(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<usize, TreatyDbError> {
        self.db.execute_write_at_host(db_name, cmd).await
    }

    async fn execute_write_at_partipant(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<usize, TreatyDbError> {
        self.db.execute_write_at_partipant(db_name, cmd).await
    }

    async fn execute_read_at_participant(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<Table, TreatyDbError> {
        self.db.execute_read_at_participant(db_name, cmd).await
    }

    async fn execute_read_at_host(&self, db_name: &str, cmd: &str) -> Result<Table, TreatyDbError> {
        self.db.execute_read_at_host(db_name, cmd).await
    }

    async fn has_cooperative_tables(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<bool, TreatyDbError> {
        self.db.has_cooperative_tables(db_name, cmd).await
    }

    async fn get_participants_for_table(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<Option<Vec<CoopDatabaseParticipantData>>, TreatyDbError> {
        self.db
            .get_participants_for_table(db_name, table_name)
            .await
    }

    async fn get_active_contract_proto(
        &self,
        db_name: &str,
    ) -> Result<Option<Contract>, TreatyDbError> {
        self.db.get_active_contract_proto(db_name).await
    }

    async fn get_participants_for_database(
        &self,
        db_name: &str,
    ) -> Result<Option<Vec<ParticipantStatus>>, TreatyDbError> {
        self.db.get_participants_for_database(db_name).await
    }

    async fn get_cooperative_tables(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<Option<Vec<String>>, TreatyDbError> {
        self.db.get_cooperative_tables(db_name, cmd).await
    }

    async fn create_database(&self, db_name: &str) -> Result<(), TreatyDbError> {
        self.db.create_database(db_name).await
    }

    async fn delete_database(&self, db_name: &str) -> Result<(), TreatyDbError> {
        self.db.delete_database(db_name).await
    }

    async fn delete_database_forcefully(&self, db_name: &str) -> Result<(), TreatyDbError> {
        self.db.delete_database_forcefully(db_name).await
    }

    async fn enable_coooperative_features(&self, db_name: &str) -> Result<bool, TreatyDbError> {
        self.db.enable_coooperative_features(db_name).await
    }

    async fn generate_contract(
        &self,
        db_name: &str,
        host_name: &str,
        desc: &str,
        remote_delete_behavior: RemoteDeleteBehavior,
    ) -> Result<bool, TreatyGenerateContractError> {
        self.db
            .generate_contract(db_name, host_name, desc, remote_delete_behavior)
            .await
    }

    async fn treaty_get_host_info(&self) -> Result<Option<HostInfo>, TreatyDbError> {
        self.db.treaty_get_host_info().await
    }

    async fn treaty_generate_host_info(&self, host_name: &str) -> Result<(), TreatyDbError> {
        self.db.treaty_generate_host_info(host_name).await
    }

    async fn if_treaty_host_info_exists(&self) -> Result<bool, TreatyDbError> {
        self.db.if_treaty_host_info_exists().await
    }

    async fn generate_and_get_host_info(&self, host_name: &str) -> Result<HostInfo, TreatyDbError> {
        self.db.generate_and_get_host_info(host_name).await
    }

    async fn configure_admin_hash(&self, login: &str, hash: Vec<u8>) -> Result<(), TreatyDbError> {
        self.db.configure_admin_hash(login, hash).await
    }

    async fn configure_admin(&self, login: &str, pw: &str) -> Result<(), TreatyDbError> {
        self.db.configure_admin(login, pw).await
    }

    async fn verify_login(&self, login: &str, pw: &str) -> Result<bool, TreatyDbError> {
        self.db.verify_login(login, pw).await
    }

    async fn configure_treaty_db(&self) -> Result<(), TreatyDbError> {
        self.db.configure_treaty_db().await
    }
}
