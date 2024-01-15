use crate::{
    crypt,
    db_interface::postgres::db::TreatyDb,
    error::TreatyDbError,
    jwt::create_jwt,
    models::{
        CdsContracts, CdsContractsTables, CdsContractsTablesColumns, CdsHosts, HostInfo, Table,
        TreatySaveContractResult,
    },
    treaty_proto::{
        ColumnSchema, Contract, DatabaseSchema, Host, HostNetwork, TableSchema, TokenReply,
    },
    user_service_handler::do_vecs_match,
};
use chrono::{NaiveDateTime, Utc};
use guid_create::GUID;
use stdext::function_name;
use tokio_postgres::{
    types::{Kind, ToSql},
    Client, NoTls, Row,
};
use tracing::{error, info, trace, warn};
use treaty_types::enums::{
    ColumnType, ContractStatus, DatabaseType, DeletesFromHostBehavior, DeletesToHostBehavior,
    HostStatus, TreatyDatabaseType, UpdatesFromHostBehavior, UpdatesToHostBehavior,
};

use super::{db::COOPERATIVE_SCHEMA_NAME, part_db::PARTIAL_SCHEMA_NAME, PostgresDbi};

pub const TREATY_DEFAULT_NAME: &str = "treaty_system";

#[derive(Debug, Clone)]
pub struct TreatySystemDb {
    host: String,
    port: Option<u32>,
    un: String,
    pw: String,
    treaty_schema_name: String,
    backing_db_name: Option<String>,
    use_treaty_schema: bool,
}

impl TreatySystemDb {
    pub async fn new(
        host: &str,
        port: Option<u32>,
        un: &str,
        pw: &str,
        treaty_schema_name: &str,
        backing_db_name: Option<String>,
        use_treaty_schema: bool,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            un: un.to_string(),
            pw: pw.to_string(),
            treaty_schema_name: treaty_schema_name.to_string(),
            backing_db_name,
            use_treaty_schema,
        }
    }

    fn dbi(&self) -> PostgresDbi {
        PostgresDbi {
            host: self.host.clone(),
            port: self.port,
            un: self.un.clone(),
            pw: self.pw.clone(),
            treaty_schema_name: self.treaty_schema_name.clone(),
            db_name: self.backing_db_name.as_ref().unwrap().clone(),
            use_treaty_schema: self.use_treaty_schema,
        }
    }

    pub async fn treaty_generate_host_info(&self, host_name: &str) -> Result<(), TreatyDbError> {
        trace!("[{}]: host_name: {host_name:?}", function_name!());

        let token_gen = GUID::rand();
        let token = crypt::hash(&token_gen.to_string());

        let sql = "SELECT COUNT(*) HOSTS FROM :schema.CDS_HOST_INFO;";
        let sql = sql.replace(":schema", &self.treaty_schema_name);

        if self.has_any_rows(&sql, None).await? {
            let sql =
                "UPDATE :schema.CDS_HOST_INFO SET HOST_NAME = ':host_name', TOKEN = $1::BYTEA ;";
            let sql = sql.replace(":schema", &self.treaty_schema_name);
            let sql = sql.replace(":host_name", host_name);

            let client = self.get_client_for_db().await?;

            trace!("[{}]: SQL: {}", function_name!(), sql);

            let result = client
                .execute(&sql, &[&token.0.as_bytes().to_vec()])
                .await?;
        } else {
            let id = GUID::rand();
            let sql = r#"
            INSERT INTO :schema.CDS_HOST_INFO
            (
                HOST_ID,
                HOST_NAME,
                TOKEN
            )
            VALUES
            (
                ':id',
                ':name',
                $1::BYTEA 
            );"#;
            let sql = sql.replace(":schema", &self.treaty_schema_name);
            let sql = sql.replace(":id", &id.to_string());
            let sql = sql.replace(":name", host_name);
            let client = self.get_client_for_db().await?;

            trace!("[{}]: SQL: {}", function_name!(), sql);

            let result = client
                .execute(&sql, &[&token.0.as_bytes().to_vec()])
                .await?;
        }

        Ok(())
    }

    pub async fn treaty_get_host_info(&self) -> Result<Option<HostInfo>, TreatyDbError> {
        let sql = r#"
        SELECT 
            HOST_ID,
            HOST_NAME,
            TOKEN
        FROM
            :schema.CDS_HOST_INFO;"#;

        let sql = sql.replace(":schema", &self.treaty_schema_name);

        trace!("[{}]: SQL: {}", function_name!(), sql);

        let client = self.get_client_for_db().await?;

        let rows = client.query(&sql, &[]).await?;

        trace!("[{}]: {rows:?}", function_name!());

        if rows.is_empty() {
            warn!("[{}] no host info found", function_name!());
            Ok(None)
        } else {
            let host_id: String = rows[0].get(0);
            let host_name: String = rows[0].get(1);
            let token: Vec<u8> = rows[0].get(2);

            trace!(
                "[{}]: host_id: {host_id:?} host_name: {host_name:?}",
                function_name!()
            );

            Ok(Some(HostInfo {
                id: host_id,
                name: host_name,
                token,
            }))
        }
    }

    pub async fn create_schema_if_not_exists(&self) -> Result<(), TreatyDbError> {
        let sql = format!("CREATE SCHEMA IF NOT EXISTS {}", self.treaty_schema_name);
        self.write(&sql).await?;

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
            "[{}]: Getting connection at: {} user: {} database: {} use_treaty_schema setting: {}",
            function_name!(),
            self.host,
            self.un,
            db_name,
            self.use_treaty_schema
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

    pub async fn get_database_names(&self) -> Result<Option<Vec<String>>, TreatyDbError> {
        let mut databases: Vec<String> = Vec::new();
        let sql = "SELECT datname FROM pg_database 
        where datname != 'postgres' and datname not like 'template%';";
        let client = self.get_client().await?;

        trace!("[{}]: SQL: {sql:?}", function_name!());

        let rows = client.query(sql, &[]).await?;

        if rows.is_empty() {
            return Ok(None);
        }

        for row in rows {
            let db_name: String = row.get(0);
            databases.push(db_name);
        }

        Ok(Some(databases))
    }

    pub async fn accept_pending_contract(&self, host_name: &str) -> Result<bool, TreatyDbError> {
        let sql = r#"
        SELECT HOST_ID FROM :schema.CDS_HOSTS WHERE HOST_NAME = ':hostname' 
        ;"#;
        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":hostname", host_name);

        let db_host_id = self.dbi().get_scalar_as_string(&sql).await?;
        let sql = r#"
        SELECT COUNT(*) TOTALCOUNT FROM :schema.CDS_CONTRACTS WHERE HOST_ID = ':host_id' AND CONTRACT_STATUS = 2
        ;"#;
        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":host_id", &db_host_id);

        let client = self.get_client_for_db().await?;

        let has_pending_contract = self.has_any_rows(&sql, Some(client)).await?;

        if has_pending_contract {
            // 1 - we need to update the treaty_db record that we are accepting this contract
            // 2 - then we actually need to create the database with the properties of the
            // contract
            // 3 - we need to notify the host that we have accepted the contract

            let sql = "SELECT CONTRACT_ID FROM :schema.CDS_CONTRACTS WHERE HOST_ID = ':hid' AND CONTRACT_STATUS = 2;";
            let sql = sql.replace(":schema", &self.treaty_schema_name);
            let sql = sql.replace(":hid", &db_host_id);

            let cid = self.dbi().get_scalar_as_string(&sql).await?;

            let sql =
                "UPDATE :schema.CDS_CONTRACTS SET CONTRACT_STATUS = 3 WHERE CONTRACT_ID = ':cid';";
            let sql = sql.replace(":schema", &self.treaty_schema_name);
            let sql = sql.replace(":cid", &cid);

            self.write_for_db(&sql).await?;

            return Ok(true);
        }

        Ok(false)
    }

    async fn get_client_for_db(&self) -> Result<Client, TreatyDbError> {
        let db_name = if self.use_treaty_schema {
            self.backing_db_name.as_ref().unwrap().clone()
        } else {
            TREATY_DEFAULT_NAME.to_string()
        };

        match tokio_postgres::connect(&self.get_conn_string_for_db(&db_name), NoTls).await {
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

    async fn write(&self, sql: &str) -> Result<(), TreatyDbError> {
        let sql = sql.replace(":schema", &self.treaty_schema_name);

        let client = self.get_client().await?;
        if let Err(e) = client.execute(&sql, &[]).await {
            error!("[{}]: {e:?}", function_name!());
            return Err(e.into());
        }

        Ok(())
    }

    async fn write_for_db(&self, sql: &str) -> Result<(), TreatyDbError> {
        let sql = sql.replace(":schema", &self.treaty_schema_name);

        trace!("[{}]: SQL: {}", function_name!(), sql);

        let client = self.get_client_for_db().await?;
        if let Err(e) = client.execute(&sql, &[]).await {
            error!("[{}]: {e:?}", function_name!());
            return Err(e.into());
        }

        Ok(())
    }

    async fn has_role(&self, role_name: &str) -> Result<bool, TreatyDbError> {
        let sql = String::from(
            "SELECT count(*) AS ROLECOUNT FROM :schema.CDS_ROLE WHERE ROLENAME = ':rolename' ;",
        );
        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":rolename", role_name);

        self.has_any_rows(&sql, None).await
    }

    pub async fn create_backing_database(&self) -> Result<(), TreatyDbError> {
        let sql = "SELECT COUNT(*) DBCOUNT FROM pg_database WHERE datname = ':db'";

        let db_name = if self.use_treaty_schema {
            self.backing_db_name.as_ref().unwrap().clone()
        } else {
            TREATY_DEFAULT_NAME.to_string()
        };

        let sql = sql.replace(":db", &db_name);

        let client = self.get_client().await?;

        if !self.has_any_rows(&sql, Some(client)).await? {
            let sql = format!("CREATE DATABASE {} ;", db_name);
            self.write(&sql).await?;
        } else {
            return Err(TreatyDbError::DbAlreadyExists(String::from(
                "Treaty database or structure already exists.",
            )));
        }

        Ok(())
    }

    async fn write_with_count(&self, sql: &str) -> Result<u64, TreatyDbError> {
        trace!("[{}]: SQL: {}", function_name!(), sql);

        let client = self.get_client_for_db().await?;
        Ok(client.execute(sql, &[]).await?)
    }

    pub async fn change_host_status_by_name(
        &self,
        host_name: &str,
        status: u32,
    ) -> Result<bool, TreatyDbError> {
        let mut cmd = String::from(
            "
        UPDATE :schema.CDS_HOSTS SET HOST_STATUS = :status WHERE HOST_NAME = :name",
        );

        cmd = cmd.replace(":schema", &self.treaty_schema_name);
        cmd = cmd.replace(":status", &status.to_string());
        cmd = cmd.replace(":name", "':name'");
        cmd = cmd.replace(":name", host_name);

        let result = self.write_with_count(&cmd).await?;

        Ok(result > 0)
    }

    pub async fn change_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        let db_name = self
            .check_database_name_for_contract_format(db_name)
            .await?;

        let mut cmd = String::from(
            "
            UPDATE :schema.CDS_CONTRACTS_TABLES 
            SET UPDATES_FROM_HOST_BEHAVIOR = :behavior 
            WHERE
                DATABASE_NAME = :db_name
            AND
                TABLE_NAME = :table_name
            ;",
        );

        cmd = cmd.replace(":schema", &self.treaty_schema_name);
        cmd = cmd.replace(":db_name", "':db_name'");
        cmd = cmd.replace(":table_name", "':table_name'");
        cmd = cmd.replace(":db_name", &db_name);
        cmd = cmd.replace(":table_name", table_name);
        cmd = cmd.replace(":behavior", &behavior.to_string());

        let result = self.write_with_count(&cmd).await?;

        Ok(result > 0)
    }

    pub async fn get_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<UpdatesToHostBehavior, TreatyDbError> {
        let db_name = self
            .check_database_name_for_contract_format(db_name)
            .await?;

        let mut cmd = String::from(
            "
            SELECT 
                UPDATES_TO_HOST_BEHAVIOR
            FROM
                :schema.CDS_CONTRACTS_TABLES 
            WHERE
                DATABASE_NAME = ':db_name'
            AND
                TABLE_NAME = ':table_name'
            ;",
        );
        cmd = cmd.replace(":db_name", &db_name);
        cmd = cmd.replace(":table_name", table_name);
        cmd = cmd.replace(":schema", &self.treaty_schema_name);

        let result = self.dbi().get_scalar_as_u32(&cmd).await?;

        match result {
            Some(behavior) => {
                let behavior: UpdatesToHostBehavior =
                    num::FromPrimitive::from_u32(behavior).unwrap();
                Ok(behavior)
            }
            None => return Err(TreatyDbError::NoRowsFound(cmd.clone())),
        }
    }

    pub async fn change_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        let db_name = self
            .check_database_name_for_contract_format(db_name)
            .await?;
        let mut cmd = String::from(
            "
            UPDATE :schema.CDS_CONTRACTS_TABLES 
            SET UPDATES_TO_HOST_BEHAVIOR = :behavior 
            WHERE
                DATABASE_NAME = :db_name
            AND
                TABLE_NAME = :table_name
            ;",
        );

        cmd = cmd.replace(":schema", &self.treaty_schema_name);
        cmd = cmd.replace(":db_name", "':db_name'");
        cmd = cmd.replace(":table_name", "':table_name'");
        cmd = cmd.replace(":db_name", &db_name);
        cmd = cmd.replace(":table_name", table_name);
        cmd = cmd.replace(":behavior", &behavior.to_string());

        let result = self.write_with_count(&cmd).await?;

        Ok(result > 0)
    }

    pub async fn get_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<DeletesFromHostBehavior, TreatyDbError> {
        let db_name = self
            .check_database_name_for_contract_format(db_name)
            .await?;
        let mut cmd = String::from(
            "
            SELECT 
                DELETES_FROM_HOST_BEHAVIOR
            FROM
                :schema.CDS_CONTRACTS_TABLES 
            WHERE
                DATABASE_NAME = ':db_name'
            AND
                TABLE_NAME = ':table_name'
            ;",
        );
        cmd = cmd.replace(":db_name", &db_name);
        cmd = cmd.replace(":table_name", table_name);
        cmd = cmd.replace(":schema", &self.treaty_schema_name);

        let result = self.dbi().get_scalar_as_u32(&cmd).await?;

        trace!("[{}]: result: {result:?}", function_name!());

        Ok(num::FromPrimitive::from_u32(result.unwrap())
            .unwrap_or(DeletesFromHostBehavior::Unknown))
    }

    pub async fn get_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<UpdatesFromHostBehavior, TreatyDbError> {
        let db_name = self
            .check_database_name_for_contract_format(db_name)
            .await?;

        let mut cmd = String::from(
            "
                SELECT 
                    UPDATES_FROM_HOST_BEHAVIOR
                FROM
                    :schema.CDS_CONTRACTS_TABLES 
                WHERE
                    DATABASE_NAME = ':db_name'
                AND
                    TABLE_NAME = ':table_name'
                ;",
        );
        cmd = cmd.replace(":db_name", &db_name);
        cmd = cmd.replace(":table_name", table_name);
        cmd = cmd.replace(":schema", &self.treaty_schema_name);

        let result = self.dbi().get_scalar_as_u32(&cmd).await?;

        trace!(
            "[{}]: UpdatesFromHostBehavior: {result:?}",
            function_name!()
        );

        Ok(UpdatesFromHostBehavior::from_u32(result.unwrap()))
    }

    async fn create_treaty_schema(&self) -> Result<(), TreatyDbError> {
        let sql = format!("CREATE SCHEMA IF NOT EXISTS {}", self.treaty_schema_name);
        self.write_for_db(&sql).await?;

        Ok(())
    }

    async fn write_for_db_with_param(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<usize, TreatyDbError> {
        trace!("[{}]: SQL: {}", function_name!(), sql);

        let client = self.get_client_for_db().await?;

        match client.execute(sql, params).await {
            Ok(rows_affected) => Ok(rows_affected as usize),
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                Err(e.into())
            }
        }
    }

    async fn create_coop_schema(&self) -> Result<(), TreatyDbError> {
        let sql = format!("CREATE SCHEMA IF NOT EXISTS {}", COOPERATIVE_SCHEMA_NAME);
        self.write_for_db(&sql).await?;

        Ok(())
    }

    /// Same as `configure_treaty_db` for sqlite databases
    pub async fn configure_treaty_schema(&self) -> Result<(), TreatyDbError> {
        trace!("[{}]: Configuring Treaty...", function_name!());

        Self::create_backing_database(self).await?;

        trace!("[{}]: Create schema...", function_name!());

        Self::create_treaty_schema(self).await?;

        trace!("[{}]: Create table...", function_name!());

        Self::create_user_table(self).await?;
        Self::create_role_table(self).await?;
        Self::create_user_role_table(self).await?;
        Self::create_host_info_table(self).await?;
        Self::create_contracts_table(self).await?;
        Self::create_cds_hosts_table(self).await?;
        Self::create_contracts_tables_table(self).await?;
        Self::create_cds_contracts_tables_schemas_table(self).await?;
        Self::create_user_tokens_table(self).await?;

        let db_has_role = self.has_role(&String::from("SysAdmin")).await?;

        if !db_has_role {
            trace!("[{}]: Adding SysAdmin..", function_name!());
            let statement =
                String::from("INSERT INTO :schema.CDS_ROLE (rolename) VALUES ('SysAdmin');");
            self.write_for_db(&statement).await?;
        }

        trace!("[{}]: Finished Configuring Treaty...", function_name!());

        Ok(())
    }

    pub async fn verify_host_by_name(
        &self,
        host_name: &str,
        token: Vec<u8>,
    ) -> Result<bool, TreatyDbError> {
        trace!("[{}]: host_name: {host_name:?}", function_name!());

        let sql =
            r#"SELECT TOKEN FROM :schema.CDS_HOSTS WHERE HOST_NAME = ':name' AND HOST_STATUS = 1;"#;
        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":name", host_name);

        let rows = self.query_db(&sql).await?;

        if rows.is_empty() {
            Ok(false)
        } else {
            if rows.len() > 1 {
                warn!("[{}]: multiple tokens found!", function_name!());
            }

            let row = &rows[0];
            let returned_token: Vec<u8> = row.get(0);

            Ok(do_vecs_match(&token, &returned_token))
        }
    }

    pub async fn verify_host_by_id(
        &self,
        host_id: &str,
        token: Vec<u8>,
    ) -> Result<bool, TreatyDbError> {
        trace!("[{}]: host_id: {host_id}", function_name!());

        let sql =
            r#"SELECT TOKEN FROM :schema.CDS_HOSTS WHERE HOST_ID = ':hid' AND HOST_STATUS = 1;"#;
        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":hid", host_id);

        let rows = self.query_db(&sql).await?;

        if rows.is_empty() {
            Ok(false)
        } else {
            if rows.len() > 1 {
                warn!("[{}]: multiple tokens found!", function_name!());
            }

            let row = &rows[0];
            let returned_token: Vec<u8> = row.get(0);

            Ok(do_vecs_match(&token, &returned_token))
        }
    }

    async fn create_user_table(&self) -> Result<(), TreatyDbError> {
        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.CDS_USER
        (
            USERNAME VARCHAR(25) UNIQUE,
            HASH BYTEA NOT NULL
        );"#;

        self.write_for_db(sql).await?;

        Ok(())
    }

    async fn create_role_table(&self) -> Result<(), TreatyDbError> {
        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.CDS_ROLE
        (
            ROLENAME VARCHAR(25) UNIQUE
        );"#;

        self.write_for_db(sql).await?;

        Ok(())
    }

    async fn create_user_role_table(&self) -> Result<(), TreatyDbError> {
        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.CDS_USER_ROLE
        (
            USERNAME VARCHAR(25) NOT NULL,
            ROLENAME VARCHAR(25) NOT NULL   
        );"#;

        self.write_for_db(sql).await?;

        Ok(())
    }

    async fn create_host_info_table(&self) -> Result<(), TreatyDbError> {
        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.CDS_HOST_INFO
        (
            HOST_ID CHAR(36) NOT NULL,
            HOST_NAME VARCHAR(50) NOT NULL,
            TOKEN BYTEA NOT NULL
        );"#;

        self.write_for_db(sql).await?;

        Ok(())
    }

    async fn create_cds_hosts_table(&self) -> Result<(), TreatyDbError> {
        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.CDS_HOSTS
        (
            HOST_ID CHAR(36) NOT NULL,
            HOST_NAME VARCHAR(50),
            TOKEN BYTEA,
            IP4ADDRESS VARCHAR(25),
            IP6ADDRESS VARCHAR(25),
            DB_PORT INT,
            INFO_PORT INT,
            LAST_COMMUNICATION_UTC TIMESTAMP,
            HOST_STATUS INT,
            HTTP_ADDR VARCHAR(50),
            HTTP_PORT INT
        );"#;

        self.write_for_db(sql).await?;

        Ok(())
    }

    async fn create_contracts_table(&self) -> Result<(), TreatyDbError> {
        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.CDS_CONTRACTS
        (
            HOST_ID CHAR(36) NOT NULL,
            CONTRACT_ID CHAR(36) NOT NULL,
            CONTRACT_VERSION_ID CHAR(36) NOT NULL,
            DATABASE_NAME VARCHAR(500) NOT NULL,
            DATABASE_ID CHAR(36) NOT NULL,
            DESCRIPTION VARCHAR(255),
            GENERATED_DATE_UTC TIMESTAMP,
            CONTRACT_STATUS INT
        );"#;

        self.write_for_db(sql).await?;

        Ok(())
    }

    async fn create_contracts_tables_table(&self) -> Result<(), TreatyDbError> {
        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.CDS_CONTRACTS_TABLES
        (
            DATABASE_ID CHAR(36) NOT NULL,
            DATABASE_NAME VARCHAR(500) NOT NULL,
            TABLE_ID CHAR(36) NOT NULL,
            TABLE_NAME VARCHAR(50) NOT NULL,
            LOGICAL_STORAGE_POLICY INT,
            UPDATES_FROM_HOST_BEHAVIOR INT,
            DELETES_FROM_HOST_BEHAVIOR INT,
            UPDATES_TO_HOST_BEHAVIOR INT,
            DELETES_TO_HOST_BEHAVIOR INT,
            USE_DATA_LOG_TABLE INT
        );"#;

        self.write_for_db(sql).await?;

        Ok(())
    }

    async fn create_cds_contracts_tables_schemas_table(&self) -> Result<(), TreatyDbError> {
        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.CDS_CONTRACTS_TABLE_SCHEMAS
        (
            TABLE_ID CHAR(36) NOT NULL,
            COLUMN_ID CHAR(36) NOT NULL,
            COLUMN_NAME VARCHAR(50) NOT NULL,
            COLUMN_TYPE INT NOT NULL,
            COLUMN_LENGTH INT NOT NULL,
            COLUMN_ORDINAL INT NOT NULL,
            IS_NULLABLE INT
        );"#;

        self.write_for_db(sql).await?;

        Ok(())
    }

    async fn create_user_tokens_table(&self) -> Result<(), TreatyDbError> {
        let sql = r#"CREATE TABLE IF NOT EXISTS :schema.CDS_USER_TOKENS
        (
            USERNAME VARCHAR(25) NOT NULL,
            TOKEN VARCHAR NOT NULL,
            ISSUED_UTC TIMESTAMP,
            EXPIRATION_UTC TIMESTAMP
        );"#;

        self.write_for_db(sql).await?;

        Ok(())
    }

    pub async fn configure_admin(&self, login: &str, pw: &str) -> Result<(), TreatyDbError> {
        if !self.has_login(login).await? {
            self.create_login(login, pw).await?;
        }

        if !self
            .login_is_in_role(login, &String::from("SysAdmin"))
            .await?
        {
            self.add_login_to_role(login, &String::from("SysAdmin"))
                .await?;
        }

        Ok(())
    }

    pub async fn save_contract(
        &self,
        contract: Contract,
    ) -> Result<TreatySaveContractResult, TreatyDbError> {
        trace!("[{}]: save contract: {contract:#?}", function_name!());

        if !self.has_contract(&contract.contract_guid).await? {
            self.save_contract_metadata(&contract).await?;
            self.save_contract_table_data(&contract).await?;
            self.save_contract_table_schema_data(&contract).await?;
            self.save_contract_host_data(&contract).await?;

            return Ok(TreatySaveContractResult {
                is_successful: true,
                contract_status: ContractStatus::from_u32(contract.status),
                participant_information: None,
            });
        } else {
            warn!(
                "[{}]: contract already exists for id {} version {}",
                function_name!(),
                &contract.contract_guid,
                &contract.contract_version
            );
        }

        todo!("save_contract")
    }

    /// saves top level contract data to treaty_db's CDS_CONTRACTS table
    async fn save_contract_metadata(&self, contract: &Contract) -> Result<(), TreatyDbError> {
        let host = contract.host_info.as_ref().unwrap().clone();
        let db = contract.schema.as_ref().unwrap();

        let sql = r#"
        INSERT INTO :schema.CDS_CONTRACTS
        (
            HOST_ID,
            CONTRACT_ID,
            CONTRACT_VERSION_ID,
            DATABASE_NAME,
            DATABASE_ID,
            DESCRIPTION,
            GENERATED_DATE_UTC,
            CONTRACT_STATUS
        )
        VALUES
        (
            ':hid',
            ':cid',
            ':cvid',
            ':dbname',
            ':dbid',
            ':desc',
            ':gdutc',
            :status
        );"#;

        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":hid", &host.host_guid);
        let sql = sql.replace(":cid", &contract.contract_guid);
        let sql = sql.replace(":cvid", &contract.contract_version);
        let sql = sql.replace(":dbname", &db.database_name);
        let sql = sql.replace(":dbid", &db.database_id);
        let sql = sql.replace(":desc", &contract.description);
        let sql = sql.replace(":gdutc", &Utc::now().to_rfc3339());
        let sql = sql.replace(":status", &contract.status.to_string());
        self.write_for_db(&sql).await
    }

    async fn save_contract_table_data(&self, contract: &Contract) -> Result<(), TreatyDbError> {
        let sql = r#"
        INSERT INTO :schema.CDS_CONTRACTS_TABLES
    (
        DATABASE_ID,
        DATABASE_NAME,
        TABLE_ID,
        TABLE_NAME,
        LOGICAL_STORAGE_POLICY,
        UPDATES_FROM_HOST_BEHAVIOR,
        DELETES_FROM_HOST_BEHAVIOR,
        UPDATES_TO_HOST_BEHAVIOR,
        DELETES_TO_HOST_BEHAVIOR,
        USE_DATA_LOG_TABLE
    )
    VALUES
    (
        ':dbid',
        ':dbname',
        ':tid',
        ':tname',
        :policy,
        1,
        1,
        1,
        1,
        0
    )
    ;"#;

        let schema = contract.schema.as_ref().unwrap();

        let db_name = schema.database_name.clone();
        let db_id = schema.database_id.clone();

        for t in &schema.tables {
            let sql = sql.replace(":schema", &self.treaty_schema_name);
            let sql = sql.replace(":dbid", &db_id);
            let sql = sql.replace(":dbname", &db_name);
            let sql = sql.replace(":tid", &t.table_id);
            let sql = sql.replace(":tname", &t.table_name);
            let sql = sql.replace(":policy", &t.logical_storage_policy.to_string());
            self.write_for_db(&sql).await?
        }

        Ok(())
    }

    async fn save_contract_table_schema_data(
        &self,
        contract: &Contract,
    ) -> Result<(), TreatyDbError> {
        let tables = contract.schema.as_ref().unwrap().tables.clone();

        for table in &tables {
            let sql = r#"
            INSERT INTO :schema.CDS_CONTRACTS_TABLE_SCHEMAS
            (
                TABLE_ID,
                COLUMN_ID,
                COLUMN_NAME,
                COLUMN_TYPE,
                COLUMN_LENGTH,
                COLUMN_ORDINAL,
                IS_NULLABLE
            )
            VALUES
            (
                ':tid',
                ':cid',
                ':cname',
                ':ctype',
                ':clength',
                ':cordinal',
                ':is_nullable'
            )
            ;"#;

            let sql = sql.replace(":schema", &self.treaty_schema_name);
            let sql = sql.replace(":tid", &table.table_id);
            for column in &table.columns {
                let cid = column.column_id.clone();
                let cname = column.column_name.clone();
                let ctype = column.column_type;
                let clength = column.column_length;
                let cordinal = column.ordinal;
                let is_nullable = i32::from(column.is_nullable);

                let sql = sql.replace(":cid", &cid);
                let sql = sql.replace(":cname", &cname);
                let sql = sql.replace(":ctype", &ctype.to_string());
                let sql = sql.replace(":clength", &clength.to_string());
                let sql = sql.replace(":cordinal", &cordinal.to_string());
                let sql = sql.replace(":is_nullable", &is_nullable.to_string());
                self.write_for_db(&sql).await?;
            }
        }

        Ok(())
    }

    async fn save_contract_host_data(&self, contract: &Contract) -> Result<(), TreatyDbError> {
        let sql = r#"
        INSERT INTO :schema.CDS_HOSTS
    (
        HOST_ID,
        HOST_NAME,
        TOKEN,
        IP4ADDRESS,
        IP6ADDRESS,
        DB_PORT,
        INFO_PORT,
        LAST_COMMUNICATION_UTC,
        HOST_STATUS,
        HTTP_ADDR,
        HTTP_PORT
    )
    VALUES
    (
        ':hid',
        ':hname',
        $1::BYTEA,
        ':ip4',
        ':ip6',
        ':db_port',
        ':info_port',
        ':last_comm',
        1,
        ':http_addr',
        ':http_port'
    )
    ;"#;

        let host = contract.host_info.as_ref().unwrap().clone();

        trace!("[{}] host: {host:#?}", function_name!());

        let mut ip4 = String::from("");
        let mut db_port: u32 = 0;
        let mut info_port: u32 = 0;
        let mut ip6 = String::from("");
        let mut http_addr = String::from("");
        let mut http_port: u32 = 0;

        if let Some(network) = host.network {
            if let Some(_ip4) = network.ip4_address {
                ip4 = _ip4;
            }

            if let Some(_port) = network.database_port_number {
                db_port = _port;
            }

            if let Some(_ip6) = network.ip6_address {
                ip6 = _ip6;
            }

            if let Some(_http_addr) = network.http_addr {
                http_addr = _http_addr;
            }

            if let Some(_http_port) = network.http_port {
                http_port = _http_port;
            }

            if let Some(_info_port) = network.info_port_number {
                info_port = _info_port;
            }
        }

        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":hid", &host.host_guid);

        let sql = sql.replace(":hname", &host.host_name);

        let sql = sql.replace(":ip4", &ip4);
        let sql = sql.replace(":ip6", &ip6);
        let sql = sql.replace(":db_port", &db_port.to_string());
        let sql = sql.replace(":info_port", &info_port.to_string());
        let sql = sql.replace(":last_comm", &Utc::now().to_string());
        let sql = sql.replace(":http_addr", &http_addr);
        let sql = sql.replace(":http_port", &http_port.to_string());

        let client = self.get_client_for_db().await?;

        trace!("[{}]: SQL: {}", function_name!(), sql);
        let result = client.execute(&sql, &[&host.token]).await?;

        trace!("[{}]: result: {result:?}", function_name!());

        Ok(())
    }

    pub async fn get_treaty_db_type(
        &self,
        db_name: &str,
    ) -> Result<TreatyDatabaseType, TreatyDbError> {
        if let Some(database_name) = &self.backing_db_name {
            if db_name == database_name.as_str()
                && self.dbi().has_schema(COOPERATIVE_SCHEMA_NAME).await?
            {
                if self.use_treaty_schema {
                    return Ok(TreatyDatabaseType::SystemAndHost);
                } else {
                    return Ok(TreatyDatabaseType::Host);
                }
            }
        }

        if self.dbi().has_schema(PARTIAL_SCHEMA_NAME).await? {
            return Ok(TreatyDatabaseType::Partial);
        }

        Ok(TreatyDatabaseType::Unknown)
    }

    pub async fn get_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<DeletesToHostBehavior, TreatyDbError> {
        let db_name = self
            .check_database_name_for_contract_format(db_name)
            .await?;

        let mut cmd = String::from(
            "
            SELECT 
                DELETES_TO_HOST_BEHAVIOR
            FROM
                :schema.CDS_CONTRACTS_TABLES 
            WHERE
                DATABASE_NAME = ':db_name'
            AND
                TABLE_NAME = ':table_name'
            ;",
        );
        cmd = cmd.replace(":db_name", &db_name);
        cmd = cmd.replace(":table_name", table_name);
        cmd = cmd.replace(":schema", &self.treaty_schema_name);

        let result = self.dbi().get_scalar_as_u32(&cmd).await?;

        Ok(num::FromPrimitive::from_u32(result.unwrap()).unwrap_or(DeletesToHostBehavior::Unknown))
    }

    pub async fn change_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        let db_name = self
            .check_database_name_for_contract_format(db_name)
            .await?;
        let mut cmd = String::from(
            "
            UPDATE :schema.CDS_CONTRACTS_TABLES 
            SET DELETES_TO_HOST_BEHAVIOR = :behavior 
            WHERE
                DATABASE_NAME = :db_name
            AND
                TABLE_NAME = :table_name
            ;",
        );

        cmd = cmd.replace(":schema", &self.treaty_schema_name);
        cmd = cmd.replace(":db_name", "':db_name'");
        cmd = cmd.replace(":table_name", "':table_name'");
        cmd = cmd.replace(":db_name", &db_name);
        cmd = cmd.replace(":table_name", table_name);
        cmd = cmd.replace(":behavior", &behavior.to_string());

        let result = self.write_with_count(&cmd).await?;

        Ok(result > 0)
    }

    pub async fn change_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        let db_name = self
            .check_database_name_for_contract_format(db_name)
            .await?;
        let mut cmd = String::from(
            "
            UPDATE :schema.CDS_CONTRACTS_TABLES 
            SET DELETES_FROM_HOST_BEHAVIOR = :behavior 
            WHERE
                DATABASE_NAME = :db_name
            AND
                TABLE_NAME = :table_name
            ;",
        );

        cmd = cmd.replace(":schema", &self.treaty_schema_name);
        cmd = cmd.replace(":db_name", "':db_name'");
        cmd = cmd.replace(":table_name", "':table_name'");
        cmd = cmd.replace(":db_name", &db_name);
        cmd = cmd.replace(":table_name", table_name);
        cmd = cmd.replace(":behavior", &behavior.to_string());

        let result = self.write_with_count(&cmd).await?;

        Ok(result > 0)
    }

    pub async fn get_cds_host_for_part_db(
        &self,
        db_name: &str,
    ) -> Result<Option<CdsHosts>, TreatyDbError> {
        let mut cmd = String::from(
            "
        SELECT 
            HOST_ID
        FROM 
            :schema.CDS_CONTRACTS 
        WHERE 
            DATABASE_NAME = ':db_name'
        ;",
        );
        cmd = cmd.replace(":schema", &self.treaty_schema_name);
        cmd = cmd.replace(":db_name", db_name);
        let host_id = self.dbi().get_scalar_as_string(&cmd).await?;

        let mut cds_host_infos: Vec<CdsHosts> = Vec::new();

        let cmd = String::from(
            "
            SELECT 
                HOST_ID,
                HOST_NAME,
                TOKEN,
                IP4ADDRESS,
                IP6ADDRESS,
                DB_PORT,
                INFO_PORT,
                LAST_COMMUNICATION_UTC,
                HTTP_ADDR,
                HTTP_PORT,
                HOST_STATUS
            FROM
                :schema.CDS_HOSTS
            WHERE
                HOST_ID = ':hid'
        ;",
        );
        let cmd = cmd.replace(":schema", &self.treaty_schema_name);
        let cmd = cmd.replace(":hid", &host_id);

        let rows = self.query_db(&cmd).await?;

        for row in &rows {
            let host_id: String = row.get(0);
            let host_name: String = row.get(1);
            let token: Vec<u8> = row.get(2);
            let ip4: String = row.get(3);
            let ip6: String = row.get(4);
            let db_port: i32 = row.get(5);
            let info_port: i32 = row.get(6);
            let last_comm_utc: NaiveDateTime = row.get(7);
            let http_addr: String = row.get(8);
            let http_port: i32 = row.get(9);
            let status: i32 = row.get(10);

            let host = CdsHosts {
                host_id,
                host_name,
                token,
                ip4,
                ip6,
                db_port: db_port as u32,
                info_port: info_port as u32,
                last_comm_utc: last_comm_utc.to_string(),
                http_addr,
                http_port: http_port as u32,
                status: num::FromPrimitive::from_u32(status as u32).unwrap_or(HostStatus::Unknown),
            };

            cds_host_infos.push(host);
        }

        trace!("[{}]: {cds_host_infos:#?}", function_name!());

        if !cds_host_infos.is_empty() {
            if cds_host_infos.len() > 1 {
                warn!(
                    "[{}]: multiple hosts found: {cds_host_infos:?}",
                    function_name!()
                );
            }

            Ok(Some(cds_host_infos.first().unwrap().clone()))
        } else {
            Ok(None)
        }
    }

    pub async fn get_contracts_by_status(
        &self,
        contract_status: ContractStatus,
    ) -> Result<Option<Vec<Contract>>, TreatyDbError> {
        let mut pending_contracts: Vec<Contract> = Vec::new();

        let mut cds_contracts: Vec<CdsContracts> = Vec::new();
        let mut cds_tables: Vec<CdsContractsTables> = Vec::new();
        let mut cds_tables_columns: Vec<CdsContractsTablesColumns> = Vec::new();
        let mut cds_host_infos: Vec<CdsHosts> = Vec::new();
        let mut db_schema: Vec<DatabaseSchema> = Vec::new();

        let sql = r#"
            SELECT 
                HOST_ID,
                CONTRACT_ID,
                CONTRACT_VERSION_ID,
                DATABASE_NAME,
                DATABASE_ID,
                DESCRIPTION,
                GENERATED_DATE_UTC,
                CONTRACT_STATUS 
            FROM 
                :schema.CDS_CONTRACTS 
            WHERE 
                CONTRACT_STATUS = ':u32_contract_status'
            ;"#;

        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(
            ":u32_contract_status",
            &ContractStatus::to_u32(contract_status).to_string(),
        );

        let rows = self.query_db(&sql).await?;

        trace!("[{}]: rows: {rows:?}", function_name!());

        for row in &rows {
            let host_id: String = row.get(0);
            let contract_id: String = row.get(1);
            let contract_version_id: String = row.get(2);
            let database_name: String = row.get(3);
            let database_id: String = row.get(4);
            let description: String = row.get(5);
            let gdutc: NaiveDateTime = row.get(6);
            let status: i32 = row.get(7);

            let cds_contract = CdsContracts {
                host_id,
                contract_id,
                contract_version_id,
                database_name,
                database_id,
                description,
                generated_date: gdutc.to_string(),
                contract_status: ContractStatus::from_u32(status as u32),
            };

            cds_contracts.push(cds_contract);
        }

        for contract in &cds_contracts {
            let dbid = &contract.database_id.clone();
            let sql = r#"
                SELECT 
                    DATABASE_ID,
                    DATABASE_NAME,
                    TABLE_ID,
                    TABLE_NAME,
                    LOGICAL_STORAGE_POLICY
                FROM 
                    :schema.CDS_CONTRACTS_TABLES 
                WHERE 
                    DATABASE_ID = ':dbid';
                "#;

            let sql = sql.replace(":dbid", dbid);
            let sql = sql.replace(":schema", &self.treaty_schema_name);
            let rows = self.query_db(&sql).await?;

            for row in &rows {
                let database_id: String = row.get(0);
                let database_name: String = row.get(1);
                let table_id: String = row.get(2);
                let table_name: String = row.get(3);
                let logical_storage_policy: i32 = row.get(4);

                let table = CdsContractsTables {
                    database_id,
                    database_name,
                    table_id,
                    table_name,
                    logical_storage_policy: logical_storage_policy as u32,
                };

                cds_tables.push(table);
            }
        }

        for table in &cds_tables {
            let tid = table.table_id.clone();
            let sql = r#"
            SELECT 
                TABLE_ID,
                COLUMN_ID,
                COLUMN_NAME,
                COLUMN_TYPE,
                COLUMN_LENGTH,
                COLUMN_ORDINAL,
                IS_NULLABLE 
            FROM 
                :schema.CDS_CONTRACTS_TABLE_SCHEMAS 
            WHERE 
                TABLE_ID = ':tid';
            "#;
            let sql = sql.replace(":tid", &tid);
            let sql = sql.replace(":schema", &self.treaty_schema_name);

            let rows = self.query_db(&sql).await?;

            for row in &rows {
                let table_id: String = row.get(0);
                let column_id: String = row.get(1);
                let column_name: String = row.get(2);
                let column_type: i32 = row.get(3);
                let column_length: i32 = row.get(4);
                let column_ordinal: i32 = row.get(5);
                let is_nullable: i32 = row.get(6);

                let column = CdsContractsTablesColumns {
                    table_id,
                    column_id,
                    column_name,
                    column_type: column_type as u32,
                    column_length: column_length as u32,
                    column_ordinal: column_ordinal as u32,
                    is_nullable: is_nullable > 0,
                };

                cds_tables_columns.push(column);
            }
        }

        for contract in &cds_contracts {
            let hid = contract.host_id.clone();
            let sql = r#"
            SELECT 
                HOST_ID,
                HOST_NAME,
                TOKEN,
                IP4ADDRESS,
                IP6ADDRESS,
                DB_PORT,
                INFO_PORT,
                LAST_COMMUNICATION_UTC,
                HTTP_ADDR,
                HTTP_PORT,
                HOST_STATUS
            FROM
                :schema.CDS_HOSTS
            WHERE
                HOST_ID = ':hid';"#;
            let sql = sql.replace(":schema", &self.treaty_schema_name);
            let sql = sql.replace(":hid", &hid);
            let rows = self.query_db(&sql).await?;

            for row in &rows {
                let host_id: String = row.get(0);
                let host_name: String = row.get(1);
                let token: Vec<u8> = row.get(2);
                let ip4: String = row.get(3);
                let ip6: String = row.get(4);
                let db_port: i32 = row.get(5);
                let info_port: i32 = row.get(6);
                let last_comm_utc: NaiveDateTime = row.get(7);
                let http_addr: String = row.get(8);
                let http_port: i32 = row.get(9);
                let status: i32 = row.get(10);

                let host = CdsHosts {
                    host_id,
                    host_name,
                    token,
                    ip4,
                    ip6,
                    db_port: db_port as u32,
                    info_port: info_port as u32,
                    last_comm_utc: last_comm_utc.to_string(),
                    http_addr,
                    http_port: http_port as u32,
                    status: num::FromPrimitive::from_u32(status as u32).unwrap(),
                };

                cds_host_infos.push(host);
            }
        }

        for contract in &cds_contracts {
            let dbid = contract.database_id.clone();
            let tables = cds_tables
                .iter()
                .enumerate()
                .filter(|&(_, t)| t.database_id == dbid)
                .map(|(_, t)| t);

            let mut table_schema: Vec<TableSchema> = Vec::new();

            for t in tables {
                let mut col_schema: Vec<ColumnSchema> = Vec::new();

                let tid = t.table_id.clone();
                let cols = cds_tables_columns
                    .iter()
                    .enumerate()
                    .filter(|&(_, c)| c.table_id == tid)
                    .map(|(_, c)| c);

                for c in cols {
                    let cs = ColumnSchema {
                        column_name: c.column_name.clone(),
                        column_type: c.column_type,
                        column_length: c.column_length,
                        is_nullable: c.is_nullable,
                        ordinal: c.column_ordinal,
                        table_id: c.table_id.clone(),
                        column_id: c.column_id.clone(),
                        is_primary_key: false,
                    };
                    col_schema.push(cs);
                }

                let ts = TableSchema {
                    table_name: t.table_name.clone(),
                    table_id: t.table_id.clone(),
                    database_name: t.database_name.clone(),
                    database_id: t.database_id.clone(),
                    columns: col_schema,
                    logical_storage_policy: t.logical_storage_policy,
                };

                table_schema.push(ts);
            }

            let mut cooperation_enabled = false;
            let mut db_has_participants = false;

            if self
                .dbi()
                .has_enable_coooperative_features(&contract.database_name)
                .await?
            {
                cooperation_enabled = true;
            }

            if cooperation_enabled {
                let db = TreatyDb::new(
                    &self.host,
                    self.port,
                    &self.un,
                    &self.pw,
                    &self.treaty_schema_name,
                    &self.backing_db_name.as_ref().unwrap().clone(),
                    self.use_treaty_schema,
                );
                if db.has_participants().await? {
                    db_has_participants = true;
                }
            }

            let ds = DatabaseSchema {
                database_name: contract.database_name.clone(),
                database_id: contract.database_id.clone(),
                tables: table_schema,
                database_type: 0,
                treaty_database_type: 0,
                cooperation_enabled,
                has_participants: db_has_participants,
            };

            db_schema.push(ds);
        }

        for c in &cds_contracts {
            let dbs = db_schema
                .iter()
                .enumerate()
                .filter(|&(_, s)| s.database_id == c.database_id)
                .map(|(_, s)| s);

            let hi = cds_host_infos
                .iter()
                .enumerate()
                .filter(|&(_, h)| h.host_id == c.host_id)
                .map(|(_, h)| h);

            let h = hi.last().unwrap().clone();

            let n = HostNetwork {
                ip4_address: Some(h.ip4.clone()),
                ip6_address: Some(h.ip6.clone()),
                database_port_number: Some(h.db_port),
                http_addr: Some(h.http_addr.clone()),
                http_port: Some(h.http_port),
                info_port_number: Some(h.info_port),
            };

            let i = Host {
                host_guid: h.host_id.clone(),
                host_name: h.host_name.clone(),
                token: h.token.clone(),
                network: Some(n),
            };

            let pc = Contract {
                contract_guid: c.contract_id.clone(),
                description: c.description.clone(),
                schema: Some(dbs.last().unwrap().clone()),
                contract_version: c.contract_version_id.clone(),
                host_info: Some(i.clone()),
                status: num::ToPrimitive::to_u32(&c.contract_status).unwrap_or(0),
            };

            pending_contracts.push(pc);
        }

        trace!("[{}]: cds_contracts: {cds_contracts:#?}", function_name!());
        trace!(
            "[{}]: pending_contracts: {pending_contracts:#?}",
            function_name!()
        );

        if pending_contracts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(pending_contracts))
        }
    }

    pub async fn has_contract(&self, contract_id: &str) -> Result<bool, TreatyDbError> {
        let sql = "SELECT COUNT(*) CTRCOUNT FROM :schema.CDS_CONTRACTS WHERE CONTRACT_ID = ':cid';";
        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":cid", contract_id);
        let client = self.get_client_for_db().await?;
        self.has_any_rows(&sql, Some(client)).await
    }

    async fn has_login(&self, login: &str) -> Result<bool, TreatyDbError> {
        let sql =
            r#"SELECT COUNT(*) AS USERCOUNT FROM :schema.CDS_USER WHERE USERNAME = ':username';"#;
        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":username", login);

        let client = self.get_client_for_db().await?;
        let rows = client.query(&sql, &[]).await?;

        trace!("[{}]: {rows:?}", function_name!());

        let count: i64 = rows[0].get(0);

        trace!("[{}]: Total rows found: {}", function_name!(), count);

        Ok(count > 0)
    }

    async fn check_database_name_for_contract_format(
        &self,
        db_name: &str,
    ) -> Result<String, TreatyDbError> {
        let mut cmd = String::from(
            "SELECT COUNT(*) FROM :schema.CDS_CONTRACTS_TABLES WHERE DATABASE_NAME = ':db_name'",
        );
        cmd = cmd.replace(":db_name", &db_name);
        cmd = cmd.replace(":schema", &self.treaty_schema_name);

        if !self.has_any_rows(&cmd, None).await? {
            let message = format!(
                "{}{}",
                "WARNING: check_database_name_for_contract_format no database named: ", db_name
            );
            warn!("[{}]: {}", function_name!(), message);

            if db_name.contains(".dbpart") {
                let mut db_name = db_name.to_string();
                let message = "check_database_name_for_contract_format: renaming database name to contract version of database";
                info!("[{}]: {}", function_name!(), message);
                db_name = db_name.replace(".dbpart", ".db");
                let message = format!("New database name is: {db_name}");
                info!("[{}]: {}", function_name!(), message);
                return Ok(db_name);
            }
        }

        Ok(db_name.to_string())
    }

    pub async fn create_login(&self, login: &str, pw: &str) -> Result<bool, TreatyDbError> {
        let sql = "INSERT INTO :schema.CDS_USER (USERNAME, HASH) VALUES (':username', $1::BYTEA);";
        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":username", login);

        let client = self.get_client_for_db().await?;

        trace!("[{}]: SQL: {}", function_name!(), sql);

        let login_hash = crypt::hash(pw);
        let result = client
            .execute(&sql, &[&login_hash.0.as_bytes().to_vec()])
            .await?;

        Ok(result > 0)
    }

    pub async fn login_is_in_role(
        &self,
        login: &str,
        role_name: &str,
    ) -> Result<bool, TreatyDbError> {
        let sql = "SELECT COUNT(*) USERCOUNT FROM :schema.CDS_USER_ROLE WHERE USERNAME = ':username' AND ROLENAME = ':rolename';";
        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":rolename", role_name);
        let sql = sql.replace(":username", login);

        let client = self.get_client_for_db().await?;

        trace!("[{}]: SQL: {}", function_name!(), sql);

        let rows = client.query(&sql, &[]).await?;

        trace!("[{}]: {rows:?}", function_name!());

        let count: i64 = rows[0].get(0);

        trace!("[{}]: Total rows found: {}", function_name!(), count);

        Ok(count > 0)
    }

    pub async fn add_login_to_role(
        &self,
        login: &str,
        role_name: &str,
    ) -> Result<bool, TreatyDbError> {
        let sql = "INSERT INTO :schema.CDS_USER_ROLE (USERNAME, ROLENAME) VALUES (':username', ':rolename');";
        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":rolename", role_name);
        let sql = sql.replace(":username", login);

        trace!("[{}]: SQL: {}", function_name!(), sql);

        let client = self.get_client_for_db().await?;
        let result = client.execute(&sql, &[]).await?;

        Ok(result > 0)
    }

    pub async fn verify_login(&self, login: &str, pw: &str) -> Result<bool, TreatyDbError> {
        let mut is_verified = false;

        let sql = "SELECT USERNAME, HASH FROM :schema.CDS_USER WHERE USERNAME = ':un' ; ";
        let sql = sql.replace(":un", login);
        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":un", login);

        let client = self.get_client_for_db().await?;
        trace!("[{}]: SQL: {}", function_name!(), sql);

        let rows = client.query(&sql, &[]).await?;

        trace!("[{}]: {rows:?}", function_name!());

        let un: String = rows[0].get(0);
        let hash: Vec<u8> = rows[0].get(1);

        let mut padded = [0u8; 128];
        hash.iter().enumerate().for_each(|(i, val)| {
            padded[i] = *val;
        });

        if crypt::verify(padded, pw) {
            trace!("[{}] user is verified", function_name!());
            is_verified = true;
        }

        Ok(is_verified)
    }

    pub async fn create_token_for_login(
        &self,
        login: &str,
        timeout_in_minutes: Option<u32>,
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), TreatyDbError> {
        trace!("[{}]: login: {login:?}", function_name!());

        let host_info = self
            .treaty_get_host_info()
            .await
            .expect("no host info is set")
            .unwrap();

        trace!("[{}]: host_info: {host_info:?}", function_name!());
        let token_data = create_jwt(&host_info.name, login, timeout_in_minutes.unwrap());
        self.save_token(login, &token_data.0, token_data.1).await?;
        Ok(token_data)
    }

    pub async fn save_token(
        &self,
        login: &str,
        token: &str,
        expiration: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), TreatyDbError> {
        let sql = r#"INSERT INTO :schema.CDS_USER_TOKENS
        (
            USERNAME,
            TOKEN,
            ISSUED_UTC,
            EXPIRATION_UTC
        )
        VALUES
        (
            ':un',
            ':token',
            ':issued',
            ':expiration'
        )
        ;"#;

        let issued: String = Utc::now().to_rfc3339();
        let expiration = expiration.to_rfc3339();

        let sql = sql.replace(":un", login);
        let sql = sql.replace(":token", token);
        let sql = sql.replace(":schema", &self.treaty_schema_name);
        let sql = sql.replace(":issued", &issued);
        let sql = sql.replace(":expiration", &expiration);

        trace!("[{}]: SQL: {}", function_name!(), sql);

        let client = self.get_client_for_db().await?;

        client.execute(&sql, &[]).await?;

        Ok(())
    }

    pub async fn verify_token(&self, token: &str) -> Result<bool, TreatyDbError> {
        let sql = "SELECT COUNT(*) TOKENCOUNT FROM :schema.CDS_USER_TOKENS WHERE TOKEN = ':token';";
        let sql = sql.replace(":token", token);
        let sql = sql.replace(":schema", &self.treaty_schema_name);

        self.has_any_rows(&sql, None).await
    }

    pub async fn has_any_rows(
        &self,
        sql: &str,
        client: Option<tokio_postgres::Client>,
    ) -> Result<bool, TreatyDbError> {
        let client = match client {
            Some(c) => c,
            None => self.get_client_for_db().await?,
        };

        trace!("[{}]: SQL: {}", function_name!(), sql);

        let rows = client.query(sql, &[]).await?;

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

    pub async fn auth_for_token(
        &self,
        login: &str,
        pw: &str,
        timeout_in_minutes: Option<u32>,
    ) -> Result<TokenReply, TreatyDbError> {
        let mut is_authorized = false;
        let mut jwt = String::from("");
        let mut expiration_utc = String::from("");

        if self.verify_login(login, pw).await? {
            is_authorized = true;

            if !self.login_has_token(login).await? {
                let token_data = self
                    .create_token_for_login(login, timeout_in_minutes)
                    .await?;
                jwt = token_data.0;
                expiration_utc = token_data.1.to_string();
            }
        }

        Ok(TokenReply {
            is_successful: is_authorized,
            expiration_utc,
            jwt,
        })
    }

    pub async fn get_cooperative_hosts(&self) -> Result<Option<Vec<CdsHosts>>, TreatyDbError> {
        trace!("[{}]", function_name!());

        let mut cds_host_infos: Vec<CdsHosts> = Vec::new();

        let sql = r#"
        SELECT 
            HOST_ID,
            HOST_NAME,
            TOKEN,
            IP4ADDRESS,
            IP6ADDRESS,
            DB_PORT,
            INFO_PORT,
            LAST_COMMUNICATION_UTC,
            HTTP_ADDR,
            HTTP_PORT,
            HOST_STATUS
        FROM
            :schema.CDS_HOSTS
        ;"#;
        let sql = sql.replace(":schema", &self.treaty_schema_name);

        let rows = self.query_db(&sql).await?;

        if rows.is_empty() {
            return Ok(None);
        }

        for row in &rows {
            let host_id: String = row.get(0);
            let host_name: String = row.get(1);
            let token: Vec<u8> = row.get(2);
            let ip4: String = row.get(3);
            let ip6: String = row.get(4);
            let db_port: i32 = row.get(5);
            let info_port: i32 = row.get(6);
            let last_comm_utc: NaiveDateTime = row.get(7);
            let http_addr: String = row.get(8);
            let http_port: i32 = row.get(9);
            let status: i32 = row.get(10);

            let cds_host = CdsHosts {
                host_id,
                host_name,
                token,
                ip4,
                ip6,
                db_port: db_port as u32,
                info_port: info_port as u32,
                last_comm_utc: last_comm_utc.to_string(),
                http_addr,
                http_port: http_port as u32,
                status: HostStatus::from_u32(status as u32),
            };

            cds_host_infos.push(cds_host);
        }

        Ok(Some(cds_host_infos))
    }

    pub async fn revoke_token(&self, token: &str) -> Result<bool, TreatyDbError> {
        let mut cmd = String::from("DELETE FROM :schema.CDS_USER_TOKENS WHERE TOKEN = ':token'");
        cmd = cmd.replace(":token", token);
        cmd = cmd.replace(":schema", &self.treaty_schema_name);
        self.write_for_db(&cmd).await?;
        Ok(true)
    }

    pub async fn login_has_token(&self, login: &str) -> Result<bool, TreatyDbError> {
        self.delete_expired_tokens().await?;
        let mut cmd =
            String::from("SELECT COUNT(*) FROM :schema.CDS_USER_TOKENS WHERE USERNAME = ':login'");
        cmd = cmd.replace(":login", login);
        cmd = cmd.replace(":schema", &self.treaty_schema_name);
        let client = self.get_client_for_db().await?;
        self.has_any_rows(&cmd, Some(client)).await
    }

    pub async fn delete_expired_tokens(&self) -> Result<(), TreatyDbError> {
        let now = Utc::now().to_rfc3339();

        let mut cmd =
            String::from("DELETE FROM :schema.CDS_USER_TOKENS WHERE EXPIRATION_UTC < ':now'");
        cmd = cmd.replace(":now", &now);
        cmd = cmd.replace(":schema", &self.treaty_schema_name);

        self.write_for_db(&cmd).await?;
        Ok(())
    }

    async fn query_db(&self, sql: &str) -> Result<Vec<Row>, TreatyDbError> {
        let client = self.get_client_for_db().await?;
        trace!("[{}]: SQL: {}", function_name!(), sql);
        Ok(client.query(sql, &[]).await?)
    }

    #[allow(unused_assignments)]
    async fn read(
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

            return Ok(table);
        }

        Ok(Table::new())
    }
}
