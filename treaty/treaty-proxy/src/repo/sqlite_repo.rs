use super::RepoActions;
use crate::sql_text::sqlite::{
    ADD_LOGIN, GET_HOST, GET_USER, SQLITE_CREATE_LOGIN_TABLE, SQLITE_CREATE_TOKENS_TABLE,
    UPDATE_USER,
};
use crate::user_info::UserInfo;
use crate::TreatyProxyErr;
#[allow(unused_imports)]
use crate::PROXY_DB;
use chrono::Utc;
use rusqlite::named_params;
use rusqlite::{Connection, Result};
use std::path::Path;
use stdext::function_name;
use thiserror::Error;
#[allow(unused_imports)]
use tracing::{debug, trace, warn};

pub struct SqliteRepo {
    db_name: String,
    dir: String,
}

#[derive(Debug, Error)]
pub enum SqliteRepoErr {
    #[error("Error `{0}`")]
    General(String),
}

impl SqliteRepo {
    pub fn new(dir: &str, db_name: &str) -> Self {
        Self {
            db_name: db_name.into(),
            dir: dir.into(),
        }
    }

    fn write(&self, sql: &str) -> Result<usize, SqliteRepoErr> {
        let result = self.conn().execute(sql, []);

        match result {
            Ok(n) => Ok(n),
            Err(e) => Err(SqliteRepoErr::General(e.to_string())),
        }
    }

    fn conn(&self) -> Connection {
        let path = Path::new(&self.dir).join(&self.db_name);
        trace!("[{}]: {path:?}", function_name!());
        Connection::open(path).unwrap()
    }

    fn get_scalar_as_u32(&self, cmd: &str) -> u32 {
        trace!("[{}]: {cmd:?}", function_name!());

        let mut value: u32 = 0;
        let conn = self.conn();
        let mut statement = conn.prepare(cmd).unwrap();
        let rows = statement.query_map([], |row| row.get(0)).unwrap();

        for item in rows {
            value = item.unwrap_or_default();
        }

        drop(statement);

        value
    }

    fn get_scalar_as_string(&self, cmd: &str) -> String {
        trace!("[{}]: {cmd:?}", function_name!());

        let conn = self.conn();
        let mut value = String::from("");
        let mut statement = conn.prepare(cmd).unwrap();
        let rows = statement.query_map([], |row| row.get(0)).unwrap();

        for item in rows {
            value = item.unwrap();
        }

        drop(statement);

        value
    }

    fn has_any_rows_with_table(&self, table_name: &str, where_clause: &str) -> bool {
        let cmd = "SELECT COUNT(*) cnt FROM :table WHERE :clause";
        let mut cmd = cmd.replace(":table", table_name);
        cmd = cmd.replace(":clause", where_clause);
        self.has_any_rows_with_cmd(&cmd)
    }

    fn has_any_rows_with_cmd(&self, cmd: &str) -> bool {
        self.get_scalar_as_u32(cmd) > 0
    }
}

impl RepoActions for SqliteRepo {
    /// Configures the backing database. This will setup the needed tables, etc for work.
    fn config(&self) {
        self.write(SQLITE_CREATE_LOGIN_TABLE).unwrap();
        self.write(SQLITE_CREATE_TOKENS_TABLE).unwrap();
    }

    fn register_user(&self, un: &str, hash: &str) -> Result<(), crate::TreatyProxyErr> {
        let conn = self.conn();
        if !self.has_user(un) {
            let mut statement = conn.prepare(ADD_LOGIN).unwrap();
            let num_rows = statement
                .execute(named_params! { ":un": un, ":hash": hash.as_bytes().to_vec() })
                .unwrap();

            if num_rows > 0 {
                Ok(())
            } else {
                Err(TreatyProxyErr::DbError("Unable to add user".to_string()))
            }
        } else {
            let msg = format!("User '{}' already exists", un);
            Err(TreatyProxyErr::UserAlreadyExists(msg))
        }
    }

    fn save_token(
        &self,
        login: &str,
        token: &str,
        expiration: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), crate::TreatyProxyErr> {
        let conn = self.conn();
        let cmd = String::from(
            "
                INSERT INTO TOKENS
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

        let mut statement = conn.prepare(&cmd).unwrap();
        statement
            .execute(named_params! {
                ":un" : login.to_string(),
                ":token" : token,
                ":issued" : issued,
                ":expiration" : expiration,
            })
            .unwrap();

        Ok(())
    }

    fn revoke_tokens_for_login(&self, un: &str) -> bool {
        let mut cmd = String::from("DELETE FROM TOKENS WHERE USERNAME = ':login'");
        cmd = cmd.replace(":login", un);
        self.write(&cmd).unwrap() > 0
    }

    fn login_has_token(&self, un: &str) -> bool {
        let mut cmd = String::from("SELECT COUNT(*) FROM TOKENS WHERE USERNAME = ':login'");
        cmd = cmd.replace(":login", un);
        self.has_any_rows_with_cmd(&cmd)
    }

    fn get_user_with_token(
        &self,
        token: &str,
    ) -> Result<crate::user_info::UserInfo, crate::TreatyProxyErr> {
        let cmd = "SELECT username FROM TOKENS WHERE token = ':token'";
        let cmd = cmd.replace(":token", token);
        let un = self.get_scalar_as_string(&cmd);
        self.get_user(&un)
    }

    fn verify_token(&self, token: &str) -> bool {
        let mut cmd = String::from("SELECT COUNT(*) FROM TOKENS WHERE TOKEN = ':token'");
        cmd = cmd.replace(":token", token);
        self.has_any_rows_with_cmd(&cmd)
    }

    fn delete_expired_tokens(&self) {
        let conn = self.conn();
        let now = Utc::now().to_rfc3339();

        let mut cmd = String::from("DELETE FROM TOKENS WHERE EXPIRATION_UTC < ':now'");
        cmd = cmd.replace(":now", &now);

        let _ = conn.execute(&cmd, []);
        let _ = conn.close();
    }

    fn has_user(&self, un: &str) -> bool {
        let where_clause = format!("username = '{}'", un);
        self.has_any_rows_with_table("LOGIN", &where_clause)
    }

    fn has_host(&self, id: &str) -> bool {
        let where_clause = format!("host_id = '{}'", id);
        self.has_any_rows_with_table("LOGIN", &where_clause)
    }

    fn get_host(&self, id: &str) -> Result<crate::user_info::UserInfo, crate::TreatyProxyErr> {
        if !self.has_host(id) {
            return Err(TreatyProxyErr::HostIdNotFound(id.to_string()));
        }

        let cmd = GET_HOST;

        let conn = self.conn();
        let mut statement = conn.prepare(cmd).unwrap();

        let row_to_user = |un: String,
                           hash: Vec<u8>,
                           folder: Option<String>,
                           host_id: Option<String>|
         -> Result<UserInfo> {
            Ok(UserInfo {
                username: un,
                hash,
                folder,
                id: host_id,
            })
        };

        let users = statement
            .query_and_then(
                named_params! {
                    ":id": id
                },
                |row| {
                    row_to_user(
                        row.get(0).unwrap(),
                        row.get(1).unwrap(),
                        row.get(2).unwrap(),
                        row.get(3).unwrap(),
                    )
                },
            )
            .unwrap();

        Ok(users.last().unwrap().unwrap())
    }

    fn get_user(&self, un: &str) -> Result<crate::user_info::UserInfo, crate::TreatyProxyErr> {
        if !self.has_user(un) {
            return Err(TreatyProxyErr::UserNotFound(un.to_string()));
        }

        let cmd = GET_USER;
        let conn = self.conn();
        let mut statement = conn.prepare(cmd).unwrap();

        let row_to_user = |un: String,
                           hash: Vec<u8>,
                           folder: Option<String>,
                           host_id: Option<String>|
         -> Result<UserInfo> {
            Ok(UserInfo {
                username: un,
                hash,
                folder,
                id: host_id,
            })
        };

        let users = statement
            .query_and_then(
                named_params! {
                    ":un": un
                },
                |row| {
                    row_to_user(
                        row.get(0).unwrap(),
                        row.get(1).unwrap(),
                        row.get(2).unwrap(),
                        row.get(3).unwrap(),
                    )
                },
            )
            .unwrap();

        Ok(users.last().unwrap().unwrap())
    }

    fn update_user(&self, u: &crate::user_info::UserInfo) -> Result<(), crate::TreatyProxyErr> {
        if !self.has_user(&u.username) {
            return Err(TreatyProxyErr::UserNotFound(u.username.to_string()));
        }

        let cmd = UPDATE_USER;
        let conn = self.conn();
        let mut statement = conn.prepare(cmd).unwrap();
        let r = statement.execute(named_params! {
            ":folder": u.folder.as_ref().unwrap(),
            ":hash": u.hash,
            ":id": u.id,
            ":un": u.username,
        });

        match r {
            Ok(n) => {
                if n > 0 {
                    Ok(())
                } else {
                    warn!("no rows affected for update user {}", u.username);
                    Err(TreatyProxyErr::NoRowsAffected)
                }
            }
            Err(e) => Err(TreatyProxyErr::DbError(e.to_string())),
        }
    }
}
