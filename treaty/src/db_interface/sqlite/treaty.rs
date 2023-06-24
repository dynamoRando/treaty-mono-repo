use std::{fs, path::Path};

use chrono::Utc;
use rusqlite::{named_params, Connection};
use stdext::function_name;
use tracing::{debug, error, instrument, trace, warn};
use treaty_types::enums::*;
use crate::crypt;
use crate::db_interface::sqlite::TreatyDbError;

use crate::jwt::create_jwt;
use crate::models::{
    CdsContracts, CdsContractsTables, CdsContractsTablesColumns, CdsHosts, HostInfo,
    TreatySaveContractResult, User,
};
use crate::sql_text::Cds;
use crate::treaty_proto::{
    ColumnSchema, Contract, DatabaseSchema, Host, HostNetwork, Participant, TableSchema, TokenReply,
};

use super::db::Db;
use super::{
    check_database_name_for_contract_format, execute_write, get_scalar_as_string,
    get_scalar_as_u32, get_statement, has_any_rows, has_enable_coooperative_features,
};

#[derive(Debug, Clone)]
pub struct TreatyDb {
    db_name: String,
    dir: String,
}

impl TreatyDb {
    pub fn new(db_name: &str, dir: &str) -> Self {
        Self {
            db_name: db_name.to_string(),
            dir: dir.to_string(),
        }
    }

    pub fn has_role_name(&self, role_name: &str) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        let mut has_role = false;

        let cmd = &String::from(&Cds::text_get_role());
        let mut statement = conn.prepare(cmd)?;

        let rows = statement
            .query_map(&[(":rolename", role_name.to_string().as_str())], |row| {
                row.get(0)
            })?;

        for item in rows {
            let count: u64 = item.unwrap();
            if count > 0 {
                has_role = true;
            }
        }

        Ok(has_role)
    }

    pub fn get_cds_host_for_part_db(
        &self,
        db_name: &str,
    ) -> Result<Option<CdsHosts>, TreatyDbError> {
        let conn = self.conn()?;
        let mut cmd = String::from(
            "
        SELECT 
            HOST_ID
        FROM 
            CDS_CONTRACTS 
        WHERE 
            DATABASE_NAME = ':db_name'
        ;",
        );

        cmd = cmd.replace(":db_name", db_name);
        let host_id = get_scalar_as_string(cmd, &conn)?;

        let mut cds_host_infos: Vec<CdsHosts> = Vec::new();

        let cmd = String::from(
            "
            SELECT 
                HOST_ID,
                HOST_NAME,
                TOKEN,
                IP4ADDRESS,
                IP6ADDRESS,
                PORT,
                LAST_COMMUNICATION_UTC,
                HTTP_ADDR,
                HTTP_PORT,
                HOST_STATUS
            FROM
                CDS_HOSTS
            WHERE
                HOST_ID = :hid
        ;",
        );

        let mut statement = conn.prepare(&cmd).unwrap();

        let row_to_host = |host_id: String,
                           host_name: String,
                           token: Vec<u8>,
                           ip4: String,
                           ip6: String,
                           port: u32,
                           last_comm_utc: String,
                           http_addr: String,
                           http_port: u32,
                           status: u32|
         -> rusqlite::Result<CdsHosts> {
            let host = CdsHosts {
                host_id,
                host_name,
                token,
                ip4,
                ip6,
                port,
                last_comm_utc,
                http_addr,
                http_port,
                status: num::FromPrimitive::from_u32(status).unwrap_or(HostStatus::Unknown),
            };
            Ok(host)
        };

        let table_hosts = statement
            .query_and_then(&[(":hid", &host_id)], |row| {
                row_to_host(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    row.get(5).unwrap(),
                    row.get(6).unwrap(),
                    row.get(7).unwrap(),
                    row.get(8).unwrap(),
                    row.get(9).unwrap(),
                )
            })
            .unwrap();

        for h in table_hosts {
            cds_host_infos.push(h.unwrap());
        }

        trace!("[{}]: {cds_host_infos:?}", function_name!());

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

    pub fn get_database_names(&self) -> Result<Option<Vec<String>>, TreatyDbError> {
        let mut databases: Vec<String> = Vec::new();

        for file in fs::read_dir(&self.dir).unwrap() {
            let fname = file.unwrap().file_name();
            let name = fname.as_os_str().to_str().unwrap();

            if name.contains(".db") || name.contains(".dbpart") {
                databases.push(name.to_string());
            }
        }

        if databases.is_empty() {
            return Ok(None);
        }

        Ok(Some(databases))
    }

    pub fn get_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<DeletesFromHostBehavior, TreatyDbError> {
        let conn = self.conn()?;
        let db_name = check_database_name_for_contract_format(db_name, &conn)?;
        let mut cmd = String::from(
            "
            SELECT 
                DELETES_FROM_HOST_BEHAVIOR
            FROM
                CDS_CONTRACTS_TABLES 
            WHERE
                DATABASE_NAME = ':db_name'
            AND
                TABLE_NAME = ':table_name'
            ;",
        );
        cmd = cmd.replace(":db_name", &db_name);
        cmd = cmd.replace(":table_name", table_name);

        let result = get_scalar_as_u32(cmd, &conn)?;

        Ok(num::FromPrimitive::from_u32(result).unwrap_or(DeletesFromHostBehavior::Unknown))
    }

    pub fn get_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<UpdatesToHostBehavior, TreatyDbError> {
        let conn = self.conn()?;
        let db_name = check_database_name_for_contract_format(db_name, &conn)?;
        let mut cmd = String::from(
            "
            SELECT 
                UPDATES_TO_HOST_BEHAVIOR
            FROM
                CDS_CONTRACTS_TABLES 
            WHERE
                DATABASE_NAME = ':db_name'
            AND
                TABLE_NAME = ':table_name'
            ;",
        );
        cmd = cmd.replace(":db_name", &db_name);
        cmd = cmd.replace(":table_name", table_name);

        let result = get_scalar_as_u32(cmd, &conn)?;
        let behavior: UpdatesToHostBehavior = num::FromPrimitive::from_u32(result).unwrap();

        Ok(behavior)
    }

    pub fn change_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        let db_name = check_database_name_for_contract_format(db_name, &conn)?;
        let cmd = String::from(
            "
            UPDATE CDS_CONTRACTS_TABLES 
            SET UPDATES_TO_HOST_BEHAVIOR = :behavior 
            WHERE
                DATABASE_NAME = :db_name
            AND
                TABLE_NAME = :table_name
            ;",
        );

        let mut statement = conn.prepare(&cmd)?;
        let result = statement.execute(
            named_params! {":behavior": behavior, ":db_name" : db_name, ":table_name" : table_name},
        )?;

        Ok(result > 0)
    }

    pub fn change_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        let db_name = check_database_name_for_contract_format(db_name, &conn)?;
        let cmd = String::from(
            "
            UPDATE CDS_CONTRACTS_TABLES 
            SET DELETES_TO_HOST_BEHAVIOR = :behavior 
            WHERE
                DATABASE_NAME = :db_name
            AND
                TABLE_NAME = :table_name
            ;",
        );

        let mut statement = conn.prepare(&cmd)?;
        let result = statement.execute(
            named_params! {":behavior": behavior, ":db_name" : db_name, ":table_name" : table_name},
        )?;

        Ok(result > 0)
    }

    #[instrument]
    pub fn verify_host_by_name(
        &self,
        host_name: &str,
        token: Vec<u8>,
    ) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;

        let mut cmd = String::from(
            "
    SELECT TOKEN FROM CDS_HOSTS WHERE HOST_NAME = ':name' AND HOST_STATUS = 1",
        );
        cmd = cmd.replace(":name", host_name);

        let mut statement = conn.prepare(&cmd)?;

        let mut returned_tokens: Vec<Vec<u8>> = Vec::new();

        let row_to_token = |token: Vec<u8>| -> rusqlite::Result<Vec<u8>> { Ok(token) };

        let tokens = statement.query_and_then([], |row| row_to_token(row.get(0).unwrap()))?;

        for t in tokens {
            returned_tokens.push(t.unwrap());
        }

        if returned_tokens.is_empty() {
            Ok(false)
        } else {
            if returned_tokens.len() > 1 {
                warn!("[{}]: multiple tokens found!", function_name!());
            }

            Ok(do_vecs_match(&token, returned_tokens.last().unwrap()))
        }
    }

    pub fn get_contracts_by_status(
        &self,
        contract_status: ContractStatus,
    ) -> Result<Option<Vec<Contract>>, TreatyDbError> {
        let conn = self.conn()?;

        let u32_contract_status = num::ToPrimitive::to_u32(&contract_status).unwrap();

        let cmd = String::from(
            "
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
                CDS_CONTRACTS 
            WHERE 
                CONTRACT_STATUS = :u32_contract_status",
        );

        let mut statement = conn.prepare(&cmd).unwrap();

        let mut pending_contracts: Vec<Contract> = Vec::new();

        let mut cds_contracts: Vec<CdsContracts> = Vec::new();
        let mut cds_tables: Vec<CdsContractsTables> = Vec::new();
        let mut cds_tables_columns: Vec<CdsContractsTablesColumns> = Vec::new();

        let row_to_contract = |host_id: String,
                               contract_id: String,
                               contract_version_id: String,
                               database_name: String,
                               database_id: String,
                               description: String,
                               gen_date: String,
                               status: u32|
         -> rusqlite::Result<CdsContracts> {
            let cds_contract = CdsContracts {
                host_id,
                contract_id,
                contract_version_id,
                database_name,
                database_id,
                description,
                generated_date: gen_date,
                contract_status: ContractStatus::from_u32(status),
            };

            Ok(cds_contract)
        };

        let contract_metadata = statement.query_and_then(
            &[(":u32_contract_status", &u32_contract_status.to_string())],
            |row| {
                row_to_contract(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    row.get(5).unwrap(),
                    row.get(6).unwrap(),
                    row.get(7).unwrap(),
                )
            },
        )?;

        for c in contract_metadata {
            cds_contracts.push(c.unwrap());
        }

        for cdata in &cds_contracts {
            let dbid = cdata.database_id.clone();

            let cmd = String::from(
                "
            SELECT 
                DATABASE_ID,
                DATABASE_NAME,
                TABLE_ID,
                TABLE_NAME,
                LOGICAL_STORAGE_POLICY
            FROM 
                CDS_CONTRACTS_TABLES 
            WHERE 
                DATABASE_ID = :dbid",
            );

            let row_to_table = |database_id: String,
                                database_name: String,
                                table_id: String,
                                table_name: String,
                                logical_storage_policy: u32|
             -> rusqlite::Result<CdsContractsTables> {
                let table = CdsContractsTables {
                    database_id,
                    database_name,
                    table_id,
                    table_name,
                    logical_storage_policy,
                };
                Ok(table)
            };

            statement = conn.prepare(&cmd).unwrap();

            let table_metadata = statement.query_and_then(&[(":dbid", &dbid)], |row| {
                row_to_table(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                )
            })?;

            for table in table_metadata {
                cds_tables.push(table.unwrap());
            }
        }

        for table in &cds_tables {
            let tid = table.table_id.clone();

            let cmd = String::from(
                "
            SELECT 
                TABLE_ID,
                COLUMN_ID,
                COLUMN_NAME,
                COLUMN_TYPE,
                COLUMN_LENGTH,
                COLUMN_ORDINAL,
                IS_NULLABLE 
            FROM 
                CDS_CONTRACTS_TABLE_SCHEMAS 
            WHERE 
                TABLE_ID = :tid",
            );

            statement = conn.prepare(&cmd)?;

            let row_to_column = |table_id: String,
                                 column_id: String,
                                 column_name: String,
                                 column_type: u32,
                                 column_length: u32,
                                 column_ordinal: u32,
                                 is_nullable: bool|
             -> rusqlite::Result<CdsContractsTablesColumns> {
                let col = CdsContractsTablesColumns {
                    table_id,
                    column_id,
                    column_name,
                    column_type,
                    column_length,
                    column_ordinal,
                    is_nullable,
                };
                Ok(col)
            };

            let table_columns = statement.query_and_then(&[(":tid", &tid)], |row| {
                row_to_column(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    row.get(5).unwrap(),
                    row.get(6).unwrap(),
                )
            })?;

            for column in table_columns {
                cds_tables_columns.push(column.unwrap());
            }
        }

        let mut cds_host_infos: Vec<CdsHosts> = Vec::new();

        let cmd = String::from(
            "
            SELECT 
                HOST_ID,
                HOST_NAME,
                TOKEN,
                IP4ADDRESS,
                IP6ADDRESS,
                PORT,
                LAST_COMMUNICATION_UTC,
                HTTP_ADDR,
                HTTP_PORT,
                HOST_STATUS
            FROM
                CDS_HOSTS
            WHERE
                HOST_ID = :hid
        ;",
        );

        statement = conn.prepare(&cmd)?;

        let row_to_host = |host_id: String,
                           host_name: String,
                           token: Vec<u8>,
                           ip4: String,
                           ip6: String,
                           port: u32,
                           last_comm_utc: String,
                           http_addr: String,
                           http_port: u32,
                           status: u32|
         -> rusqlite::Result<CdsHosts> {
            let host = CdsHosts {
                host_id,
                host_name,
                token,
                ip4,
                ip6,
                port,
                last_comm_utc,
                http_addr,
                http_port,
                status: num::FromPrimitive::from_u32(status).unwrap(),
            };
            Ok(host)
        };

        for c in &cds_contracts {
            let h = c.host_id.clone();

            let table_hosts = statement.query_and_then(&[(":hid", &h)], |row| {
                row_to_host(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    row.get(5).unwrap(),
                    row.get(6).unwrap(),
                    row.get(7).unwrap(),
                    row.get(8).unwrap(),
                    row.get(9).unwrap(),
                )
            })?;

            for h in table_hosts {
                cds_host_infos.push(h.unwrap());
            }
        }

        let mut db_schema: Vec<DatabaseSchema> = Vec::new();

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

            if let Ok(is_enabled) =
                has_enable_coooperative_features(&contract.database_name, &self.dir)
            {
                cooperation_enabled = is_enabled;
            }

            if cooperation_enabled {
                let treaty = TreatyDb::new(&self.db_name, &self.dir);
                let db = Db::new(&contract.database_name, &self.dir, &treaty);
                if let Ok(x) = db.has_participants() {
                    db_has_participants = x;
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
                database_port_number: Some(h.port),
                http_addr: Some(h.http_addr.clone()),
                http_port: Some(h.http_port),
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

        if pending_contracts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(pending_contracts))
        }
    }

    /// checks treaty_db's CDS_CONTRACTS table to see if there already is a record
    /// for this contract by contract_id
    fn has_contract(&self, contract_id: &str) -> Result<bool, TreatyDbError> {
        let mut cmd = String::from(
            "SELECT COUNT(*) TOTALCOUNT FROM CDS_CONTRACTS WHERE CONTRACT_ID = ':cid'",
        );
        cmd = cmd.replace(":cid", contract_id);

        has_any_rows(cmd, &self.conn()?)
    }

    fn create_user_tokens_table(conn: &Connection) {
        conn.execute(&Cds::text_create_user_tokens_table(), [])
            .unwrap();
    }

    fn create_user_table(conn: &Connection) {
        conn.execute(&Cds::text_create_user_table(), []).unwrap();
    }

    fn create_role_table(conn: &Connection) {
        conn.execute(&Cds::text_create_role_table(), []).unwrap();
    }

    fn create_user_role_table(conn: &Connection) {
        conn.execute(&Cds::text_create_user_role_table(), [])
            .unwrap();
    }

    fn create_host_info_table(conn: &Connection) {
        trace!("creating CDS_HOST_INFO on: {conn:?}");

        conn.execute(&Cds::text_create_host_info_table(), [])
            .unwrap();
    }

    fn create_contracts_table(conn: &Connection) {
        conn.execute(&Cds::text_create_cds_contracts_table(), [])
            .unwrap();
    }

    fn create_contracts_table_table(conn: &Connection) {
        conn.execute(&Cds::text_create_cds_contracts_tables_table(), [])
            .unwrap();
    }

    fn create_contracts_table_table_schemas(conn: &Connection) {
        conn.execute(&Cds::text_create_cds_contracts_tables_schemas_table(), [])
            .unwrap();
    }

    fn create_cds_hosts_table(conn: &Connection) {
        conn.execute(&Cds::text_create_cds_hosts_table(), [])
            .unwrap();
    }

    pub fn configure_treaty_db(&self) -> Result<(), TreatyDbError> {
        let root = &self.dir;
        let db_name = &self.db_name;

        trace!(
            "[{}]: cwd is {}, db_name is {}",
            function_name!(),
            &root,
            &db_name
        );

        let db_path = Path::new(&root).join(db_name);
        trace!(
            "[{}]: db_path is {}",
            function_name!(),
            db_path.as_os_str().to_str().unwrap()
        );

        if !db_path.exists() {
            let db_conn = Connection::open(&db_path).unwrap();
            Self::create_user_table(&db_conn);
            Self::create_role_table(&db_conn);
            Self::create_user_role_table(&db_conn);
            Self::create_host_info_table(&db_conn);
            Self::create_contracts_table(&db_conn);
            Self::create_cds_hosts_table(&db_conn);
            Self::create_contracts_table_table(&db_conn);
            Self::create_contracts_table_table_schemas(&db_conn);
            Self::create_user_tokens_table(&db_conn);

            let db_has_role = self.has_role_name(&String::from("SysAdmin"))?;

            if !db_has_role {
                let statement =
                    String::from("INSERT INTO CDS_ROLE (ROLENAME) VALUES ('SysAdmin');");
                self.execute_write_on_connection(db_name, &statement)?;
            }

            Ok(())
        } else {
            trace!("[{}]: dir already exists: {db_path:?}", function_name!());
            trace!(
                "[{}]: this can happen if treaty is running via a proxy",
                function_name!()
            );

            Ok(())
        }
    }

    pub fn configure_admin(&self, login: &str, pw: &str) -> Result<(), TreatyDbError> {
        if !self.has_login(login)? {
            self.create_login(login, pw)?;
        }

        if !self.login_is_in_role(login, &String::from("SysAdmin"))? {
            self.add_login_to_role(login, &String::from("SysAdmin"))?;
        }

        Ok(())
    }

    pub fn has_login(&self, login: &str) -> Result<bool, TreatyDbError> {
        let mut has_login = false;
        let cmd =
            &String::from("SELECT count(*) AS USERCOUNT FROM CDS_USER WHERE USERNAME = :username");

        let conn = self.conn()?;
        let mut statement = conn.prepare(cmd).unwrap();

        let rows = statement.query_map([(login.to_string().as_str())], |row| row.get(0))?;

        for item in rows {
            let count: u64 = item.unwrap();
            if count > 0 {
                has_login = true;
            }
        }

        Ok(has_login)
    }

    pub fn save_contract(
        &self,
        contract: Contract,
    ) -> Result<TreatySaveContractResult, TreatyDbError> {
        let conn = self.conn()?;

        trace!("save_contract called with {:?}", contract);

        if !self.has_contract(&contract.contract_guid)? {
            self.save_contract_metadata(&contract)?;
            self.save_contract_table_data(&contract)?;
            self.save_contract_table_schema_data(&contract)?;
            self.save_contract_host_data(&contract)?;

            Ok(TreatySaveContractResult {
                is_successful: true,
                contract_status: ContractStatus::from_u32(contract.status),
                participant_information: None,
            })
        } else {
            warn!(
                "contract already exists for: {} version: {}",
                contract.contract_guid, contract.contract_version
            );

            let cmd = "SELECT COUNT(*) cnt from CDS_CONTRACTS WHERE CONTRACT_ID = ':cid' AND CONTRACT_VERSION_ID = ':vid'";
            let cmd = cmd
                .replace(":cid", &contract.contract_guid)
                .replace(":vid", &contract.contract_version);
            if has_any_rows(cmd, &conn)? {
                let cmd = "SELECT CONTRACT_STATUS FROM CDS_CONTRACTS WHERE CONTRACT_ID = ':cid' AND CONTRACT_VERSION_ID = ':vid'";
                let cmd = cmd
                    .replace(":cid", &contract.contract_guid)
                    .replace(":vid", &contract.contract_version);

                let status = get_scalar_as_u32(cmd, &conn)?;
                let contract_status = ContractStatus::from_u32(status);

                if contract_status == ContractStatus::Accepted {
                    debug!(
                        "[{}]: contract was already accepted, sending back acceptance info",
                        function_name!()
                    );

                    let host_info = self.treaty_get_host_info().unwrap();
                    if let Some(host_info) = host_info {
                        let participant_information = Some(Participant {
                            participant_guid: host_info.id.clone(),
                            alias: host_info.name.clone(),
                            ip4_address: "".to_string(),
                            ip6_address: String::from(""),
                            database_port_number: 0,
                            token: host_info.token,
                            internal_participant_guid: "".to_string(),
                            http_addr: "".to_string(),
                            http_port: 0,
                        });

                        let result = TreatySaveContractResult {
                            is_successful: false,
                            contract_status,
                            participant_information,
                        };

                        debug!("{result:?}");
                        Ok(result)
                    } else {
                        let result = TreatySaveContractResult {
                            is_successful: false,
                            contract_status,
                            participant_information: None,
                        };
                        debug!("{result:?}");
                        Ok(result)
                    }
                } else {
                    let result = TreatySaveContractResult {
                        is_successful: false,
                        contract_status,
                        participant_information: None,
                    };
                    debug!("{result:?}");
                    Ok(result)
                }
            } else {
                let result = TreatySaveContractResult {
                    is_successful: false,
                    contract_status: ContractStatus::Unknown,
                    participant_information: None,
                };
                debug!("{result:?}");
                Ok(result)
            }
        }
    }

    pub fn accept_pending_contract(&self, host_name: &str) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;

        let mut cmd = String::from("SELECT HOST_ID FROM CDS_HOSTS WHERE HOST_NAME = ':hostname'");
        cmd = cmd.replace(":hostname", host_name);

        let db_host_id = get_scalar_as_string(cmd, &conn)?;
        cmd = String::from(
            "SELECT COUNT(*) TOTALCOUNT FROM CDS_CONTRACTS WHERE HOST_ID = ':hid'
        AND CONTRACT_STATUS = 2",
        );
        cmd = cmd.replace(":hid", &db_host_id);

        let has_pending_contract = has_any_rows(cmd, &conn)?;

        if has_pending_contract {
            // 1 - we need to update the treaty_db record that we are accepting this contract
            // 2 - then we actually need to create the database with the properties of the
            // contract
            // 3 - we need to notify the host that we have accepted the contract

            cmd = String::from(
                "SELECT CONTRACT_ID FROM CDS_CONTRACTS WHERE HOST_ID = ':hid' AND CONTRACT_STATUS = 2",
            );
            cmd = cmd.replace(":hid", &db_host_id);

            let cid = get_scalar_as_string(cmd, &conn)?;

            cmd = String::from(
                "UPDATE CDS_CONTRACTS SET CONTRACT_STATUS = 3 WHERE CONTRACT_ID = ':cid'",
            );
            cmd = cmd.replace(":cid", &cid);

            let total_count = execute_write(&conn, &cmd)?;
            return Ok(total_count > 0);
        }

        Ok(false)
    }

    pub fn verify_host_by_id(&self, host_id: &str, token: Vec<u8>) -> Result<bool, TreatyDbError> {
        trace!("[{}]: host_id: {host_id}", function_name!());

        let conn = self.conn()?;

        let mut cmd = String::from(
            "
        SELECT TOKEN FROM CDS_HOSTS WHERE HOST_ID = ':hid' AND HOST_STATUS = 1",
        );
        cmd = cmd.replace(":hid", host_id);

        let mut statement = conn.prepare(&cmd)?;

        let mut returned_tokens: Vec<Vec<u8>> = Vec::new();

        let row_to_token = |token: Vec<u8>| -> rusqlite::Result<Vec<u8>> { Ok(token) };

        let tokens = statement.query_and_then([], |row| row_to_token(row.get(0).unwrap()))?;

        for t in tokens {
            returned_tokens.push(t.unwrap());
        }

        if returned_tokens.is_empty() {
            Ok(false)
        } else {
            if returned_tokens.len() > 1 {
                warn!("[{}]: multiple tokens found!", function_name!());
            }

            Ok(do_vecs_match(&token, returned_tokens.last().unwrap()))
        }
    }

    pub fn change_host_status_by_name(
        &self,
        host_name: &str,
        status: u32,
    ) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;

        let cmd = String::from(
            "
        UPDATE CDS_HOSTS SET HOST_STATUS = :status WHERE HOST_NAME = :name",
        );
        let mut statement = conn.prepare(&cmd)?;
        let result = statement.execute(named_params! {":status": status, ":name" : host_name})?;

        Ok(result > 0)
    }

    pub fn change_host_status_by_id(
        &self,
        host_id: &str,
        status: u32,
    ) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;

        let cmd = String::from(
            "
        UPDATE CDS_HOSTS SET HOST_STATUS = :status WHERE HOST_ID = ':hid'",
        );
        let mut statement = conn.prepare(&cmd)?;
        let result = statement.execute(named_params! {":status": status, ":hid" : host_id})?;

        Ok(result > 0)
    }

    pub fn change_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        let db_name = check_database_name_for_contract_format(db_name, &conn)?;
        let cmd = String::from(
            "
            UPDATE CDS_CONTRACTS_TABLES 
            SET DELETES_FROM_HOST_BEHAVIOR = :behavior 
            WHERE
                DATABASE_NAME = :db_name
            AND
                TABLE_NAME = :table_name
            ;",
        );

        let mut statement = conn.prepare(&cmd)?;
        let result = statement.execute(
            named_params! {":behavior": behavior, ":db_name" : db_name, ":table_name" : table_name},
        )?;

        Ok(result > 0)
    }

    pub fn change_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        let db_name = check_database_name_for_contract_format(db_name, &conn)?;

        let cmd = String::from(
            "
            UPDATE CDS_CONTRACTS_TABLES 
            SET UPDATES_FROM_HOST_BEHAVIOR = :behavior 
            WHERE
                DATABASE_NAME = :db_name
            AND
                TABLE_NAME = :table_name
            ;",
        );

        let mut statement = conn.prepare(&cmd)?;
        let result = statement.execute(
            named_params! {":behavior": behavior, ":db_name" : db_name, ":table_name" : table_name},
        )?;

        Ok(result > 0)
    }

    pub fn get_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<UpdatesFromHostBehavior, TreatyDbError> {
        let conn = self.conn()?;
        let db_name = check_database_name_for_contract_format(db_name, &conn)?;
        let mut cmd = String::from(
            "
            SELECT 
                UPDATES_FROM_HOST_BEHAVIOR
            FROM
                CDS_CONTRACTS_TABLES 
            WHERE
                DATABASE_NAME = ':db_name'
            AND
                TABLE_NAME = ':table_name'
            ;",
        );
        cmd = cmd.replace(":db_name", &db_name);
        cmd = cmd.replace(":table_name", table_name);

        let result = get_scalar_as_u32(cmd, &conn)?;

        Ok(UpdatesFromHostBehavior::from_u32(result))
    }

    pub fn get_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<DeletesToHostBehavior, TreatyDbError> {
        let conn = self.conn()?;
        let db_name = check_database_name_for_contract_format(db_name, &conn)?;
        let mut cmd = String::from(
            "
            SELECT 
                DELETES_TO_HOST_BEHAVIOR
            FROM
                CDS_CONTRACTS_TABLES 
            WHERE
                DATABASE_NAME = ':db_name'
            AND
                TABLE_NAME = ':table_name'
            ;",
        );
        cmd = cmd.replace(":db_name", &db_name);
        cmd = cmd.replace(":table_name", table_name);

        let result = get_scalar_as_u32(cmd, &conn)?;

        Ok(num::FromPrimitive::from_u32(result).unwrap_or(DeletesToHostBehavior::Unknown))
    }

    pub fn delete_expired_tokens(&self) -> Result<(), TreatyDbError> {
        let conn = self.conn()?;

        let now = Utc::now().to_rfc3339();

        let mut cmd = String::from("DELETE FROM CDS_USER_TOKENS WHERE EXPIRATION_UTC < ':now'");
        cmd = cmd.replace(":now", &now);

        let _ = conn.execute(&cmd, []);
        let _ = conn.close();

        Ok(())
    }

    pub fn get_treaty_db_type(&self, db_name: &str) -> Result<TreatyDatabaseType, TreatyDbError> {
        if db_name == self.db_name {
            return Ok(TreatyDatabaseType::System);
        }

        if db_name.contains("dbpart") {
            return Ok(TreatyDatabaseType::Partial);
        }

        let mut db_part_name = db_name.replace(".db", "");
        db_part_name = db_part_name.replace(".dbpart", "");
        db_part_name = format!("{}{}", db_part_name, String::from(".dbpart"));
        let db_path = Path::new(&self.dir).join(&db_part_name);
        if db_path.exists() {
            return Ok(TreatyDatabaseType::Partial);
        }

        let path = Path::new(&self.dir).join(db_name);
        if path.exists() {
            return Ok(TreatyDatabaseType::Host);
        }

        Ok(TreatyDatabaseType::Unknown)
    }

    pub fn revoke_token(&self, token: &str) -> Result<bool, TreatyDbError> {
        let mut cmd = String::from("DELETE FROM CDS_USER_TOKENS WHERE TOKEN = ':token'");
        cmd = cmd.replace(":token", token);
        let conn = self.conn()?;
        let result = execute_write(&conn, &cmd)?;
        Ok(result > 0)
    }

    pub fn login_has_token(&self, login: &str) -> Result<bool, TreatyDbError> {
        self.delete_expired_tokens()?;
        let conn = &self.conn()?;
        let mut cmd =
            String::from("SELECT COUNT(*) FROM CDS_USER_TOKENS WHERE USERNAME = ':login'");
        cmd = cmd.replace(":login", login);
        has_any_rows(cmd, conn)
    }

    pub fn create_login(&self, login: &str, pw: &str) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        // https://www.reddit.com/r/rust/comments/2sipzj/is_there_an_easy_way_to_hash_passwords_in_rust/
        // https://blue42.net/code/rust/examples/sodiumoxide-password-hashing/post/

        let login_hash = crypt::hash(pw);
        let cmd = Cds::text_add_user();
        let mut statement = conn.prepare(&cmd).unwrap();
        statement
            .execute(
                named_params! { ":username": login, ":hash": login_hash.0.as_bytes().to_vec() },
            )
            .unwrap();

        Ok(true)
    }

    pub fn create_login_with_hash(&self, login: &str, hash: Vec<u8>) -> Result<(), TreatyDbError> {
        let conn = self.conn()?;
        let cmd = Cds::text_add_user();
        let mut statement = conn.prepare(&cmd).unwrap();
        statement.execute(named_params! { ":username": login, ":hash": hash })?;

        Ok(())
    }

    pub fn verify_login(&self, login: &str, pw: &str) -> Result<bool, TreatyDbError> {
        let mut is_verified = false;

        let cmd = Cds::text_get_user();
        let conn = self.conn()?;

        let mut statement = get_statement(&cmd, &conn)?;

        let user_iter = statement
            .query_map([login.to_string().as_str()], |row| {
                Ok(User {
                    username: row.get(0).unwrap(),
                    hash: row.get(1).unwrap(),
                })
            })
            .unwrap();

        for user in user_iter {
            let returned_value = user?;

            let mut padded = [0u8; 128];
            returned_value.hash.iter().enumerate().for_each(|(i, val)| {
                padded[i] = *val;
            });

            if crypt::verify(padded, pw) {
                is_verified = true;
                break;
            }
        }

        trace!(
            "[{}]: {is_verified:?} for login: {login:?} ",
            function_name!()
        );

        Ok(is_verified)
    }

    pub fn configure_admin_hash(&self, login: &str, hash: Vec<u8>) -> Result<(), TreatyDbError> {
        self.create_login_with_hash(login, hash)
    }

    pub fn auth_for_token(&self, login: &str, pw: &str) -> Result<TokenReply, TreatyDbError> {
        let mut is_authorized = false;
        let mut jwt = String::from("");
        let mut expiration_utc = String::from("");

        if self.verify_login(login, pw)? {
            is_authorized = true;

            if !self.login_has_token(login)? {
                let token_data = self.create_token_for_login(login)?;
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

    pub fn save_token(
        &self,
        login: &str,
        token: &str,
        expiration: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), TreatyDbError> {
        let conn = self.conn()?;

        let cmd = String::from(
            "
            INSERT INTO CDS_USER_TOKENS
            (
                USERNAME,
                TOKEN,
                ISSUED_UTC,
                EXPIRATION_UTC
            )
            VALUES
            (
                :un,
                :token,
                :issued,
                :expiration
            );",
        );

        let issued = Utc::now().to_rfc3339();
        let expiration = expiration.to_rfc3339();

        let mut statement = get_statement(&cmd, &conn)?;

        statement.execute(named_params! {
            ":un" : login.to_string(),
            ":token" : token,
            ":issued" : issued,
            ":expiration" : expiration,
        })?;

        Ok(())
    }

    pub fn get_cooperative_hosts(&self) -> Result<Option<Vec<CdsHosts>>, TreatyDbError> {
        let mut cds_host_infos: Vec<CdsHosts> = Vec::new();

        let conn = self.conn()?;
        let cmd = String::from(
            "
    SELECT 
        HOST_ID,
        HOST_NAME,
        TOKEN,
        IP4ADDRESS,
        IP6ADDRESS,
        PORT,
        LAST_COMMUNICATION_UTC,
        HTTP_ADDR,
        HTTP_PORT,
        HOST_STATUS
    FROM
        CDS_HOSTS
    ;",
        );

        let mut statement = get_statement(&cmd, &conn)?;

        let row_to_host = |host_id: String,
                           host_name: String,
                           token: Vec<u8>,
                           ip4: String,
                           ip6: String,
                           port: u32,
                           last_comm_utc: String,
                           http_addr: String,
                           http_port: u32,
                           status: u32|
         -> rusqlite::Result<CdsHosts> {
            let host = CdsHosts {
                host_id,
                host_name,
                token,
                ip4,
                ip6,
                port,
                last_comm_utc,
                http_addr,
                http_port,
                status: num::FromPrimitive::from_u32(status).unwrap_or(HostStatus::Unknown),
            };
            Ok(host)
        };

        let table_hosts = statement
            .query_and_then([], |row| {
                row_to_host(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    row.get(5).unwrap(),
                    row.get(6).unwrap(),
                    row.get(7).unwrap(),
                    row.get(8).unwrap(),
                    row.get(9).unwrap(),
                )
            })
            .unwrap();

        for h in table_hosts {
            cds_host_infos.push(h.unwrap());
        }

        if cds_host_infos.is_empty() {
            Ok(None)
        } else {
            Ok(Some(cds_host_infos))
        }
    }

    pub fn verify_token(&self, token: &str) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        let mut cmd = String::from("SELECT COUNT(*) FROM CDS_USER_TOKENS WHERE TOKEN = ':token'");
        cmd = cmd.replace(":token", token);
        has_any_rows(cmd, &conn)
    }

    pub fn treaty_get_host_info(&self) -> Result<Option<HostInfo>, TreatyDbError> {
        let conn = self.conn()?;
        let cmd = String::from(
            "
    SELECT 
        HOST_ID, 
        HOST_NAME, 
        TOKEN 
    FROM 
        CDS_HOST_INFO;",
        );

        let row_to_host_info =
            |host_id: String, host_name: String, token: String| -> rusqlite::Result<HostInfo> {
                let host = HostInfo {
                    id: host_id,
                    name: host_name,
                    token: token.as_bytes().to_vec(),
                };

                Ok(host)
            };

        let mut results: Vec<HostInfo> = Vec::new();

        let mut statement = conn.prepare(&cmd).unwrap();
        let host_infos = statement
            .query_and_then([], |row| {
                row_to_host_info(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                )
            })
            .unwrap();

        for hi in host_infos {
            results.push(hi.unwrap());
        }

        if !results.is_empty() {
            return Ok(Some(results.first().unwrap().clone()));
        }

        Ok(None)
    }

    pub fn create_token_for_login(
        &self,
        login: &str,
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), TreatyDbError> {
        let host_info = self
            .treaty_get_host_info()
            .expect("no host info is set")
            .unwrap();
        let token_data = create_jwt(&host_info.name, login);
        self.save_token(login, &token_data.0, token_data.1)?;
        Ok(token_data)
    }

    fn conn(&self) -> Result<Connection, TreatyDbError> {
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

    fn execute_write_on_connection(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<usize, TreatyDbError> {
        let db_path = Path::new(&self.dir).join(db_name);
        let result = Connection::open(db_path);
        match result {
            Ok(c) => {
                let result = c.execute(cmd, [])?;
                Ok(result)
            }
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                Err(e.into())
            }
        }
    }

    pub fn login_is_in_role(&self, login: &str, role_name: &str) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        let mut login_is_in_role = false;
        let cmd = &Cds::text_get_user_role();
        let mut statement = get_statement(cmd, &conn)?;

        let params = [(":username", login), (":rolename", role_name)];

        let rows = statement.query_map(&params, |row| row.get(0))?;

        for item in rows {
            let count: u64 = item.unwrap();
            if count > 0 {
                login_is_in_role = true;
            }
        }

        Ok(login_is_in_role)
    }

    pub fn add_login_to_role(&self, login: &str, role_name: &str) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        let cmd = &String::from(&Cds::text_add_user_role());
        let mut statement = conn.prepare(cmd).unwrap();
        statement.execute(named_params! { ":username": login, ":rolename": role_name })?;

        Ok(true)
    }

    /// saves top level contract data to treaty_db's CDS_CONTRACTS table
    fn save_contract_metadata(&self, contract: &Contract) -> Result<(), TreatyDbError> {
        let host = contract.host_info.as_ref().unwrap().clone();
        let db = contract.schema.as_ref().unwrap();

        let cmd = String::from(
            "INSERT INTO CDS_CONTRACTS
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
        :hid,
        :cid,
        :cvid,
        :dbname,
        :dbid,
        :desc,
        :gdutc,
        :status
    )
    ;",
        );

        let conn = self.conn()?;
        let mut statement = conn.prepare(&cmd).unwrap();
        statement.execute(named_params! {
            ":hid": host.host_guid,
            ":cid" : contract.contract_guid,
            ":cvid" : contract.contract_version,
            ":dbname" : db.database_name,
            ":dbid" : db.database_id,
            ":desc" : contract.description,
            ":gdutc" : Utc::now().to_string(),
            ":status" : contract.status.to_string()
        })?;

        Ok(())
    }

    /// save's a contract's table schema information to CDS_CONTRACTS_TABLE_SCHEMAS
    fn save_contract_table_schema_data(&self, contract: &Contract) -> Result<(), TreatyDbError> {
        let conn = self.conn()?;
        let tables = contract.schema.as_ref().unwrap().tables.clone();

        for table in &tables {
            let cmd = String::from(
                "INSERT INTO CDS_CONTRACTS_TABLE_SCHEMAS
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
            :tid,
            :cid,
            :cname,
            :ctype,
            :clength,
            :cordinal,
            :is_nullable
        )
        ;",
            );

            let tid = table.table_id.clone();
            for column in &table.columns {
                let cid = column.column_id.clone();
                let cname = column.column_name.clone();
                let ctype = column.column_type;
                let clength = column.column_length;
                let cordinal = column.ordinal;
                let is_nullable = i32::from(column.is_nullable);

                let mut statement = conn.prepare(&cmd).unwrap();
                statement.execute(named_params! {
                    ":tid": tid,
                    ":cid" : cid,
                    ":cname" : cname,
                    ":ctype" : ctype,
                    ":clength" : clength,
                    ":cordinal" : cordinal,
                    ":is_nullable" : is_nullable,
                })?;
            }
        }

        Ok(())
    }

    // save a contract's host information to CDS_HOSTS
    fn save_contract_host_data(&self, contract: &Contract) -> Result<(), TreatyDbError> {
        trace!("[{}]: {contract:?}", function_name!());

        let cmd = String::from(
            "INSERT INTO CDS_HOSTS
    (
        HOST_ID,
        HOST_NAME,
        TOKEN,
        IP4ADDRESS,
        IP6ADDRESS,
        PORT,
        LAST_COMMUNICATION_UTC,
        HOST_STATUS,
        HTTP_ADDR,
        HTTP_PORT
    )
    VALUES
    (
        :hid,
        :hname,
        :token,
        :ip4,
        :ip6,
        :port,
        :last_comm,
        1,
        :http_addr,
        :http_port
    )
    ;",
        );

        let host = contract.host_info.as_ref().unwrap().clone();

        let mut ip4 = String::from("");
        let mut port: u32 = 0;
        let mut ip6 = String::from("");
        let mut http_addr = String::from("");
        let mut http_port: u32 = 0;

        if let Some(network) = host.network {
            if let Some(_ip4) = network.ip4_address {
                ip4 = _ip4;
            }

            if let Some(_port) = network.database_port_number {
                port = _port;
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
        }

        let conn = self.conn()?;
        let mut statement = conn.prepare(&cmd)?;
        statement.execute(named_params! {
            ":hid": &host.host_guid,
            ":hname" : &host.host_name,
            ":token" : &host.token,
            ":ip4" : &ip4,
            ":ip6" : &ip6,
            ":port" : &port,
            ":last_comm" : Utc::now().to_string(),
            ":http_addr" : &http_addr,
            ":http_port" : &http_port,
        })?;

        Ok(())
    }

    /// saves a contract's table information to CDS_CONTRACTS_TABLES
    fn save_contract_table_data(&self, contract: &Contract) -> Result<(), TreatyDbError> {
        let conn = self.conn()?;

        let cmd = String::from(
            "INSERT INTO CDS_CONTRACTS_TABLES
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
        :dbid,
        :dbname,
        :tid,
        :tname,
        :policy,
        1,
        1,
        1,
        1,
        0
    )
    ;
    ",
        );

        let schema = contract.schema.as_ref().unwrap();

        let db_name = schema.database_name.clone();
        let db_id = schema.database_id.clone();

        for t in &schema.tables {
            let mut statement = conn.prepare(&cmd)?;
            statement.execute(named_params! {
                ":dbid": &db_id,
                ":dbname" : &db_name,
                ":tid" : &t.table_id,
                ":tname" : &t.table_name,
                ":policy" : &t.logical_storage_policy
            })?;
        }

        Ok(())
    }
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
