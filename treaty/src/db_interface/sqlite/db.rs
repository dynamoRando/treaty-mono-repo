use std::path::Path;

use chrono::Utc;
use guid_create::GUID;
use rusqlite::{named_params, Connection};
use stdext::function_name;
use tracing::{error, trace};

use crate::db_interface::sqlite::{
    get_scalar_as_string, get_statement, has_enable_coooperative_features,
};
use crate::models::{
    get_metadata_table_name, CoopDatabaseContract, CoopDatabaseParticipant,
    CoopDatabaseParticipantData, Table,
};
use crate::{defaults, sql_text};
use treaty_types::enums::*;

use crate::error::{TreatyDbError, TreatyGenerateContractError};
use crate::treaty_proto::{
    ColumnSchema, DatabaseSchema, Participant, ParticipantStatus, TableSchema,
};

use chrono::TimeZone;

use super::treaty::TreatyDb;
use super::{
    execute_read, execute_write, get_scalar_as_u32, get_scalar_as_u64, has_any_rows, has_table,
};

#[derive(Debug, Clone)]
pub struct Db {
    db_name: String,
    dir: String,
    treaty: TreatyDb,
}

impl Db {
    pub fn new(db_name: &str, dir: &str, treaty: &TreatyDb) -> Self {
        Self {
            db_name: db_name.to_string(),
            dir: dir.to_string(),
            treaty: treaty.clone(),
        }
    }

    fn conn(&self) -> Result<Connection, TreatyDbError> {
        let db_path = Path::new(&self.dir).join(&self.db_name);
        trace!("[{}]: {db_path:?}", function_name!());
        Ok(Connection::open(db_path).unwrap())
    }

    fn write(&self, cmd: &str) -> Result<usize, TreatyDbError> {
        execute_write(&self.conn()?, cmd)
    }

    pub fn create_database(&self) -> Result<(), TreatyDbError> {
        let db_path = Path::new(&self.dir).join(&self.db_name);
        if db_path.exists() {
            return Err(TreatyDbError::DbAlreadyExists(self.db_name.clone()));
        };

        let r = self.conn();
        match r {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn delete_database(&self) -> Result<(), TreatyDbError> {
        todo!();
        let r = self.conn();
        match r {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn get_logical_storage_policy_for_all_user_tables(
        &self,
    ) -> Result<Vec<(String, LogicalStoragePolicy)>, TreatyDbError> {
        let conn = self.conn()?;

        let mut result: Vec<(String, LogicalStoragePolicy)> = Vec::new();

        let table_names = self.get_all_user_table_names_in_db()?;

        for table_name in &table_names {
            let l_policy = self.get_logical_storage_policy(&table_name.to_string())?;
            let item = (table_name.to_string(), l_policy);
            result.push(item);
        }

        Ok(result)
    }

    /// Returns the logical storage policy for the specified table. If the table does not exist in the database, it will
    /// return an error. If the table exist but does not have a logical storage policy defined for it, it will default
    /// to `LogicalStoragePolicy::None`
    pub fn get_logical_storage_policy(
        &self,
        table_name: &str,
    ) -> Result<LogicalStoragePolicy, TreatyDbError> {
        let conn = &self.conn()?;
        let policy: LogicalStoragePolicy;

        if has_table(table_name, conn)? {
            // insert or update on the coop tables

            if !has_table("COOP_REMOTES", conn)? {
                return Ok(LogicalStoragePolicy::None);
            }

            let mut cmd = String::from(
                "SELECT COUNT(*) TOTALCOUNT FROM COOP_REMOTES WHERE TABLENAME = ':table_name';",
            );
            cmd = cmd.replace(":table_name", table_name);
            if has_any_rows(cmd, conn)? {
                // then we have a record for the policy of the table
                let mut cmd = String::from(
                "SELECT LOGICAL_STORAGE_POLICY FROM COOP_REMOTES WHERE TABLENAME = ':table_name';",
            );

                cmd = cmd.replace(":table_name", table_name);
                let i_policy = get_scalar_as_u32(cmd, conn)?;
                policy = LogicalStoragePolicy::from_i64(i_policy as i64);
            } else {
                /*
                    let error_message = format!(
                        "logical storage policy not saved in COOP_REMOTES for table {} in db {}",
                        table_name, db_name
                    );
                    let err = treatyDbError::LogicalStoragePolicyNotSet(error_message);
                    return Err(err);
                */
                return Ok(LogicalStoragePolicy::None);
            }
        } else {
            let err = TreatyDbError::TableNotFoundInDatabase(
                table_name.to_string(),
                self.db_name.to_string(),
            );
            return Err(err);
        }

        Ok(policy)
    }

    fn get_all_user_table_names_in_db(&self) -> Result<Vec<String>, TreatyDbError> {
        let conn = &self.conn()?;
        let mut result: Vec<String> = Vec::new();
        let cmd = String::from(
            "SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE 'COOP_%'",
        );
        let names = execute_read(&cmd, conn)?;

        for row in names.rows {
            for val in row.vals {
                let name = val.data.unwrap().data_string;
                result.push(name);
            }
        }

        Ok(result)
    }

    fn save_contract(&self, contract: CoopDatabaseContract) -> Result<(), TreatyDbError> {
        let conn = self.conn()?;

        let mut cmd = String::from(
            "SELECT COUNT(*) TOTALCOUNT FROM COOP_DATABASE_CONTRACT WHERE VERSION_ID = ':vid'",
        );
        cmd = cmd.replace(":vid", &contract.version_id.to_string());
        if has_any_rows(cmd, &conn)? {
            // this is an update
            if contract.is_retired() {
                let mut cmd = String::from(
                    "
                UPDATE COOP_DATABASE_CONTRACT 
                SET 
                    CONTRACT_ID = ':cid',
                    GENERATED_DATE_UTC = ':gen_date',
                    DESCRIPTION = ':desc',
                    RETIRED_DATE_UTC = ':ret_date',
                    REMOTE_DELETE_BEHAVIOR = ':remote_behavior'
                WHERE
                    VERSION_ID = ':vid'",
                );
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
                self.write(&cmd)?;

                Ok(())
            } else {
                let mut cmd = String::from(
                    "
                UPDATE COOP_DATABASE_CONTRACT 
                SET 
                    CONTRACT_ID = ':cid',
                    GENERATED_DATE_UTC = ':gen_date',
                    DESCRIPTION = ':desc',
                    REMOTE_DELETE_BEHAVIOR = ':remote_behavior'
                WHERE
                    VERSION_ID = ':vid'",
                );
                cmd = cmd.replace(":cid", &contract.contract_id.to_string());
                cmd = cmd.replace(":gen_date", &contract.generated_date.to_string());
                cmd = cmd.replace(":desc", &contract.description);
                cmd = cmd.replace(":vid", &contract.version_id.to_string());
                cmd = cmd.replace(
                    ":remote_behavior",
                    &contract.remote_delete_behavior.to_string(),
                );
                self.write(&cmd)?;

                Ok(())
            }
        } else {
            // this is an insert
            if contract.is_retired() {
                let mut cmd = String::from(
                    "
                INSERT INTO COOP_DATABASE_CONTRACT
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
                self.write(&cmd)?;
                Ok(())
            } else {
                let mut cmd = String::from(
                    "
                INSERT INTO COOP_DATABASE_CONTRACT
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

                cmd = cmd.replace(":cid", &contract.contract_id.to_string());
                trace!("{}", &contract.generated_date);
                cmd = cmd.replace(":gen_date", &contract.generated_date.to_string());
                cmd = cmd.replace(":desc", &contract.description);
                cmd = cmd.replace(":vid", &contract.version_id.to_string());
                cmd = cmd.replace(
                    ":remote_behavior",
                    &contract.remote_delete_behavior.to_string(),
                );
                self.write(&cmd)?;

                Ok(())
            }
        }
    }

    pub fn get_all_database_contracts(&self) -> Result<Vec<CoopDatabaseContract>, TreatyDbError> {
        let conn = self.conn()?;
        let mut result: Vec<CoopDatabaseContract> = Vec::new();

        /*
            "CREATE TABLE IF NOT EXISTS COOP_DATABASE_CONTRACT
            (
                CONTRACT_ID CHAR(36) NOT NULL,
                GENERATED_DATE_UTC DATETIME NOT NULL,
                DESCRIPTION VARCHAR(255),
                RETIRED_DATE_UTC DATETIME,
                VERSION_ID CHAR(36) NOT NULL,
                REMOTE_DELETE_BEHAVIOR INT
            );",
        */

        let cmd = String::from(
            "SELECT 
        CONTRACT_ID,
        GENERATED_DATE_UTC,
        DESCRIPTION,
        RETIRED_DATE_UTC,
        VERSION_ID,
        REMOTE_DELETE_BEHAVIOR
    FROM
        COOP_DATABASE_CONTRACT
        ;
        ",
        );

        let table = execute_read(&cmd, &conn)?;

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

    /// Marks this contract as retired in the database with today's UTC date
    pub fn retire_contract(&self, version_id: GUID) -> Result<(), TreatyDbError> {
        let conn = self.conn()?;
        let mut cmd = String::from("UPDATE COOP_DATABASE_CONTRACT SET RETIRED_DATE_UTC = ':retire_date' WHERE VERSION_ID = ':vid'");
        cmd = cmd.replace(":retire_date", &Utc::now().to_string());
        cmd = cmd.replace(":vid", &version_id.to_string());
        execute_write(&conn, &cmd)?;

        Ok(())
    }

    pub fn generate_contract(
        &self,
        desc: &str,
        remote_delete_behavior: RemoteDeleteBehavior,
    ) -> Result<bool, TreatyGenerateContractError> {
        /*
           First, we should check to see if there is a logical storage policy
           defined on all user tables. If any are not set, then this should return
           a treatyGenerateContractError.

           We need to see if there are other database contracts.
           If there are none, then this is the first contract ever.

           If there are existing contracts, we need to find the active one
           and retire it, then generate the current one.
        */

        // trace!("generate contract: start for {}", db_name);

        let policies = self.get_logical_storage_policy_for_all_user_tables()?;

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
            return Err(error);
        }

        let cmd = String::from("SELECT COUNT(*) TOTALCONTRACTS FROM COOP_DATABASE_CONTRACT");
        if !has_any_rows(cmd, &self.conn()?)? {
            // this is the first contract
            // trace!("generate contract: first_contract");
            let contract = CoopDatabaseContract {
                contract_id: GUID::rand(),
                generated_date: Utc::now(),
                description: desc.to_string(),
                retired_date: None,
                version_id: GUID::rand(),
                remote_delete_behavior: RemoteDeleteBehavior::to_u32(remote_delete_behavior),
            };
            self.save_contract(contract)?;
        } else {
            // there are other contracts, we need to find the active one and retire it
            // then generate a new contract
            let contracts = self.get_all_database_contracts()?;
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
                    self.retire_contract(con.version_id)?;
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
                remote_delete_behavior: RemoteDeleteBehavior::to_u32(remote_delete_behavior),
            };
            self.save_contract(new_contract)?;
        }
        Ok(true)
    }

    pub fn enable_coooperative_features(&self) -> Result<bool, TreatyDbError> {
        self.create_remotes_table()?;
        self.create_participant_table()?;
        self.create_coop_contracts_table()?;
        self.create_data_host_tables()?;
        self.populate_data_host_tables()?;
        trace!("[{}]: enable_coooperative_features done", function_name!());
        Ok(true)
    }

    pub fn get_active_contract(&self) -> Result<CoopDatabaseContract, TreatyDbError> {
        let conn = self.conn()?;

        let cmd = String::from(
            "
            SELECT 
                CONTRACT_ID,
                GENERATED_DATE_UTC,
                DESCRIPTION,
                VERSION_ID,
                REMOTE_DELETE_BEHAVIOR 
            FROM 
                COOP_DATABASE_CONTRACT 
            WHERE 
                RETIRED_DATE_UTC IS NULL
            ;",
        );

        let row_to_active_contract = |contract_id: String,
                                      generated_date_utc: String,
                                      description: String,
                                      version_id: String,
                                      remote_delete_behavior: u32|
         -> rusqlite::Result<CoopDatabaseContract> {
            let contract = CoopDatabaseContract {
                contract_id: GUID::parse(&contract_id).unwrap(),
                generated_date: Utc::datetime_from_str(
                    &Utc,
                    &generated_date_utc,
                    defaults::DATETIME_STRING_FORMAT,
                )
                .unwrap(),
                description,
                retired_date: None,
                version_id: GUID::parse(&version_id).unwrap(),
                remote_delete_behavior,
            };

            Ok(contract)
        };

        let mut results: Vec<CoopDatabaseContract> = Vec::new();

        let mut statement = conn.prepare(&cmd).unwrap();
        let contracts = statement
            .query_and_then([], |row| {
                row_to_active_contract(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                )
            })
            .unwrap();

        for contract in contracts {
            results.push(contract.unwrap());
        }

        if results.is_empty() {
            error!("there is no active contract!");
        }

        return Ok(results.first().unwrap().clone());
    }

    pub fn update_participant_accepts_contract(
        &self,
        participant: CoopDatabaseParticipant,
        participant_message: Participant,
        accepted_contract_version_id: &str,
    ) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;

        let internal_id = participant.internal_id;
        let participant_id = participant_message.participant_guid.clone();
        let token = participant_message.token;

        let cmd = String::from(
            "
        UPDATE 
            COOP_PARTICIPANT
        SET 
            CONTRACT_STATUS = 3, 
            ACCEPTED_CONTRACT_VERSION_ID = :cid,
            PARTICIPANT_ID = :pid,
            TOKEN = :token
        WHERE 
            INTERNAL_PARTICIPANT_ID = :iid
        ;
        ",
        );

        let mut statement = conn.prepare(&cmd)?;

        let rows_affected = statement.execute(named_params! {
            ":cid" : accepted_contract_version_id.to_string(),
            ":pid" : participant_id,
            ":token" : token,
            ":iid" : internal_id.to_string(),
        })?;

        Ok(rows_affected > 0)
    }

    pub fn has_participants(&self) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        if has_table("COOP_PARTICIPANT", &conn)? {
            Ok(has_any_rows(
                "SELECT COUNT(*) PARTICIPANTS FROM COOP_PARTICIPANT".to_string(),
                &conn,
            )?)
        } else {
            Err(TreatyDbError::TableNotFoundInDatabase(
                "COOP_PARTICIPANT".to_string(),
                self.db_name.to_string(),
            ))
        }
    }

    pub fn get_participant_by_alias(
        &self,
        alias: &str,
    ) -> Result<Option<CoopDatabaseParticipant>, TreatyDbError> {
        let conn = self.conn()?;
        let cmd = String::from(
            "
            SELECT 
                INTERNAL_PARTICIPANT_ID,
                ALIAS,
                IP4ADDRESS,
                IP6ADDRESS,
                PORT,
                CONTRACT_STATUS,
                ACCEPTED_CONTRACT_VERSION_ID,
                TOKEN,
                PARTICIPANT_ID,
                HTTP_ADDR,
                HTTP_PORT
            FROM
                COOP_PARTICIPANT
            WHERE
                ALIAS = :alias
            ;
            ",
        );
        let row_to_participant = |internal_id: String,
                                  alias: String,
                                  ip4addr: String,
                                  ip6addr: String,
                                  port: u32,
                                  contract_status: u32,
                                  accepted_contract_version_id: String,
                                  token: Vec<u8>,
                                  id: String,
                                  http_addr: String,
                                  http_port: u16|
         -> rusqlite::Result<CoopDatabaseParticipant> {
            let participant = CoopDatabaseParticipant {
                internal_id: GUID::parse(&internal_id).unwrap(),
                alias,
                ip4addr,
                ip6addr,
                db_port: port,
                contract_status: ContractStatus::from_i64(contract_status as i64),
                accepted_contract_version: GUID::parse(&accepted_contract_version_id).unwrap(),
                token,
                id: GUID::parse(&id).unwrap(),
                http_addr,
                http_port,
            };

            Ok(participant)
        };

        let mut results: Vec<CoopDatabaseParticipant> = Vec::new();

        let mut statement = conn.prepare(&cmd).unwrap();
        let participants = statement
            .query_and_then(&[(":alias", &alias)], |row| {
                row_to_participant(
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
                    row.get(10).unwrap(),
                )
            })
            .unwrap();

        for participant in participants {
            results.push(participant.unwrap());
        }

        if !results.is_empty() {
            Ok(Some(results.first().unwrap().clone()))
        } else {
            Ok(None)
        }
    }

    pub fn get_participant_by_id(
        &self,
        id: &str,
    ) -> Result<Option<CoopDatabaseParticipant>, TreatyDbError> {
        let conn = self.conn()?;
        let cmd = String::from(
            "
            SELECT 
                INTERNAL_PARTICIPANT_ID,
                ALIAS,
                IP4ADDRESS,
                IP6ADDRESS,
                PORT,
                CONTRACT_STATUS,
                ACCEPTED_CONTRACT_VERSION_ID,
                TOKEN,
                PARTICIPANT_ID,
                HTTP_ADDR,
                HTTP_PORT
            FROM
                COOP_PARTICIPANT
            WHERE
            PARTICIPANT_ID = :id
            ;
            ",
        );
        // cmd = cmd.replace(":alias", &alias);

        // trace!("{:?}", cmd);
        // trace!("{}", alias);

        let row_to_participant = |internal_id: String,
                                  alias: String,
                                  ip4addr: String,
                                  ip6addr: String,
                                  port: u32,
                                  contract_status: u32,
                                  accepted_contract_version_id: String,
                                  token: Vec<u8>,
                                  id: String,
                                  http_addr: String,
                                  http_port: u16|
         -> rusqlite::Result<CoopDatabaseParticipant> {
            let participant = CoopDatabaseParticipant {
                internal_id: GUID::parse(&internal_id).unwrap(),
                alias,
                ip4addr,
                ip6addr,
                db_port: port,
                contract_status: ContractStatus::from_i64(contract_status as i64),
                accepted_contract_version: GUID::parse(&accepted_contract_version_id).unwrap(),
                token,
                id: GUID::parse(&id).unwrap(),
                http_addr,
                http_port,
            };

            Ok(participant)
        };

        let mut results: Vec<CoopDatabaseParticipant> = Vec::new();

        let mut statement = conn.prepare(&cmd).unwrap();
        let participants = statement
            .query_and_then(&[(":id", &id)], |row| {
                row_to_participant(
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
                    row.get(10).unwrap(),
                )
            })
            .unwrap();

        for participant in participants {
            results.push(participant.unwrap());
        }

        if !results.is_empty() {
            Ok(Some(results.first().unwrap().clone()))
        } else {
            Ok(None)
        }
    }

    pub fn has_participant(&self, alias: &str) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        let mut cmd =
            String::from("SELECT COUNT(*) TOTALCOUNT FROM COOP_PARTICIPANT WHERE ALIAS = ':alias'");
        cmd = cmd.replace(":alias", alias);
        has_any_rows(cmd, &conn)
    }

    pub fn get_schema_of_table(
        &self,
        table_name: String,
    ) -> core::result::Result<Table, TreatyDbError> {
        let mut cmd = String::from("PRAGMA table_info(\":table_name\")");
        cmd = cmd.replace(":table_name", &table_name);
        execute_read(&cmd, &self.conn()?)
    }

    pub fn insert_metadata_into_host_db(
        &self,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        let metadata_table_name = get_metadata_table_name(table_name);

        if !has_table(&metadata_table_name, &conn)? {
            //  need to create table
            let mut cmd = sql_text::Coop::text_create_metadata_table();
            cmd = cmd.replace(":table_name", &metadata_table_name);
            self.write(&cmd)?;
        }

        let mut cmd = sql_text::Coop::text_insert_row_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name);
        let mut statement = conn.prepare(&cmd)?;

        let rows = statement
            .execute(named_params! {":row": row_id, ":hash" : hash.to_ne_bytes(), ":pid" : internal_participant_id })
            ?;

        Ok(rows > 0)
    }

    pub fn update_metadata_in_host_db(
        &self,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        let metadata_table_name = get_metadata_table_name(table_name);

        if !has_table(&metadata_table_name, &conn)? {
            //  need to create table
            let mut cmd = sql_text::Coop::text_create_metadata_table();
            cmd = cmd.replace(":table_name", &metadata_table_name);
            self.write(&cmd)?;
        }

        let mut cmd = sql_text::Coop::text_update_row_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name);
        let mut statement = conn.prepare(&cmd).unwrap();

        let rows = statement
            .execute(named_params! {":row": row_id, ":hash" : hash.to_ne_bytes(), ":pid" : internal_participant_id })
            .unwrap();

        Ok(rows > 0)
    }

    pub fn delete_metadata_in_host_db(
        &self,
        table_name: &str,
        row_id: u32,
        internal_participant_id: &str,
    ) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        let metadata_table_name = get_metadata_table_name(table_name);

        if !has_table(&metadata_table_name, &conn)? {
            //  need to create table
            let mut cmd = sql_text::Coop::text_create_metadata_table();
            cmd = cmd.replace(":table_name", &metadata_table_name);
            self.write(&cmd)?;
        }

        let mut cmd = sql_text::Coop::text_delete_row_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name);
        let mut statement = conn.prepare(&cmd)?;

        trace!("[{}]: statement: {statement:?}", function_name!());

        let rows =
            statement.execute(named_params! {":row": row_id, ":pid" : internal_participant_id })?;

        trace!("[{}]: rows affected: {rows:?}", function_name!());

        Ok(rows > 0)
    }

    fn create_coop_contracts_table(&self) -> Result<(), TreatyDbError> {
        let cmd = String::from(
            "CREATE TABLE IF NOT EXISTS COOP_DATABASE_CONTRACT
        (
            CONTRACT_ID CHAR(36) NOT NULL,
            GENERATED_DATE_UTC DATETIME NOT NULL,
            DESCRIPTION VARCHAR(255),
            RETIRED_DATE_UTC DATETIME,
            VERSION_ID CHAR(36) NOT NULL,
            REMOTE_DELETE_BEHAVIOR INT
        );",
        );

        let conn = self.conn()?;
        trace!("[{}]: {conn:?}", function_name!());
        conn.execute(&cmd, [])?;

        Ok(())
    }

    /// Queries the COOP_REMOTES table for the table name and policy for each table in the database.
    /// If this returns an empty vector it means either this is a new database or we haven't audited the
    /// tables in the database. Generally, whenever we create a new table we should be adding the policy
    /// to this table an defaulting the policy to NONE.
    fn get_remote_status_for_tables(conn: &Connection) -> Vec<(String, LogicalStoragePolicy)> {
        let cmd = sql_text::Coop::text_get_logical_storage_policy_tables();
        let mut table_policies: Vec<(String, LogicalStoragePolicy)> = Vec::new();
        let mut statement = conn.prepare(&cmd).unwrap();

        let row_to_tuple =
            |table_name: String, policy: i64| -> rusqlite::Result<(String, LogicalStoragePolicy)> {
                Ok((table_name, LogicalStoragePolicy::from_i64(policy)))
            };

        let statuses = statement
            .query_and_then([], |row| {
                row_to_tuple(row.get(0).unwrap(), row.get(1).unwrap())
            })
            .unwrap();

        for status in statuses {
            table_policies.push(status.unwrap());
        }

        table_policies
    }

    /// Checks the COOP_DATA_HOST table to see if a database id has been generated and if not, creates and saves one.
    /// This is the id we will use to identify this database as having cooperative tables to participants
    fn populate_database_id(&self) -> Result<(), TreatyDbError> {
        let conn = self.conn()?;

        let cmd = sql_text::Coop::text_get_count_from_data_host();
        let has_database_id = has_any_rows(cmd, &conn)?;

        if !has_database_id {
            let cmd = sql_text::Coop::text_add_database_id_to_host();
            let db_id = GUID::rand();
            let mut statement = conn.prepare(&cmd).unwrap();
            statement
            .execute(named_params! {":database_id": db_id.to_string(), ":database_name" : &self.db_name})
            .unwrap();
        }

        Ok(())
    }

    fn populate_data_host_tables(&self) -> Result<(), TreatyDbError> {
        let conn = self.conn()?;
        self.populate_database_id()?;
        let table_statuses = Self::get_remote_status_for_tables(&conn);

        for status in table_statuses {
            // for each table that we have a logical storage policy
            // we want to make sure that the contract tables (COOP_DATA_HOST_*)
            // have the latest correct schema for each table. Note that
            // we add tables even if the logical storage policy is NONE, because in treaty
            // we want to be transparent about all the tables in the database

            let table_name = &status.0;
            let table_id = GUID::rand();

            let statement =
                sql_text::Coop::text_get_count_from_data_host_tables_for_table(table_name);
            if !has_any_rows(statement, &conn)? {
                let cmd = sql_text::Coop::text_add_table_to_data_host_table(
                    table_name.to_string(),
                    table_id.to_string(),
                );
                let mut statement = conn.prepare(&cmd).unwrap();
                statement.execute([]).unwrap();
            }

            // need to get schema and save it to the table
            let schema = self.get_schema_of_table(table_name.to_string());
            self.save_schema_to_data_host_tables(table_id.to_string(), &schema.unwrap())?;
        }

        Ok(())
    }

    fn create_data_host_tables(&self) -> Result<(), TreatyDbError> {
        let conn = self.conn()?;

        let mut cmd = sql_text::Coop::text_create_data_host_table();
        conn.execute(&cmd, [])?;
        cmd = sql_text::Coop::text_create_data_host_tables_table();
        conn.execute(&cmd, [])?;
        cmd = sql_text::Coop::text_create_data_host_tables_columns_table();
        conn.execute(&cmd, [])?;
        cmd = sql_text::Coop::text_create_data_remotes_table();
        conn.execute(&cmd, [])?;
        trace!("[{}]: {conn:?}", function_name!());

        Ok(())
    }

    fn create_participant_table(&self) -> Result<(), TreatyDbError> {
        let cmd = String::from(
            "CREATE TABLE IF NOT EXISTS COOP_PARTICIPANT
        (
            INTERNAL_PARTICIPANT_ID CHAR(36) NOT NULL,
            ALIAS VARCHAR(50) NOT NULL,
            IP4ADDRESS VARCHAR(25),
            IP6ADDRESS VARCHAR(25),
            PORT INT,
            CONTRACT_STATUS INT,
            ACCEPTED_CONTRACT_VERSION_ID CHAR(36),
            TOKEN BLOB NOT NULL,
            PARTICIPANT_ID CHAR(36),
            HTTP_ADDR VARCHAR(50),
            HTTP_PORT INT
        );",
        );

        let conn = self.conn()?;
        trace!("[{}]: {conn:?}", function_name!());

        conn.execute(&cmd, [])?;

        Ok(())
    }

    /// Creates the COOP_REMOTES table if it does not exist. This holds
    /// the logical storage policy for every table in the database.
    fn create_remotes_table(&self) -> Result<(), TreatyDbError> {
        let cmd = String::from(
            "CREATE TABLE IF NOT EXISTS COOP_REMOTES
    (
        TABLENAME VARCHAR(255) NOT NULL,
        LOGICAL_STORAGE_POLICY INT NOT NULL
    );",
        );

        let conn = self.conn()?;

        trace!("[{}]: {conn:?}", function_name!());

        conn.execute(&cmd, [])?;

        Ok(())
    }

    pub fn get_data_hash_at_host(
        &self,
        table_name: &str,
        row_id: u32,
    ) -> Result<Option<u64>, TreatyDbError> {
        let conn = self.conn()?;
        let metadata_table_name = get_metadata_table_name(table_name);
        let mut cmd = String::from("SELECT HASH FROM :metadata WHERE ROW_ID = :row_id");
        cmd = cmd.replace(":metadata", &metadata_table_name);
        cmd = cmd.replace(":row_id", &row_id.to_string());

        get_scalar_as_u64(cmd, &conn)
    }

    pub fn remove_remote_row_reference_from_host(
        &self,
        table_name: &str,
        row_id: u32,
    ) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        let metadata_table_name = get_metadata_table_name(table_name);

        let mut cmd = String::from(
            "DELETE FROM :table_name
             WHERE ROW_ID = :rid
        ;",
        );

        trace!("[{}]: {cmd:?}", function_name!());

        cmd = cmd.replace(":table_name", &metadata_table_name);

        let mut statement = get_statement(&cmd, &conn)?;

        let rows = statement.execute(named_params! {":rid": row_id})?;

        trace!(
            "[{}]: total row_references_deleted: {rows}",
            function_name!()
        );

        Ok(rows > 0)
    }

    pub fn save_participant(
        &self,
        participant: CoopDatabaseParticipant,
    ) -> Result<(), TreatyDbError> {
        let conn = self.conn()?;

        if self.has_participant(&participant.alias)? {
            // this is an update
            let cmd = String::from(
                "
            UPDATE COOP_PARTICIPANT
            SET
                IP4ADDRESS = ':ip4addr',
                IP6ADDRESS = ':ip6addr',
                PORT = ':db_port',
                CONTRACT_STATUS = ':contract_status',
                ACCEPTED_CONTRACT_VERSION_ID = ':accepted_contract_version',
                TOKEN = ':token',
                PARTICIPANT_ID = ':id',
                HTTP_ADDR = ':http_addr',
                HTTP_PORT = ':http_port',
            WHERE
                ALIAS = ':alias'
            ;
            ",
            );

            let mut statement = conn.prepare(&cmd).unwrap();
            statement
                .execute(named_params! {
                        ":ip4addr": participant.ip4addr,
                        ":ip6addr": participant.ip6addr,
                        ":db_port": participant.db_port.to_string(),
                        ":contract_status": &num::ToPrimitive::to_u32(&participant.contract_status).unwrap_or(0),
                        ":accepted_contract_version": &participant.accepted_contract_version.to_string(),
                        ":token": &participant.token,
                        ":id": &participant.id.to_string(),
                        ":alias": &participant.alias,
                        ":http_addr": &participant.http_addr,
                        ":http_port": &participant.http_port,
                })
                .unwrap();

            Ok(())
        } else {
            // this is an insert

            // trace!("{:?}", &self);

            let cmd = String::from(
                "
            INSERT INTO COOP_PARTICIPANT
            (
                INTERNAL_PARTICIPANT_ID,
                ALIAS,
                IP4ADDRESS,
                IP6ADDRESS,
                PORT,
                CONTRACT_STATUS,
                ACCEPTED_CONTRACT_VERSION_ID,
                TOKEN,
                PARTICIPANT_ID,
                HTTP_ADDR,
                HTTP_PORT
            )
            VALUES
            (
                :internal_id,
                :alias,
                :ip4addr,
                :ip6addr,
                :db_port,
                :contract_status,
                :accepted_contract_version,
                :token,
                :id,
                :http_addr,
                :http_port
            );
            ",
            );

            let mut statement = conn.prepare(&cmd).unwrap();
            statement
                .execute(named_params! {
                        ":internal_id": &participant.internal_id.to_string(),
                        ":alias": &participant.alias,
                        ":ip4addr": &participant.ip4addr,
                        ":ip6addr": &participant.ip6addr,
                        ":db_port": &participant.db_port.to_string(),
                        ":contract_status": num::ToPrimitive::to_u32(&participant.contract_status).unwrap_or(0),
                        ":accepted_contract_version": &participant.accepted_contract_version.to_string(),
                        ":token": &participant.token,
                        ":id": &participant.id.to_string(),
                        ":http_addr": &participant.http_addr,
                        ":http_port": &participant.http_port
                })
                .unwrap();

            Ok(())
        }
    }

    pub fn add_participant(
        &self,
        alias: &str,
        ip4addr: &str,
        db_port: u32,
        http_addr: String,
        http_port: u16,
        id: Option<String>,
    ) -> Result<bool, TreatyDbError> {
        let db_host_id = match id {
            Some(id) => GUID::parse(&id).unwrap(),
            None => GUID::parse(defaults::EMPTY_GUID).unwrap(),
        };

        let is_added: bool = if self.has_participant(alias)? {
            false
        } else {
            let participant = CoopDatabaseParticipant {
                internal_id: GUID::rand(),
                alias: alias.to_string(),
                ip4addr: ip4addr.to_string(),
                ip6addr: String::from(""),
                db_port,
                contract_status: ContractStatus::NotSent,
                accepted_contract_version: GUID::parse(defaults::EMPTY_GUID).unwrap(),
                id: db_host_id,
                token: Vec::new(),
                http_addr,
                http_port,
            };
            self.save_participant(participant)?;
            true
        };

        Ok(is_added)
    }

    pub fn get_db_schema(&self) -> Result<DatabaseSchema, TreatyDbError> {
        let mut cooperation_enabled = false;
        let mut db_has_participants = false;

        let conn = self.conn()?;
        let db_name = &self.db_name;

        trace!("[{}]: {conn:?}", function_name!());

        // if this is a host db
        if has_table("COOP_DATA_HOST", &conn)? {
            let mut cmd = String::from("SELECT DATABASE_ID FROM COOP_DATA_HOST");
            let db_id = get_scalar_as_string(cmd, &conn)?;

            if let Ok(is_enabled) = has_enable_coooperative_features(&self.db_name, &self.dir) {
                cooperation_enabled = is_enabled;
            }

            if cooperation_enabled {
                if let Ok(x) = self.has_participants() {
                    db_has_participants = x;
                }
            }

            let mut db_schema = DatabaseSchema {
                database_id: db_id.clone(),
                database_name: self.db_name.to_string(),
                tables: Vec::new(),
                database_type: DatabaseType::to_u32(DatabaseType::Sqlite),
                treaty_database_type: num::ToPrimitive::to_u32(&TreatyDatabaseType::Host)
                    .unwrap_or(0),
                cooperation_enabled,
                has_participants: db_has_participants,
            };

            cmd = String::from("SELECT TABLE_ID, TABLE_NAME FROM COOP_DATA_TABLES");

            let row_to_tuple =
                |table_id: String, table_name: String| -> rusqlite::Result<(String, String)> {
                    Ok((table_id, table_name))
                };

            let mut tables_in_db: Vec<(String, String)> = Vec::new();

            let mut statement = conn.prepare(&cmd).unwrap();

            let tables = statement
                .query_and_then([], |row| {
                    row_to_tuple(row.get(0).unwrap(), row.get(1).unwrap())
                })
                .unwrap();

            for table in tables {
                tables_in_db.push(table.unwrap());
            }

            trace!("tables_in_db: {:?}", tables_in_db);

            for t in &tables_in_db {
                let policy = self.get_logical_storage_policy(&t.1)?;

                let mut ts = TableSchema {
                    table_name: t.1.clone(),
                    table_id: t.0.clone(),
                    database_id: db_id.clone(),
                    database_name: db_name.to_string(),
                    columns: Vec::new(),
                    logical_storage_policy: LogicalStoragePolicy::to_u32(policy),
                };

                let schema = self.get_schema_of_table(t.1.to_string());

                // # Columns:
                // 1. columnId
                // 2. name
                // 3. type
                // 4. NotNull
                // 5. defaultValue
                // 6. IsPK

                let table = t.1.clone();
                trace!(
                    "[{}]: schema of table: {table:?} {schema:?}",
                    function_name!()
                );

                for row in schema.unwrap().rows {
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
                        if val.col.name == "columnId" {
                            let item = val.data.clone().unwrap();
                            cs.ordinal = item.data_string.parse().unwrap();
                        }

                        if val.col.name == "name" {
                            let item = val.data.clone().unwrap();
                            cs.column_name = item.data_string.parse().unwrap();
                        }

                        if val.col.name == "type" {
                            let item = val.data.clone().unwrap();
                            let ct = ColumnType::data_type_to_enum_u32(item.data_string.clone());
                            let len = ColumnType::data_type_len(item.data_string.clone());

                            cs.column_type = ct;
                            cs.column_length = len;
                        }

                        if val.col.name == "NotNull" {
                            let item = val.data.clone().unwrap();
                            cs.is_nullable = item.data_string.parse().unwrap();
                        }

                        if val.col.name == "IsPK" {
                            let item = val.data.clone().unwrap();
                            cs.is_primary_key = item.data_string.parse().unwrap();
                        }
                    }

                    ts.columns.push(cs);
                }

                db_schema.tables.push(ts);
            }

            trace!("[{}]: {db_schema:?}", function_name!());

            // get all remaining tables that don't have a policy defined, because we may want to set them
            let table_names = self.get_all_user_table_names_in_db()?;

            let mut existing_tables: Vec<String> = Vec::new();
            for t in &tables_in_db {
                existing_tables.push(t.1.clone());
            }

            for table_name in &table_names {
                if !existing_tables.contains(table_name) {
                    let mut ts = TableSchema {
                        table_name: table_name.clone(),
                        table_id: String::from(""),
                        database_id: String::from(""),
                        database_name: db_name.to_string(),
                        columns: Vec::new(),
                        logical_storage_policy: LogicalStoragePolicy::to_u32(
                            LogicalStoragePolicy::None,
                        ),
                    };

                    let schema = self.get_schema_of_table(table_name.clone().to_string());

                    // # Columns:
                    // 1. columnId
                    // 2. name
                    // 3. type
                    // 4. NotNull
                    // 5. defaultValue
                    // 6. IsPK

                    for row in schema.unwrap().rows {
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
                            trace!("{val:?}");

                            if val.col.name == "columnId" {
                                let item = val.data.clone().unwrap();
                                cs.ordinal = item.data_string.parse().unwrap();
                            }

                            if val.col.name == "name" {
                                let item = val.data.clone().unwrap();
                                cs.column_name = item.data_string.parse().unwrap();
                            }

                            if val.col.name == "type" {
                                let item = val.data.clone().unwrap();
                                let ct =
                                    ColumnType::data_type_to_enum_u32(item.data_string.clone());
                                let len = ColumnType::data_type_len(item.data_string.clone());

                                cs.column_type = ct;
                                cs.column_length = len;
                            }

                            if val.col.name == "NotNull" {
                                let item = val.data.clone().unwrap();
                                cs.is_nullable = item.data_string.parse().unwrap();
                            }

                            if val.col.name == "IsPK" {
                                let item = val.data.clone().unwrap();
                                cs.is_primary_key = item.data_string.parse().unwrap();
                            }
                        }

                        ts.columns.push(cs);
                    }

                    db_schema.tables.push(ts);
                }
            }

            return Ok(db_schema);
        }

        if let Ok(is_enabled) = has_enable_coooperative_features(&self.db_name, &self.dir) {
            cooperation_enabled = is_enabled;
        }

        if cooperation_enabled {
            if let Ok(x) = self.has_participants() {
                db_has_participants = x;
            }
        }

        let mut db_schema = DatabaseSchema {
            database_id: String::from(""),
            database_name: db_name.to_string(),
            tables: Vec::new(),
            database_type: DatabaseType::to_u32(DatabaseType::Sqlite),
            treaty_database_type: num::ToPrimitive::to_u32(&TreatyDatabaseType::Partial)
                .unwrap_or(0),
            cooperation_enabled,
            has_participants: db_has_participants,
        };

        let table_names = self.get_all_user_table_names_in_db()?;

        for table_name in &table_names {
            let mut ts = TableSchema {
                table_name: table_name.clone(),
                table_id: String::from(""),
                database_id: String::from(""),
                database_name: db_name.to_string(),
                columns: Vec::new(),
                logical_storage_policy: LogicalStoragePolicy::to_u32(LogicalStoragePolicy::None),
            };

            let schema = self.get_schema_of_table(table_name.clone().to_string());

            // # Columns:
            // 1. columnId
            // 2. name
            // 3. type
            // 4. NotNull
            // 5. defaultValue
            // 6. IsPK

            for row in schema.unwrap().rows {
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
                    trace!("[{}]: {val:?}", function_name!());

                    if val.col.name == "columnId" {
                        let item = val.data.clone().unwrap();
                        cs.ordinal = item.data_string.parse().unwrap();
                    }

                    if val.col.name == "name" {
                        let item = val.data.clone().unwrap();
                        cs.column_name = item.data_string.parse().unwrap();
                    }

                    if val.col.name == "type" {
                        let item = val.data.clone().unwrap();
                        let ct = ColumnType::data_type_to_enum_u32(item.data_string.clone());
                        let len = ColumnType::data_type_len(item.data_string.clone());

                        cs.column_type = ct;
                        cs.column_length = len;
                    }

                    if val.col.name == "NotNull" {
                        let item = val.data.clone().unwrap();
                        cs.is_nullable = item.data_string.parse().unwrap();
                    }

                    if val.col.name == "IsPK" {
                        let item = val.data.clone().unwrap();
                        cs.is_primary_key = item.data_string.parse().unwrap();
                    }
                }

                ts.columns.push(cs);
            }

            db_schema.tables.push(ts);
        }

        Ok(db_schema)
    }

    pub fn set_logical_storage_policy(
        &self,
        table_name: &str,
        policy: LogicalStoragePolicy,
    ) -> Result<bool, TreatyDbError> {
        let conn = self.conn()?;
        if has_table(table_name, &conn)? {
            // insert or update on the coop tables
            let mut cmd = String::from(
                "SELECT COUNT(*) TOTALCOUNT FROM COOP_REMOTES WHERE TABLENAME = ':table_name';",
            );
            cmd = cmd.replace(":table_name", table_name);
            if has_any_rows(cmd, &conn)? {
                // then this is an update
                let mut cmd = String::from(
                    "UPDATE COOP_REMOTES
                SET LOGICAL_STORAGE_POLICY = :policy
                WHERE TABLENAME = ':table_name';
                ",
                );

                cmd = cmd.replace(":table_name", table_name);
                cmd = cmd.replace(":policy", &LogicalStoragePolicy::to_u32(policy).to_string());
                let result = execute_write(&conn, &cmd);
                if result.is_err() {
                    return Err(result.err().unwrap());
                }
            } else {
                // then this is an insert
                let mut cmd = String::from(
                    "INSERT INTO COOP_REMOTES
                (
                    TABLENAME,
                    LOGICAL_STORAGE_POLICY  
                )
                VALUES
                (
                    ':table_name',
                    :policy
                );",
                );

                cmd = cmd.replace(":table_name", table_name);
                cmd = cmd.replace(":policy", &LogicalStoragePolicy::to_u32(policy).to_string());
                let result = execute_write(&conn, &cmd);
                if result.is_err() {
                    return Err(result.err().unwrap());
                }
            }

            self.populate_data_host_tables()?;
        } else {
            let err = TreatyDbError::TableNotFoundInDatabase(
                table_name.to_string(),
                self.db_name.to_string(),
            );
            return Err(err);
        }
        Ok(true)
    }

    fn save_schema_to_data_host_tables(
        &self,
        table_id: String,
        schema: &Table,
    ) -> Result<(), TreatyDbError> {
        let conn = self.conn()?;
        /*
        Columns:
            columnId
            name
            type
            NotNull
            defaultValue
            IsPK
         */

        let rows = &schema.rows;
        for row in rows {
            if row.vals[1].col.name == "name" {
                let col_name = &row.vals[1].data.as_ref().unwrap().data_string;

                let mut col_check = String::from(
                    "SELECT 
                        COUNT(*) COUNT
                    FROM 
                        COOP_DATA_HOST_TABLE_COLUMNS
                    WHERE
                        COLUMN_NAME = ':col_name'
                ;",
                );

                col_check = col_check.replace(":col_name", col_name);
                if !has_any_rows(col_check, &conn)? {
                    // we need to add the column schema to the data host tables
                    let col_id = GUID::rand();

                    let mut cmd = String::from(
                        "
                        INSERT INTO COOP_DATA_HOST_TABLE_COLUMNS
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
                    ;",
                    );

                    cmd = cmd.replace(":table_id", &table_id);
                    cmd = cmd.replace(":col_id", &col_id.to_string());
                    cmd = cmd.replace(":col_name", col_name);
                    conn.execute(&cmd, []).unwrap();
                }
            }
        }

        Ok(())
    }

    pub fn execute_write_at_host(&self, cmd: &str) -> Result<usize, TreatyDbError> {
        let conn = self.conn()?;

        trace!("[{}]: {conn:?} {cmd:?}", function_name!());

        let result = conn.execute(cmd, [])?;

        let _ = conn.close();

        Ok(result)
    }

    pub fn execute_read_at_host(&self, cmd: &str) -> core::result::Result<Table, TreatyDbError> {
        let conn = &self.conn()?;
        execute_read(cmd, conn)
    }

    pub fn has_cooperative_tables(&self, cmd: &str) -> Result<bool, TreatyDbError> {
        use crate::query_parser::get_table_names;

        let mut has_cooperative_tables = false;

        let tables = get_table_names(cmd, DatabaseType::Sqlite);

        for table in tables {
            let result = self.get_logical_storage_policy(&table);

            if let Ok(policy) = result {
                match policy {
                    LogicalStoragePolicy::Mirror => {
                        has_cooperative_tables = true;
                        break;
                    }
                    LogicalStoragePolicy::ParticpantOwned => {
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

    fn get_participant_by_internal_id(
        &self,
        internal_id: &str,
    ) -> Result<Option<CoopDatabaseParticipant>, TreatyDbError> {
        let conn = &self.conn()?;
        let cmd = String::from(
            "
            SELECT 
                INTERNAL_PARTICIPANT_ID,
                ALIAS,
                IP4ADDRESS,
                IP6ADDRESS,
                PORT,
                CONTRACT_STATUS,
                ACCEPTED_CONTRACT_VERSION_ID,
                TOKEN,
                PARTICIPANT_ID,
                HTTP_ADDR,
                HTTP_PORT
            FROM
                COOP_PARTICIPANT
            WHERE
                INTERNAL_PARTICIPANT_ID = :pid
            ;
            ",
        );
        // cmd = cmd.replace(":alias", &alias);

        let row_to_participant = |internal_id: String,
                                  alias: String,
                                  ip4addr: String,
                                  ip6addr: String,
                                  port: u32,
                                  contract_status: u32,
                                  accepted_contract_version_id: String,
                                  token: Vec<u8>,
                                  id: String,
                                  http_addr: String,
                                  http_port: u16|
         -> rusqlite::Result<CoopDatabaseParticipant> {
            let participant = CoopDatabaseParticipant {
                internal_id: GUID::parse(&internal_id).unwrap(),
                alias,
                ip4addr,
                ip6addr,
                db_port: port,
                contract_status: ContractStatus::from_i64(contract_status as i64),
                accepted_contract_version: GUID::parse(&accepted_contract_version_id).unwrap(),
                token,
                id: GUID::parse(&id).unwrap(),
                http_addr,
                http_port,
            };

            Ok(participant)
        };

        let mut results: Vec<CoopDatabaseParticipant> = Vec::new();

        let mut statement = conn.prepare(&cmd).unwrap();
        let participants = statement
            .query_and_then(&[(":pid", &internal_id)], |row| {
                row_to_participant(
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
                    row.get(10).unwrap(),
                )
            })
            .unwrap();

        for participant in participants {
            results.push(participant.unwrap());
        }

        if results.is_empty() {
            Ok(None)
        } else {
            Ok(Some(results.first().unwrap().clone()))
        }
    }

    pub fn get_participants_for_database(
        &self,
    ) -> core::result::Result<Option<Vec<ParticipantStatus>>, TreatyDbError> {
        let mut result: Vec<ParticipantStatus> = Vec::new();

        let conn = self.conn()?;

        // if the table doesn't exist, we should return an error here
        if !has_table("COOP_PARTICIPANT", &conn)? {
            return Err(TreatyDbError::TableNotFoundInDatabase(
                "COOP_PARTICIPANT".to_string(),
                self.db_name.to_string(),
            ));
        }

        let cmd = "
        SELECT 
            INTERNAL_PARTICIPANT_ID,
            ALIAS,
            IP4ADDRESS,
            IP6ADDRESS,
            PORT,
            CONTRACT_STATUS,
            PARTICIPANT_ID,
            HTTP_ADDR,
            HTTP_PORT
        FROM
            COOP_PARTICIPANT
        ";

        let mut statement = conn.prepare(cmd).unwrap();

        let row_to_participant = |internal_participant_id: String,
                                  alias: String,
                                  ip4: String,
                                  ip6: String,
                                  port: u32,
                                  contract_status: u32,
                                  participant_id: String,
                                  http_addr: String,
                                  http_port: u32|
         -> rusqlite::Result<ParticipantStatus> {
            let p = Participant {
                participant_guid: participant_id,
                alias,
                ip4_address: ip4,
                ip6_address: ip6,
                database_port_number: port,
                token: Vec::new(),
                internal_participant_guid: internal_participant_id,
                http_addr,
                http_port,
            };

            let ps = ParticipantStatus {
                participant: Some(p),
                contract_status,
            };

            Ok(ps)
        };

        let participants = statement
            .query_and_then([], |row| {
                row_to_participant(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    row.get(5).unwrap(),
                    row.get(6).unwrap(),
                    row.get(7).unwrap(),
                    row.get(8).unwrap(),
                )
            })
            .unwrap();

        for p in participants {
            result.push(p.unwrap());
        }

        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }

    pub fn get_cooperative_tables(&self, cmd: &str) -> Result<Option<Vec<String>>, TreatyDbError> {
        use crate::query_parser::get_table_names;

        let mut cooperative_tables: Vec<String> = Vec::new();

        let tables = get_table_names(cmd, DatabaseType::Sqlite);

        for table in &tables {
            let result = self.get_logical_storage_policy(&table.to_string());

            if let Ok(policy) = result {
                match policy {
                    LogicalStoragePolicy::Mirror => {
                        cooperative_tables.push(table.clone());
                    }
                    LogicalStoragePolicy::ParticpantOwned => {
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

    pub fn get_participants_for_table(
        &self,
        table_name: &str,
    ) -> Result<Option<Vec<CoopDatabaseParticipantData>>, TreatyDbError> {
        // note - we will need another table to track the remote row id
        // let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);

        let conn = &self.conn()?;
        let metadata_table_name = get_metadata_table_name(table_name);

        if !has_table(&metadata_table_name, conn)? {
            //  need to create table
            let mut cmd = sql_text::Coop::text_create_metadata_table();
            cmd = cmd.replace(":table_name", &metadata_table_name);
            execute_write(conn, &cmd)?;
        }

        let conn = self.conn()?;

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

        let mut statement = conn.prepare(&cmd).unwrap();
        let mut participant_ids: Vec<String> = Vec::new();
        let mut db_participants: Vec<CoopDatabaseParticipant> = Vec::new();

        let row_to_id = |participant_id: String| -> rusqlite::Result<String> { Ok(participant_id) };

        let participants = statement
            .query_and_then([], |row| row_to_id(row.get(0).unwrap()))
            .unwrap();

        for p in participants {
            participant_ids.push(p.unwrap());
        }

        for pid in &participant_ids {
            if let Some(participant) = self.get_participant_by_internal_id(pid)? {
                db_participants.push(participant);
            }
        }

        let row_to_data =
            |row_id: u32, hash: Vec<u8>| -> rusqlite::Result<(u32, Vec<u8>)> { Ok((row_id, hash)) };

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

            statement = conn.prepare(&cmd).unwrap();

            let row_data = statement
                .query_and_then([], |row| {
                    row_to_data(row.get(0).unwrap(), row.get(1).unwrap())
                })
                .unwrap();

            let mut row_data_results: Vec<(u32, Vec<u8>)> = Vec::new();

            for data in row_data {
                row_data_results.push(data.unwrap());
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
}
