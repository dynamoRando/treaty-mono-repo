use std::path::Path;

use crate::{
    crypt, defaults,
    error::{TreatyDbError, TreatyGenerateContractError},
    models::{
        CdsHosts, CoopDatabaseContract, CoopDatabaseParticipant, CoopDatabaseParticipantData,
        HostInfo, LogEntry, PartialDataResult, Table, TreatySaveContractResult,
    },
    treaty_proto::{
        ColumnSchema, Contract, DatabaseSchema, Participant, ParticipantStatus, PendingStatement,
        Row, RowValue, TokenReply,
    },
};
use async_trait::async_trait;
use guid_create::GUID;
use rusqlite::{named_params, types::Type, Connection, Statement};
use stdext::function_name;
use tracing::{error, info, instrument, trace, warn};
use treaty_types::enums::*;

use self::{db::Db, part_db::PartDb, treaty::TreatyDb};

use super::dbi_actions::DbiActions;

mod db;
mod part_db;
mod treaty;

/// Takes a native treaty `ColumnType` and converts to the corresponding Sqlite type
#[derive(Debug, Clone)]
pub struct ColumnTypeSqlite {}

impl ColumnTypeSqlite {
    pub fn data_type_to_native_sqlite(column_type: &ColumnType) -> rusqlite::types::Type {
        match column_type {
            ColumnType::Unknown => todo!(),
            ColumnType::Int => todo!(),
            ColumnType::Bit => todo!(),
            ColumnType::Char => todo!(),
            ColumnType::DateTime => todo!(),
            ColumnType::Decimal => todo!(),
            ColumnType::Varchar => todo!(),
            ColumnType::Binary => todo!(),
            ColumnType::Varbinary => todo!(),
            ColumnType::Text => todo!(),
        }
    }
}

/// The `dbi` or Database Implemenation for Treaty in Sqlite. Represents the backing Sqlite databases that Treaty will work with.
#[derive(Debug, Clone)]
pub struct SqliteDbi {
    db_name: String,
    dir: String,
}

impl SqliteDbi {
    pub fn new(db_name: &str, dir: &str) -> Self {
        Self {
            db_name: db_name.to_string(),
            dir: dir.to_string(),
        }
    }

    fn treaty(&self) -> TreatyDb {
        TreatyDb::new(&self.db_name, &self.dir)
    }

    fn db(&self, db_name: &str) -> Db {
        Db::new(db_name, &self.dir, &self.treaty())
    }

    fn part_db(&self, db_name: &str) -> PartDb {
        PartDb::new(db_name, &self.dir, &self.treaty())
    }

    fn get_treaty_conn(&self) -> Result<Connection, TreatyDbError> {
        let db_path = Path::new(&self.dir).join(&self.db_name);
        let result = Connection::open(db_path);
        match result {
            Ok(c) => Ok(c),
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                Err(e.into())
            }
        }
    }
}

#[async_trait]
impl DbiActions for SqliteDbi {
    async fn verify_token(&self, token: &str) -> Result<bool, TreatyDbError> {
        self.treaty().verify_token(token)
    }

    async fn get_cooperative_hosts(&self) -> Result<Option<Vec<CdsHosts>>, TreatyDbError> {
        self.treaty().get_cooperative_hosts()
    }

    async fn save_token(
        &self,
        login: &str,
        token: &str,
        expiration: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), TreatyDbError> {
        self.treaty().save_token(login, token, expiration)
    }

    async fn auth_for_token(
        &self,
        login: &str,
        pw: &str,
        timeout_in_minutes: Option<u32>,
    ) -> Result<TokenReply, TreatyDbError> {
        self.treaty().auth_for_token(login, pw, timeout_in_minutes)
    }

    async fn login_has_token(&self, login: &str) -> Result<bool, TreatyDbError> {
        self.treaty().login_has_token(login)
    }

    async fn revoke_token(&self, token: &str) -> Result<bool, TreatyDbError> {
        self.treaty().revoke_token(token)
    }

    async fn delete_expired_tokens(&self) -> Result<(), TreatyDbError> {
        self.treaty().delete_expired_tokens()
    }

    async fn get_last_log_entries(
        &self,
        number_of_entries: u32,
    ) -> Result<Vec<LogEntry>, TreatyDbError> {
        todo!("get_last_log_entries")
    }

    async fn create_token_for_login(
        &self,
        login: &str,
        timeout_in_minutes: Option<u32>,
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), TreatyDbError> {
        self.treaty()
            .create_token_for_login(login, timeout_in_minutes)
    }

    async fn accept_pending_action_at_participant(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.part_db(db_name)
            .accept_pending_action_at_participant(db_name, table_name, row_id)
    }

    async fn get_pending_actions(
        &self,
        db_name: &str,
        table_name: &str,
        action: &str,
    ) -> Result<Option<Vec<PendingStatement>>, TreatyDbError> {
        self.part_db(db_name)
            .get_pending_actions(table_name, action)
    }

    async fn get_data_hash_at_host(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Option<u64>, TreatyDbError> {
        self.db(db_name).get_data_hash_at_host(table_name, row_id)
    }

    async fn get_data_hash_at_participant(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Option<u64>, TreatyDbError> {
        self.part_db(db_name)
            .get_data_hash_at_participant(table_name, row_id)
    }

    async fn read_row_id_from_part_db(
        &self,
        db_name: &str,
        table_name: &str,
        where_clause: &str,
    ) -> Result<u32, TreatyDbError> {
        self.part_db(db_name)
            .read_row_id_from_part_db(table_name, where_clause)
    }

    async fn remove_remote_row_reference_from_host(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<bool, TreatyDbError> {
        self.db(db_name)
            .remove_remote_row_reference_from_host(table_name, row_id)
    }

    async fn get_cds_host_for_part_db(
        &self,
        db_name: &str,
    ) -> Result<Option<CdsHosts>, TreatyDbError> {
        self.treaty().get_cds_host_for_part_db(db_name)
    }

    async fn get_treaty_db_type(&self, db_name: &str) -> Result<TreatyDatabaseType, TreatyDbError> {
        self.treaty().get_treaty_db_type(db_name)
    }

    async fn db_type(&self) -> DatabaseType {
        DatabaseType::Sqlite
    }

    async fn get_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<UpdatesToHostBehavior, TreatyDbError> {
        self.treaty()
            .get_updates_to_host_behavior(db_name, table_name)
    }

    async fn get_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<DeletesToHostBehavior, TreatyDbError> {
        self.treaty()
            .get_deletes_to_host_behavior(db_name, table_name)
    }

    async fn get_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<DeletesFromHostBehavior, TreatyDbError> {
        self.treaty()
            .get_deletes_from_host_behavior(db_name, table_name)
    }

    async fn get_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<UpdatesFromHostBehavior, TreatyDbError> {
        self.treaty()
            .get_updates_from_host_behavior(db_name, table_name)
    }

    async fn change_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        self.treaty()
            .change_updates_from_host_behavior(db_name, table_name, behavior)
    }

    async fn change_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        self.treaty()
            .change_deletes_from_host_behavior(db_name, table_name, behavior)
    }

    async fn change_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        self.treaty()
            .change_updates_to_host_behavior(db_name, table_name, behavior)
    }

    async fn change_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        self.treaty()
            .change_deletes_to_host_behavior(db_name, table_name, behavior)
    }

    async fn get_row_from_partial_database(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Row, TreatyDbError> {
        self.part_db(db_name)
            .get_row_from_partial_database(table_name, row_id)
    }

    async fn change_host_status_by_id(
        &self,
        host_id: &str,
        status: u32,
    ) -> Result<bool, TreatyDbError> {
        self.treaty().change_host_status_by_id(host_id, status)
    }

    async fn change_host_status_by_name(
        &self,
        host_name: &str,
        status: u32,
    ) -> Result<bool, TreatyDbError> {
        self.treaty().change_host_status_by_name(host_name, status)
    }

    async fn verify_host_by_id(
        &self,
        host_id: &str,
        token: Vec<u8>,
    ) -> Result<bool, TreatyDbError> {
        self.treaty().verify_host_by_id(host_id, token)
    }

    async fn verify_host_by_name(
        &self,
        host_name: &str,
        token: Vec<u8>,
    ) -> Result<bool, TreatyDbError> {
        self.treaty().verify_host_by_name(host_name, token)
    }

    async fn delete_metadata_in_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError> {
        self.db(db_name)
            .delete_metadata_in_host_db(table_name, row_id, internal_participant_id)
    }

    async fn update_metadata_in_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError> {
        self.db(db_name).update_metadata_in_host_db(
            table_name,
            row_id,
            hash,
            internal_participant_id,
        )
    }

    async fn insert_metadata_into_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError> {
        self.db(db_name).insert_metadata_into_host_db(
            table_name,
            row_id,
            hash,
            internal_participant_id,
        )
    }

    async fn delete_data_in_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host_id: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.part_db(part_db_name)
            .delete_data_in_partial_db(table_name, cmd, where_clause, host_id)
    }

    async fn update_data_into_partial_db_queue(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host: &CdsHosts,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.part_db(part_db_name)
            .update_data_into_partial_db_queue(table_name, cmd, where_clause, &host.host_id)
    }

    async fn update_data_into_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        host_id: &str,
        where_clause: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.part_db(part_db_name).update_data_into_partial_db(
            table_name,
            cmd,
            where_clause,
            host_id,
        )
    }

    async fn insert_data_into_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.part_db(part_db_name)
            .insert_data_into_partial_db(table_name, cmd)
    }

    async fn update_participant_accepts_contract(
        &self,
        db_name: &str,
        participant: CoopDatabaseParticipant,
        participant_message: Participant,
        accepted_contract_id: &str,
    ) -> Result<bool, TreatyDbError> {
        self.db(db_name).update_participant_accepts_contract(
            participant,
            participant_message,
            accepted_contract_id,
        )
    }

    async fn create_partial_database_from_contract(
        &self,
        contract: &Contract,
    ) -> Result<bool, TreatyDbError> {
        let db_name = contract.schema.as_ref().unwrap().database_name.clone();
        self.part_db(&db_name)
            .create_partial_database_from_contract(contract)
    }

    async fn accept_pending_contract(&self, host_name: &str) -> Result<bool, TreatyDbError> {
        self.treaty().accept_pending_contract(host_name)
    }

    async fn get_pending_contracts(&self) -> Result<Option<Vec<Contract>>, TreatyDbError> {
        self.treaty()
            .get_contracts_by_status(ContractStatus::Pending)
    }

    async fn get_accepted_contracts(&self) -> Result<Option<Vec<Contract>>, TreatyDbError> {
        self.treaty()
            .get_contracts_by_status(ContractStatus::Accepted)
    }

    async fn save_contract(
        &self,
        contract: Contract,
    ) -> Result<TreatySaveContractResult, TreatyDbError> {
        self.treaty().save_contract(contract)
    }

    async fn get_table_id(&self, db_name: &str, table_name: &str) -> Result<String, TreatyDbError> {
        todo!("get_table_id")
        // this was never implemented
    }

    async fn create_table_in_partial_database(
        &self,
        db_name: &str,
        table_name: &str,
        schema: Vec<ColumnSchema>,
    ) -> Result<bool, TreatyDbError> {
        todo!("create_table_in_partial_database")
        // this was never implemented
    }

    async fn get_db_id(&self, db_name: &str) -> Result<String, TreatyDbError> {
        todo!("get_db_id")
        // this was never implemented
    }

    async fn create_partial_database(&self, db_name: &str) -> Result<bool, TreatyDbError> {
        self.part_db(db_name).create_partial_database()
    }

    async fn has_role_name(&self, role_name: &str) -> Result<bool, TreatyDbError> {
        self.treaty().has_role_name(role_name)
    }

    async fn add_login_to_role(&self, login: &str, role_name: &str) -> Result<bool, TreatyDbError> {
        self.treaty().add_login_to_role(login, role_name)
    }

    async fn login_is_in_role(&self, login: &str, role_name: &str) -> Result<bool, TreatyDbError> {
        self.treaty().login_is_in_role(login, role_name)
    }

    async fn create_login(&self, login: &str, pw: &str) -> Result<bool, TreatyDbError> {
        self.treaty().create_login(login, pw)
    }

    async fn get_database_names(&self) -> Result<Option<Vec<String>>, TreatyDbError> {
        self.treaty().get_database_names()
    }

    async fn has_login(&self, login: &str) -> Result<bool, TreatyDbError> {
        self.treaty().has_login(login)
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
        self.db(db_name)
            .add_participant(alias, ip4addr, db_port, info_port, http_addr, http_port, id)
    }

    async fn get_database_schema(&self, db_name: &str) -> Result<DatabaseSchema, TreatyDbError> {
        self.db(db_name).get_db_schema()
    }

    async fn get_participant_by_alias(
        &self,
        db_name: &str,
        participant_alias: &str,
    ) -> Result<Option<CoopDatabaseParticipant>, TreatyDbError> {
        self.db(db_name).get_participant_by_alias(participant_alias)
    }

    async fn get_participant_by_id(
        &self,
        db_name: &str,
        participant_id: &str,
    ) -> Result<Option<CoopDatabaseParticipant>, TreatyDbError> {
        self.db(db_name).get_participant_by_id(participant_id)
    }

    async fn has_participant(
        &self,
        db_name: &str,
        participant_alias: &str,
    ) -> Result<bool, TreatyDbError> {
        self.db(db_name).has_participant(participant_alias)
    }

    async fn get_active_contract(
        &self,
        db_name: &str,
    ) -> Result<CoopDatabaseContract, TreatyDbError> {
        self.db(db_name).get_active_contract()
    }

    async fn get_logical_storage_policy(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<LogicalStoragePolicy, TreatyDbError> {
        self.db(db_name).get_logical_storage_policy(table_name)
    }

    async fn set_logical_storage_policy(
        &self,
        db_name: &str,
        table_name: &str,
        policy: LogicalStoragePolicy,
    ) -> Result<bool, TreatyDbError> {
        self.db(db_name)
            .set_logical_storage_policy(table_name, policy)
    }

    async fn has_table(&self, db_name: &str, table_name: &str) -> Result<bool, TreatyDbError> {
        // this needs to be able to check any database, so in this case
        // we don't bother with figuring out if it's a partial or user db
        let db_path = Path::new(&self.dir).join(db_name);
        trace!("[{}]: {db_path:?}", function_name!());
        let conn = Connection::open(db_path).unwrap();
        let mut cmd = String::from(
            "SELECT count(*) AS TABLECOUNT FROM sqlite_master WHERE type='table' AND name=':table_name'",
        );
        cmd = cmd.replace(":table_name", table_name);
        has_any_rows(cmd, &conn)
    }

    async fn execute_write_at_host(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<usize, TreatyDbError> {
        self.db(db_name).execute_write_at_host(cmd)
    }

    async fn execute_write_at_partipant(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<usize, TreatyDbError> {
        self.part_db(db_name).execute_write_at_participant(cmd)
    }

    async fn execute_read_at_participant(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<Table, TreatyDbError> {
        self.part_db(db_name).execute_read_at_participant(cmd)
    }

    async fn execute_read_at_host(&self, db_name: &str, cmd: &str) -> Result<Table, TreatyDbError> {
        self.db(db_name).execute_read_at_host(cmd)
    }

    async fn has_cooperative_tables(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<bool, TreatyDbError> {
        self.db(db_name).has_cooperative_tables(cmd)
    }

    async fn get_participants_for_table(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<Option<Vec<CoopDatabaseParticipantData>>, TreatyDbError> {
        self.db(db_name).get_participants_for_table(table_name)
    }

    #[allow(suspicious_double_ref_op)]
    async fn get_active_contract_proto(
        &self,
        db_name: &str,
    ) -> Result<Option<Contract>, TreatyDbError> {
        if !db_name.contains(".dbpart") {
            let active_contract = self.get_active_contract(db_name).await?;
            let db_schema = self.get_database_schema(db_name).await?;
            let opt_host_info = self.treaty_get_host_info().await?;
            match opt_host_info {
                Some(host_info) => Ok(Some(active_contract.convert_to_protobuf(
                    &host_info,
                    db_schema,
                    ContractStatus::Unknown,
                    "",
                    0,
                    0,
                ))),
                None => {
                    warn!("[{}]: No active contract has been set", function_name!());
                    Ok(None)
                }
            }
        } else {
            let opt_contracts = self.get_accepted_contracts().await?;

            match opt_contracts {
                Some(contracts) => {
                    trace!("[{}]: {contracts:?}", function_name!());

                    let name: String = if db_name.contains(".dbpart") {
                        db_name.replace(".dbpart", ".db")
                    } else {
                        db_name.to_string()
                    };

                    trace!("[{}]: {}", name, function_name!());

                    let contracts: Vec<&Contract> = contracts
                        .iter()
                        .enumerate()
                        .filter(|&(_, c)| c.schema.as_ref().unwrap().database_name == name)
                        .map(|(_, c)| c)
                        .collect();

                    let has_contracts = contracts.last().is_some();

                    if has_contracts {
                        return Ok(Some(contracts.clone().last().unwrap().clone().clone()));
                    } else {
                        Ok(None)
                    }
                }
                None => Ok(None),
            }
        }
    }

    async fn get_participants_for_database(
        &self,
        db_name: &str,
    ) -> Result<Option<Vec<ParticipantStatus>>, TreatyDbError> {
        self.db(db_name).get_participants_for_database()
    }

    async fn get_cooperative_tables(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<Option<Vec<String>>, TreatyDbError> {
        self.db(db_name).get_cooperative_tables(cmd)
    }

    async fn create_database(&self, db_name: &str) -> Result<(), TreatyDbError> {
        self.db(db_name).create_database()
    }

    async fn delete_database(&self, db_name: &str) -> Result<(), TreatyDbError> {
        self.db(db_name).delete_database()
    }

    async fn delete_database_forcefully(&self, db_name: &str) -> Result<(), TreatyDbError> {
        self.db(db_name).delete_database_forcefully()
    }

    async fn enable_coooperative_features(&self, db_name: &str) -> Result<bool, TreatyDbError> {
        self.db(db_name).enable_coooperative_features()
    }

    async fn generate_contract(
        &self,
        db_name: &str,
        host_name: &str,
        desc: &str,
        remote_delete_behavior: RemoteDeleteBehavior,
    ) -> Result<bool, TreatyGenerateContractError> {
        self.generate_and_get_host_info(host_name).await?;
        self.db(db_name)
            .generate_contract(desc, remote_delete_behavior)
    }

    async fn treaty_get_host_info(&self) -> Result<Option<HostInfo>, TreatyDbError> {
        self.treaty().treaty_get_host_info()
    }

    async fn treaty_generate_host_info(&self, host_name: &str) -> Result<(), TreatyDbError> {
        trace!("[{}]: host_name: {host_name:?}", function_name!());

        let conn = self.get_treaty_conn()?;
        let token_gen = GUID::rand();
        let token = crypt::hash(&token_gen.to_string());

        let cmd = "SELECT COUNT(*) HOSTS FROM CDS_HOST_INFO".to_string();
        let has_rows = has_any_rows(cmd, &conn)?;

        if has_rows {
            let cmd = String::from(
                "
                UPDATE CDS_HOST_INFO
                SET 
                    HOST_NAME = :name,
                    TOKEN = :token
                ;",
            );
            let mut statement = get_statement(&cmd, &conn)?;
            let total_rows =
                statement.execute(named_params! {":name" : host_name, ":token" : token.0 })?;

            if total_rows == 0 {
                error!(
                    "[{}]: Host info was not updated; this should not happen.",
                    function_name!()
                );
            }
        } else {
            let id = GUID::rand();
            let cmd = String::from(
                "
                INSERT INTO CDS_HOST_INFO
                (
                    HOST_ID,
                    HOST_NAME,
                    TOKEN
                )
                VALUES
                (
                    :id,
                    :name,
                    :token
                );",
            );
            let mut statement = get_statement(&cmd, &conn)?;
            statement.execute(
                named_params! {":id" : id.to_string(), ":name" : host_name, ":token" : token.0 },
            )?;
        }

        Ok(())
    }

    async fn if_treaty_host_info_exists(&self) -> Result<bool, TreatyDbError> {
        let cmd = String::from("SELECT COUNT(*) TOTALCOUNT FROM CDS_HOST_INFO");
        has_any_rows(cmd, &self.get_treaty_conn()?)
    }

    async fn generate_and_get_host_info(&self, host_name: &str) -> Result<HostInfo, TreatyDbError> {
        self.treaty_generate_host_info(host_name).await?;
        Ok(self.treaty_get_host_info().await.unwrap().unwrap())
    }

    async fn configure_admin_hash(&self, login: &str, hash: Vec<u8>) -> Result<(), TreatyDbError> {
        self.treaty().configure_admin_hash(login, hash)
    }

    async fn configure_admin(&self, login: &str, pw: &str) -> Result<(), TreatyDbError> {
        self.treaty().configure_admin(login, pw)
    }

    async fn verify_login(&self, login: &str, pw: &str) -> Result<bool, TreatyDbError> {
        self.treaty().verify_login(login, pw)
    }

    async fn configure_treaty_db(&self) -> Result<(), TreatyDbError> {
        self.treaty().configure_treaty_db()
    }
}

fn get_statement<'c>(cmd: &str, conn: &'c Connection) -> Result<Statement<'c>, TreatyDbError> {
    trace!("[{}]: {cmd:?} {conn:?}", function_name!());
    let result = conn.prepare(cmd);
    match result {
        Ok(statement) => Ok(statement),
        Err(e) => {
            error!("[{}]: {e:?}", function_name!());
            Err(e.into())
        }
    }
}

fn get_scalar_as_string(cmd: String, conn: &Connection) -> Result<String, TreatyDbError> {
    let mut value = String::from("");
    let mut statement = conn.prepare(&cmd).unwrap();
    let rows = statement.query_map([], |row| row.get(0))?;

    for item in rows {
        value = item.unwrap();
    }

    drop(statement);

    Ok(value)
}

fn get_scalar_as_u32(cmd: String, conn: &Connection) -> Result<u32, TreatyDbError> {
    trace!("[{}]: {cmd:?} {conn:?}", function_name!());
    let mut value: u32 = 0;
    let mut statement = get_statement(&cmd, conn)?;

    let rows = statement.query_map([], |row| row.get(0))?;

    for item in rows {
        trace!("[{}]: {item:?}", function_name!());
        value = item.unwrap_or_default();
    }

    drop(statement);

    Ok(value)
}

fn total_count(cmd: String, conn: &Connection) -> Result<u32, TreatyDbError> {
    get_scalar_as_u32(cmd, conn)
}

fn has_any_rows(cmd: String, conn: &Connection) -> Result<bool, TreatyDbError> {
    let result = total_count(cmd, conn)?;
    Ok(result > 0)
}

fn execute_write(conn: &Connection, cmd: &str) -> Result<usize, TreatyDbError> {
    trace!("[{}]: {cmd:?} {conn:?}", function_name!());
    let result = conn.execute(cmd, []);
    match result {
        Ok(rows) => Ok(rows),
        Err(e) => {
            error!("[{}]: {e:?}", function_name!());
            Err(e.into())
        }
    }
}

fn check_database_name_for_contract_format(
    db_name: &str,
    conn: &Connection,
) -> Result<String, TreatyDbError> {
    let mut db_name = db_name.to_string();

    let mut cmd =
        String::from("SELECT COUNT(*) FROM CDS_CONTRACTS_TABLES WHERE DATABASE_NAME = ':db_name'");
    cmd = cmd.replace(":db_name", &db_name);
    if !has_any_rows(cmd, conn)? {
        let message = format!(
            "{}{}",
            "WARNING: check_database_name_for_contract_format no database named: ", db_name
        );
        warn!("[{}]: {}", function_name!(), message);

        if db_name.contains(".dbpart") {
            let message = "check_database_name_for_contract_format: renaming database name to contract version of database";
            info!("[{}]: {}", function_name!(), message);
            db_name = db_name.replace(".dbpart", ".db");
            let message = format!("New database name is: {db_name}");
            info!("[{}]: {}", function_name!(), message);
        }
    }

    Ok(db_name)
}

fn get_table_col_names(table_name: &str, conn: &Connection) -> Result<Vec<String>, TreatyDbError> {
    let mut result: Vec<String> = Vec::new();

    let mut cmd = String::from("select NAME from pragma_table_info(\":table_name\") as tblInfo;");
    cmd = cmd.replace(":table_name", table_name);

    let row_to_string = |column_name: String| -> rusqlite::Result<String> { Ok(column_name) };

    let mut statement = get_statement(&cmd, conn)?;

    let names = statement.query_and_then([], |row| row_to_string(row.get(0).unwrap()))?;

    for name in names {
        result.push(name.unwrap());
    }

    drop(statement);

    Ok(result)
}

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

/// Returns a table describing the schema of the table
/// # Columns:
/// 1. columnId
/// 2. name
/// 3. type
/// 4. NotNull
/// 5. defaultValue
/// 6. IsPK
fn get_schema_of_table(
    table_name: String,
    conn: &Connection,
) -> core::result::Result<Table, TreatyDbError> {
    let mut cmd = String::from("PRAGMA table_info(\":table_name\")");
    cmd = cmd.replace(":table_name", &table_name);
    execute_read(&cmd, conn)
}

fn execute_read(cmd: &str, conn: &Connection) -> Result<Table, TreatyDbError> {
    use crate::models::{Column, Data, Row, Value};

    let mut statement = get_statement(cmd, conn)?;
    let total_columns = statement.column_count();
    let cols = statement.columns();
    let mut table = Table::new();
    table.set_database_type(DatabaseType::Sqlite);

    trace!("[{}]: {:?}", function_name!(), cmd);

    for col in cols {
        let col_idx = statement.column_index(col.name())?;

        trace!("[{}]: {col:?}", function_name!());
        let mut data_type = String::from("");

        let col_type = col.decl_type();
        if let Some(dt) = col_type {
            data_type = dt.to_string();
        };

        let c = Column {
            name: col.name().to_string(),
            is_nullable: false,
            idx: col_idx,
            data_type,
            is_primary_key: false,
        };

        trace!("[{}]: adding col {}", function_name!(), c.name);

        table.add_column(c);
    }

    let mut rows = statement.query([])?;

    while let Some(row) = rows.next()? {
        let mut data_row = Row::new();

        for i in 0..total_columns {
            let dt = row.get_ref_unwrap(i).data_type();

            let string_value: String = match dt {
                Type::Blob => String::from(""),
                Type::Integer => row.get_ref_unwrap(i).as_i64().unwrap().to_string(),
                Type::Real => row.get_ref_unwrap(i).as_f64().unwrap().to_string(),
                Type::Text => row.get_ref_unwrap(i).as_str().unwrap().to_string(),
                _ => String::from(""),
            };

            let string_value = string_value;
            let col = table.get_column_by_index(i).unwrap();

            let data_item = Data {
                data_string: string_value,
                data_byte: Vec::new(),
            };

            let data_value = Value {
                data: Some(data_item),
                col,
            };

            data_row.add_value(data_value);
        }

        table.add_row(data_row);
    }

    trace!("[{}]: Table Result: {table:?}", function_name!());

    Ok(table)
}

fn has_table(table_name: &str, conn: &Connection) -> Result<bool, TreatyDbError> {
    let mut cmd = String::from(
        "SELECT count(*) AS TABLECOUNT FROM sqlite_master WHERE type='table' AND name=':table_name'",
    );
    cmd = cmd.replace(":table_name", table_name);
    has_any_rows(cmd, conn)
}

fn get_scalar_as_u64(cmd: String, conn: &Connection) -> Result<Option<u64>, TreatyDbError> {
    trace!("[{}]: {cmd:?} {conn:?}", function_name!());

    let mut statement = conn.prepare(&cmd).unwrap();
    let mut returned_arrays: Vec<Vec<u8>> = Vec::new();

    let row_to_token = |data: Vec<u8>| -> rusqlite::Result<Vec<u8>> { Ok(data) };

    let tokens = statement
        .query_and_then([], |row| row_to_token(row.get(0).unwrap()))
        .unwrap();

    for t in tokens {
        returned_arrays.push(t.unwrap());
    }

    if !returned_arrays.is_empty() {
        let array = returned_arrays.first().unwrap().clone();
        let value: u64 = u64::from_ne_bytes(vec_to_array(array));
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

fn execute_read_on_connection_for_row(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    cmd: String,
    conn: &Connection,
) -> Result<Row, TreatyDbError> {
    let mut statement = conn.prepare(&cmd).unwrap();
    let total_columns = statement.column_count();
    let cols = statement.columns();

    let mut values: Vec<RowValue> = Vec::new();
    let mut columns: Vec<ColumnSchema> = Vec::new();

    for col in cols {
        let col_idx = statement.column_index(col.name()).unwrap();
        let empty_string = String::from("");
        let col_type = match col.decl_type() {
            Some(c) => c,
            None => &empty_string,
        };

        let c = ColumnSchema {
            column_name: col.name().to_string(),
            column_type: ColumnType::to_u32(ColumnType::try_parse(col_type).unwrap()),
            column_length: 0,
            is_nullable: false,
            ordinal: col_idx as u32,
            table_id: String::from(""),
            column_id: col_idx.to_string(),
            is_primary_key: false,
        };
        columns.push(c);
    }

    trace!("[{}] {statement:?}", function_name!());

    let mut rows = statement.query([])?;

    while let Some(row) = rows.next()? {
        for i in 0..total_columns {
            let dt = row.get_ref_unwrap(i).data_type();
            let string_value: String = match dt {
                Type::Blob => String::from(""),
                Type::Integer => row.get_ref_unwrap(i).as_i64().unwrap().to_string(),
                Type::Real => row.get_ref_unwrap(i).as_f64().unwrap().to_string(),
                Type::Text => row.get_ref_unwrap(i).as_str().unwrap().to_string(),
                _ => String::from(""),
            };

            let mut row_value = RowValue {
                column: None,
                is_null_value: false,
                value: Vec::new(),
                string_value: String::from(""),
            };

            let column = columns
                .iter()
                .enumerate()
                .filter(|&(_, c)| c.ordinal == i as u32)
                .map(|(_, c)| c);

            let col = column.last().unwrap().clone();
            row_value.column = Some(col);
            row_value.value = string_value.as_bytes().to_vec();
            row_value.string_value = string_value.clone();
            values.push(row_value);
        }
    }

    let mut cmd = String::from("SELECT HASH FROM :table_name WHERE ROW_ID = :rid");
    let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);
    cmd = cmd.replace(":table_name", &metadata_table_name);
    cmd = cmd.replace(":rid", &row_id.to_string());

    let mut statement = conn.prepare(&cmd).unwrap();

    trace!("[{}]: {statement:?}", function_name!());

    let row_to_hash = |hash: Vec<u8>| -> rusqlite::Result<Vec<u8>> { Ok(hash) };

    let mut hashes = statement
        .query_and_then([], |row| row_to_hash(row.get(0).unwrap()))
        .unwrap();

    let mut hash: Vec<u8> = Vec::new();

    if let Some(h) = hashes.next() {
        hash = h.unwrap()
    }

    /*
    for h in hashes {
        hash = h.unwrap();
        break;
    }
    */

    let result = Row {
        database_name: db_name.to_string(),
        table_name: table_name.to_string(),
        row_id,
        values,
        is_remoteable: true,
        remote_metadata: None,
        hash,
    };

    trace!("[{}]: {result:?}", function_name!());

    Ok(result)
}

/// Checks to see if the database has enabled cooperative features
pub fn has_enable_coooperative_features(db_name: &str, dir: &str) -> Result<bool, TreatyDbError> {
    if !has_database(dir, db_name) {
        Err(TreatyDbError::DbNotFound(db_name.to_string()))
    } else if db_name.contains(".dbpart") {
        Ok(true)
    } else {
        let conn = get_db_conn(dir, db_name);
        Ok(has_table("COOP_REMOTES", &conn)?)
    }
}

#[instrument]
pub fn has_database(dir: &str, db_name: &str) -> bool {
    let mut db_exists_as_regular_db = false;
    let mut db_exists_as_partial_db = false;

    if !db_name.ends_with(".db") {
        let db = db_name.to_owned() + ".db";
        let path = Path::new(&dir).join(db);
        db_exists_as_regular_db = Path::exists(&path);
    }

    if !db_name.ends_with(".dbpart") {
        let db = db_name.to_owned() + ".dbpart";
        let path = Path::new(&dir).join(db);
        db_exists_as_partial_db = Path::exists(&path);

        if !db_exists_as_partial_db && db_name.ends_with(".db") {
            let mut db_part_name = db_name.replace(".db", "");
            db_part_name = db_part_name.replace(".dbpart", "");
            db_part_name = format!("{}{}", db_part_name, String::from(".dbpart"));
            let path = Path::new(&dir).join(db_part_name);
            db_exists_as_partial_db = Path::exists(&path);
        }
    }

    let path = Path::new(&dir).join(db_name);
    let db_exists: bool = Path::exists(&path);

    db_exists || db_exists_as_regular_db || db_exists_as_partial_db
}

fn get_db_conn(dir: &str, db_name: &str) -> Connection {
    let db_path = Path::new(&dir).join(db_name);
    trace!("[{}]: {db_path:?}", function_name!());
    Connection::open(db_path).unwrap()
}
