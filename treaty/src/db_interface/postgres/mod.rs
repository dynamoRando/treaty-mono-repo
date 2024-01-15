use async_trait::async_trait;
use stdext::function_name;
use tokio_postgres::{Client, NoTls, Row};
use tracing::{error, trace, warn};
use treaty_types::enums::{ColumnType, ContractStatus, TreatyDatabaseType};

use crate::{
    db_interface::postgres::{
        db::COOPERATIVE_SCHEMA_NAME, part_db::PARTIAL_SCHEMA_NAME, treaty::TreatySystemDb,
    },
    error::TreatyDbError,
    treaty_proto::Contract,
};

use self::{db::TreatyDb, part_db::TreatyPartDb};

use super::dbi_actions::DbiActions;

mod db;
mod part_db;
mod treaty;

pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

/// Takes a native treaty `ColumnType` and converts to the corresponding Postgres type
#[derive(Debug, Clone)]
pub struct ColumnTypePostgres {}

impl ColumnTypePostgres {
    pub fn data_type_to_native_postgres(column_type: &ColumnType) -> postgres::types::Type {
        match column_type {
            ColumnType::Unknown => todo!(),
            ColumnType::Int => todo!(),
            ColumnType::Bit => todo!(),
            ColumnType::Char => todo!(),
            ColumnType::DateTime => todo!(),
            ColumnType::Decimal => todo!(),
            ColumnType::Varchar => postgres::types::Type::VARCHAR,
            ColumnType::Binary => todo!(),
            ColumnType::Varbinary => todo!(),
            ColumnType::Text => todo!(),
        }
    }
}

/// The `dbi` or Database Implementation for Treaty in Postgres. Represents the Postgres server that Treaty will work with.
#[derive(Debug, Clone)]
pub struct PostgresDbi {
    /// The host for the Postgres server
    pub host: String,
    /// The port for the Postgre server. If this is `None` then use the default for Postgres
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

impl From<TreatyDb> for PostgresDbi {
    fn from(value: TreatyDb) -> Self {
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

impl From<TreatyPartDb> for PostgresDbi {
    fn from(value: TreatyPartDb) -> Self {
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

impl PostgresDbi {
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

    pub async fn treaty_system_db(&self) -> TreatySystemDb {
        if self.use_treaty_schema {
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
        } else {
            TreatySystemDb::new(
                &self.host,
                self.port,
                &self.un,
                &self.pw,
                &self.treaty_schema_name,
                None,
                self.use_treaty_schema,
            )
            .await
        }
    }

    pub fn db(&self, db_name: &str) -> TreatyDb {
        TreatyDb::new(
            &self.host,
            self.port,
            &self.un,
            &self.pw,
            &self.treaty_schema_name,
            db_name,
            self.use_treaty_schema,
        )
    }

    pub fn part_db(&self, db_name: &str) -> TreatyPartDb {
        TreatyPartDb::new(
            &self.host,
            self.port,
            &self.un,
            &self.pw,
            &self.treaty_schema_name,
            db_name,
            self.use_treaty_schema,
        )
    }

    pub async fn has_enable_coooperative_features(
        &self,
        db_name: &str,
    ) -> Result<bool, TreatyDbError> {
        if self.has_database(db_name).await? {
            Ok((self.db(db_name).has_schema(COOPERATIVE_SCHEMA_NAME).await?
                && self
                    .db(db_name)
                    .has_table_in_schema("REMOTES", COOPERATIVE_SCHEMA_NAME)
                    .await?)
                || self.db(db_name).has_schema(PARTIAL_SCHEMA_NAME).await?)
        } else {
            Err(TreatyDbError::DbNotFound(db_name.to_string()))
        }
    }

    /// Returns a Treaty Table representing the schema of the requested table
    /// in the following format: column_name, data_type, ordinal_position, is_nullable
    ///
    /// Note that this will be the Postgres native type
    pub async fn get_schema_of_table(
        &self,
        table_name: &str,
        table_schema: &str,
    ) -> Result<crate::models::Table, TreatyDbError> {
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
        self.db(&self.db_name).read(&sql, &client).await
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

    async fn has_database(&self, db_name: &str) -> Result<bool, TreatyDbError> {
        let sql = "select exists(
            SELECT datname FROM pg_catalog.pg_database WHERE lower(datname) = lower(':db_name')
           );";

        let sql = sql.replace(":db_name", db_name);
        self.get_scalar_as_bool(&sql).await
    }

    async fn get_treaty_db_type(&self, db_name: &str) -> Result<TreatyDatabaseType, TreatyDbError> {
        if !self.has_database(db_name).await? {
            return Err(TreatyDbError::DbNotFound(db_name.to_string()));
        }

        if self.has_schema(PARTIAL_SCHEMA_NAME).await? {
            return Ok(TreatyDatabaseType::Partial);
        }

        if self.has_schema(COOPERATIVE_SCHEMA_NAME).await? {
            return Ok(TreatyDatabaseType::Host);
        }

        Ok(TreatyDatabaseType::Unknown)
    }

    pub async fn has_schema(&self, schema: &str) -> Result<bool, TreatyDbError> {
        let sql = "SELECT exists(select schema_name FROM information_schema.schemata WHERE schema_name = ':schema')";
        let sql = sql.replace(":schema", schema);
        self.get_scalar_as_bool(&sql).await
    }

    pub async fn get_scalar_as_u64(&self, sql: &str) -> Result<Option<u64>, TreatyDbError> {
        let rows = self.query_db(sql).await?;

        trace!("[{}]: Rows: {rows:?}", function_name!());

        let row = &rows[0];
        trace!("[{}]: Row: {row:?}", function_name!());
        let result = row.try_get::<usize, Option<Vec<u8>>>(0);

        match result {
            Ok(item) => match item {
                Some(i) => {
                    trace!("[{}]: hash check is: {i:?}", function_name!());
                    let hash = u64::from_ne_bytes(vec_to_array(i));
                    Ok(Some(hash))
                }
                None => Ok(None),
            },
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                Err(TreatyDbError::General(
                    "Unable to get value as u32".to_string(),
                ))
            }
        }
    }

    pub async fn get_scalar_as_u32(&self, sql: &str) -> Result<Option<u32>, TreatyDbError> {
        let rows = self.query_db(sql).await?;

        trace!("[{}]: Rows: {rows:?}", function_name!());

        let row = &rows[0];
        trace!("[{}]: Row: {row:?}", function_name!());
        let result = row.try_get::<usize, Option<i32>>(0);

        match result {
            Ok(item) => match item {
                Some(i) => Ok(Some(i as u32)),
                None => Ok(None),
            },
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                Err(TreatyDbError::General(
                    "Unable to get value as u32".to_string(),
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

    /// Checks to see if the specified table name exists in the specified schema
    async fn has_table_in_schema(
        &self,
        table_name: &str,
        schema: &str,
    ) -> Result<bool, TreatyDbError> {
        let table_name = table_name.to_lowercase();

        let sql = r#"
       SELECT EXISTS (
        SELECT FROM 
            pg_tables
        WHERE 
            schemaname = ':schema' AND 
            tablename  = ':table_name'
        );"#;

        let sql = sql.replace(":table_name", &table_name);
        let sql = sql.replace(":schema", schema);

        let rows = self.query_db(&sql).await?;

        let has_table: bool = rows[0].get(0);

        trace!("[{}]: has_table: {}", function_name!(), has_table);

        Ok(has_table)
    }

    pub async fn get_scalar_as_bool(&self, sql: &str) -> Result<bool, TreatyDbError> {
        let rows = self.query_db(sql).await?;

        trace!("[{}]: Rows: {rows:?}", function_name!());

        let row = &rows[0];
        trace!("[{}]: Row: {row:?}", function_name!());
        let result: Result<bool, tokio_postgres::Error> = row.try_get::<usize, bool>(0);

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

    async fn query_db(&self, sql: &str) -> Result<Vec<Row>, TreatyDbError> {
        let client = self.get_client_for_db(&self.db_name).await?;
        trace!("[{}]: SQL: {}", function_name!(), sql);
        Ok(client.query(sql, &[]).await?)
    }

    async fn get_treaty_database_type(
        &self,
        db_name: &str,
    ) -> Result<TreatyDatabaseType, TreatyDbError> {
        if self.has_database(db_name).await? {
            if self.has_schema(PARTIAL_SCHEMA_NAME).await? {
                return Ok(TreatyDatabaseType::Partial);
            } else if self.has_schema(COOPERATIVE_SCHEMA_NAME).await? && self.use_treaty_schema {
                return Ok(TreatyDatabaseType::Host);
            } else {
                return Ok(TreatyDatabaseType::System);
            }
        }

        Err(TreatyDbError::DbNotFound(String::from(db_name)))
    }
}

#[async_trait]
impl DbiActions for PostgresDbi {
    async fn verify_token(&self, token: &str) -> Result<bool, crate::error::TreatyDbError> {
        Ok(self.treaty_system_db().await.verify_token(token).await?)
    }

    async fn get_cooperative_hosts(
        &self,
    ) -> Result<Option<Vec<crate::models::CdsHosts>>, crate::error::TreatyDbError> {
        self.treaty_system_db().await.get_cooperative_hosts().await
    }

    async fn save_token(
        &self,
        login: &str,
        token: &str,
        expiration: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .save_token(login, token, expiration)
            .await
    }

    async fn auth_for_token(
        &self,
        login: &str,
        pw: &str,
        timeout_in_minutes: Option<u32>,
    ) -> Result<crate::treaty_proto::TokenReply, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .auth_for_token(login, pw, timeout_in_minutes)
            .await
    }

    async fn login_has_token(&self, login: &str) -> Result<bool, crate::error::TreatyDbError> {
        self.treaty_system_db().await.login_has_token(login).await
    }

    async fn revoke_token(&self, token: &str) -> Result<bool, crate::error::TreatyDbError> {
        self.treaty_system_db().await.revoke_token(token).await
    }

    async fn delete_expired_tokens(&self) -> Result<(), crate::error::TreatyDbError> {
        self.treaty_system_db().await.delete_expired_tokens().await
    }

    async fn get_last_log_entries(
        &self,
        number_of_entries: u32,
    ) -> Result<Vec<crate::models::LogEntry>, crate::error::TreatyDbError> {
        todo!("get_last_log_entries")
    }

    async fn create_token_for_login(
        &self,
        login: &str,
        timeout_in_minutes: Option<u32>,
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), crate::error::TreatyDbError> {
        Ok(self
            .treaty_system_db()
            .await
            .create_token_for_login(login, timeout_in_minutes)
            .await?)
    }

    async fn accept_pending_action_at_participant(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<crate::models::PartialDataResult, crate::error::TreatyDbError> {
        self.part_db(db_name)
            .accept_pending_action_at_participant(db_name, table_name, row_id)
            .await
    }

    async fn get_pending_actions(
        &self,
        db_name: &str,
        table_name: &str,
        action: &str,
    ) -> Result<Option<Vec<crate::treaty_proto::PendingStatement>>, crate::error::TreatyDbError>
    {
        self.part_db(db_name)
            .get_pending_actions(table_name, action)
            .await
    }

    async fn get_data_hash_at_host(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Option<u64>, crate::error::TreatyDbError> {
        self.db(db_name)
            .get_data_hash_at_host(table_name, row_id)
            .await
    }

    async fn get_data_hash_at_participant(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<Option<u64>, crate::error::TreatyDbError> {
        self.part_db(db_name)
            .get_data_hash_at_participant(table_name, row_id)
            .await
    }

    async fn read_row_id_from_part_db(
        &self,
        db_name: &str,
        table_name: &str,
        where_clause: &str,
    ) -> Result<u32, crate::error::TreatyDbError> {
        self.part_db(db_name)
            .read_row_id_from_part_db(table_name, where_clause)
            .await
    }

    async fn remove_remote_row_reference_from_host(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.db(db_name)
            .remove_remote_row_reference_from_host(table_name, row_id)
            .await
    }

    async fn get_cds_host_for_part_db(
        &self,
        db_name: &str,
    ) -> Result<Option<crate::models::CdsHosts>, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .get_cds_host_for_part_db(db_name)
            .await
    }

    async fn get_treaty_db_type(
        &self,
        db_name: &str,
    ) -> Result<treaty_types::enums::TreatyDatabaseType, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .get_treaty_db_type(db_name)
            .await
    }

    async fn db_type(&self) -> treaty_types::enums::DatabaseType {
        treaty_types::enums::DatabaseType::Postgres
    }

    async fn get_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<treaty_types::enums::UpdatesToHostBehavior, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .get_updates_to_host_behavior(db_name, table_name)
            .await
    }

    async fn get_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<treaty_types::enums::DeletesToHostBehavior, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .get_deletes_to_host_behavior(db_name, table_name)
            .await
    }

    async fn get_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<treaty_types::enums::DeletesFromHostBehavior, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .get_deletes_from_host_behavior(db_name, table_name)
            .await
    }

    async fn get_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<treaty_types::enums::UpdatesFromHostBehavior, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .get_updates_from_host_behavior(db_name, table_name)
            .await
    }

    async fn change_updates_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .change_updates_from_host_behavior(db_name, table_name, behavior)
            .await
    }

    async fn change_deletes_from_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .change_deletes_from_host_behavior(db_name, table_name, behavior)
            .await
    }

    async fn change_updates_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .change_updates_to_host_behavior(db_name, table_name, behavior)
            .await
    }

    async fn change_deletes_to_host_behavior(
        &self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .change_deletes_to_host_behavior(db_name, table_name, behavior)
            .await
    }

    async fn get_row_from_partial_database(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<crate::treaty_proto::Row, crate::error::TreatyDbError> {
        self.part_db(db_name)
            .get_row_from_partial_database(table_name, row_id)
            .await
    }

    async fn change_host_status_by_id(
        &self,
        host_id: &str,
        status: u32,
    ) -> Result<bool, crate::error::TreatyDbError> {
        todo!("change_host_status_by_id")
    }

    async fn change_host_status_by_name(
        &self,
        host_name: &str,
        status: u32,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .change_host_status_by_name(host_name, status)
            .await
    }

    async fn verify_host_by_id(
        &self,
        host_id: &str,
        token: Vec<u8>,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .verify_host_by_id(host_id, token)
            .await
    }

    async fn verify_host_by_name(
        &self,
        host_name: &str,
        token: Vec<u8>,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .verify_host_by_name(host_name, token)
            .await
    }

    async fn delete_metadata_in_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        internal_participant_id: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.db(db_name)
            .delete_metadata_in_host_db(table_name, row_id, internal_participant_id)
            .await
    }

    async fn update_metadata_in_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.db(db_name)
            .update_metadata_in_host_db(table_name, row_id, hash, internal_participant_id)
            .await
    }

    async fn insert_metadata_into_host_db(
        &self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.db(db_name)
            .insert_metadata_into_host_db(table_name, row_id, hash, internal_participant_id)
            .await
    }

    async fn delete_data_in_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host_id: &str,
    ) -> Result<crate::models::PartialDataResult, crate::error::TreatyDbError> {
        self.part_db(part_db_name)
            .delete_data_in_partial_db(table_name, cmd, where_clause, host_id)
            .await
    }

    async fn update_data_into_partial_db_queue(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host: &crate::models::CdsHosts,
    ) -> Result<crate::models::PartialDataResult, crate::error::TreatyDbError> {
        self.part_db(part_db_name)
            .update_data_into_partial_db_queue(table_name, cmd, where_clause, &host.host_id)
            .await
    }

    async fn update_data_into_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        host_id: &str,
        where_clause: &str,
    ) -> Result<crate::models::PartialDataResult, crate::error::TreatyDbError> {
        self.part_db(part_db_name)
            .update_data_into_partial_db(table_name, cmd, where_clause, host_id)
            .await
    }

    async fn insert_data_into_partial_db(
        &self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
    ) -> Result<crate::models::PartialDataResult, crate::error::TreatyDbError> {
        self.part_db(part_db_name)
            .insert_data_into_partial_db(table_name, cmd)
            .await
    }

    async fn update_participant_accepts_contract(
        &self,
        db_name: &str,
        participant: crate::models::CoopDatabaseParticipant,
        participant_message: crate::treaty_proto::Participant,
        accepted_contract_id: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.db(db_name)
            .update_participant_accepts_contract(
                participant,
                participant_message,
                accepted_contract_id,
            )
            .await
    }

    async fn create_partial_database_from_contract(
        &self,
        contract: &crate::treaty_proto::Contract,
    ) -> Result<bool, crate::error::TreatyDbError> {
        let db_name = contract.schema.as_ref().unwrap().database_name.clone();

        trace!("[{}]: creating partial db: {db_name:?}", function_name!());

        self.part_db(&db_name)
            .create_partial_database_from_contract(contract)
            .await
    }

    async fn accept_pending_contract(
        &self,
        host_name: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .accept_pending_contract(host_name)
            .await
    }

    async fn get_pending_contracts(
        &self,
    ) -> Result<Option<Vec<crate::treaty_proto::Contract>>, crate::error::TreatyDbError> {
        Ok(self
            .treaty_system_db()
            .await
            .get_contracts_by_status(ContractStatus::Pending)
            .await?)
    }

    async fn get_accepted_contracts(
        &self,
    ) -> Result<Option<Vec<crate::treaty_proto::Contract>>, crate::error::TreatyDbError> {
        todo!("get_accepted_contracts")
    }

    async fn save_contract(
        &self,
        contract: crate::treaty_proto::Contract,
    ) -> Result<crate::models::TreatySaveContractResult, crate::error::TreatyDbError> {
        if let Ok(()) = self
            .treaty_system_db()
            .await
            .create_backing_database()
            .await
        {
            self.treaty_system_db()
                .await
                .configure_treaty_schema()
                .await?;
        }

        Ok(self
            .treaty_system_db()
            .await
            .save_contract(contract)
            .await?)
    }

    async fn get_table_id(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<String, crate::error::TreatyDbError> {
        todo!("get_table_id")
    }

    async fn create_table_in_partial_database(
        &self,
        db_name: &str,
        table_name: &str,
        schema: Vec<crate::treaty_proto::ColumnSchema>,
    ) -> Result<bool, crate::error::TreatyDbError> {
        todo!("create_table_in_partial_database")
    }

    async fn get_db_id(&self, db_name: &str) -> Result<String, crate::error::TreatyDbError> {
        todo!("get_db_id")
    }

    async fn create_partial_database(
        &self,
        db_name: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        todo!("create_partial_database")
    }

    async fn has_role_name(&self, role_name: &str) -> Result<bool, crate::error::TreatyDbError> {
        todo!("has_role_name")
    }

    async fn add_login_to_role(
        &self,
        login: &str,
        role_name: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        todo!("add_login_to_role")
    }

    async fn login_is_in_role(
        &self,
        login: &str,
        role_name: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        todo!("login_is_in_role")
    }

    async fn create_login(
        &self,
        login: &str,
        pw: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        todo!("create_login")
    }

    async fn get_database_names(&self) -> Result<Option<Vec<String>>, crate::error::TreatyDbError> {
        Ok(self.treaty_system_db().await.get_database_names().await?)
    }

    async fn has_login(&self, login: &str) -> Result<bool, crate::error::TreatyDbError> {
        todo!("has_login")
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
    ) -> Result<bool, crate::error::TreatyDbError> {
        Ok(self
            .db(db_name)
            .add_participant(alias, ip4addr, db_port, info_port, http_addr, http_port, id)
            .await?)
    }

    async fn get_database_schema(
        &self,
        db_name: &str,
    ) -> Result<crate::treaty_proto::DatabaseSchema, crate::error::TreatyDbError> {
        trace!(
            "[{}]: get database schema for {db_name:?}",
            function_name!()
        );
        Ok(self.db(db_name).get_db_schema().await?)
    }

    async fn get_participant_by_alias(
        &self,
        db_name: &str,
        participant_alias: &str,
    ) -> Result<Option<crate::models::CoopDatabaseParticipant>, crate::error::TreatyDbError> {
        Ok(self
            .db(db_name)
            .get_participant_by_alias(participant_alias)
            .await?)
    }

    async fn get_participant_by_id(
        &self,
        db_name: &str,
        participant_id: &str,
    ) -> Result<Option<crate::models::CoopDatabaseParticipant>, crate::error::TreatyDbError> {
        todo!("get_participant_by_id")
    }

    async fn has_participant(
        &self,
        db_name: &str,
        participant_alias: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        Ok(self.db(db_name).has_participant(participant_alias).await?)
    }

    async fn get_active_contract(
        &self,
        db_name: &str,
    ) -> Result<crate::models::CoopDatabaseContract, crate::error::TreatyDbError> {
        Ok(self.db(db_name).get_active_contract().await?)
    }

    async fn get_logical_storage_policy(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<treaty_types::enums::LogicalStoragePolicy, crate::error::TreatyDbError> {
        self.db(db_name)
            .get_logical_storage_policy(table_name)
            .await
    }

    async fn set_logical_storage_policy(
        &self,
        db_name: &str,
        table_name: &str,
        policy: treaty_types::enums::LogicalStoragePolicy,
    ) -> Result<bool, crate::error::TreatyDbError> {
        Ok(self
            .db(db_name)
            .set_logical_storage_policy(table_name, policy)
            .await?)
    }

    async fn has_table(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.db(db_name)
            .has_table_in_schema(table_name, "public")
            .await
    }

    async fn execute_write_at_host(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<usize, crate::error::TreatyDbError> {
        Ok(self.db(db_name).execute_write_at_host(cmd).await?)
    }

    async fn execute_write_at_partipant(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<usize, crate::error::TreatyDbError> {
        todo!("execute_write_at_partipant")
    }

    async fn execute_read_at_participant(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<crate::models::Table, crate::error::TreatyDbError> {
        self.part_db(db_name).execute_read_at_participant(cmd).await
    }

    async fn execute_read_at_host(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<crate::models::Table, crate::error::TreatyDbError> {
        self.db(db_name).execute_read_at_host(cmd).await
    }

    async fn has_cooperative_tables(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        self.db(db_name).has_cooperative_tables(cmd).await
    }

    async fn get_participants_for_table(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<Option<Vec<crate::models::CoopDatabaseParticipantData>>, crate::error::TreatyDbError>
    {
        self.db(db_name)
            .get_participants_for_table(table_name)
            .await
    }

    #[allow(suspicious_double_ref_op)]
    async fn get_active_contract_proto(
        &self,
        db_name: &str,
    ) -> Result<Option<crate::treaty_proto::Contract>, crate::error::TreatyDbError> {
        let treaty_db_type = self.get_treaty_database_type(db_name).await?;

        match treaty_db_type {
            TreatyDatabaseType::Unknown => {
                return Err(TreatyDbError::General(String::from(
                    "Could not determine database type",
                )))
            }
            TreatyDatabaseType::System => todo!(),
            TreatyDatabaseType::Host | TreatyDatabaseType::SystemAndHost => {
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
            }
            TreatyDatabaseType::Partial => {
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
    }

    async fn get_participants_for_database(
        &self,
        db_name: &str,
    ) -> Result<Option<Vec<crate::treaty_proto::ParticipantStatus>>, crate::error::TreatyDbError>
    {
        self.db(db_name).get_participants_for_database().await
    }

    async fn get_cooperative_tables(
        &self,
        db_name: &str,
        cmd: &str,
    ) -> Result<Option<Vec<String>>, crate::error::TreatyDbError> {
        self.db(db_name).get_cooperative_tables(cmd).await
    }

    async fn create_database(&self, db_name: &str) -> Result<(), crate::error::TreatyDbError> {
        Ok(self.db(db_name).create_database().await?)
    }

    async fn delete_database(&self, db_name: &str) -> Result<(), crate::error::TreatyDbError> {
        todo!("delete_database")
    }

    async fn delete_database_forcefully(
        &self,
        db_name: &str,
    ) -> Result<(), crate::error::TreatyDbError> {
        let sql = format!("DROP DATABASE IF EXISTS {};", db_name);
        let client = self.get_client().await?;
        warn!(
            "[{}]: DROPPING DATABASE FORCEFULLY: {db_name:?}",
            function_name!()
        );
        trace!("[{}]: SQL: {sql:?}", function_name!());
        client.execute(&sql, &[]).await?;
        Ok(())
    }

    async fn enable_coooperative_features(
        &self,
        db_name: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        Ok(self.db(db_name).enable_coooperative_features().await?)
    }

    async fn generate_contract(
        &self,
        db_name: &str,
        host_name: &str,
        desc: &str,
        remote_delete_behavior: treaty_types::enums::RemoteDeleteBehavior,
    ) -> Result<bool, crate::error::TreatyGenerateContractError> {
        self.generate_and_get_host_info(host_name).await?;
        Ok(self
            .db(db_name)
            .generate_contract(desc, &remote_delete_behavior)
            .await?)
    }

    async fn treaty_get_host_info(
        &self,
    ) -> Result<Option<crate::models::HostInfo>, crate::error::TreatyDbError> {
        Ok(self.treaty_system_db().await.treaty_get_host_info().await?)
    }

    async fn treaty_generate_host_info(
        &self,
        host_name: &str,
    ) -> Result<(), crate::error::TreatyDbError> {
        Ok(self
            .treaty_system_db()
            .await
            .treaty_generate_host_info(host_name)
            .await?)
    }

    async fn if_treaty_host_info_exists(&self) -> Result<bool, crate::error::TreatyDbError> {
        todo!("if_treaty_host_info_exists")
    }

    async fn generate_and_get_host_info(
        &self,
        host_name: &str,
    ) -> Result<crate::models::HostInfo, crate::error::TreatyDbError> {
        self.treaty_generate_host_info(host_name).await?;
        Ok(self.treaty_get_host_info().await.unwrap().unwrap())
    }

    async fn configure_admin_hash(
        &self,
        login: &str,
        hash: Vec<u8>,
    ) -> Result<(), crate::error::TreatyDbError> {
        todo!()
    }

    async fn configure_admin(
        &self,
        login: &str,
        pw: &str,
    ) -> Result<(), crate::error::TreatyDbError> {
        self.treaty_system_db()
            .await
            .configure_admin(login, pw)
            .await?;

        Ok(())
    }

    async fn verify_login(
        &self,
        login: &str,
        pw: &str,
    ) -> Result<bool, crate::error::TreatyDbError> {
        Ok(self
            .treaty_system_db()
            .await
            .verify_login(login, pw)
            .await?)
    }

    async fn configure_treaty_db(&self) -> Result<(), crate::error::TreatyDbError> {
        trace!("[{}]: Configuring Postgres...", function_name!());
        self.treaty_system_db()
            .await
            .configure_treaty_schema()
            .await?;

        Ok(())
    }
}
