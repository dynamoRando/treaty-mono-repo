use crate::{
    crypt, defaults,
    error::TreatyDbError,
    models::{
        get_data_log_table_name, get_data_queue_table_name, get_metadata_table_name,
        PartialDataResult, Table,
    },
    sql_text,
    treaty_proto::{
        ColumnSchema, Contract, PendingStatement, Row as TreatyRow, RowValue, TableSchema,
    },
};
use chrono::{NaiveDateTime, Utc};
use stdext::function_name;
use tokio_postgres::{
    types::{Kind, ToSql},
    Client, NoTls, Row as TokioRow,
};
use tracing::{error, trace, warn};
use treaty_types::enums::{
    ColumnType, DatabaseType, DeletesFromHostBehavior, LogicalStoragePolicy,
    PartialDataResultAction, PartialDataStatus, UpdatesFromHostBehavior,
};

use super::{treaty::TreatySystemDb, PostgresDbi};

// / if the Postgres database has this schema, this is a partial database
pub const PARTIAL_SCHEMA_NAME: &str = "part";

#[derive(Debug, Clone)]
pub struct TreatyPartDb {
    /// The host for the Postgres server
    pub host: String,
    /// The port for the Postgres server. If this is `None` then use the default for Postgres
    pub port: Option<u32>,
    /// The un for the server
    pub un: String,
    /// The pw for the server
    pub pw: String,
    /// The name for the `Treaty` schema
    pub treaty_schema_name: String,
    /// The name for the user
    pub db_name: String,
    /// Specifies if Treaty should save it's configuration in the same database
    /// as the specified `db_name` in the `treaty_schema_name`. If this value is false
    /// this means that there should be a dedicated Treaty database named by the constant `TREATY_DEFAULT_NAME`.
    /// Otherwise the user database will also have the Treaty system configuration tables in the `treaty_schema_name` schema.
    pub use_treaty_schema: bool,
}

impl From<PostgresDbi> for TreatyPartDb {
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

impl TreatyPartDb {
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

    pub async fn treaty(&self) -> TreatySystemDb {
        TreatySystemDb::new(
            &self.host,
            self.port,
            &self.un,
            &self.pw,
            &self.treaty_schema_name,
            Some(self.db_name.clone()),
            self.use_treaty_schema,
        )
        .await
    }

    async fn query_db(&self, sql: &str) -> Result<Vec<TokioRow>, TreatyDbError> {
        let client = self.get_client_for_db(&self.db_name).await?;
        trace!("[{}]: SQL: {}", function_name!(), sql);
        Ok(client.query(sql, &[]).await?)
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
                error!("[{}]: {e:?}", function_name!());
                Err(TreatyDbError::General(e.to_string()))
            }
        }
    }

    async fn add_record_to_log_table(
        &self,
        table_name: &str,
        where_clause: &str,
        action: &str,
    ) -> Result<bool, TreatyDbError> {
        let data_log_table = get_data_log_table_name(table_name);

        if !self
            .dbi()
            .has_table_in_schema(&data_log_table, "public")
            .await?
        {
            trace!(
                "[{}]: data log table missing, creating it",
                function_name!()
            );

            let mut cmd = sql_text::Coop::text_create_data_log_table();
            let table_col_names = self
                .get_table_col_names_with_data_type_as_string(table_name)
                .await?;
            cmd = cmd.replace(":column_list", &table_col_names);
            cmd = cmd.replace(":table_name", &data_log_table);
            cmd = cmd.replace("DATETIME", "TIMESTAMP");
            cmd = cmd.replace("ROW_ID", "__treaty_id");

            self.write_for_db(&cmd, &self.db_name).await?;
        }

        // we first need to determine the rows that we're about to overwrite and get them so we can insert them
        let col_names_vec = self.table_col_names(table_name).await?;
        let mut col_names = String::from("");
        let mut original_col_names = String::from("");

        for name in &col_names_vec {
            let item = format!("{}{}", name, ",");
            col_names = format!("{col_names}{item}");
            original_col_names = format!("{original_col_names}{item}");
        }

        // remove the final comma from the list of original column names
        original_col_names = original_col_names[0..original_col_names.len() - 1].to_string();

        // for the list of column names, add __treaty_id as a column to get from the db
        // col_names = format!("{}{}", col_names, "__treaty_id");

        if col_names.ends_with(",") {
            col_names = col_names[0..col_names.len() - 1].to_string();
        }

        let mut select_cmd = String::from("SELECT :col_names FROM :table_name WHERE :where_clause");
        select_cmd = select_cmd.replace(":col_names", &col_names);
        select_cmd = select_cmd.replace(":table_name", table_name);
        select_cmd = select_cmd.replace(":where_clause", where_clause);

        let table_schema = self.dbi().get_schema_of_table(table_name, "public").await?;

        trace!("[{}]: select_cmd: {select_cmd:?}", function_name!());

        let rows = self.query_db(&select_cmd).await?;

        // for every row that we find that we're going to change, we want to insert a copy of it into the data_log_table
        for row in &rows {
            let mut insert_cmd = String::from("INSERT INTO :data_log_table ( :cols, ACTION, TS_UTC ) VALUES ( :col_vals, ':action', ':ts_utc') ");
            insert_cmd = insert_cmd.replace(":data_log_table", &data_log_table);
            insert_cmd = insert_cmd.replace(":cols", &original_col_names);

            // need to build the rest of the insert statement - the column values, rowid, etc.
            let mut col_vals = String::from("");
            let total_cols = col_names_vec.len();

            // iterate over the column names and get the value for each as a string
            // remember, the last item is the ROWID, which is not in this list and we will need to get
            for i in 0..col_names_vec.len() {
                /*
                  SELECT
                    column_name,
                    data_type,
                    ordinal_position,
                    is_nullable
                FROM
                    information_schema.columns
                 */

                // find the data type for this column

                trace!("[{}]: Current index: {i:?}", function_name!());
                trace!("[{}]: Current row: {row:?}", function_name!());
                let column = &row.columns()[i];
                trace!("[{}]: Current column: {column:?}", function_name!());

                let data_type = table_schema.rows[i].vals[1]
                    .data
                    .as_ref()
                    .unwrap()
                    .data_string
                    .clone();
                trace!(
                    "[{}]: Data type for column: {data_type:?}",
                    function_name!()
                );

                let column_type = ColumnType::try_parse(&data_type).expect(&format!(
                    "[{}]: Unknown data type: {data_type:?}",
                    function_name!()
                ));

                let mut string_value = String::from("");

                match column_type {
                    ColumnType::Unknown => todo!(),
                    ColumnType::Int => {
                        trace!("[{}]: Attempting integer", function_name!());
                        match row.try_get::<usize, i32>(i) {
                            Ok(val) => {
                                let value: i32 = row.get(i);
                                string_value = value.to_string();
                            }
                            Err(e) => {
                                error!("[{}]: {e:?}", function_name!());
                            }
                        }
                    }
                    ColumnType::Bit => todo!(),
                    ColumnType::Char => todo!(),
                    ColumnType::DateTime => todo!(),
                    ColumnType::Decimal => todo!(),
                    ColumnType::Varchar => {
                        trace!("[{}]: Attempting varchar", function_name!());
                        match row.try_get::<usize, String>(i) {
                            Ok(val) => {
                                let value: String = row.get(i);
                                string_value = format!("'{}'", value);
                            }
                            Err(e) => {
                                error!("[{}]: {e:?}", function_name!());
                            }
                        }
                    }
                    ColumnType::Binary => todo!(),
                    ColumnType::Varbinary => todo!(),
                    ColumnType::Text => {
                        trace!("[{}]: Attempting text", function_name!());
                        match row.try_get::<usize, String>(i) {
                            Ok(val) => {
                                let value: String = row.get(i);
                                string_value = format!("'{}'", value);
                            }
                            Err(e) => {
                                error!("[{}]: {e:?}", function_name!());
                            }
                        }
                    }
                }

                // add the value to the list of values to insert
                col_vals = format!("{}{}{}", col_vals, string_value, ",");

                trace!("[{}]: col_vals: {col_vals:?}", function_name!());
            }

            col_vals = col_vals[0..col_vals.len() - 1].to_string();
            insert_cmd = insert_cmd.replace(":col_vals", &col_vals);

            trace!("[{}]: insert_cmd: {insert_cmd:?}", function_name!());

            // let row_id_val: i32 = row.get(total_cols);

            // insert_cmd = insert_cmd.replace(":rid", &row_id_val.to_string());
            insert_cmd = insert_cmd.replace(":action", action);
            insert_cmd = insert_cmd.replace(":ts_utc", &Utc::now().to_string());

            trace!("[{}]: insert_cmd: {insert_cmd:?}", function_name!());

            self.write_for_db(&insert_cmd, &self.db_name).await?;
        }

        Ok(true)
    }

    async fn table_col_names(&self, table_name: &str) -> Result<Vec<String>, TreatyDbError> {
        let table = self.dbi().get_schema_of_table(table_name, "public").await?;

        let mut col_names: Vec<String> = Vec::new();

        for row in &table.rows {
            let col_name = row.vals[0].data.as_ref().unwrap().data_string.clone();
            col_names.push(col_name);
        }

        trace!("[{}]: {col_names:?}", function_name!());

        Ok(col_names)
    }

    async fn get_table_col_names_with_data_type_as_string(
        &self,
        table_name: &str,
    ) -> Result<String, TreatyDbError> {
        let table = self.dbi().get_schema_of_table(table_name, "public").await?;

        let mut col_names = String::from("");

        for row in &table.rows {
            let col_name = row.vals[0].data.as_ref().unwrap().data_string.clone();

            if col_name == "__treaty_id" {
                trace!("[{}]: Skipping __treaty_id in schema", function_name!());
                continue;
            }

            let data_type = row.vals[1].data.as_ref().unwrap().data_string.clone();
            col_names = format!("{} {} {}{}", col_names, col_name, data_type, ",");
        }

        trace!("[{}]: {col_names:?}", function_name!());

        let result: &str = &col_names[1..col_names.len() - 1];

        trace!("[{}]: {result:?}", function_name!());

        Ok(result.to_string())
    }

    async fn execute_delete_with_log(
        &self,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.add_record_to_log_table(table_name, where_clause, "DELETE")
            .await?;
        self.execute_delete(table_name, cmd, where_clause).await
    }

    pub async fn execute_read_at_participant(&self, cmd: &str) -> Result<Table, TreatyDbError> {
        self.read(cmd, &self.get_client_for_db(&self.db_name).await?)
            .await
    }

    async fn handle_delete_pending_action(
        &self,
        table_name: &str,
        sql_update_statement: &str,
        where_clause: &str,
        row_id: u32,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let queue_table_name = get_data_queue_table_name(table_name);

        let has_table = self
            .dbi()
            .has_table_in_schema(&queue_table_name, "public")
            .await?;
        if !has_table {
            warn!("[{}]: queue table not found", function_name!());
        }

        let behavior = self
            .treaty()
            .await
            .get_deletes_from_host_behavior(&self.db_name, table_name)
            .await?;

        let mut update_result = PartialDataResult {
            is_successful: false,
            row_id: 0,
            data_hash: None,
            partial_data_status: None,
            action: Some(PartialDataResultAction::Delete),
        };

        if behavior == DeletesFromHostBehavior::QueueForReview {
            trace!("[{}]: Executing delete and queuing", function_name!());

            update_result = self
                .execute_delete(table_name, sql_update_statement, where_clause)
                .await?;

            trace!("[{}]: update_result: {update_result:?}", function_name!());

            if update_result.is_successful {
                let mut cmd = String::from("DELETE FROM :table_name WHERE ID = :rid");
                cmd = cmd.replace(":table_name", &queue_table_name);
                cmd = cmd.replace(":rid", &row_id.to_string());
                self.write_for_db(&cmd, &self.db_name).await?;
            }
        } else if behavior == DeletesFromHostBehavior::QueueForReviewAndLog {
            update_result = self
                .execute_delete_with_log(table_name, sql_update_statement, where_clause)
                .await?;

            trace!("[{}]: update_result: {update_result:?}", function_name!());

            if update_result.is_successful {
                let mut cmd = String::from("DELETE FROM :table_name WHERE ID = :rid");
                cmd = cmd.replace(":table_name", &queue_table_name);
                cmd = cmd.replace(":rid", &row_id.to_string());
                self.write_for_db(&cmd, &self.db_name).await?;
            }
        }

        Ok(update_result)
    }

    pub async fn get_data_hash_at_participant(
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

    pub async fn read_row_id_from_part_db(
        &self,
        table_name: &str,
        where_clause: &str,
    ) -> Result<u32, TreatyDbError> {
        let mut cmd = String::from("SELECT ROWID FROM :table_name WHERE :where_clause");
        cmd = cmd.replace(":table_name", table_name);
        cmd = cmd.replace(":where_clause", where_clause);
        cmd = cmd.replace("ROWID", "__treaty_id");
        match self.dbi().get_scalar_as_u32(&cmd).await? {
            Some(row_id) => Ok(row_id),
            None => Err(TreatyDbError::NoRowsFound(cmd.clone())),
        }
    }

    async fn handle_update_pending_action(
        &self,
        db_name: &str,
        table_name: &str,
        sql_update_statement: &str,
        where_clause: &str,
        row_id: u32,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let queue_table_name = get_data_queue_table_name(table_name);

        let has_table = self
            .dbi()
            .has_table_in_schema(&queue_table_name, "public")
            .await?;
        if !has_table {
            warn!("[{}]: queue table not found", function_name!());
        }

        let behavior = self
            .treaty()
            .await
            .get_updates_from_host_behavior(db_name, table_name)
            .await?;

        let mut update_result = PartialDataResult {
            is_successful: false,
            row_id: 0,
            data_hash: None,
            partial_data_status: None,
            action: Some(PartialDataResultAction::Update),
        };

        if behavior == UpdatesFromHostBehavior::QueueForReview {
            update_result = self
                .execute_update_overwrite(table_name, sql_update_statement, where_clause)
                .await?;

            trace!("[{}]: update_result: {update_result:?}", function_name!());

            if update_result.is_successful {
                let mut cmd = String::from("DELETE FROM :table_name WHERE ID = :rid");
                cmd = cmd.replace(":table_name", &queue_table_name);
                cmd = cmd.replace(":rid", &row_id.to_string());
                self.write_for_db(&cmd, &self.db_name).await?;
            }
        } else if behavior == UpdatesFromHostBehavior::QueueForReviewAndLog {
            update_result = self
                .execute_update_with_log(table_name, sql_update_statement, where_clause)
                .await?;

            trace!("[{}]: update_result: {update_result:?}", function_name!());

            if update_result.is_successful {
                let mut cmd = String::from("DELETE FROM :table_name WHERE ID = :rid");
                cmd = cmd.replace(":table_name", &queue_table_name);
                cmd = cmd.replace(":rid", &row_id.to_string());
                self.write_for_db(&cmd, &self.db_name).await?;
            }
        }

        Ok(update_result)
    }

    pub async fn accept_pending_action_at_participant(
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

        let queue_table_name = get_data_queue_table_name(table_name);

        let has_table = self
            .dbi()
            .has_table_in_schema(&queue_table_name, "public")
            .await?;
        if !has_table {
            warn!("[{}]: queue table not found", function_name!());
        }

        let mut cmd = String::from("SELECT STATEMENT FROM :table_name WHERE ID = :rid");
        cmd = cmd.replace(":table_name", &queue_table_name);
        cmd = cmd.replace(":rid", &row_id.to_string());

        let sql_update_statement = self.dbi().get_scalar_as_string(&cmd).await?;
        let mut cmd = String::from("SELECT WHERE_CLAUSE FROM :table_name WHERE ID = :rid");
        cmd = cmd.replace(":table_name", &queue_table_name);
        cmd = cmd.replace(":rid", &row_id.to_string());
        let where_clause = self.dbi().get_scalar_as_string(&cmd).await?;

        cmd = String::from("SELECT ACTION FROM :table_name WHERE ID = :rid");
        cmd = cmd.replace(":table_name", &queue_table_name);
        cmd = cmd.replace(":rid", &row_id.to_string());

        let action = self.dbi().get_scalar_as_string(&cmd).await?;

        if action == "UPDATE" {
            action_result = self
                .handle_update_pending_action(
                    db_name,
                    table_name,
                    &sql_update_statement,
                    &where_clause,
                    row_id,
                )
                .await?;
        }

        if action == "DELETE" {
            action_result = self
                .handle_delete_pending_action(
                    table_name,
                    &sql_update_statement,
                    &where_clause,
                    row_id,
                )
                .await?;
        }

        Ok(action_result)
    }

    pub async fn get_pending_actions(
        &self,
        table_name: &str,
        action: &str,
    ) -> Result<Option<Vec<PendingStatement>>, TreatyDbError> {
        let update_queue = get_data_queue_table_name(table_name);

        let has_table = self
            .dbi()
            .has_table_in_schema(&update_queue, "public")
            .await?;
        if !has_table {
            warn!("[{}]: queue table not found", function_name!());
        }

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

        let rows = self.query_db(&cmd).await?;

        if rows.is_empty() {
            return Ok(None);
        }

        for row in &rows {
            let row_id: i32 = row.get(0);
            let statement: String = row.get(1);
            let requested_ts_utc: NaiveDateTime = row.get(2);
            let host_id: String = row.get(3);

            let pending_statement = PendingStatement {
                row_id: row_id as u32,
                statement,
                requested_ts_utc: requested_ts_utc.to_string(),
                host_id,
                action: action.to_string(),
            };

            pending_statements.push(pending_statement);
        }

        trace!(
            "[{}]: pending statements: {pending_statements:#?}",
            function_name!()
        );

        Ok(Some(pending_statements))
    }

    async fn delete_data_into_partial_db_queue(
        &self,
        table_name: &str,
        delete_statement: &str,
        where_clause: &str,
        host_id: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let queue_log_table = get_data_queue_table_name(table_name);

        if !self
            .dbi()
            .has_table_in_schema(&queue_log_table, "public")
            .await?
        {
            let mut cmd = sql_text::Coop::text_create_data_queue_table();
            cmd = cmd.replace(":table_name", &queue_log_table);
            cmd = cmd.replace("DATETIME", "TIMESTAMP");
            trace!("[{}]: Creating queue table", function_name!());
            self.write_for_db(&cmd, &self.db_name).await?;
        }

        let mut cmd = String::from("SELECT MAX(ID) FROM :table_name");
        cmd = cmd.replace(":table_name", &queue_log_table);

        let max_id = self.dbi().get_scalar_as_u32(&cmd).await?;

        let next_id = match max_id {
            Some(id) => id + 1,
            None => 1,
        };

        let mut cmd = String::from(
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
        cmd = cmd.replace(":statement", "':statement'");
        cmd = cmd.replace(":where_clause", "':where_clause'");
        cmd = cmd.replace(":ts", "':ts'");
        cmd = cmd.replace(":hid", "':hid'");
        cmd = cmd.replace(":statement", delete_statement);
        cmd = cmd.replace(":where_clause", where_clause);
        cmd = cmd.replace(":hid", host_id);
        cmd = cmd.replace(":ts", &Utc::now().to_string());
        cmd = cmd.replace(":id", &next_id.to_string());

        let rows_affected = self.write_with_count(&cmd).await?;

        let result = PartialDataResult {
            is_successful: rows_affected > 0,
            row_id: next_id,
            data_hash: None,
            partial_data_status: None,
            action: Some(PartialDataResultAction::Delete),
        };

        trace!("[{}]: partial data result: {result:#?}", function_name!());

        Ok(result)
    }

    async fn execute_delete(
        &self,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let original_cmd = cmd;

        trace!("[{}] {cmd:?}", function_name!());

        let mut cmd;
        cmd = String::from("SELECT __treaty_id FROM :table_name WHERE :where_clause")
            .replace(":table_name", table_name);

        if !where_clause.is_empty() {
            cmd = cmd.replace(":where_clause", where_clause);
        } else {
            cmd = cmd.replace("WHERE", "");
            cmd = cmd.replace(":where_clause", "");
        }

        let rows = self.query_db(&cmd).await?;
        let rows_found = rows.len();

        trace!("[{}]: rows found: {rows_found:?}", function_name!());

        let mut row_ids: Vec<u32> = Vec::new();

        for row in rows {
            let row_id: i32 = row.get(0);
            row_ids.push(row_id as u32);
        }

        trace!(
            "[{}]: row_ids as part of delete: {row_ids:?}",
            function_name!()
        );

        let total_rows = self.write_with_count(&original_cmd).await?;

        trace!("[{}]: total rows deleted: {total_rows}", function_name!());
        if total_rows != row_ids.len() as u64 {
            error!("the delete statement did not match the expected count of affected rows");
        }

        let mut deleted_row_id: u32 = 0;

        for row in row_ids {
            if row == 0 {
                warn!("[{}]: row_id is 0!", function_name!());
            }

            // now we need to delete data from the metadata table
            let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);
            let mut cmd = String::from("DELETE FROM :table_name WHERE ROW_ID = :rid");
            cmd = cmd.replace(":table_name", &metadata_table_name);
            cmd = cmd.replace(":rid", &row.to_string());

            let total_rows = self.write_with_count(&cmd).await?;

            deleted_row_id = if total_rows != 0 { row } else { 0 };

            trace!("[{}]: deleted_row_id: {deleted_row_id:?}", function_name!());
        }

        // https://dba.stackexchange.com/questions/292617/restarting-identity-columns-in-postgresql
        // after deleting, we need to reset the __treaty_id count if there are no rows in the table
        // if we don't do this, we might wind up in a situation where what should be a unique id for
        // a row isn't actaully unique. this is because we are faking using the serial column type
        // as a unique id for a row in postgres, which doesn't actually exist. this feature
        // does exist in sqlite though.

        self.reset_row_id(table_name).await?;

        if deleted_row_id == 0 {
            warn!("[{}]: deleted_row_id is 0!", function_name!());
        }

        let result = PartialDataResult {
            is_successful: true,
            row_id: deleted_row_id,
            data_hash: None,
            partial_data_status: None,
            action: Some(PartialDataResultAction::Delete),
        };

        trace!("[{}]: {result:?}", function_name!());

        Ok(result)
    }

    pub async fn update_data_into_partial_db(
        &self,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host_id: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let behavior = self
            .treaty()
            .await
            .get_updates_from_host_behavior(&self.db_name, table_name)
            .await?;

        match behavior {
            UpdatesFromHostBehavior::AllowOverwrite => {
                self.execute_update_overwrite(table_name, cmd, where_clause)
                    .await
            }
            UpdatesFromHostBehavior::Unknown => todo!(),
            UpdatesFromHostBehavior::QueueForReview => {
                self.update_data_into_partial_db_queue(table_name, cmd, where_clause, host_id)
                    .await
            }
            UpdatesFromHostBehavior::OverwriteWithLog => {
                self.execute_update_with_log(table_name, cmd, where_clause)
                    .await
            }
            UpdatesFromHostBehavior::Ignore => todo!(),
            UpdatesFromHostBehavior::QueueForReviewAndLog => todo!(),
        }
    }

    // Returns a list of column names for the specified table
    async fn table_col_names_from_info_schema(
        &self,
        table_name: &str,
    ) -> Result<Vec<(String, String)>, TreatyDbError> {
        let table_name = table_name.to_lowercase();

        let sql = r#"
        SELECT
            column_name,
            data_type
        FROM
            information_schema.columns
        WHERE
            table_name = ':table_name';
        "#;

        let sql = sql.replace(":table_name", &table_name);

        let rows = self.query_db(&sql).await?;

        trace!("[{}]: Rows: {rows:?}", function_name!());

        let mut columns: Vec<(String, String)> = Vec::new();

        for row in &rows {
            let name: String = row.get(0);
            let dt: String = row.get(1);
            columns.push((name, dt));
        }

        Ok(columns)
    }

    async fn execute_update_overwrite(
        &self,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let original_cmd = cmd;
        let mut cmd;
        cmd = String::from("SELECT __treaty_id FROM :table_name WHERE :where_clause")
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
        // once we have the row ids, then we will need to get the hash of the rows after they've been updated.

        let mut row_ids: Vec<u32> = Vec::new();

        let rows = self.query_db(&cmd).await?;

        trace!("[{}]: Rows: {rows:?}", function_name!());

        for row in &rows {
            let id: i32 = row.get(0);
            row_ids.push(id as u32);
        }

        trace!("[{}]: row_ids: {row_ids:?}", function_name!());

        let total_rows = self.write_with_count(original_cmd).await?;

        trace!("[{}]: total_rows: {row_ids:?}", function_name!());

        if total_rows != row_ids.len() as u64 {
            error!("[{}]: The update statement did not match the expected count of affected rows. This should not happen.", function_name!());
        }

        // now we need to update the data hashes for every row that was changed
        let col_names = self.table_col_names_from_info_schema(table_name).await?;

        trace!("[{}]: column_names: {col_names:?}", function_name!());

        let mut cmd;
        cmd = String::from("SELECT :col_names FROM :table_name WHERE __treaty_id = :rid");
        cmd = cmd.replace(":table_name", table_name);

        let mut col_name_list = String::from("");

        for name in &col_names {
            col_name_list = col_name_list + &name.0 + ",";
        }

        let completed_col_name_list = &col_name_list[0..&col_name_list.len() - 1];

        cmd = cmd.replace(":col_names", completed_col_name_list);

        let mut row_hashes: Vec<(u32, u64)> = Vec::new();

        for id in &row_ids {
            let sql = cmd.replace(":rid", &id.to_string());
            let table = self
                .read(&sql, &self.get_client_for_db(&self.db_name).await?)
                .await?;

            // we need to get the data type for the row
            // convert it to a string value
            // and then calculate a hash for it
            for row in &table.rows {
                let mut row_values: Vec<String> = Vec::new();

                for value in &row.vals {
                    match &value.data {
                        Some(data) => {
                            row_values.push(data.data_string.clone());
                        }
                        None => {
                            error!(
                                "[{}]: No string representation for {value:?}",
                                function_name!()
                            );
                        }
                    }
                }

                let hash_value = crypt::calculate_hash_for_struct(&row_values);
                let row_hash: (u32, u64) = (*id, hash_value);
                row_hashes.push(row_hash);
            }
        }

        // now that we have the row hashes, we should save them back to the table
        let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);
        let mut cmd = String::from("UPDATE :table_name SET HASH = $1::BYTEA WHERE ROW_ID = :rid");
        cmd = cmd.replace(":table_name", &metadata_table_name);

        for row in &row_hashes {
            let row_id = row.0;
            trace!(
                "[{}]: Updating data hash for row: {row_id:?}",
                function_name!()
            );
            let cmd = cmd.replace(":rid", &row.0.to_string());
            let hash = row.1.to_ne_bytes().to_vec();

            trace!("[{}]: hash check is: {hash:?}", function_name!());

            let rows_affected = self
                .write_for_db_with_param(&cmd, &self.db_name, &[&hash])
                .await?;
            trace!("[{}]: Rows affected: {rows_affected:?}", function_name!());
        }

        let row_data = row_hashes.first().unwrap();

        trace!(
            "[{}]: hash check row_data is: {row_data:?}",
            function_name!()
        );

        let final_result = PartialDataResult {
            is_successful: true,
            row_id: row_data.0,
            data_hash: Some(row_data.1),
            partial_data_status: Some(1),
            action: Some(PartialDataResultAction::Update),
        };

        trace!("[{}]: final_result: {final_result:?}", function_name!());

        Ok(final_result)
    }

    pub async fn update_data_into_partial_db_queue(
        &self,
        table_name: &str,
        update_statement: &str,
        where_clause: &str,
        host_id: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let queue_log_table = get_data_queue_table_name(table_name);

        if !self
            .dbi()
            .has_table_in_schema(&queue_log_table, "public")
            .await?
        {
            trace!("[{}]: Creating queue log table", function_name!());
            let mut cmd = sql_text::Coop::text_create_data_queue_table();
            cmd = cmd.replace(":table_name", &queue_log_table);
            cmd = cmd.replace("DATETIME", "TIMESTAMP");
            self.write_for_db(&cmd, &self.db_name).await?;
        }

        let mut cmd = String::from("SELECT MAX(ID) FROM :table_name");
        cmd = cmd.replace(":table_name", &queue_log_table);

        let next_id;
        let max_id = self.dbi().get_scalar_as_u32(&cmd).await?;

        if let Some(max_id) = max_id {
            next_id = max_id + 1;
        } else {
            next_id = 1;
        }

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
                $1::TEXT,
                :where_clause,
                :ts,
                :hid,
                'UPDATE'
            )
        ;",
        );

        cmd = cmd.replace(":table_name", &queue_log_table);

        cmd = cmd.replace(":where_clause", "':where_clause'");
        cmd = cmd.replace(":ts", "':ts'");
        cmd = cmd.replace(":hid", "':hid'");

        cmd = cmd.replace(":id", &next_id.to_string());
        cmd = cmd.replace(":hid", host_id);
        cmd = cmd.replace(":ts", &Utc::now().to_string());
        cmd = cmd.replace(":where_clause", where_clause);

        let rows_inserted = self
            .write_for_db_with_param(&cmd, &self.db_name, &[&update_statement])
            .await?;

        trace!("[{}]: rows_inserted: {rows_inserted:?}", function_name!());

        Ok(PartialDataResult {
            is_successful: rows_inserted > 0,
            row_id: next_id,
            data_hash: None,
            partial_data_status: Some(PartialDataStatus::to_u32(PartialDataStatus::Pending)),
            action: Some(PartialDataResultAction::Update),
        })
    }

    async fn execute_update_with_log(
        &self,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        self.add_record_to_log_table(table_name, where_clause, "UPDATE")
            .await?;
        self.execute_update_overwrite(table_name, cmd, where_clause)
            .await
    }

    async fn write_with_count(&self, sql: &str) -> Result<u64, TreatyDbError> {
        trace!("[{}]: SQL: {}", function_name!(), sql);
        let client = self.get_client_for_db(&self.db_name).await?;
        Ok(client.execute(sql, &[]).await?)
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

    pub async fn delete_data_in_partial_db(
        &self,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host_id: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        let behavior = self
            .treaty()
            .await
            .get_deletes_from_host_behavior(&self.db_name, table_name)
            .await?;

        trace!("[{}] behavior: {behavior:?}", function_name!());

        match behavior {
            DeletesFromHostBehavior::Unknown => todo!(),
            DeletesFromHostBehavior::AllowRemoval => {
                Ok(self.execute_delete(table_name, cmd, where_clause).await?)
            }
            DeletesFromHostBehavior::QueueForReview => {
                self.delete_data_into_partial_db_queue(table_name, cmd, where_clause, host_id)
                    .await
            }
            DeletesFromHostBehavior::DeleteWithLog => {
                self.execute_delete_with_log(table_name, cmd, where_clause)
                    .await
            }
            DeletesFromHostBehavior::Ignore => todo!(),
            DeletesFromHostBehavior::QueueForReviewAndLog => todo!(),
        }
    }

    async fn write(&self, sql: &str) -> Result<(), TreatyDbError> {
        let sql = sql.replace(":schema", &self.treaty_schema_name);
        trace!("[{}]: SQL: {}", function_name!(), sql);

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

    pub async fn create_partial_database_from_contract(
        &self,
        contract: &Contract,
    ) -> Result<bool, TreatyDbError> {
        trace!("[{}]: contract: {contract:#?}", function_name!());
        self.create_partial_database().await?;

        for table in &contract.schema.as_ref().unwrap().tables {
            self.create_table_from_schema(table).await?;
        }

        Ok(true)
    }

    async fn reset_row_id(&self, table_name: &str) -> Result<(), TreatyDbError> {
        // debug
        let sql = r#"SELECT COUNT(*) TOTALROWS FROM :table_name"#;
        let sql = sql.replace(":table_name", table_name);
        let total_rows_in_table = self.query_db(&sql).await?;
        let total_rows_in_table: i64 = total_rows_in_table[0].get(0);

        trace!(
            "[{}]: total_rows_in_table: {total_rows_in_table:?}",
            function_name!()
        );

        if total_rows_in_table == 0 {
            let mut cmd = r#"
            SELECT 
                pg_catalog.setval(pg_get_serial_sequence(':table_name', '__treaty_id'), coalesce(MAX(__treaty_id), 1), false)
            FROM 
                :table_name 
            ;
            "#.to_string();
            cmd = cmd.replace(":table_name", table_name);
            trace!(
                "[{}]: Resetting __treaty_id because no rows",
                function_name!()
            );
            trace!("[{}]: SQL: {cmd:?}", function_name!());
            self.write_for_db(&cmd, &self.db_name).await?;

            /*
                // calling nextval advances the sequence
                let cmd = r#"SELECT pg_catalog.nextval(pg_get_serial_sequence(':table_name', '__treaty_id'));"#;
                let cmd = cmd.replace(":table_name", table_name);

                let rows = self.query_db(&cmd).await?;
                let next_id: i64 = rows[0].get(0);
                trace!(
                    "[{}]: The next value in the sequence is: {next_id:?}",
                    function_name!()
                );
            */
        }

        Ok(())
    }

    pub async fn insert_data_into_partial_db(
        &self,
        table_name: &str,
        cmd: &str,
    ) -> Result<PartialDataResult, TreatyDbError> {
        use crate::query_parser::get_values_from_insert_statement;

        self.reset_row_id(table_name).await?;

        // we'll need to make some changes here from Sqlite
        // because Postgres doesn't have `last_insert_rowid()`
        // so we'll need to figure out a way to give each row a unique identifier
        // that doesn't change
        //
        // references:
        // https://stackoverflow.com/questions/48129465/postgresql-equivalent-to-sqlite-rowid
        // https://stackoverflow.com/questions/14626481/rowid-equivalent-in-postgres-9-2
        // may look at creating a SERIAL column for every table that is intended for treaty purposes
        // such as __treaty_id as a SERIAL column added onto tables

        let mut row_id = 0;

        let rows_affected_count = self.write_for_db(cmd, &self.db_name).await?;
        let total_rows = rows_affected_count;
        if total_rows > 0 {
            let sql = "SELECT MAX(__treaty_id) maxid FROM :table";
            let sql = sql.replace(":table", table_name);
            let row_treaty_id = self.get_scalar_as_u32(&sql).await?;
            trace!(
                "[{}]: last inserted row_id: {row_treaty_id:?}",
                function_name!()
            );
            row_id = row_treaty_id;
        }

        // we need to parse the values of this row
        // and create a data hash for it
        let insert_values = get_values_from_insert_statement(cmd, DatabaseType::Postgres);
        let hash_value = crypt::calculate_hash_for_struct(&insert_values);

        // we need to determine if there is a metadata table for this table or not
        // and if there is not one, create it
        // then we need to save the data hash along with the row id
        let metadata_table_name = get_metadata_table_name(table_name);

        let dbi: PostgresDbi = self.clone().into();

        if !dbi
            .has_table_in_schema(&metadata_table_name, "public")
            .await?
        {
            // need to create table
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
        warn!("[{}]: Ignoring Participant Id", function_name!());
        cmd = cmd.replace(":pid", "''");

        trace!("{row_id:?}");
        trace!("{hash_value:?}");

        if row_id == 0 {
            warn!("potentially invalid row_id");
        }

        trace!("[{}]: SQL: {}", function_name!(), cmd);
        let client = self.get_client_for_db(&self.db_name).await?;
        warn!("[{}]: Converting u64 to Vec<u8>", function_name!());
        let hash_value_vec = hash_value.to_ne_bytes().to_vec();

        trace!("[{}]: hash check is: {hash_value_vec:?}", function_name!());

        let result = client.execute(&cmd, &[&hash_value_vec]).await?;

        if result == 0 {
            warn!("[{}]: no rows inserted", function_name!());
        }

        Ok(PartialDataResult {
            is_successful: total_rows > 0,
            row_id,
            data_hash: Some(hash_value),
            partial_data_status: None,
            action: Some(PartialDataResultAction::Insert),
        })
    }

    pub async fn get_row_from_partial_database(
        &self,
        table_name: &str,
        row_id: u32,
    ) -> Result<TreatyRow, TreatyDbError> {
        let mut cmd = String::from("SELECT * from :table_name WHERE ROWID = :rid");

        cmd = cmd.replace(":table_name", table_name);
        cmd = cmd.replace(":rid", &row_id.to_string());

        self.execute_read_for_row(&self.db_name, table_name, row_id, cmd)
            .await
    }

    /// Similiar to `execute_read_on_connection_for_row` - reads the database
    /// and creates a TreatySchema to return
    async fn execute_read_for_row(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        cmd: String,
    ) -> Result<TreatyRow, TreatyDbError> {
        trace!("[{}]: SQL {cmd:?}", function_name!());
        trace!("[{}]: Re-writing ROWID TO __treaty_id", function_name!());
        let cmd = cmd.replace("ROWID", "__treaty_id");

        let client = self.get_client_for_db(&self.db_name).await?;
        let table = self.read(&cmd, &client).await?;

        trace!("[{}]: Table: {table:#?}", function_name!());

        if table.rows.is_empty() {
            warn!(
                "[{}]: No rows found to convert to Treaty table. Defaulting all values.",
                function_name!()
            );
            return Ok(TreatyRow::default());
        }

        let u_values = &table.rows.first().as_ref().unwrap().vals;
        let mut values: Vec<RowValue> = Vec::new();

        for value in u_values {
            let column = &value.col;

            let cs = ColumnSchema {
                column_name: column.name.clone(),
                column_type: column.data_type_to_enum_u32(),
                column_length: column.data_type_len(),
                is_nullable: column.is_nullable,
                ordinal: column.idx as u32,
                table_id: String::from(""),
                column_id: String::from(""),
                is_primary_key: column.is_primary_key,
            };

            let rv = RowValue {
                column: Some(cs),
                is_null_value: value.is_null(),
                value: value.data.as_ref().unwrap().data_byte.clone(),
                string_value: value.data.as_ref().unwrap().data_string.clone(),
            };
            values.push(rv);
        }

        let mut cmd = String::from("SELECT HASH FROM :table_name WHERE ROW_ID = :rid");
        let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);
        cmd = cmd.replace(":table_name", &metadata_table_name);
        cmd = cmd.replace(":rid", &row_id.to_string());

        let rows = self.query_db(&cmd).await?;

        trace!("[{}]: Rows: {rows:?}", function_name!());

        if rows.is_empty() {
            return Err(TreatyDbError::NoRowsFound(cmd.clone()));
        }

        let row = &rows[0];
        let hash: Vec<u8> = row.get(0);

        let result = TreatyRow {
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            row_id,
            values,
            is_remoteable: true,
            remote_metadata: None,
            hash,
        };

        trace!("[{}]: {result:#?}", function_name!());

        Ok(result)
    }

    /// Executes the requested query and returns a Treaty table
    /// representing the returned results
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
                                    data_byte: value.to_string().as_bytes().to_vec(),
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
                                    data_string: value.clone(),
                                    data_byte: value.clone().to_string().as_bytes().to_vec(),
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
                                    data_string: value.clone(),
                                    data_byte: value.clone().to_string().as_bytes().to_vec(),
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

    async fn create_partial_database(&self) -> Result<bool, TreatyDbError> {
        let dbi: PostgresDbi = self.clone().into();
        let db_name = self.db_name.clone();
        let use_treaty_schema = self.use_treaty_schema;
        trace!("[{}]: checking for db_name: {db_name:?}", function_name!());
        trace!(
            "[{}]: use_treaty_schema: {use_treaty_schema:?}",
            function_name!()
        );

        let has_database = dbi.has_database(&db_name).await?;

        trace!("[{}]: has_database: {has_database:?}", function_name!());

        let not_use_treaty_schema = !use_treaty_schema;
        let not_has_database = !has_database;

        if !has_database && !use_treaty_schema {
            trace!(
                "[{}]: creating new partial only database...",
                function_name!()
            );
            let sql = format!("CREATE DATABASE {} ;", db_name);
            self.write(&sql).await?;
            Ok(true)
        } else if has_database && use_treaty_schema {
            trace!(
                "[{}]: creating partial schema in treaty database...",
                function_name!()
            );
            let sql = "CREATE SCHEMA IF NOT EXISTS :schema ;";
            let sql = sql.replace(":schema", PARTIAL_SCHEMA_NAME);
            self.write_for_db(&sql, &self.db_name).await?;

            return Ok(true);
        } else {
            warn!(
                "[{}]: use_treaty_schema is: {use_treaty_schema:?} and database exists: {has_database:?}.",
                function_name!()
            );
            return Err(TreatyDbError::DbAlreadyExists(db_name));
        }
    }

    async fn create_table_from_schema(
        &self,
        table_schema: &TableSchema,
    ) -> Result<(), TreatyDbError> {
        trace!("[{}]: table_schema: {table_schema:?}", function_name!());
        let table_name = table_schema.table_name.clone();

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
                // Postgres does not have a unique row identifier ROWID like Sqlite does, so
                // we will need a treaty specific unique identifier for rows
                //
                // references
                // https://stackoverflow.com/questions/14626481/rowid-equivalent-in-postgres-9-2
                // https://stackoverflow.com/questions/48129465/postgresql-equivalent-to-sqlite-rowid
                format!(" {col_name} {col_type} {col_length} {col_nullable},  __treaty_id SERIAL ")
            } else {
                format!(" {col_name} {col_type} {col_length} {col_nullable} , ")
            };

            cmd = cmd + &col_statement;
        }

        cmd += " ) ";

        trace!("{cmd:?}");

        self.write_for_db(&cmd, &self.db_name).await?;

        let logical_storage_policy =
            LogicalStoragePolicy::from_u32(table_schema.logical_storage_policy);
        match logical_storage_policy {
            LogicalStoragePolicy::None => {}
            LogicalStoragePolicy::HostOnly => {}
            LogicalStoragePolicy::ParticipantOwned => {}
            LogicalStoragePolicy::Shared => {
                trace!("[{}]: LogicalStoragePolicy is Shared, defaulting table {} to UpdatesFromHostBehavior::QueueForReview", 
                function_name!(),
            table_name);

                let behavior = UpdatesFromHostBehavior::QueueForReview;
                let behavior = UpdatesFromHostBehavior::to_u32(behavior);

                self.treaty()
                    .await
                    .change_updates_from_host_behavior(&self.db_name, &table_name, behavior)
                    .await
                    .unwrap();

                let behavior = DeletesFromHostBehavior::QueueForReview;
                let behavior = DeletesFromHostBehavior::to_u32(behavior);

                trace!("[{}]: LogicalStoragePolicy is Shared, defaulting table {} to DeletesFromHostBehavior::QueueForReview", 
                function_name!(),
            table_name);

                self.treaty()
                    .await
                    .change_deletes_from_host_behavior(&self.db_name, &table_name, behavior)
                    .await
                    .unwrap();
            }
            LogicalStoragePolicy::Mirror => {
                trace!("[{}]: LogicalStoragePolicy is Mirror, defaulting table {} to UpdatesFromHostBehavior::OverwriteWithLog", 
                function_name!(),
            table_name);

                let behavior = UpdatesFromHostBehavior::OverwriteWithLog;
                let behavior = UpdatesFromHostBehavior::to_u32(behavior);

                self.treaty()
                    .await
                    .change_updates_from_host_behavior(&self.db_name, &table_name, behavior)
                    .await
                    .unwrap();

                let behavior = DeletesFromHostBehavior::DeleteWithLog;
                let behavior = DeletesFromHostBehavior::to_u32(behavior);

                trace!("[{}]: LogicalStoragePolicy is Mirror, defaulting table {} to DeletesFromHostBehavior::DeleteWithLog", 
                function_name!(),
            table_name);

                self.treaty()
                    .await
                    .change_deletes_from_host_behavior(&self.db_name, &table_name, behavior)
                    .await
                    .unwrap();
            }
        }

        Ok(())
    }

    pub async fn has_schema(&self, schema: &str) -> Result<bool, TreatyDbError> {
        let sql = "select exists (select schema_name
        from information_schema.schemata
        where schema_name = ':schema_name');";

        let sql = sql.replace(":schema_name", schema);

        self.get_scalar_as_bool(&sql).await
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
}
