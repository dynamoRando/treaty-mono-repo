use crate::{
    db_interface::postgres::part_db::PARTIAL_SCHEMA_NAME,
    defaults,
    error::{TreatyDbError, TreatyGenerateContractError},
    models::{
        get_metadata_table_name, CoopDatabaseContract, CoopDatabaseParticipant,
        CoopDatabaseParticipantData, Table,
    },
    sql_text,
    treaty_proto::{ColumnSchema, DatabaseSchema, Participant, ParticipantStatus, TableSchema},
};
use chrono::TimeZone;
use chrono::{NaiveDateTime, Utc};
use guid_create::GUID;
use stdext::function_name;
use tokio_postgres::{
    types::{Kind, ToSql},
    Client, NoTls, Row,
};
use tracing::{error, trace, warn};
use treaty_types::enums::{
    ColumnType, ContractStatus, DatabaseType, LogicalStoragePolicy, RemoteDeleteBehavior,
    TreatyDatabaseType,
};

use super::PostgresDbi;

#[derive(Debug, Clone)]
pub struct TreatyDb {
    pub host: String,
    pub port: Option<u32>,
    pub un: String,
    pub pw: String,
    pub treaty_schema_name: String,
    pub db_name: String,
    pub use_treaty_schema: bool,
}

pub const COOPERATIVE_SCHEMA_NAME: &str = "coop";

impl From<PostgresDbi> for TreatyDb {
    fn from(value: PostgresDbi) -> Self {
        Self {
            host: value.host,
            port: value.port,
            un: value.un,
            pw: value.pw,
            treaty_schema_name: value.treaty_schema_name,
            db_name: value.db_name,
            use_treaty_schema: value.use_treaty_schema,
        }
    }
}

impl TreatyDb {
    pub fn new(
        host: &str,
        port: Option<u32>,
        un: &str,
        pw: &str,
        treaty_schema_name: &str,
        db_name: &str,
        use_treaty_schema: bool,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            un: un.to_string(),
            pw: pw.to_string(),
            treaty_schema_name: treaty_schema_name.to_string(),
            db_name: db_name.to_string(),
            use_treaty_schema,
        }
    }

    pub fn dbi(&self) -> PostgresDbi {
        PostgresDbi {
            host: self.host.clone(),
            port: self.port,
            un: self.un.clone(),
            pw: self.pw.clone(),
            treaty_schema_name: self.treaty_schema_name.clone(),
            db_name: self.db_name.clone(),
            use_treaty_schema: self.use_treaty_schema,
        }
    }

    pub async fn get_participants_for_table(
        &self,
        table_name: &str,
    ) -> Result<Option<Vec<CoopDatabaseParticipantData>>, TreatyDbError> {
        // note - we will need another table to track the remote row id
        // let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);

        let metadata_table_name = get_metadata_table_name(table_name);

        if !self.has_table(&metadata_table_name).await? {
            trace!(
                "[{}]: creating metadata table: {metadata_table_name:?}",
                function_name!()
            );
            //  need to create table
            let mut cmd = sql_text::Coop::text_create_metadata_table();
            cmd = cmd.replace(":table_name", &metadata_table_name);
            warn!("[{}]: Converting BLOB TO BYTEA", function_name!());
            cmd = cmd.replace("BLOB", "BYTEA");
            self.write_for_db(&cmd, &self.db_name).await?;
        }

        let mut result: Vec<CoopDatabaseParticipantData> = Vec::new();

        let mut cmd = String::from(
            "
            SELECT DISTINCT
                INTERNAL_PARTICIPANT_ID
            FROM
                :table_name
            ;",
        );
        cmd = cmd.replace(":table_name", &metadata_table_name);

        let mut participant_ids: Vec<String> = Vec::new();
        let mut db_participants: Vec<CoopDatabaseParticipant> = Vec::new();

        let rows = self.query_db(&cmd).await?;

        trace!(
            "[{}]: Total Rows: {}, Rows: {rows:#?}",
            function_name!(),
            rows.len()
        );

        for row in &rows {
            let pid: String = row.get(0);
            participant_ids.push(pid);
        }

        for pid in &participant_ids {
            if let Some(participant) = self.get_participant_by_internal_id(pid).await? {
                db_participants.push(participant);
            }
        }

        for p in &db_participants {
            cmd = String::from(
                "
                SELECT
                    ROW_ID,
                    HASH
                FROM
                    :table_name
                WHERE
                    INTERNAL_PARTICIPANT_ID = ':pid'
                ;",
            );
            cmd = cmd.replace(":table_name", &metadata_table_name);
            cmd = cmd.replace(":pid", &p.internal_id.to_string());

            let rows = self.query_db(&cmd).await?;

            let mut row_data_results: Vec<(u32, Vec<u8>)> = Vec::new();

            for row in &rows {
                let row_id: i32 = row.get(0);
                let hash: Vec<u8> = row.get(1);

                // hash.to_ne_bytes().to_vec();
                // let row_id = self::vec_to_array(row_id);
                // let row_id: u32 = u32::from_ne_bytes(row_id);

                row_data_results.push((row_id as u32, hash));
            }

            let participant_data = CoopDatabaseParticipantData {
                participant: p.clone(),
                db_name: self.db_name.clone(),
                table_name: table_name.to_string(),
                row_data: row_data_results,
            };

            result.push(participant_data);
        }

        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }

    async fn get_participant_by_internal_id(
        &self,
        internal_id: &str,
    ) -> Result<Option<CoopDatabaseParticipant>, TreatyDbError> {
        let mut results: Vec<CoopDatabaseParticipant> = Vec::new();

        let mut cmd = String::from(
            "
            SELECT 
                INTERNAL_PARTICIPANT_ID,
                ALIAS,
                IP4ADDRESS,
                IP6ADDRESS,
                DB_PORT,
                INFO_PORT,
                CONTRACT_STATUS,
                ACCEPTED_CONTRACT_VERSION_ID,
                TOKEN,
                PARTICIPANT_ID,
                HTTP_ADDR,
                HTTP_PORT
            FROM
                :schema.PARTICIPANT
            WHERE
                INTERNAL_PARTICIPANT_ID = ':pid'
            ;
            ",
        );
        cmd = cmd.replace(":pid", internal_id);
        cmd = cmd.replace(":schema", COOPERATIVE_SCHEMA_NAME);

        let rows = self.query_db(&cmd).await?;
        for row in rows {
            let internal_id: String = row.get(0);
            let alias: String = row.get(1);
            let ip4addr: String = row.get(2);
            let ip6addr: String = row.get(3);
            let db_port: i32 = row.get(4);
            let info_port: i32 = row.get(5);
            let contract_status: i32 = row.get(6);
            let accepted_contract_version: String = row.get(7);
            let token: Vec<u8> = row.get(8);
            let id: String = row.get(9);
            let http_addr: String = row.get(10);
            let http_port: i32 = row.get(11);

            let cdp = CoopDatabaseParticipant {
                internal_id: GUID::parse(&internal_id).unwrap(),
                alias,
                ip4addr,
                ip6addr,
                db_port: db_port as u32,
                info_port: info_port as u32,
                contract_status: ContractStatus::from_u32(contract_status as u32),
                accepted_contract_version: GUID::parse(&accepted_contract_version).unwrap(),
                token,
                id: GUID::parse(&id).unwrap(),
                http_addr,
                http_port: http_port as u16,
            };

            results.push(cdp);
        }

        if results.is_empty() {
            Ok(None)
        } else {
            Ok(Some(results.first().unwrap().clone()))
        }
    }

    pub async fn get_cooperative_tables(
        &self,
        cmd: &str,
    ) -> Result<Option<Vec<String>>, TreatyDbError> {
        use crate::query_parser::get_table_names;

        let mut cooperative_tables: Vec<String> = Vec::new();

        let tables = get_table_names(cmd, DatabaseType::Sqlite);

        for table in &tables {
            let result = self.get_logical_storage_policy(&table.to_string()).await;

            if let Ok(policy) = result {
                match policy {
                    LogicalStoragePolicy::Mirror => {
                        cooperative_tables.push(table.clone());
                    }
                    LogicalStoragePolicy::ParticipantOwned => {
                        cooperative_tables.push(table.clone());
                    }
                    LogicalStoragePolicy::Shared => {
                        cooperative_tables.push(table.clone());
                    }
                    _ => {}
                }
            } else {
                break;
            }
        }

        if cooperative_tables.is_empty() {
            Ok(None)
        } else {
            Ok(Some(cooperative_tables))
        }
    }

    pub async fn has_cooperative_tables(&self, cmd: &str) -> Result<bool, TreatyDbError> {
        use crate::query_parser::get_table_names;

        let mut has_cooperative_tables = false;

        let tables = get_table_names(cmd, DatabaseType::Sqlite);

        for table in tables {
            let result = self.get_logical_storage_policy(&table).await;

            if let Ok(policy) = result {
                match policy {
                    LogicalStoragePolicy::Mirror => {
                        has_cooperative_tables = true;
                        break;
                    }
                    LogicalStoragePolicy::ParticipantOwned => {
                        has_cooperative_tables = true;
                        break;
                    }
                    LogicalStoragePolicy::Shared => {
                        has_cooperative_tables = true;
                        break;
                    }
                    _ => {}
                }
            } else {
                break;
            }
        }

        Ok(has_cooperative_tables)
    }

    pub async fn insert_metadata_into_host_db(
        &self,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError> {
        trace!("[{}]: inserting metadata at host", function_name!());

        let metadata_table_name = get_metadata_table_name(table_name);

        if !self.has_table(&metadata_table_name).await? {
            //  need to create table
            trace!(
                "[{}]: creating metadata table: {metadata_table_name:?}",
                function_name!()
            );
            let mut cmd = sql_text::Coop::text_create_metadata_table();
            cmd = cmd.replace(":table_name", &metadata_table_name);
            warn!("[{}]: Converting BLOB TO BYTEA", function_name!());
            cmd = cmd.replace("BLOB", "BYTEA");
            self.write_for_db(&cmd, &self.db_name).await?;
        }

        let mut cmd = sql_text::Coop::text_insert_row_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name);
        cmd = cmd.replace(":row", &row_id.to_string());
        cmd = cmd.replace(":hash", "$1::BYTEA");
        cmd = cmd.replace(":pid", "':pid'");
        cmd = cmd.replace(":pid", internal_participant_id);

        trace!("[{}]: SQL: {}", function_name!(), cmd);
        let client = self.get_client_for_db(&self.db_name).await?;
        warn!("[{}]: Converting u64 to Vec<u8>", function_name!());
        let hash_value_vec = hash.to_ne_bytes().to_vec();

        trace!("[{}]: hash check u64: {hash:?}", function_name!());
        trace!("[{}]: hash check is: {hash_value_vec:?}", function_name!());

        let result = client.execute(&cmd, &[&hash_value_vec]).await?;

        Ok(result > 0)
    }

    pub async fn update_participant_accepts_contract(
        &self,
        participant: CoopDatabaseParticipant,
        participant_message: Participant,
        accepted_contract_version_id: &str,
    ) -> Result<bool, TreatyDbError> {
        let internal_id = participant.internal_id;
        let participant_id = participant_message.participant_guid.clone();
        let token = participant_message.token;

        let sql = r#"
    UPDATE 
        :schema.PARTICIPANT
    SET 
        CONTRACT_STATUS = 3, 
        ACCEPTED_CONTRACT_VERSION_ID = ':cid',
        PARTICIPANT_ID = ':pid',
        TOKEN = $1::BYTEA
    WHERE 
        INTERNAL_PARTICIPANT_ID = ':iid';"#;
        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
        let sql = sql.replace(":iid", &internal_id.to_string());
        let sql = sql.replace(":cid", accepted_contract_version_id);
        let sql = sql.replace(":pid", &participant_id);

        let client = self.get_client_for_db(&self.db_name).await?;

        trace!("[{}]: SQL: {}", function_name!(), sql);

        let result = client.execute(&sql, &[&token]).await?;

        Ok(result > 0)
    }

    pub async fn enable_coooperative_features(&self) -> Result<bool, TreatyDbError> {
        let sql = "CREATE SCHEMA IF NOT EXISTS :schema ;";
        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
        self.write_for_db(&sql, &self.db_name).await?;

        self.create_remotes_table().await?;
        self.create_participant_table().await?;
        self.create_coop_contracts_table().await?;
        self.create_data_host_tables().await?;
        self.populate_data_host_tables().await?;
        trace!("[{}]: enable_coooperative_features done", function_name!());

        Ok(true)
    }

    async fn create_remotes_table(&self) -> Result<(), TreatyDbError> {
        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.REMOTES 
        (
            TABLENAME VARCHAR(255) NOT NULL,
            LOGICAL_STORAGE_POLICY INT NOT NULL
        );"#;

        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
        self.write_for_db(&sql, &self.db_name).await?;

        Ok(())
    }

    async fn create_participant_table(&self) -> Result<(), TreatyDbError> {
        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.PARTICIPANT 
        (
            INTERNAL_PARTICIPANT_ID CHAR(36) NOT NULL,
            ALIAS VARCHAR(50) NOT NULL,
            IP4ADDRESS VARCHAR(25),
            IP6ADDRESS VARCHAR(25),
            DB_PORT INT,
            INFO_PORT INT,
            CONTRACT_STATUS INT,
            ACCEPTED_CONTRACT_VERSION_ID CHAR(36),
            TOKEN BYTEA NOT NULL,
            PARTICIPANT_ID CHAR(36),
            HTTP_ADDR VARCHAR(50),
            HTTP_PORT INT
        );"#;

        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
        self.write_for_db(&sql, &self.db_name).await?;

        Ok(())
    }

    async fn create_coop_contracts_table(&self) -> Result<(), TreatyDbError> {
        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.DATABASE_CONTRACT 
        (
            CONTRACT_ID CHAR(36) NOT NULL,
            GENERATED_DATE_UTC TIMESTAMP NOT NULL,
            DESCRIPTION VARCHAR(255),
            RETIRED_DATE_UTC TIMESTAMP,
            VERSION_ID CHAR(36) NOT NULL,
            REMOTE_DELETE_BEHAVIOR INT
        );"#;

        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
        self.write_for_db(&sql, &self.db_name).await?;

        Ok(())
    }

    async fn create_data_host_tables(&self) -> Result<(), TreatyDbError> {
        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.DATA_HOST 
        (
            DATABASE_ID CHAR(36) NOT NULL,
            DATABASE_NAME VARCHAR(500) NOT NULL
         );"#;

        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
        self.write_for_db(&sql, &self.db_name).await?;

        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.DATA_TABLES
        (
            TABLE_ID CHAR(36) NOT NULL,
            TABLE_NAME VARCHAR(500) NOT NULL
        );"#;

        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
        self.write_for_db(&sql, &self.db_name).await?;

        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.DATA_HOST_TABLE_COLUMNS
        (
            TABLE_ID CHAR(36) NOT NULL,
            COLUMN_ID CHAR(36) NOT NULL,
            COLUMN_NAME VARCHAR(500) NOT NULL
        );"#;

        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
        self.write_for_db(&sql, &self.db_name).await?;

        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.REMOTES
        (
            TABLENAME VARCHAR(255) NOT NULL,
            LOGICAL_STORAGE_POLICY INT NOT NULL
        );"#;

        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
        self.write_for_db(&sql, &self.db_name).await?;

        Ok(())
    }

    pub async fn get_active_contract(&self) -> Result<CoopDatabaseContract, TreatyDbError> {
        let sql = r#"
        SELECT 
                CONTRACT_ID,
                GENERATED_DATE_UTC,
                DESCRIPTION,
                VERSION_ID,
                REMOTE_DELETE_BEHAVIOR 
            FROM 
                :schema.DATABASE_CONTRACT 
            WHERE 
                RETIRED_DATE_UTC IS NULL
            ;
            "#;
        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);

        let rows = self.query_db(&sql).await?;

        let mut results: Vec<CoopDatabaseContract> = Vec::new();
        for row in rows {
            let contract_id: String = row.get(0);
            let generated_date: NaiveDateTime = row.get(1);
            let version_id: String = row.get(3);
            let remote_delete_behavior: i32 = row.get(4);

            let contract = CoopDatabaseContract {
                contract_id: GUID::parse(&contract_id).unwrap(),
                generated_date: Utc::from_local_datetime(&Utc, &generated_date).unwrap(),
                description: row.get(2),
                retired_date: None,
                version_id: GUID::parse(&version_id).unwrap(),
                remote_delete_behavior: remote_delete_behavior as u32,
            };

            results.push(contract);
        }

        if results.is_empty() {
            warn!("There is no active contract!");
        }

        trace!("[{}]: contracts: {results:#?}", function_name!());

        return Ok(results.first().unwrap().clone());
    }

    async fn populate_data_host_tables(&self) -> Result<(), TreatyDbError> {
        self.populate_database_id().await?;
        let table_statues = self.get_remote_status_for_tables().await?;

        for status in table_statues {
            // for each table that we have a logical storage policy
            // we want to make sure that the contract tables (COOP_DATA_HOST_*)
            // have the latest correct schema for each table. Note that
            // we add tables even if the logical storage policy is NONE, because in treaty
            // we want to be transparent about all the tables in the database

            let table_name = &status.0;
            let table_id = GUID::rand();

            let sql = "SELECT COUNT(*) TABLECOUNT FROM :schema.DATA_TABLES WHERE TABLE_NAME = ':table_name';";
            let sql = sql.replace(":table_name", table_name);
            let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);

            trace!("[{}]: SQL: {}", function_name!(), sql);
            if !self.has_any_rows(&sql).await? {
                let sql = r#"
                INSERT INTO :schema.DATA_TABLES
                (
                    TABLE_ID,
                    TABLE_NAME
                )
                VALUES
                (
                    ':table_id',
                    ':table_name'
                );
                "#;
                let sql = sql.replace(":table_name", table_name);
                let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
                let sql = sql.replace(":table_id", &table_id.to_string());

                let client = self.get_client_for_db(&self.db_name).await?;
                trace!("[{}]: SQL: {}", function_name!(), sql);
                client.execute(&sql, &[]).await?;
            }

            // need to get schema and save it to the table
            let schema = self.get_schema_of_table(table_name, "public").await?;
            self.save_schema_to_data_host_tables(&table_id.to_string(), &schema)
                .await?;
        }

        Ok(())
    }

    /// Returns a Treaty Table representing the schema of the requested table
    /// in the following format: column_name, data_type, ordinal_position, is_nullable
    ///
    /// Note that this will be the Postgres native type
    pub async fn get_schema_of_table(
        &self,
        table_name: &str,
        table_schema: &str,
    ) -> Result<Table, TreatyDbError> {
        let sql = r#"
        SELECT 
            column_name, 
            data_type,
            ordinal_position,
            is_nullable
        FROM 
            information_schema.columns
        WHERE 
            table_name = ':table_name'
        AND
            table_schema = ':schema';
        "#;

        let sql = sql.replace(":table_name", &table_name.to_lowercase());
        let sql = sql.replace(":schema", table_schema);

        let client = self.get_client_for_db(&self.db_name).await?;
        self.read(&sql, &client).await
    }

    pub async fn generate_contract(
        &self,
        desc: &str,
        remote_delete_behavior: &RemoteDeleteBehavior,
    ) -> Result<bool, TreatyGenerateContractError> {
        let policies = self
            .get_logical_storage_policy_for_all_user_tables()
            .await?;

        let policies = self
            .get_logical_storage_policy_for_all_user_tables()
            .await?;

        // check to see if all user tables have a logical storage policy set
        // if any don't, return an error.
        if policies.iter().any(|p| p.1 == LogicalStoragePolicy::None) {
            let mut missing_policies = String::from("policy not set for ");

            for p in policies {
                if p.1 == LogicalStoragePolicy::None {
                    let message = format!("table {}, ", p.0);
                    missing_policies.push_str(&message);
                }
            }

            let error = TreatyGenerateContractError::NotAllTablesSet(missing_policies);
            warn!("[{}]: {:#?}", function_name!(), error);
            return Err(error);
        }

        trace!(
            "[{}]: all tables have policies: {policies:?}",
            function_name!()
        );

        let sql = "SELECT COUNT(*) TOTALCONTRACTS FROM :schema.DATABASE_CONTRACT";
        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);

        if !self.has_any_rows(&sql).await? {
            // this is the first contract
            trace!("[{}]: generate contract: first_contract", function_name!());
            let contract = CoopDatabaseContract {
                contract_id: GUID::rand(),
                generated_date: Utc::now(),
                description: desc.to_string(),
                retired_date: None,
                version_id: GUID::rand(),
                remote_delete_behavior: RemoteDeleteBehavior::to_u32(*remote_delete_behavior),
            };
            self.save_contract(&contract).await?;
        } else {
            // there are other contracts, we need to find the active one and retire it
            // then generate a new contract
            let contracts = self.get_all_database_contracts().await?;
            // trace!("generate contract: retire contracts");
            trace!(
                "generate contract: retire contracts count: {}",
                contracts.len()
            );
            for con in contracts {
                if !con.is_retired() {
                    trace!(
                        "generate contract: retire contract {}",
                        &con.contract_id.to_string()
                    );
                    self.retire_contract(&con.version_id).await?;
                    // trace!(
                    //     "generate contract: save retired contract {}",
                    //     &con.contract_id.to_string()
                    // );
                    // save_contract_at_connection(con, conn);
                }
            }

            trace!("generate contract: retired. create new contract");
            let new_contract = CoopDatabaseContract {
                contract_id: GUID::rand(),
                generated_date: Utc::now(),
                description: desc.to_string(),
                retired_date: None,
                version_id: GUID::rand(),
                remote_delete_behavior: RemoteDeleteBehavior::to_u32(*remote_delete_behavior),
            };
            self.save_contract(&new_contract).await?;
        }

        Ok(true)
    }

    async fn retire_contract(&self, version_id: &GUID) -> Result<(), TreatyDbError> {
        let mut cmd = String::from("UPDATE :schema.DATABASE_CONTRACT SET RETIRED_DATE_UTC = ':retire_date' WHERE VERSION_ID = ':vid'");
        cmd = cmd.replace(":retire_date", &Utc::now().to_string());
        cmd = cmd.replace(":vid", &version_id.to_string());
        cmd = cmd.replace(":schema", COOPERATIVE_SCHEMA_NAME);
        self.write_for_db(&cmd, &self.db_name).await?;

        Ok(())
    }

    async fn save_contract(&self, contract: &CoopDatabaseContract) -> Result<(), TreatyDbError> {
        trace!("[{}]: contract: {contract:#?}", function_name!());

        let sql = "SELECT COUNT(*) CONTRACTCOUNTS FROM :schema.DATABASE_CONTRACT WHERE VERSION_ID = ':vid';";
        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
        let sql = sql.replace(":vid", &contract.version_id.to_string());

        if self.has_any_rows(&sql).await? {
            // this is an update
            trace!("[{}]: Update an existing contract...", function_name!());

            if contract.is_retired() {
                // update the status to show retirement
                let sql = r#"
                UPDATE :schema.DATABASE_CONTRACT 
                SET
                    CONTRACT_ID = ':cid',
                    GENERATED_DATE_UTC = ':gen_date',
                    DESCRIPTION = ':desc',
                    RETIRED_DATE_UTC = ':ret_date',
                    REMOTE_DELETE_BEHAVIOR = ':remote_behavior'
                WHERE
                    VERSION_ID = ':vid'"
                ;
                "#;
                let mut sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
                sql = sql.replace(":cid", &contract.contract_id.to_string());
                sql = sql.replace(":gen_date", &contract.generated_date.to_string());
                sql = sql.replace(":desc", &contract.description);
                let ret = &contract.retired_date.unwrap().to_string();
                sql = sql.replace(":ret_date", ret);
                sql = sql.replace(":vid", &contract.version_id.to_string());
                sql = sql.replace(
                    ":remote_behavior",
                    &contract.remote_delete_behavior.to_string(),
                );

                self.write_for_db(&sql, &self.db_name).await?;
                Ok(())
            } else {
                // something else about the contract needs to be updated
                let mut sql = r#"
                UPDATE :schema.DATABASE_CONTRACT 
                SET 
                    CONTRACT_ID = ':cid',
                    GENERATED_DATE_UTC = ':gen_date',
                    DESCRIPTION = ':desc',
                    REMOTE_DELETE_BEHAVIOR = ':remote_behavior'
                WHERE
                    VERSION_ID = ':vid'"
                ;
                "#
                .to_string();
                sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
                sql = sql.replace(":cid", &contract.contract_id.to_string());
                sql = sql.replace(":gen_date", &contract.generated_date.to_string());
                sql = sql.replace(":desc", &contract.description);
                sql = sql.replace(":vid", &contract.version_id.to_string());
                sql = sql.replace(
                    ":remote_behavior",
                    &contract.remote_delete_behavior.to_string(),
                );

                self.write_for_db(&sql, &self.db_name).await?;
                Ok(())
            }
        } else {
            // this is an insert
            trace!("[{}]: Insert a new contract...", function_name!());
            if contract.is_retired() {
                let mut cmd = String::from(
                    "
                INSERT INTO :schema.DATABASE_CONTRACT
                (
                    CONTRACT_ID,
                    GENERATED_DATE_UTC,
                    DESCRIPTION,
                    RETIRED_DATE_UTC,
                    VERSION_ID,
                    REMOTE_DELETE_BEHAVIOR
                )
                VALUES
                (
                    ':cid',
                    ':gen_date',
                    ':desc',
                    ':ret_date',
                    ':vid',
                    ':remote_behavior'
                );
                ",
                );
                cmd = cmd.replace(":schema", COOPERATIVE_SCHEMA_NAME);
                cmd = cmd.replace(":cid", &contract.contract_id.to_string());
                cmd = cmd.replace(":gen_date", &contract.generated_date.to_string());
                cmd = cmd.replace(":desc", &contract.description);
                let ret = &contract.retired_date.unwrap().to_string();
                cmd = cmd.replace(":ret_date", ret);
                cmd = cmd.replace(":vid", &contract.version_id.to_string());
                cmd = cmd.replace(
                    ":remote_behavior",
                    &contract.remote_delete_behavior.to_string(),
                );

                trace!("[{}]: Insert retired contract...", function_name!());
                self.write_for_db(&cmd, &self.db_name).await?;
                Ok(())
            } else {
                let mut cmd = String::from(
                    "
                INSERT INTO :schema.DATABASE_CONTRACT
                (
                    CONTRACT_ID,
                    GENERATED_DATE_UTC,
                    DESCRIPTION,
                    VERSION_ID,
                    REMOTE_DELETE_BEHAVIOR
                )
                VALUES
                (
                    ':cid',
                    ':gen_date',
                    ':desc',
                    ':vid',
                    ':remote_behavior'
                );
                ",
                );

                cmd = cmd.replace(":schema", COOPERATIVE_SCHEMA_NAME);
                cmd = cmd.replace(":cid", &contract.contract_id.to_string());
                trace!(
                    "[{}]: contract generated date: {}",
                    function_name!(),
                    &contract.generated_date
                );
                cmd = cmd.replace(":gen_date", &contract.generated_date.to_string());
                cmd = cmd.replace(":desc", &contract.description);
                cmd = cmd.replace(":vid", &contract.version_id.to_string());
                cmd = cmd.replace(
                    ":remote_behavior",
                    &contract.remote_delete_behavior.to_string(),
                );
                trace!("[{}]: Writing new contract...", function_name!());
                self.write_for_db(&cmd, &self.db_name).await?;

                Ok(())
            }
        }
    }

    async fn get_all_database_contracts(&self) -> Result<Vec<CoopDatabaseContract>, TreatyDbError> {
        let mut result: Vec<CoopDatabaseContract> = Vec::new();

        let sql = String::from(
            "SELECT 
                CONTRACT_ID,
                GENERATED_DATE_UTC,
                DESCRIPTION,
                RETIRED_DATE_UTC,
                VERSION_ID,
                REMOTE_DELETE_BEHAVIOR
            FROM
                :schema.DATABASE_CONTRACT
        ;
        ",
        );

        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);

        let table = self
            .read(&sql, &self.get_client_for_db(&self.db_name).await?)
            .await?;

        for row in table.rows {
            for val in &row.vals {
                let mut cid = GUID::rand();
                let mut gen_date = Utc::now();
                let mut desc = String::from("");
                let mut is_retired = false;
                let mut ret_date = Utc::now();
                let mut vid = GUID::rand();
                let mut delete_behavior: u32 = 0;

                if val.col.name == "CONTRACT_ID" {
                    let vcid = val.data.as_ref().unwrap().data_string.clone();
                    let tcid = GUID::parse(&vcid).unwrap();
                    cid = tcid;
                }

                if val.col.name == "GENERATED_DATE_UTC" {
                    let vgen_date = val.data.as_ref().unwrap().data_string.clone();
                    // trace!("{}", vgen_date);
                    let tgen_date =
                        Utc::datetime_from_str(&Utc, &vgen_date, defaults::DATETIME_STRING_FORMAT);
                    gen_date = tgen_date.unwrap();
                }

                if val.col.name == "DESCRIPTION" {
                    let vdesc = val.data.as_ref().unwrap().data_string.clone();
                    desc = vdesc.clone();
                }

                if val.col.name == "RETIRED_DATE_UTC" {
                    if val.is_null() {
                        is_retired = false;
                    } else {
                        let vret_date = val.data.as_ref().unwrap().data_string.clone();
                        if !vret_date.is_empty() {
                            // trace!("{}", vret_date);
                            let tret_date = Utc::datetime_from_str(
                                &Utc,
                                &vret_date,
                                defaults::DATETIME_STRING_FORMAT,
                            );
                            ret_date = tret_date.unwrap();
                            is_retired = true;
                        } else {
                            is_retired = false;
                        }
                    }
                }

                if val.col.name == "VERSION_ID" {
                    let vvid = val.data.as_ref().unwrap().data_string.clone();
                    let tvid = GUID::parse(&vvid).unwrap();
                    vid = tvid;
                }

                if val.col.name == "REMOTE_DELETE_BEHAVIOR" {
                    let vbehavior = val.data.as_ref().unwrap().data_string.clone();
                    delete_behavior = vbehavior.parse().unwrap();
                }

                let item = CoopDatabaseContract {
                    contract_id: cid,
                    generated_date: gen_date,
                    description: desc,
                    retired_date: if is_retired { Some(ret_date) } else { None },
                    version_id: vid,
                    remote_delete_behavior: delete_behavior,
                };

                result.push(item);
            }
        }

        Ok(result)
    }

    async fn get_logical_storage_policy_for_all_user_tables(
        &self,
    ) -> Result<Vec<(String, LogicalStoragePolicy)>, TreatyDbError> {
        let mut result: Vec<(String, LogicalStoragePolicy)> = Vec::new();

        let table_names = self.get_all_user_table_names_in_db().await?;
        trace!("[{}]: table_names: {:#?}", function_name!(), table_names);

        for name in table_names {
            let policy = self.get_logical_storage_policy(&name).await?;
            let item = (name.clone(), policy);
            result.push(item);
        }

        Ok(result)
    }

    /// Returns all the table names in the 'public' schema
    async fn get_all_user_table_names_in_db(&self) -> Result<Vec<String>, TreatyDbError> {
        let mut result: Vec<String> = Vec::new();

        let sql = "SELECT table_name FROM information_schema.tables WHERE table_catalog = ':db_name' AND table_schema = 'public';";
        let sql = sql.replace(":db_name", &self.db_name);

        let rows = self.query_db(&sql).await?;

        for row in rows {
            let table_name: String = row.get(0);
            result.push(table_name);
        }

        Ok(result)
    }

    pub async fn remove_remote_row_reference_from_host(
        &self,
        table_name: &str,
        row_id: u32,
    ) -> Result<bool, TreatyDbError> {
        let metadata_table_name = get_metadata_table_name(table_name);
        trace!("[{}]: row_id: {row_id:?}", function_name!());
        trace!(
            "[{}]: metadata_table_name: {metadata_table_name:?}",
            function_name!()
        );

        let mut cmd = String::from(
            "DELETE FROM :table_name
             WHERE ROW_ID = :rid
        ;",
        );

        // trace!("[{}]: Re-writing ROWID TO __treaty_id", function_name!());
        // cmd = cmd.replace("ROW_ID", "__treaty_id");
        cmd = cmd.replace(":rid", &row_id.to_string());
        cmd = cmd.replace(":table_name", &metadata_table_name);

        trace!("[{}]: {cmd:?}", function_name!());

        let rows = self.write_with_count(&cmd).await?;

        trace!(
            "[{}]: total row_references_deleted: {rows}",
            function_name!()
        );

        Ok(rows > 0)
    }

    /// Returns the logical storage policy for the specified table. If the table does not exist in the database, it will
    /// return an error. If the table exist but does not have a logical storage policy defined for it, it will default
    /// to `LogicalStoragePolicy::None`
    pub async fn get_logical_storage_policy(
        &self,
        table_name: &str,
    ) -> Result<LogicalStoragePolicy, TreatyDbError> {
        trace!("[{}]: Looking for table: {table_name:?}", function_name!());
        if self.has_table(table_name).await? {
            trace!("[{}]: Has table...", function_name!());

            // why is this here?
            // if !self.has_table("COOP_REMOTES").await? {
            //     return Ok(LogicalStoragePolicy::None);
            // }

            let ut = table_name.to_uppercase();
            let sql =
                "SELECT COUNT(*) TOTALCOUNT FROM :schema.REMOTES WHERE TABLENAME = ':table_name' OR TABLENAME = ':ut';";
            let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
            let sql = sql.replace(":table_name", table_name);
            let sql = sql.replace(":ut", &ut);

            trace!(
                "[{}]: Looking for table in remotes: {table_name:?}",
                function_name!()
            );
            if self.has_any_rows(&sql).await? {
                trace!("[{}]: Has table in remotes", function_name!());
                let sql = "SELECT LOGICAL_STORAGE_POLICY FROM :schema.REMOTES WHERE TABLENAME = ':table_name' OR TABLENAME = ':ut';";
                let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
                let sql = sql.replace(":table_name", table_name);
                let sql = sql.replace(":ut", &ut);

                let i_policy = self.get_scalar_as_u32(&sql).await?;
                Ok(LogicalStoragePolicy::from_u32(i_policy))
            } else {
                Ok(LogicalStoragePolicy::None)
            }
        } else {
            let err = TreatyDbError::TableNotFoundInDatabase(
                table_name.to_string(),
                self.db_name.to_string(),
            );
            Err(err)
        }
    }

    async fn save_schema_to_data_host_tables(
        &self,
        table_id: &str,
        schema: &Table,
    ) -> Result<(), TreatyDbError> {
        trace!("[{}]: Schema: {schema:#?}", function_name!());

        /*
        table_name
        column_name
        data_type
         */

        let rows = &schema.rows;

        for row in rows {
            if row.vals[1].col.name == "column_name" {
                let col_name = &row.vals[1].data.as_ref().unwrap().data_string;

                let col_check = r#"
                    SELECT 
                        COUNT(*) COLCOUNT 
                    FROM 
                        :schema.DATA_HOST_TABLE_COLUMNS
                    WHERE
                        COLUMN_NAME = ':col_name'
                    ;"#;
                let col_check = col_check.replace(":schema", COOPERATIVE_SCHEMA_NAME);
                let col_check = col_check.replace(":col_name", col_name);

                trace!("[{}]: Checking for column", function_name!());
                if !self.has_any_rows(&col_check).await? {
                    trace!("[{}]: Column was not found", function_name!());

                    let col_id = GUID::rand().to_string();

                    let sql = r#"
                    INSERT INTO :schema.DATA_HOST_TABLE_COLUMNS
                    (
                        TABLE_ID,
                        COLUMN_ID,
                        COLUMN_NAME
                    )
                    VALUES
                    (
                        ':table_id',
                        ':col_id',
                        ':col_name'
                    )
                    ;"#;
                    let sql = sql.replace(":table_id", table_id);
                    let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
                    let sql = sql.replace(":col_name", col_name);
                    let sql = sql.replace(":col_id", &col_id.to_string());
                    self.write_for_db(&sql, &self.db_name).await?;
                }
            }
        }

        Ok(())
    }

    async fn get_remote_status_for_tables(
        &self,
    ) -> Result<Vec<(String, LogicalStoragePolicy)>, TreatyDbError> {
        let sql = "SELECT TABLENAME, LOGICAL_STORAGE_POLICY FROM :schema.REMOTES;";
        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);

        let mut table_policies: Vec<(String, LogicalStoragePolicy)> = Vec::new();

        let rows = self.query_db(&sql).await?;

        for row in rows {
            let table_name: String = row.get(0);
            let int_table_policy: i32 = row.get(1);
            let table_policy = LogicalStoragePolicy::from_i64(int_table_policy as i64);

            table_policies.push((table_name, table_policy))
        }

        trace!("[{}]: Table Policies: {table_policies:?}", function_name!());
        Ok(table_policies)
    }

    pub async fn get_db_schema(&self) -> Result<DatabaseSchema, TreatyDbError> {
        // need to de-dupe this code
        if self.has_schema(COOPERATIVE_SCHEMA_NAME).await?
            || self.has_schema(PARTIAL_SCHEMA_NAME).await?
        {
            let mut cooperation_enabled = false;
            let mut db_has_participants = false;

            // if this is a host db
            if self
                .has_table_in_schema("DATA_HOST", COOPERATIVE_SCHEMA_NAME)
                .await?
            {
                trace!("[{}]: getting schema for host database", function_name!());

                let sql = "SELECT DATABASE_ID FROM :schema.DATA_HOST";
                let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
                let db_id = self.get_scalar_as_string(&sql).await?;

                if let Ok(is_enabled) = self
                    .dbi()
                    .has_enable_coooperative_features(&self.db_name)
                    .await
                {
                    cooperation_enabled = is_enabled;
                }

                if cooperation_enabled {
                    if let Ok(has_participants) = self.has_participants().await {
                        db_has_participants = has_participants;
                    }
                }

                let mut db_schema = DatabaseSchema {
                    database_id: db_id.clone(),
                    database_name: self.db_name.to_string(),
                    tables: Vec::new(),
                    database_type: DatabaseType::to_u32(DatabaseType::Postgres),
                    treaty_database_type: num::ToPrimitive::to_u32(&TreatyDatabaseType::Host)
                        .unwrap_or(0),
                    cooperation_enabled,
                    has_participants: db_has_participants,
                };

                let mut tables_in_db: Vec<(String, String)> = Vec::new();

                let sql = "SELECT TABLE_ID, TABLE_NAME FROM :schema.DATA_TABLES;";
                let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);

                let rows = self.query_db(&sql).await?;

                for row in rows {
                    let table_id: String = row.get(0);
                    let table_name: String = row.get(1);
                    tables_in_db.push((table_id, table_name));
                }

                trace!("tables_in_db: {:?}", tables_in_db);

                for t in &tables_in_db {
                    let policy = self.get_logical_storage_policy(&t.1).await?;

                    let mut ts = TableSchema {
                        table_name: t.1.clone(),
                        table_id: t.0.clone(),
                        database_id: db_id.clone(),
                        database_name: self.db_name.clone(),
                        columns: Vec::new(),
                        logical_storage_policy: LogicalStoragePolicy::to_u32(policy),
                    };

                    // table_name, column_name, data_type
                    let schema = self.get_schema_of_table(&t.1, "public").await?;

                    let table_name = t.1.clone();

                    trace!(
                        "[{}]:  schema of table: {table_name:?} {schema:#?}",
                        function_name!()
                    );

                    for row in schema.rows {
                        trace!("[{}]: row {row:?}", function_name!());

                        let mut cs = ColumnSchema {
                            column_id: String::from(""),
                            column_name: String::from(""),
                            column_type: 0,
                            column_length: 0,
                            is_nullable: false,
                            ordinal: 0,
                            table_id: t.0.to_string(),
                            is_primary_key: false,
                        };

                        for val in row.vals {
                            if val.col.name == "ordinal_position" {
                                let item = val.data.clone().unwrap();
                                cs.ordinal = item.data_string.parse().unwrap();
                            }

                            if val.col.name == "column_name" {
                                let item = val.data.clone().unwrap();
                                cs.column_name = item.data_string.parse().unwrap();
                            }

                            if val.col.name == "data_type" {
                                let item = val.data.clone().unwrap();
                                let ct =
                                    ColumnType::data_type_to_enum_u32(item.data_string.clone());
                                let len = ColumnType::data_type_len(item.data_string.clone());

                                cs.column_type = ct;
                                cs.column_length = len;
                            }

                            if val.col.name == "is_nullable" {
                                let item = val.data.clone().unwrap();
                                if item.data_string == "YES" {
                                    cs.is_nullable = true
                                }
                            }

                            /*
                            if val.col.name == "IsPK" {
                                let item = val.data.clone().unwrap();
                                cs.is_primary_key = item.data_string.parse().unwrap();
                            }
                            */
                        }
                        ts.columns.push(cs.clone());
                    }
                    db_schema.tables.push(ts);
                }

                trace!("[{}]: {db_schema:#?}", function_name!());

                return Ok(db_schema);
            } else {
                trace!(
                    "[{}]: schema is for a partial Treaty database {}",
                    function_name!(),
                    &self.db_name
                );

                if let Ok(is_enabled) = self
                    .dbi()
                    .has_enable_coooperative_features(&self.db_name)
                    .await
                {
                    cooperation_enabled = is_enabled;
                }

                if cooperation_enabled {
                    if let Ok(has_participants) = self.has_participants().await {
                        db_has_participants = has_participants;
                    }
                }

                let mut db_schema = DatabaseSchema {
                    database_id: String::from(""),
                    database_name: self.db_name.to_string(),
                    tables: Vec::new(),
                    database_type: DatabaseType::to_u32(DatabaseType::Postgres),
                    treaty_database_type: num::ToPrimitive::to_u32(&TreatyDatabaseType::Host)
                        .unwrap_or(0),
                    cooperation_enabled,
                    has_participants: db_has_participants,
                };

                let table_names = self.get_all_user_table_names_in_db().await?;

                for table in &table_names {
                    let mut ts = TableSchema {
                        table_name: table.clone(),
                        table_id: String::from(""),
                        database_id: String::from(""),
                        database_name: self.db_name.clone(),
                        columns: Vec::new(),
                        logical_storage_policy: LogicalStoragePolicy::to_u32(
                            LogicalStoragePolicy::None,
                        ),
                    };

                    // table_name, column_name, data_type
                    let schema = self.get_schema_of_table(table, "public").await?;

                    let table_name = table.clone();

                    trace!(
                        "[{}]:  schema of table: {table_name:?} {schema:#?}",
                        function_name!()
                    );

                    for row in schema.rows {
                        trace!("[{}]: row {row:?}", function_name!());

                        let mut cs = ColumnSchema {
                            column_id: String::from(""),
                            column_name: String::from(""),
                            column_type: 0,
                            column_length: 0,
                            is_nullable: false,
                            ordinal: 0,
                            table_id: String::from(""),
                            is_primary_key: false,
                        };

                        for val in row.vals {
                            if val.col.name == "ordinal_position" {
                                let item = val.data.clone().unwrap();
                                cs.ordinal = item.data_string.parse().unwrap();
                            }

                            if val.col.name == "column_name" {
                                let item = val.data.clone().unwrap();
                                cs.column_name = item.data_string.parse().unwrap();
                            }

                            if val.col.name == "data_type" {
                                let item = val.data.clone().unwrap();
                                let ct =
                                    ColumnType::data_type_to_enum_u32(item.data_string.clone());
                                let len = ColumnType::data_type_len(item.data_string.clone());

                                cs.column_type = ct;
                                cs.column_length = len;
                            }

                            if val.col.name == "is_nullable" {
                                let item = val.data.clone().unwrap();
                                if item.data_string == "YES" {
                                    cs.is_nullable = true
                                }
                            }

                            /*
                            if val.col.name == "IsPK" {
                                let item = val.data.clone().unwrap();
                                cs.is_primary_key = item.data_string.parse().unwrap();
                            }
                            */
                        }
                        ts.columns.push(cs.clone());
                    }
                    db_schema.tables.push(ts);
                }

                trace!("[{}]: {db_schema:#?}", function_name!());

                return Ok(db_schema);
            }
        }

        let db_name = self.db_name.clone();
        warn!(
            "[{}]: Unknown database type named: {db_name:?}",
            function_name!()
        );

        let schema = DatabaseSchema {
            database_name: db_name,
            database_id: String::from(""),
            tables: Vec::new(),
            database_type: 0,
            treaty_database_type: 0,
            cooperation_enabled: false,
            has_participants: false,
        };

        Ok(schema)
    }

    async fn query_db(&self, sql: &str) -> Result<Vec<Row>, TreatyDbError> {
        let client = self.get_client_for_db(&self.db_name).await?;
        trace!("[{}]: SQL: {}", function_name!(), sql);
        Ok(client.query(sql, &[]).await?)
    }

    async fn query_generic(&self, sql: &str) -> Result<Vec<Row>, TreatyDbError> {
        let client = self.get_client().await?;
        trace!("[{}]: SQL: {}", function_name!(), sql);
        Ok(client.query(sql, &[]).await?)
    }

    pub async fn execute_write_at_host(&self, sql: &str) -> Result<usize, TreatyDbError> {
        self.write_for_db(sql, &self.db_name).await
    }

    pub async fn has_participants(&self) -> Result<bool, TreatyDbError> {
        let sql = "SELECT COUNT(*) PARTICIPANTCOUNT FROM :schema.PARTICIPANT";
        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);

        if !self
            .has_table_in_schema("PARTICIPANT", COOPERATIVE_SCHEMA_NAME)
            .await?
        {
            return Err(TreatyDbError::TableNotFoundInDatabase(
                "PARTICIPANT".to_string(),
                self.db_name.clone(),
            ));
        }

        self.has_any_rows(&sql).await
    }

    pub async fn delete_metadata_in_host_db(
        &self,
        table_name: &str,
        row_id: u32,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError> {
        let metadata_table_name = get_metadata_table_name(table_name);

        if !self.has_table_in_schema(table_name, "public").await? {
            //  need to create table
            let mut cmd = sql_text::Coop::text_create_metadata_table();
            cmd = cmd.replace(":table_name", &metadata_table_name);
            warn!("[{}]: Converting BLOB TO BYTEA", function_name!());
            cmd = cmd.replace("BLOB", "BYTEA");
            self.write_for_db(&cmd, &self.db_name).await?;
        }

        let mut cmd = sql_text::Coop::text_delete_row_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name);
        cmd = cmd.replace(":pid", "':pid'");
        cmd = cmd.replace(":pid", internal_participant_id);
        cmd = cmd.replace(":row", &row_id.to_string());

        let total = self.write_with_count(&cmd).await?;

        trace!("[{}]: rows affected: {total:?}", function_name!());

        Ok(total > 0)
    }

    pub async fn get_participants_for_database(
        &self,
    ) -> core::result::Result<Option<Vec<ParticipantStatus>>, TreatyDbError> {
        let mut result: Vec<ParticipantStatus> = Vec::new();

        // if the table doesn't exist, we should return an error here
        if !self
            .has_table_in_schema("PARTICIPANT", COOPERATIVE_SCHEMA_NAME)
            .await?
        {
            return Err(TreatyDbError::TableNotFoundInDatabase(
                "PARTICIPANT".to_string(),
                self.db_name.to_string(),
            ));
        }

        let cmd = "
        SELECT 
            INTERNAL_PARTICIPANT_ID,
            ALIAS,
            IP4ADDRESS,
            IP6ADDRESS,
            DB_PORT,
            INFO_PORT,
            CONTRACT_STATUS,
            PARTICIPANT_ID,
            HTTP_ADDR,
            HTTP_PORT
        FROM
            :schema.PARTICIPANT
        ;";

        let cmd = cmd.replace(":schema", COOPERATIVE_SCHEMA_NAME);

        let rows = self.query_db(&cmd).await?;

        for row in &rows {
            let internal_participant_guid: String = row.get(0);
            let alias: String = row.get(1);
            let ip4_address: String = row.get(2);
            let ip6_address: String = row.get(3);
            let database_port_number: i32 = row.get(4);
            let info_port_number: i32 = row.get(5);
            let token: Vec<u8> = Vec::new();
            let contract_status: i32 = row.get(6);
            let participant_guid: String = row.get(7);
            let http_addr: String = row.get(8);
            let http_port: i32 = row.get(9);

            let participant = Participant {
                participant_guid,
                alias,
                ip4_address,
                ip6_address,
                database_port_number: database_port_number as u32,
                info_port_number: info_port_number as u32,
                token,
                internal_participant_guid,
                http_addr,
                http_port: http_port as u32,
            };

            let ps = ParticipantStatus {
                participant: Some(participant),
                contract_status: contract_status as u32,
            };

            result.push(ps);
        }

        trace!("[{}]: participants found: {result:?}", function_name!());

        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }

    pub async fn update_metadata_in_host_db(
        &self,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError> {
        trace!("[{}]: updating metadata at host", function_name!());

        let metadata_table_name = get_metadata_table_name(table_name);

        if !self.has_table(&metadata_table_name).await? {
            //  need to create table
            let mut cmd = sql_text::Coop::text_create_metadata_table();
            cmd = cmd.replace(":table_name", &metadata_table_name);
            warn!("[{}]: Converting BLOB TO BYTEA", function_name!());
            cmd = cmd.replace("BLOB", "BYTEA");
            self.write_for_db(&cmd, &self.db_name).await?;
        }

        let mut cmd = sql_text::Coop::text_update_row_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name);
        cmd = cmd.replace(":hash", "$1::BYTEA");
        cmd = cmd.replace(":pid", "':pid'");
        cmd = cmd.replace(":pid", internal_participant_id);
        cmd = cmd.replace(":row", &row_id.to_string());
        let hash = hash.to_ne_bytes().to_vec();

        trace!("[{}]: hash check is: {hash:?}", function_name!());

        let rows = self
            .write_for_db_with_param(&cmd, &self.db_name, &[&hash])
            .await?;

        Ok(rows > 0)
    }

    async fn write_with_count(&self, sql: &str) -> Result<u64, TreatyDbError> {
        trace!("[{}]: SQL: {}", function_name!(), sql);

        let client = self.get_client_for_db(&self.db_name).await?;
        Ok(client.execute(sql, &[]).await?)
    }

    async fn populate_database_id(&self) -> Result<(), TreatyDbError> {
        let sql = "SELECT COUNT(*) DBIDCOUNT FROM :schema.DATA_HOST;";
        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);

        if !self.has_any_rows(&sql).await? {
            trace!("[{}]: Adding data host", function_name!());

            let sql = r#"
            INSERT INTO :schema.DATA_HOST
            (
                DATABASE_ID,
                DATABASE_NAME
            )
            VALUES
            (
                ':database_id',
                ':database_name'
            );"#;

            let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
            let db_id = GUID::rand().to_string();

            let sql = sql.replace(":database_id", &db_id);
            let sql = sql.replace(":database_name", &self.db_name);

            self.write_for_db(&sql, &self.db_name).await?;
        }

        Ok(())
    }

    pub async fn create_database(&self) -> Result<(), TreatyDbError> {
        let db_name = self.db_name.clone();
        trace!("[{}]: checking for database: {db_name:?}", function_name!());

        let sql = "SELECT COUNT(*) DBCOUNT FROM pg_database WHERE datname = ':db'";
        let sql = sql.replace(":db", &self.db_name);

        if !self.has_any_rows_generic(&sql).await? {
            let sql = format!("CREATE DATABASE {} ;", self.db_name);
            self.write(&sql).await?;

            let sql = "CREATE SCHEMA IF NOT EXISTS :schema ;";
            let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
            self.write_for_db(&sql, &self.db_name).await?;
        } else {
            return Err(TreatyDbError::DbAlreadyExists(self.db_name.clone()));
        }

        Ok(())
    }

    fn get_conn_string(&self) -> String {
        trace!(
            "[{}]: Getting connection at: {} user: {}",
            function_name!(),
            self.host,
            self.un,
        );

        match self.port {
            Some(port) => format!(
                "host={} user={} password={} port={}",
                self.host, self.un, self.pw, port
            ),
            None => format!("host={} user={} password={} ", self.host, self.un, self.pw,),
        }
    }

    fn get_conn_string_for_db(&self, db_name: &str) -> String {
        trace!(
            "[{}]: Getting connection at: {} user: {} database: {}",
            function_name!(),
            self.host,
            self.un,
            db_name
        );

        match self.port {
            Some(port) => format!(
                "host={} user={} password={} dbname={} port={}",
                self.host, self.un, self.pw, db_name, port
            ),
            None => format!(
                "host={} user={} password={} dbname={}",
                self.host, self.un, self.pw, db_name
            ),
        }
    }

    async fn get_client(&self) -> Result<Client, TreatyDbError> {
        match tokio_postgres::connect(&self.get_conn_string(), NoTls).await {
            Ok(pair) => {
                tokio::spawn(async move {
                    if let Err(e) = pair.1.await {
                        error!("connection error: {}", e);
                    }
                });

                Ok(pair.0)
            }
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                Err(TreatyDbError::General(e.to_string()))
            }
        }
    }

    pub async fn get_data_hash_at_host(
        &self,
        table_name: &str,
        row_id: u32,
    ) -> Result<Option<u64>, TreatyDbError> {
        let metadata_table_name = get_metadata_table_name(table_name);
        let mut cmd = String::from("SELECT HASH FROM :metadata WHERE ROW_ID = :row_id");
        cmd = cmd.replace(":metadata", &metadata_table_name);
        cmd = cmd.replace(":row_id", &row_id.to_string());

        match self.dbi().get_scalar_as_u64(&cmd).await {
            Ok(hash) => Ok(hash),
            Err(_) => Err(TreatyDbError::NoRowsFound(cmd.clone())),
        }
    }

    async fn get_client_for_db(&self, db_name: &str) -> Result<Client, TreatyDbError> {
        match tokio_postgres::connect(&self.get_conn_string_for_db(db_name), NoTls).await {
            Ok(pair) => {
                tokio::spawn(async move {
                    if let Err(e) = pair.1.await {
                        error!("connection error: {}", e);
                    }
                });

                Ok(pair.0)
            }
            Err(e) => {
                error!(
                    "[{}]: failed to get connection for db: {db_name:?} error: {e:?}",
                    function_name!()
                );
                Err(TreatyDbError::General(e.to_string()))
            }
        }
    }

    async fn write(&self, sql: &str) -> Result<(), TreatyDbError> {
        let sql = sql.replace(":schema", &self.treaty_schema_name);

        let client = self.get_client().await?;
        if let Err(e) = client.execute(&sql, &[]).await {
            error!("[{}]: {e:?}", function_name!());
            return Err(e.into());
        }

        Ok(())
    }

    async fn write_for_db(&self, sql: &str, db_name: &str) -> Result<usize, TreatyDbError> {
        trace!("[{}]: SQL: {}", function_name!(), sql);

        let client = self.get_client_for_db(db_name).await?;

        match client.execute(sql, &[]).await {
            Ok(rows_affected) => Ok(rows_affected as usize),
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                Err(e.into())
            }
        }
    }

    async fn write_for_db_with_param(
        &self,
        sql: &str,
        db_name: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<usize, TreatyDbError> {
        trace!("[{}]: SQL: {}", function_name!(), sql);

        let client = self.get_client_for_db(db_name).await?;

        match client.execute(sql, params).await {
            Ok(rows_affected) => Ok(rows_affected as usize),
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                Err(e.into())
            }
        }
    }

    pub async fn has_schema(&self, schema: &str) -> Result<bool, TreatyDbError> {
        let sql = "select exists (select schema_name
        from information_schema.schemata
        where schema_name = ':schema_name');";

        let sql = sql.replace(":schema_name", schema);

        self.get_scalar_as_bool(&sql).await
    }

    /// Checks to see if the specified table name exists in the specified schema
    pub async fn has_table_in_schema(
        &self,
        table_name: &str,
        schema: &str,
    ) -> Result<bool, TreatyDbError> {
        let table_name = table_name.to_lowercase();

        let sql = r#"
       SELECT EXISTS (
        SELECT * FROM 
            pg_tables
        WHERE 
            schemaname = ':schema' AND 
            tablename  = ':table_name'
        );"#;

        let sql = sql.replace(":table_name", &table_name);
        let sql = sql.replace(":schema", schema);

        let rows = self.query_db(&sql).await?;

        trace!("[{}]: Rows: {rows:?}", function_name!());

        let has_table: bool = rows[0].get(0);

        trace!("[{}]: has_table: {}", function_name!(), has_table);

        Ok(has_table)
    }

    /// Checks to see if the specified table name exists in the `public` schema
    async fn has_table(&self, table_name: &str) -> Result<bool, TreatyDbError> {
        let table_name = table_name.to_lowercase();

        let sql = r#"
       SELECT EXISTS (
        SELECT FROM 
            pg_tables
        WHERE 
            schemaname = 'public' AND 
            tablename  = ':table_name'
        );"#;

        let sql = sql.replace(":table_name", &table_name);

        let rows = self.query_db(&sql).await?;

        let has_table: bool = rows[0].get(0);

        trace!("[{}]: has_table: {}", function_name!(), has_table);

        Ok(has_table)
    }

    pub async fn set_logical_storage_policy(
        &self,
        table_name: &str,
        policy: LogicalStoragePolicy,
    ) -> Result<bool, TreatyDbError> {
        trace!(
            "[{}]: Set Logical Storage Policy for table:  {}",
            function_name!(),
            table_name
        );

        if self.has_table(table_name).await? {
            trace!("[{}]: Table found: {}", function_name!(), table_name);

            let sql =
                "SELECT COUNT(*) TOTALCOUNT FROM :schema.REMOTES WHERE TABLENAME = ':table_name';";
            let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
            let sql = sql.replace(":table_name", table_name);

            trace!("[{}]: SQL: {}", function_name!(), sql);
            if self.has_any_rows(&sql).await? {
                trace!("[{}]: Updating logical storage policy", function_name!());
                // this is an update
                let sql = "UPDATE :schema.REMOTES SET LOGICAL_STORAGE_POLICY = $1::INT WHERE TABLENAME = ':table_name';";
                let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
                let sql = sql.replace(":table_name", table_name);

                let policy = LogicalStoragePolicy::to_u32(policy) as i32;

                let client = self.get_client_for_db(&self.db_name).await?;

                trace!("[{}]: SQL: {}", function_name!(), sql);
                if let Err(e) = client.execute(&sql, &[&policy]).await {
                    error!("[{}]: {e:?}", function_name!());
                    return Err(e.into());
                }
            } else {
                // this is an insert
                trace!("[{}]: Adding logical storage policy", function_name!());
                trace!("[{}]: Table NOT found: {}", function_name!(), table_name);

                let sql = r#"INSERT INTO :schema.REMOTES 
                (
                    TABLENAME,
                    LOGICAL_STORAGE_POLICY
                )
                VALUES
                (
                    ':table_name',
                    $1::INT
                )
                ;"#;

                let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
                let sql = sql.replace(":table_name", table_name);

                let policy = LogicalStoragePolicy::to_u32(policy);
                let policy: i32 = policy as i32;

                let client = self.get_client_for_db(&self.db_name).await?;

                trace!("[{}]: SQL: {}", function_name!(), sql);
                if let Err(e) = client.execute(&sql, &[&policy]).await {
                    error!("[{}]: {e:?}", function_name!());
                    return Err(e.into());
                }
            }

            self.populate_data_host_tables().await?;
        } else {
            warn!("[{}]: Table not found: {}", function_name!(), table_name);
            let err = TreatyDbError::TableNotFoundInDatabase(
                table_name.to_string(),
                self.db_name.to_string(),
            );
            return Err(err);
        }

        Ok(true)
    }

    pub async fn execute_read_at_host(&self, sql: &str) -> Result<Table, TreatyDbError> {
        let client = self.get_client_for_db(&self.db_name).await?;
        self.read(sql, &client).await
    }

    /// Executes the requested query and returns a Treaty table
    /// representing the returned results
    pub async fn read(
        &self,
        sql: &str,
        client: &tokio_postgres::Client,
    ) -> Result<Table, TreatyDbError> {
        use crate::models::{Column, Data, Row, Value};

        let rows = self.query_db(sql).await?;

        trace!("[{}]: Rows: {rows:?}", function_name!());

        if !rows.is_empty() {
            // read the columns from the first row
            let columns = rows[0].columns();
            let total_columns = columns.len();
            let mut table = Table::new();
            table.set_database_type(DatabaseType::Postgres);

            let mut index = 0;
            for col in columns {
                let column_name = col.name();
                let column_type = col.type_();
                let column_kind = column_type.kind();
                if *column_kind == Kind::Simple {
                    // then we can compare the type to a Table type
                    let mut manual_column_name = String::from("");

                    // do this instead of column_type.name().to_string()
                    if column_type.name() == "varchar"
                        || column_type.name() == "text"
                        || column_type.name() == "name"
                        || column_type.name() == "bpchar"
                    {
                        manual_column_name = String::from("VARCHAR");
                    }

                    if column_type.name() == "bytea" {
                        manual_column_name = String::from("BLOB");
                    }

                    if column_type.name() == "int4" {
                        manual_column_name = String::from("INT");
                    }

                    if column_type.name() == "timestamp" {
                        manual_column_name = String::from("DATETIME");
                    }

                    // if we can't figure it out, log it for debugging
                    if manual_column_name.is_empty() {
                        warn!(
                            "[{}]: Unable to determine column type for {}",
                            function_name!(),
                            column_type.name()
                        )
                    }

                    // how can we tell if the column is nullable?
                    let column = Column {
                        name: column_name.to_string(),
                        is_nullable: false,
                        idx: index,
                        data_type: manual_column_name,
                        is_primary_key: false,
                    };

                    index += 1;

                    table.add_column(column);
                } else {
                    return Err(TreatyDbError::General(
                        "Can't determine complex Postgres types for tables".to_string(),
                    ));
                }
            }

            // now add the values of the rows to the table
            for row in rows {
                let mut data_row = Row::new();

                trace!("[{}]: total_columns: {}", function_name!(), total_columns);

                for i in 0..total_columns {
                    trace!("[{}]: column idx: {}", function_name!(), i);

                    let table_column = table.get_column_by_index(i);
                    trace!("[{}]: table_column: {table_column:?}", function_name!());

                    if let Some(tc) = table_column {
                        let idt = tc.data_type_to_enum_u32();
                        let dt = ColumnType::from_u32(idt);
                        match dt {
                            ColumnType::Unknown => {
                                warn!(
                                    "[{}]: Defaulting unknown column type values.",
                                    function_name!()
                                );

                                let data_item = Data::default();
                                let data_value = Value::default();

                                data_row.add_value(data_value);
                            }
                            ColumnType::Int => {
                                trace!("[{}]: Data type i32", function_name!());
                                let value: i32 = row.get(i);

                                let data_item = Data {
                                    data_string: value.to_string(),
                                    data_byte: Vec::new(),
                                };

                                let data_value = Value {
                                    data: Some(data_item),
                                    col: tc,
                                };

                                data_row.add_value(data_value);
                            }
                            ColumnType::Bit => {
                                trace!("[{}]: Data type bit", function_name!());
                                let value: bool = row.get(i);

                                let data_item = Data {
                                    data_string: value.to_string(),
                                    data_byte: Vec::new(),
                                };

                                let data_value = Value {
                                    data: Some(data_item),
                                    col: tc,
                                };

                                data_row.add_value(data_value);
                            }
                            ColumnType::Char => {
                                trace!("[{}]: Data type char", function_name!());
                                let value: String = row.get(i);

                                let data_item = Data {
                                    data_string: value,
                                    data_byte: Vec::new(),
                                };

                                let data_value = Value {
                                    data: Some(data_item),
                                    col: tc,
                                };

                                data_row.add_value(data_value);
                            }
                            ColumnType::DateTime => {
                                trace!("[{}]: Data type datetime", function_name!());
                                let value: Option<NaiveDateTime> = row.get(i);

                                let data_item = Data {
                                    data_string: value.unwrap_or_default().to_string(),
                                    data_byte: Vec::new(),
                                };

                                let data_value = Value {
                                    data: Some(data_item),
                                    col: tc,
                                };

                                data_row.add_value(data_value);
                            }
                            ColumnType::Decimal => todo!(),
                            ColumnType::Varchar => {
                                trace!("[{}]: Data type varchar", function_name!());
                                let value: String = row.get(i);

                                let data_item = Data {
                                    data_string: value,
                                    data_byte: Vec::new(),
                                };

                                let data_value = Value {
                                    data: Some(data_item),
                                    col: tc,
                                };

                                data_row.add_value(data_value);
                            }
                            ColumnType::Binary => {
                                trace!("[{}]: Data type binary", function_name!());
                                let value: Vec<u8> = row.get(i);

                                let data_item = Data {
                                    data_string: String::from(""),
                                    data_byte: value,
                                };

                                let data_value = Value {
                                    data: Some(data_item),
                                    col: tc,
                                };

                                data_row.add_value(data_value);
                            }
                            ColumnType::Varbinary => {
                                trace!("[{}]: Data type varbinary", function_name!());
                                let value: Vec<u8> = row.get(i);

                                let data_item = Data {
                                    data_string: String::from(""),
                                    data_byte: value,
                                };

                                let data_value = Value {
                                    data: Some(data_item),
                                    col: tc,
                                };

                                data_row.add_value(data_value);
                            }
                            ColumnType::Text => {
                                trace!("[{}]: Data type text", function_name!());
                                let value: String = row.get(i);

                                let data_item = Data {
                                    data_string: value,
                                    data_byte: Vec::new(),
                                };

                                let data_value = Value {
                                    data: Some(data_item),
                                    col: tc,
                                };

                                data_row.add_value(data_value);
                            }
                        }
                    } else {
                        warn!(
                            "[{}]: Column index {i:?} does not have a column",
                            function_name!()
                        );
                    }
                }

                table.add_row(data_row);
            }

            trace!("[{}]: Table Result: {table:#?}", function_name!());
            return Ok(table);
        }

        trace!("[{}]: Returning default table.", function_name!());
        Ok(Table::new())
    }

    pub async fn add_participant(
        &self,
        alias: &str,
        ip4addr: &str,
        db_port: Option<u32>,
        info_port: u32,
        http_addr: String,
        http_port: u16,
        id: Option<String>,
    ) -> Result<bool, TreatyDbError> {
        trace!(
            "[{}]: adding participant alias: {alias:?}",
            function_name!()
        );
        trace!("[{}]: adding participant id: {id:?}", function_name!());

        let db_host_id = match id {
            Some(id) => GUID::parse(&id).unwrap(),
            None => GUID::parse(defaults::EMPTY_GUID).unwrap(),
        };

        let is_added: bool = if self.has_participant(alias).await? {
            false
        } else {
            let participant = CoopDatabaseParticipant {
                internal_id: GUID::rand(),
                alias: alias.to_string(),
                ip4addr: ip4addr.to_string(),
                ip6addr: String::from(""),
                db_port: db_port.unwrap_or(0),
                info_port,
                contract_status: ContractStatus::NotSent,
                accepted_contract_version: GUID::parse(defaults::EMPTY_GUID).unwrap(),
                id: db_host_id,
                token: Vec::new(),
                http_addr,
                http_port,
            };
            self.save_participant(participant).await?;
            true
        };

        Ok(is_added)
    }

    pub async fn has_participant(&self, alias: &str) -> Result<bool, TreatyDbError> {
        let sql = "SELECT COUNT(*) TOTALCOUNT FROM :schema.PARTICIPANT WHERE ALIAS = ':alias'";
        let sql = sql.replace(":alias", alias);
        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);

        self.has_any_rows(&sql).await
    }

    pub async fn get_scalar_as_u32(&self, sql: &str) -> Result<u32, TreatyDbError> {
        let rows = self.query_db(sql).await?;

        trace!("[{}]: Rows: {rows:?}", function_name!());

        let row = &rows[0];
        trace!("[{}]: Row: {row:?}", function_name!());
        let result = row.try_get::<usize, i32>(0);

        match result {
            Ok(item) => Ok(item as u32),
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                Err(TreatyDbError::General(
                    "Unable to get value as u32".to_string(),
                ))
            }
        }
    }

    pub async fn get_participant_by_alias(
        &self,
        alias: &str,
    ) -> Result<Option<CoopDatabaseParticipant>, TreatyDbError> {
        let sql = r#"
        SELECT 
            INTERNAL_PARTICIPANT_ID,
            ALIAS,
            IP4ADDRESS,
            IP6ADDRESS,
            DB_PORT,
            INFO_PORT,
            CONTRACT_STATUS,
            ACCEPTED_CONTRACT_VERSION_ID,
            TOKEN,
            PARTICIPANT_ID,
            HTTP_ADDR,
            HTTP_PORT
        FROM
            :schema.PARTICIPANT
        WHERE
            ALIAS = ':alias'
        ;
        "#;
        let sql = sql.replace(":alias", alias);
        let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);

        let rows = self.query_db(&sql).await?;

        if rows.is_empty() {
            return Ok(None);
        };

        if rows.len() > 1 {
            return Err(TreatyDbError::MultipleParticipantAlias(alias.to_string()));
        }

        let row = &rows[0];

        let iid: String = row.get(0);
        let alias: String = row.get(1);
        let ip4addr: String = row.get(2);
        let ip6addr: String = row.get(3);
        let db_port: i32 = row.get(4);
        let info_port: i32 = row.get(5);
        let contract_status: i32 = row.get(6);
        let accepted_contract_version: String = row.get(7);
        let token: Vec<u8> = row.get(8);
        let id: String = row.get(9);
        let http_addr: String = row.get(10);
        let http_port: i32 = row.get(11);

        let participant = CoopDatabaseParticipant {
            internal_id: GUID::parse(&iid).unwrap(),
            alias,
            ip4addr,
            ip6addr,
            db_port: db_port as u32,
            info_port: info_port as u32,
            contract_status: ContractStatus::from_u32(contract_status as u32),
            accepted_contract_version: GUID::parse(&accepted_contract_version).unwrap(),
            token,
            id: GUID::parse(&id).unwrap(),
            http_addr,
            http_port: http_port as u16,
        };

        Ok(Some(participant))
    }

    pub async fn save_participant(
        &self,
        participant: CoopDatabaseParticipant,
    ) -> Result<bool, TreatyDbError> {
        trace!("[{}]: participant: {participant:#?}", function_name!());

        let ip4addr = participant.ip4addr;
        let ip6addr = participant.ip6addr;
        let db_port = participant.db_port.to_string();
        let info_port = participant.info_port.to_string();
        let contract_status = &num::ToPrimitive::to_u32(&participant.contract_status).unwrap_or(0);
        let accepted_contract_version = &participant.accepted_contract_version.to_string();
        let token = participant.token.clone();
        let id = participant.id.to_string();
        let alias = participant.alias.clone();
        let http_addr = participant.http_addr;
        let http_port = participant.http_port.to_string();
        let internal_id = participant.internal_id.to_string();

        let ip4addr = ip4addr.replace(':', "");
        let http_addr = http_addr.replace(':', "");

        if self.has_participant(&participant.alias).await? {
            // this is an update
            let sql = r#"
            UPDATE :schema.PARTICIPANT
            SET
                IP4ADDRESS = ':ip4addr',
                IP6ADDRESS = ':ip6addr',
                DB_PORT = ':db_port',
                INFO_PORT = ':info_port',
                CONTRACT_STATUS = ':contract_status',
                ACCEPTED_CONTRACT_VERSION_ID = ':accepted_contract_version',
                TOKEN = $1::BYTEA,
                PARTICIPANT_ID = ':id',
                HTTP_ADDR = ':http_addr',
                HTTP_PORT = ':http_port',
            WHERE
                ALIAS = ':alias'
            ;
            "#;

            let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
            let sql = sql.replace(":ip4addr", &ip4addr);
            let sql = sql.replace(":ip6addr", &ip6addr);
            let sql = sql.replace(":db_port", &db_port);
            let sql = sql.replace(":info_port", &info_port);
            let sql = sql.replace(":contract_status", &contract_status.to_string());
            let sql = sql.replace(":accepted_contract_version", accepted_contract_version);
            let sql = sql.replace(":id", &id);
            let sql = sql.replace(":http_addr", &http_addr);
            let sql = sql.replace(":http_port", &http_port);

            let client = self.get_client_for_db(&self.db_name).await?;

            trace!("[{}]: SQL: {}", function_name!(), sql);

            let result = client.execute(&sql, &[&token]).await?;

            Ok(result > 0)
        } else {
            // this is an insert

            // trace!("{:?}", &self);

            let sql = r#"
            INSERT INTO :schema.PARTICIPANT
            (
                INTERNAL_PARTICIPANT_ID,
                ALIAS,
                IP4ADDRESS,
                IP6ADDRESS,
                DB_PORT,
                INFO_PORT,
                CONTRACT_STATUS,
                ACCEPTED_CONTRACT_VERSION_ID,
                TOKEN,
                PARTICIPANT_ID,
                HTTP_ADDR,
                HTTP_PORT
            )
            VALUES
            (
                ':internal_id',
                ':alias',
                ':ip4addr',
                ':ip6addr',
                ':db_port',
                ':info_port',
                ':contract_status',
                ':accepted_contract_version',
                $1::BYTEA,
                ':id',
                ':http_addr',
                ':http_port'
            );
            "#;
            let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
            let sql = sql.replace(":alias", &alias);
            let sql = sql.replace(":internal_id", &internal_id);
            let sql = sql.replace(":schema", COOPERATIVE_SCHEMA_NAME);
            let sql = sql.replace(":ip4addr", &ip4addr);
            let sql = sql.replace(":ip6addr", &ip6addr);
            let sql = sql.replace(":db_port", &db_port);
            let sql = sql.replace(":info_port", &info_port);
            let sql = sql.replace(":contract_status", &contract_status.to_string());
            let sql = sql.replace(":accepted_contract_version", accepted_contract_version);
            let sql = sql.replace(":id", &id);
            let sql = sql.replace(":http_addr", &http_addr);
            let sql = sql.replace(":http_port", &http_port);

            let client = self.get_client_for_db(&self.db_name).await?;

            trace!("[{}]: SQL: {}", function_name!(), sql);
            let result = client.execute(&sql, &[&token]).await?;
            Ok(result > 0)
        }
    }

    pub async fn get_scalar_as_bool(&self, sql: &str) -> Result<bool, TreatyDbError> {
        let rows = self.query_db(sql).await?;

        trace!("[{}]: Rows: {rows:?}", function_name!());

        let row = &rows[0];
        trace!("[{}]: Row: {row:?}", function_name!());
        let result = row.try_get::<usize, bool>(0);

        match result {
            Ok(item) => Ok(item),
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                Err(TreatyDbError::General(
                    "Unable to get value as bool".to_string(),
                ))
            }
        }
    }

    pub async fn get_scalar_as_string(&self, sql: &str) -> Result<String, TreatyDbError> {
        let rows = self.query_db(sql).await?;

        if rows.is_empty() {
            return Ok(String::from(""));
        }

        trace!("[{}]: Rows: {rows:?}", function_name!());

        let row = &rows[0];
        trace!("[{}]: Row: {row:?}", function_name!());
        let result: Result<String, tokio_postgres::Error> = row.try_get::<usize, String>(0);

        match result {
            Ok(item) => Ok(item as String),
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                Err(TreatyDbError::General(
                    "Unable to get value as String".to_string(),
                ))
            }
        }
    }

    pub async fn has_any_rows_generic(&self, sql: &str) -> Result<bool, TreatyDbError> {
        let rows = self.query_generic(sql).await?;

        trace!("[{}]: Rows: {rows:?}", function_name!());

        let row = &rows[0];
        trace!("[{}]: Row: {row:?}", function_name!());
        let result = row.try_get::<usize, i64>(0);

        trace!("[{}]: result: {result:?}", function_name!());

        match result {
            Ok(count) => Ok(count > 0),
            Err(_) => {
                let result = row.try_get::<usize, i32>(0);
                trace!("[{}]: result: {result:?}", function_name!());
                match result {
                    Ok(count) => Ok(count > 0),
                    Err(_) => {
                        let result = row.try_get::<usize, i16>(0);
                        trace!("[{}]: result: {result:?}", function_name!());
                        match result {
                            Ok(count) => Ok(count > 0),
                            Err(_) => {
                                let result = row.try_get::<usize, i8>(0);
                                trace!("[{}]: result: {result:?}", function_name!());
                                match result {
                                    Ok(count) => Ok(count > 0),
                                    Err(_) => {
                                        let result = row.try_get::<usize, i8>(0);
                                        trace!("[{}]: result: {result:?}", function_name!());
                                        match result {
                                            Ok(count) => Ok(count > 0),
                                            Err(_) => {
                                                let result = row.try_get::<usize, u32>(0);
                                                trace!(
                                                    "[{}]: result: {result:?}",
                                                    function_name!()
                                                );
                                                match result {
                                                    Ok(count) => Ok(count > 0),
                                                    Err(_) => {
                                                        error!(
                                                            "[{}]: Cannot determine int type",
                                                            function_name!()
                                                        );
                                                        Err(TreatyDbError::General(String::from(
                                                            "Cannot determine int type",
                                                        )))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub async fn has_any_rows(&self, sql: &str) -> Result<bool, TreatyDbError> {
        let rows = self.query_db(sql).await?;

        trace!("[{}]: Rows: {rows:?}", function_name!());

        let row = &rows[0];
        trace!("[{}]: Row: {row:?}", function_name!());
        let result = row.try_get::<usize, i64>(0);

        trace!("[{}]: result: {result:?}", function_name!());

        match result {
            Ok(count) => Ok(count > 0),
            Err(_) => {
                let result = row.try_get::<usize, i32>(0);
                trace!("[{}]: result: {result:?}", function_name!());
                match result {
                    Ok(count) => Ok(count > 0),
                    Err(_) => {
                        let result = row.try_get::<usize, i16>(0);
                        trace!("[{}]: result: {result:?}", function_name!());
                        match result {
                            Ok(count) => Ok(count > 0),
                            Err(_) => {
                                let result = row.try_get::<usize, i8>(0);
                                trace!("[{}]: result: {result:?}", function_name!());
                                match result {
                                    Ok(count) => Ok(count > 0),
                                    Err(_) => {
                                        let result = row.try_get::<usize, i8>(0);
                                        trace!("[{}]: result: {result:?}", function_name!());
                                        match result {
                                            Ok(count) => Ok(count > 0),
                                            Err(_) => {
                                                let result = row.try_get::<usize, u32>(0);
                                                trace!(
                                                    "[{}]: result: {result:?}",
                                                    function_name!()
                                                );
                                                match result {
                                                    Ok(count) => Ok(count > 0),
                                                    Err(_) => {
                                                        error!(
                                                            "[{}]: Cannot determine int type",
                                                            function_name!()
                                                        );
                                                        Err(TreatyDbError::General(String::from(
                                                            "Cannot determine int type",
                                                        )))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
