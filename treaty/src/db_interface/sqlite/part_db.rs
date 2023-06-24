use std::path::Path;

use chrono::Utc;

use crate::db_interface::sqlite::{get_scalar_as_string, get_schema_of_table, get_statement};
use rusqlite::types::Type;
use rusqlite::{named_params, Connection};
use stdext::function_name;
use tracing::{error, trace, warn};
use treaty_types::enums::*;

use crate::models::{
    get_data_log_table_name, get_data_queue_table_name, get_metadata_table_name, PartialDataResult,
    Table,
};
use crate::{crypt, defaults, sql_text};

use crate::error::TreatyDbError;
use crate::treaty_proto::{Contract, PendingStatement, Row, TableSchema};

use super::treaty::TreatyDb;
use super::{
    execute_read, execute_read_on_connection_for_row, execute_write, get_scalar_as_u32,
    get_scalar_as_u64, has_table,
};

#[derive(Debug, Clone)]
pub struct PartDb {
    db_name: String,
    dir: String,
    treaty: TreatyDb,
}

impl PartDb {
    pub fn new(db_name: &str, dir: &str, treaty: &TreatyDb) -> Self {
        Self {
            db_name: db_name.to_string(),
            dir: dir.to_string(),
            treaty: treaty.clone(),
        }
    }

    fn execute_update_overwrite(
        &self,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let original_cmd = cmd;
        let mut cmd;
        cmd = String::from("SELECT ROWID FROM :table_name WHERE :where_clause")
            .replace(":table_name", table_name);

        if !where_clause.is_empty() {
            cmd = cmd.replace(":where_clause", where_clause);
        } else {
            cmd = cmd.replace("WHERE", "");
            cmd = cmd.replace(":where_clause", "");
        }

        trace!("[{}]: cmd: {cmd:?}", function_name!());

        // we need to determine the row_ids that we're going to update because we're going to need to update
        // the data hashes for them
        let c = self.conn()?;
        let mut statement = get_statement(&cmd, &c)?;

        // once we have the row ids, then we will need to get the hash of the rows after they've been updated.

        let mut row_ids: Vec<u32> = Vec::new();
        let row_to_id = |rowid: u32| -> rusqlite::Result<u32> { Ok(rowid) };

        let ids = statement.query_and_then([], |row| row_to_id(row.get(0).unwrap()))?;

        for id in ids {
            row_ids.push(id.unwrap());
        }

        trace!("[{}]: row_ids: {row_ids:?}", function_name!());

        let total_rows = self.write(original_cmd)?;

        trace!("[{}]: total_rows: {row_ids:?}", function_name!());

        if total_rows != row_ids.len() {
            error!("[{}]: The update statement did not match the expected count of affected rows. This should not happen.", function_name!());
        }

        // now we need to update the data hashes for every row that was changed

        let col_names = self.table_col_names(table_name)?;
        let mut cmd;
        cmd = String::from("SELECT :col_names FROM :table_name WHERE ROWID = :rid");
        cmd = cmd.replace(":table_name", table_name);

        let mut col_name_list = String::from("");

        for name in &col_names {
            col_name_list = col_name_list + name + ",";
        }

        let completed_col_name_list = &col_name_list[0..&col_name_list.len() - 1];

        cmd = cmd.replace(":col_names", completed_col_name_list);

        let mut row_hashes: Vec<(u32, u64)> = Vec::new();

        for id in &row_ids {
            let sql = cmd.replace(":rid", &id.to_string());
            let conn = &self.conn()?;
            let mut stmt = get_statement(&sql, conn)?;
            let mut rows = stmt.query([]).unwrap();

            // for a single row
            while let Some(row) = rows.next().unwrap() {
                let mut row_values: Vec<String> = Vec::new();
                for i in 0..col_names.len() {
                    let dt = row.get_ref_unwrap(i).data_type();

                    let string_value: String = match dt {
                        Type::Blob => String::from(""),
                        Type::Integer => row.get_ref_unwrap(i).as_i64().unwrap().to_string(),
                        Type::Real => row.get_ref_unwrap(i).as_f64().unwrap().to_string(),
                        Type::Text => row.get_ref_unwrap(i).as_str().unwrap().to_string(),
                        _ => String::from(""),
                    };

                    row_values.push(string_value);
                }

                let hash_value = crypt::calculate_hash_for_struct(&row_values);
                let row_hash: (u32, u64) = (*id, hash_value);
                row_hashes.push(row_hash);
            }
        }

        // now that we have the row hashes, we should save them back to the table
        let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);
        let mut cmd = String::from("UPDATE :table_name SET HASH = :hash WHERE ROW_ID = :rid");
        cmd = cmd.replace(":table_name", &metadata_table_name);

        for row in &row_hashes {
            let conn = self.conn()?;
            let mut statement = get_statement(&cmd, &conn)?;
            statement
                .execute(named_params! {":hash": row.1.to_ne_bytes(), ":rid" : row.0})
                .unwrap();
        }

        let row_data = row_hashes.first().unwrap();

        drop(statement);

        Ok(PartialDataResult {
            is_successful: true,
            row_id: row_data.0,
            data_hash: Some(row_data.1),
            partial_data_status: Some(1),
            action: Some(PartialDataResultAction::Update),
        })
    }

    fn get_partial_db_connection(&self, db_name: &str) -> Result<Connection, TreatyDbError> {
        let mut db_part_name = db_name.replace(".db", "");
        db_part_name = db_part_name.replace(".dbpart", "");
        db_part_name = format!("{}{}", db_part_name, String::from(".dbpart"));
        let db_path = Path::new(&self.dir).join(&db_part_name);
        trace!("[{}]: {db_path:?}", function_name!());
        Ok(Connection::open(db_path)?)
    }

    pub fn create_partial_database(&self) -> Result<bool, TreatyDbError> {
        let mut db_part_name = self.db_name.replace(".db", "");
        db_part_name = db_part_name.replace(".dbpart", "");
        db_part_name = format!("{}{}", db_part_name, String::from(".dbpart"));
        let db_path = Path::new(&self.dir).join(db_part_name);
        Connection::open(db_path)?;
        Ok(true)
    }

    fn table_col_names(&self, table_name: &str) -> Result<Vec<String>, TreatyDbError> {
        let mut result: Vec<String> = Vec::new();

        let mut cmd =
            String::from("select NAME from pragma_table_info(\":table_name\") as tblInfo;");
        cmd = cmd.replace(":table_name", table_name);

        let row_to_string = |column_name: String| -> rusqlite::Result<String> { Ok(column_name) };

        let c = self.conn()?;
        let mut statement = get_statement(&cmd, &c)?;

        let names = statement.query_and_then([], |row| row_to_string(row.get(0).unwrap()))?;

        for name in names {
            result.push(name.unwrap());
        }

        drop(statement);

        Ok(result)
    }

    fn read(&self, cmd: &str) -> Result<Table, TreatyDbError> {
        execute_read(cmd, &self.conn()?)
    }

    pub fn get_pending_actions(
        &self,
        table_name: &str,
        action: &str,
    ) -> Result<Option<Vec<PendingStatement>>, TreatyDbError> {
        let update_queue = get_data_queue_table_name(table_name);

        let mut pending_statements: Vec<PendingStatement> = Vec::new();

        let mut cmd = String::from(
            "
            SELECT 
                ID,
                STATEMENT,
                REQUESTED_TS_UTC,
                HOST_ID
            FROM
                :table
            WHERE
                ACTION = ':action'
            ;",
        );
        cmd = cmd.replace(":table", &update_queue);
        cmd = cmd.replace(":action", action);

        let pending_rows = self.read(&cmd)?;

        for row in &pending_rows.rows {
            let mut rid: u32 = 0;
            let mut statement: String = String::from("");
            let mut ts: String = String::from("");
            let mut host_id: String = String::from("");

            for val in &row.vals {
                if val.col.name == "ID" {
                    rid = val
                        .data
                        .as_ref()
                        .unwrap()
                        .data_string
                        .parse::<u32>()
                        .unwrap();
                }

                if val.col.name == "STATEMENT" {
                    statement = val.data.as_ref().unwrap().data_string.clone();
                }

                if val.col.name == "REQUESTED_TS_UTC" {
                    ts = val.data.as_ref().unwrap().data_string.clone();
                }

                if val.col.name == "HOST_ID" {
                    host_id = val.data.as_ref().unwrap().data_string.clone();
                }
            }

            let ps = PendingStatement {
                row_id: rid,
                statement,
                requested_ts_utc: ts,
                host_id,
                action: action.to_string(),
            };

            pending_statements.push(ps);
        }

        if pending_statements.is_empty() {
            Ok(None)
        } else {
            Ok(Some(pending_statements))
        }
    }

    pub fn execute_write_at_participant(&self, cmd: &str) -> Result<usize, TreatyDbError> {
        let conn = self.conn()?;
        let result = conn.execute(cmd, [])?;
        let _ = conn.close();

        Ok(result)
    }

    pub fn execute_read_at_participant(&self, cmd: &str) -> Result<Table, TreatyDbError> {
        let conn = self.conn()?;
        execute_read(cmd, &conn)
    }

    pub fn get_row_from_partial_database(
        &self,
        table_name: &str,
        row_id: u32,
    ) -> Result<Row, TreatyDbError> {
        let conn = self.conn()?;
        let mut cmd = String::from("SELECT * from :table_name WHERE ROWID = :rid");

        cmd = cmd.replace(":table_name", table_name);
        cmd = cmd.replace(":rid", &row_id.to_string());

        execute_read_on_connection_for_row(&self.db_name, table_name, row_id, cmd, &conn)
    }

    pub fn create_partial_database_from_contract(
        &self,
        contract: &Contract,
    ) -> Result<bool, TreatyDbError> {
        trace!("{contract:?}");

        let db_name = contract.schema.as_ref().unwrap().database_name.clone();
        self.create_partial_database()?;

        for table in &contract.schema.as_ref().unwrap().tables {
            self.create_table_from_schema(table)?;
        }

        Ok(true)
    }

    fn create_table_from_schema(&self, table_schema: &TableSchema) -> Result<(), TreatyDbError> {
        trace!("{table_schema:?}");

        let table_name = table_schema.table_name.clone();

        if table_name.contains("_COOP_") {
            warn!("create_table_from_schema - skipping table: {table_name:?}");
            return Ok(());
        }

        let mut cmd = String::from("CREATE TABLE IF NOT EXISTS :tablename ");
        cmd = cmd.replace(":tablename", &table_name);
        cmd += " ( ";

        for column in &table_schema.columns {
            let col_name = column.column_name.clone();
            let col_type = ColumnType::from_u32(column.column_type).data_type_as_string_sqlite();
            let mut col_length = String::from("");

            if column.column_length > 0 {
                col_length += " ( ";
                col_length = col_length + &column.column_length.to_string();
                col_length += " ) ";
            }

            let mut col_nullable = String::from("");

            if !column.is_nullable {
                col_nullable = String::from("NOT NULL");
            }

            let last_column = &table_schema.columns.last().unwrap().column_name;

            let col_statement: String = if *last_column == column.column_name {
                format!(" {col_name} {col_type} {col_length} {col_nullable} ")
            } else {
                format!(" {col_name} {col_type} {col_length} {col_nullable} , ")
            };

            cmd = cmd + &col_statement;
        }
        cmd += " ) ";

        trace!("{cmd:?}");

        self.write(&cmd)?;

        Ok(())
    }

    pub fn insert_data_into_partial_db(
        &self,
        table_name: &str,
        cmd: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        use crate::query_parser::get_values_from_insert_statement;

        let conn = self.conn()?;
        let mut row_id = 0;
        let mut total_rows = 0;

        /*
         * in order to accurately get the row id back from the insert in Sqlite
         * we have to hold open our connection to the database from our INSERT statement.
         * in other words `select last_insert_rowid()` won't return the correct row id
         * if we have closed our connection.
         *
         * this code below isn't working because we close the connection from the statement
         * let total_rows = self.write(cmd)?;
         */

        let result = conn.execute(cmd, []);
        match result {
            Ok(rows) => {
                total_rows = rows;
                if rows > 0 {
                    let cmd = String::from("select last_insert_rowid()");
                    row_id = get_scalar_as_u32(cmd, &conn)?;
                    trace!("[{}]: {row_id:?}", function_name!());
                }
            }
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
            }
        }

        // we need to parse the values of this row
        // and create a data hash for it
        let insert_values = get_values_from_insert_statement(cmd, DatabaseType::Sqlite);
        let hash_value = crypt::calculate_hash_for_struct(&insert_values);

        // we need to determine if there is a metadata table for this table or not
        // and if there is not one, create it
        // then we need to save the data hash along with the row id
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

        trace!("{row_id:?}");
        trace!("{hash_value:?}");

        if row_id == 0 {
            warn!("potentially invalid row_id");
        }

        statement.execute(named_params! {":row": row_id, ":hash" : hash_value.to_ne_bytes() })?;

        drop(statement);

        Ok(PartialDataResult {
            is_successful: total_rows > 0,
            row_id,
            data_hash: Some(hash_value),
            partial_data_status: None,
            action: Some(PartialDataResultAction::Insert),
        })
    }

    pub fn update_data_into_partial_db(
        &self,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host_id: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let behavior = self
            .treaty
            .get_updates_from_host_behavior(&self.db_name, table_name)?;
        match behavior {
            UpdatesFromHostBehavior::AllowOverwrite => {
                self.execute_update_overwrite(table_name, cmd, where_clause)
            }
            UpdatesFromHostBehavior::Unknown => todo!(),
            UpdatesFromHostBehavior::QueueForReview => {
                self.update_data_into_partial_db_queue(table_name, cmd, where_clause, host_id)
            }
            UpdatesFromHostBehavior::OverwriteWithLog => {
                self.execute_update_with_log(table_name, cmd, where_clause)
            }
            UpdatesFromHostBehavior::Ignore => todo!(),
            UpdatesFromHostBehavior::QueueForReviewAndLog => todo!(),
        }
    }

    pub fn update_data_into_partial_db_queue(
        &self,
        table_name: &str,
        update_statement: &str,
        where_clause: &str,
        host_id: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let queue_log_table = get_data_queue_table_name(table_name);
        let conn = self.conn()?;

        if !has_table(&queue_log_table, &conn)? {
            let mut cmd = sql_text::Coop::text_create_data_queue_table();
            cmd = cmd.replace(":table_name", &queue_log_table);
            self.write(&cmd)?;
        }

        let mut cmd = String::from("SELECT MAX(ID) FROM :table_name");
        cmd = cmd.replace(":table_name", &queue_log_table);

        let max_id = get_scalar_as_u32(cmd, &conn)?;
        let next_id = max_id + 1;

        cmd = String::from(
            "
            INSERT INTO :table_name 
            (
                ID,
                STATEMENT,
                WHERE_CLAUSE,
                REQUESTED_TS_UTC,
                HOST_ID,
                ACTION
            )
            VALUES
            (
                :id,
                :statement,
                :where_clause,
                :ts,
                :hid,
                'UPDATE'
            )
        ;",
        );

        cmd = cmd.replace(":table_name", &queue_log_table);

        let mut statement = conn.prepare(&cmd)?;
        let rows_inserted = statement.execute(named_params! {
            ":id": next_id,
            ":statement": update_statement,
            ":where_clause": where_clause,
            ":ts": Utc::now().to_string(),
            ":hid": host_id,
        })?;

        Ok(PartialDataResult {
            is_successful: rows_inserted > 0,
            row_id: next_id,
            data_hash: None,
            partial_data_status: Some(PartialDataStatus::to_u32(PartialDataStatus::Pending)),
            action: Some(PartialDataResultAction::Update),
        })
    }

    fn delete_data_into_partial_db_queue(
        &self,
        table_name: &str,
        delete_statement: &str,
        where_clause: &str,
        host_id: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let queue_log_table = get_data_queue_table_name(table_name);
        let conn = self.conn()?;

        if !has_table(&queue_log_table, &conn)? {
            let mut cmd = sql_text::Coop::text_create_data_queue_table();
            cmd = cmd.replace(":table_name", &queue_log_table);
            self.write(&cmd)?;
        }

        let mut cmd = String::from("SELECT MAX(ID) FROM :table_name");
        cmd = cmd.replace(":table_name", &queue_log_table);

        let max_id = get_scalar_as_u32(cmd, &conn)?;
        let next_id = max_id + 1;

        cmd = String::from(
            "
            INSERT INTO :table_name 
            (
                ID,
                STATEMENT,
                WHERE_CLAUSE,
                REQUESTED_TS_UTC,
                HOST_ID,
                ACTION
            )
            VALUES
            (
                :id,
                :statement,
                :where_clause,
                :ts,
                :hid,
                'DELETE'
            )
        ;",
        );

        cmd = cmd.replace(":table_name", &queue_log_table);

        trace!("[{}] {cmd:?}", function_name!());

        let mut statement = conn.prepare(&cmd).unwrap();
        let rows_affected = statement
            .execute(named_params! {
                ":id": next_id,
                ":statement": delete_statement,
                ":where_clause": where_clause,
                ":ts": Utc::now().to_string(),
                ":hid": host_id,
            })
            .unwrap();

        trace!("[{}] rows_affected: {rows_affected:?}", function_name!());

        Ok(PartialDataResult {
            is_successful: rows_affected > 0,
            row_id: next_id,
            data_hash: None,
            partial_data_status: None,
            action: Some(PartialDataResultAction::Delete),
        })
    }

    pub fn delete_data_in_partial_db(
        &self,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host_id: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let behavior = self
            .treaty
            .get_deletes_from_host_behavior(&self.db_name, table_name)?;

        trace!("[{}] behavior: {behavior:?}", function_name!());

        match behavior {
            DeletesFromHostBehavior::Unknown => todo!(),
            DeletesFromHostBehavior::AllowRemoval => {
                Ok(self.execute_delete(table_name, cmd, where_clause)?)
            }
            DeletesFromHostBehavior::QueueForReview => {
                self.delete_data_into_partial_db_queue(table_name, cmd, where_clause, host_id)
            }
            DeletesFromHostBehavior::DeleteWithLog => {
                self.execute_delete_with_log(table_name, cmd, where_clause)
            }
            DeletesFromHostBehavior::Ignore => todo!(),
            DeletesFromHostBehavior::QueueForReviewAndLog => todo!(),
        }
    }

    pub fn get_data_hash_at_participant(
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

    pub fn read_row_id_from_part_db(
        &self,
        table_name: &str,
        where_clause: &str,
    ) -> Result<u32, TreatyDbError> {
        let conn = self.conn()?;
        let mut cmd = String::from("SELECT ROWID FROM :table_name WHERE :where_clause");
        cmd = cmd.replace(":table_name", table_name);
        cmd = cmd.replace(":where_clause", where_clause);
        get_scalar_as_u32(cmd, &conn)
    }

    fn conn(&self) -> Result<Connection, TreatyDbError> {
        let db_name = self.db_name.clone();
        trace!("[{}]: {db_name:?}", function_name!());
        let db_part_name = if !db_name.contains(".dbpart") {
            let mut _db_part_name = self.db_name.replace(".db", "");
            _db_part_name = _db_part_name.replace(".dbpart", "");
            _db_part_name = format!("{}{}", _db_part_name, String::from(".dbpart"));
            _db_part_name
        } else {
            db_name.to_string()
        };

        let db_path = Path::new(&self.dir).join(&db_part_name);
        trace!("[{}]: {db_path:?}", function_name!());
        Ok(Connection::open(db_path)?)
    }

    fn write(&self, cmd: &str) -> Result<usize, TreatyDbError> {
        let conn = self.conn()?;

        let is_auto_commit = conn.is_autocommit();
        trace!(
            "[{}]: {is_auto_commit:?} for autocommit on db",
            function_name!()
        );

        conn.cache_flush().unwrap();
        if conn.is_busy() {
            warn!("[{}]: {conn:?} is already active", function_name!());
        }

        let result = execute_write(&conn, cmd);

        if let Err(e) = result.clone() {
            error!("[{}]: {e:?}", function_name!());
        }

        match conn.close() {
            Ok(_) => {
                trace!("[{}]: connection closed", function_name!());
            }
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
            }
        }

        result
    }

    fn execute_update_with_log(
        &self,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.add_record_to_log_table(table_name, where_clause, "UPDATE")?;
        self.execute_update_overwrite(table_name, cmd, where_clause)
    }

    fn get_table_col_names_with_data_type_as_string(
        &self,
        table_name: &str,
    ) -> Result<String, TreatyDbError> {
        let table = get_schema_of_table(table_name.to_string(), &self.conn()?).unwrap();

        let mut col_names = String::from("");

        for row in &table.rows {
            let col_name = row.vals[1].data.as_ref().unwrap().data_string.clone();
            let data_type = row.vals[2].data.as_ref().unwrap().data_string.clone();
            col_names = format!("{} {} {}{}", col_names, col_name, data_type, ",");
        }

        trace!("[{}]: {col_names:?}", function_name!());

        let result: &str = &col_names[1..col_names.len() - 1];

        trace!("[{}]: {result:?}", function_name!());

        Ok(result.to_string())
    }

    fn add_record_to_log_table(
        &self,
        table_name: &str,
        where_clause: &str,
        action: &str,
    ) -> Result<bool, TreatyDbError> {
        let data_log_table = get_data_log_table_name(table_name);

        if !has_table(&data_log_table, &self.conn()?)? {
            trace!(
                "[{}]: data log table missing, creating it",
                function_name!()
            );

            let mut cmd = sql_text::Coop::text_create_data_log_table();
            let table_col_names = self.get_table_col_names_with_data_type_as_string(table_name)?;
            cmd = cmd.replace(":column_list", &table_col_names);
            cmd = cmd.replace(":table_name", &data_log_table);

            self.write(&cmd)?;
        }

        // we first need to determine the rows that we're about to overwrite and get them so we can insert them
        let col_names_vec = self.table_col_names(table_name)?;
        let mut col_names = String::from("");
        let mut original_col_names = String::from("");

        for name in &col_names_vec {
            let item = format!("{}{}", name, ",");
            col_names = format!("{col_names}{item}");
            original_col_names = format!("{original_col_names}{item}");
        }

        // remove the final comma from the list of original column names
        original_col_names = original_col_names[0..original_col_names.len() - 1].to_string();

        // for the list of column names, add rowid as a column to get from the db
        col_names = format!("{}{}", col_names, "ROWID");

        let mut select_cmd = String::from("SELECT :col_names FROM :table_name WHERE :where_clause");
        select_cmd = select_cmd.replace(":col_names", &col_names);
        select_cmd = select_cmd.replace(":table_name", table_name);
        select_cmd = select_cmd.replace(":where_clause", where_clause);

        let c = &self.conn()?;
        let mut stmt = get_statement(&select_cmd, &c)?;
        let mut rows = stmt.query([]).unwrap();

        // for every row that we find that we're going to change, we want to insert a copy of it into the data_log_table
        while let Some(row) = rows.next().unwrap() {
            let mut insert_cmd = String::from("INSERT INTO :data_log_table ( :cols, ROW_ID, ACTION, TS_UTC ) VALUES ( :col_vals, :rid, ':action', ':ts_utc') ");
            insert_cmd = insert_cmd.replace(":data_log_table", &data_log_table);
            insert_cmd = insert_cmd.replace(":cols", &original_col_names);

            // need to build the rest of the insert statement - the column values, rowid, etc.
            let mut col_vals = String::from("");
            let total_cols = col_names_vec.len();

            // iterate over the column names and get the value for each as a string
            // remember, the last item is the ROWID, which is not in this list and we will need to get
            for i in 0..col_names_vec.len() {
                let dt = row.get_ref_unwrap(i).data_type();

                let string_value: String = match dt {
                    Type::Blob => String::from(""),
                    Type::Integer => row.get_ref_unwrap(i).as_i64().unwrap().to_string(),
                    Type::Real => row.get_ref_unwrap(i).as_f64().unwrap().to_string(),
                    Type::Text => {
                        format!("'{}'", row.get_ref_unwrap(i).as_str().unwrap())
                    }
                    _ => String::from(""),
                };

                // add the value to the list of values to insert
                col_vals = format!("{}{}{}", col_vals, string_value, ",");
            }

            col_vals = col_vals[0..col_vals.len() - 1].to_string();
            insert_cmd = insert_cmd.replace(":col_vals", &col_vals);

            trace!("{insert_cmd:?}");

            let row_id_val = row.get_ref_unwrap(total_cols).as_i64().unwrap().to_string();

            insert_cmd = insert_cmd.replace(":rid", &row_id_val);
            insert_cmd = insert_cmd.replace(":action", action);
            insert_cmd = insert_cmd.replace(":ts_utc", &Utc::now().to_string());

            trace!("[{}]: {insert_cmd:?}", function_name!());

            // this holds open the database connection for some reason
            // self.write(&insert_cmd)?;

            execute_write(c, &insert_cmd)?;
        }

        drop(rows);
        drop(stmt);
        drop(c);

        Ok(true)
    }

    fn handle_update_pending_action(
        &self,
        db_name: &str,
        table_name: &str,
        sql_update_statement: &str,
        where_clause: &str,
        row_id: u32,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let queue_table_name = get_data_queue_table_name(table_name);

        let behavior = self
            .treaty
            .get_updates_from_host_behavior(db_name, table_name)?;

        let mut update_result = PartialDataResult {
            is_successful: false,
            row_id: 0,
            data_hash: None,
            partial_data_status: None,
            action: Some(PartialDataResultAction::Update),
        };

        if behavior == UpdatesFromHostBehavior::QueueForReview {
            update_result =
                self.execute_update_overwrite(table_name, sql_update_statement, where_clause)?;

            if update_result.is_successful {
                let mut cmd = String::from("DELETE FROM :table_name WHERE ID = :rid");
                cmd = cmd.replace(":table_name", &queue_table_name);
                cmd = cmd.replace(":rid", &row_id.to_string());
                self.write(&cmd)?;
            }
        } else if behavior == UpdatesFromHostBehavior::QueueForReviewAndLog {
            update_result =
                self.execute_update_with_log(table_name, sql_update_statement, where_clause)?;

            if update_result.is_successful {
                let mut cmd = String::from("DELETE FROM :table_name WHERE ID = :rid");
                cmd = cmd.replace(":table_name", &queue_table_name);
                cmd = cmd.replace(":rid", &row_id.to_string());
                self.write(&cmd)?;
            }
        }

        Ok(update_result)
    }

    pub fn accept_pending_action_at_participant(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let mut action_result = PartialDataResult {
            is_successful: false,
            row_id: 0,
            data_hash: None,
            partial_data_status: None,
            action: None,
        };

        let conn = self.get_partial_db_connection(db_name)?;
        let queue_table_name = get_data_queue_table_name(table_name);
        let mut cmd = String::from("SELECT STATEMENT FROM :table_name WHERE ID = :rid");
        cmd = cmd.replace(":table_name", &queue_table_name);
        cmd = cmd.replace(":rid", &row_id.to_string());

        let sql_update_statement = get_scalar_as_string(cmd, &conn)?;
        let mut cmd = String::from("SELECT WHERE_CLAUSE FROM :table_name WHERE ID = :rid");
        cmd = cmd.replace(":table_name", &queue_table_name);
        cmd = cmd.replace(":rid", &row_id.to_string());
        let where_clause = get_scalar_as_string(cmd, &conn)?;

        cmd = String::from("SELECT ACTION FROM :table_name WHERE ID = :rid");
        cmd = cmd.replace(":table_name", &queue_table_name);
        cmd = cmd.replace(":rid", &row_id.to_string());

        let action = get_scalar_as_string(cmd, &conn)?;

        if action == "UPDATE" {
            action_result = self.handle_update_pending_action(
                db_name,
                table_name,
                &sql_update_statement,
                &where_clause,
                row_id,
            )?;
        }

        if action == "DELETE" {
            action_result = self.handle_delete_pending_action(
                table_name,
                &sql_update_statement,
                &where_clause,
                row_id,
            )?;
        }

        Ok(action_result)
    }

    fn execute_delete(
        &self,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let original_cmd = cmd;

        trace!("[{}] {cmd:?}", function_name!());

        let mut cmd;
        cmd = String::from("SELECT ROWID FROM :table_name WHERE :where_clause")
            .replace(":table_name", table_name);

        if !where_clause.is_empty() {
            cmd = cmd.replace(":where_clause", where_clause);
        } else {
            cmd = cmd.replace("WHERE", "");
            cmd = cmd.replace(":where_clause", "");
        }

        // we need to determine the row_ids that we're going to update because we're going to need to delete them
        let conn = self.conn()?;
        let mut statement = conn.prepare(&cmd).unwrap();

        // once we have the row ids, then we will delete the rows in the actual and metadata table

        let mut row_ids: Vec<u32> = Vec::new();
        let row_to_id = |rowid: u32| -> rusqlite::Result<u32> { Ok(rowid) };

        let ids = statement
            .query_and_then([], |row| row_to_id(row.get(0).unwrap()))
            .unwrap();

        for id in ids {
            row_ids.push(id.unwrap());
        }

        trace!("[{}]: {row_ids:?}", function_name!());

        let total_rows = execute_write(&conn, original_cmd)?;

        trace!("[{}]: total rows deleted: {total_rows}", function_name!());

        if total_rows != row_ids.len() {
            error!("the delete statement did not match the expected count of affected rows");
        }

        // now we need to delete data from the metadata table
        let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);
        let mut cmd = String::from("DELETE FROM :table_name WHERE ROW_ID = :rid");
        cmd = cmd.replace(":table_name", &metadata_table_name);

        for row in &row_ids {
            let mut statement = conn.prepare(&cmd).unwrap();
            statement.execute(named_params! {":rid" : row}).unwrap();
            trace!("[{}]: {statement:?}", function_name!());
        }

        let deleted_row_id = if !row_ids.is_empty() {
            row_ids.first().unwrap()
        } else {
            &0
        };

        trace!("[{}]: deleted_row_id: {deleted_row_id:?}", function_name!());

        let result = PartialDataResult {
            is_successful: true,
            row_id: *deleted_row_id,
            data_hash: None,
            partial_data_status: None,
            action: Some(PartialDataResultAction::Delete),
        };

        trace!("[{}]: {result:?}", function_name!());

        Ok(result)
    }

    fn handle_delete_pending_action(
        &self,
        table_name: &str,
        sql_update_statement: &str,
        where_clause: &str,
        row_id: u32,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let conn = self.conn()?;
        let queue_table_name = get_data_queue_table_name(table_name);

        let behavior = self
            .treaty
            .get_deletes_from_host_behavior(&self.db_name, table_name)?;

        let mut update_result = PartialDataResult {
            is_successful: false,
            row_id: 0,
            data_hash: None,
            partial_data_status: None,
            action: Some(PartialDataResultAction::Delete),
        };

        if behavior == DeletesFromHostBehavior::QueueForReview {
            update_result = self.execute_delete(table_name, sql_update_statement, where_clause)?;

            if update_result.is_successful {
                let mut cmd = String::from("DELETE FROM :table_name WHERE ID = :rid");
                cmd = cmd.replace(":table_name", &queue_table_name);
                cmd = cmd.replace(":rid", &row_id.to_string());
                execute_write(&conn, &cmd)?;
            }
        } else if behavior == DeletesFromHostBehavior::QueueForReviewAndLog {
            update_result =
                self.execute_delete_with_log(table_name, sql_update_statement, where_clause)?;

            if update_result.is_successful {
                let mut cmd = String::from("DELETE FROM :table_name WHERE ID = :rid");
                cmd = cmd.replace(":table_name", &queue_table_name);
                cmd = cmd.replace(":rid", &row_id.to_string());
                execute_write(&conn, &cmd)?;
            }
        }

        Ok(update_result)
    }

    fn execute_delete_with_log(
        &self,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.add_record_to_log_table(table_name, where_clause, "DELETE")?;
        self.execute_delete(table_name, cmd, where_clause)
    }
}
