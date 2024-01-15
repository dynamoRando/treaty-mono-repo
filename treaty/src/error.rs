use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;
use thiserror::Error;
use tracing::error;

use crate::treaty_proto::{
    DeleteDataResult, GetRowFromPartialDatabaseResult, InsertDataResult,
    NotifyHostOfRemovedRowResult, ParticipantAcceptsContractResult, SaveContractResult,
    TreatyError, UpdateDataResult, UpdateRowDataHashForHostResult,
};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum TreatyErrorKind {
    Unknown = 0,
    General = 1,
    DbNotFound = 2,
    TableNotFoundInDatabase = 3,
    LogicalStoragePolicyNotSet = 4,
    DbAlreadyExists = 5,
    MultipleParticipantAlias = 6,
    NoRowsFound = 7,
}

#[derive(Error, Debug, Clone)]
pub enum TreatyDbError {
    #[error("`{0}`")]
    General(String),
    #[error("the database `{0}` does not exist")]
    DbNotFound(String),
    #[error("the table `{0}` does not exist in database `{1}`")]
    TableNotFoundInDatabase(String, String),
    #[error("storage policy not defined for table `{0}`")]
    LogicalStoragePolicyNotSet(String),
    #[error("Database `{0}` already exists")]
    DbAlreadyExists(String),
    #[error("Multiple participant aliases found for `{0}`")]
    MultipleParticipantAlias(String),
    #[error("No rows found for SQL `{0}`")]
    NoRowsFound(String),
}

impl From<rusqlite::Error> for TreatyDbError {
    fn from(error: rusqlite::Error) -> Self {
        TreatyDbError::General(error.to_string())
    }
}

impl From<tokio_postgres::error::Error> for TreatyDbError {
    fn from(error: tokio_postgres::error::Error) -> Self {
        TreatyDbError::General(error.to_string())
    }
}

#[derive(Error, Debug, Clone)]
pub enum TreatyGenerateContractError {
    #[error("`{0}`")]
    General(String),
    #[error("Not all tables have a logical storage policy in database: `{0}`")]
    NotAllTablesSet(String),
}

impl TreatyError {
    pub fn message(msg: &str) -> Self {
        Self {
            message: msg.into(),
            help: None,
            number: 0,
        }
    }

    pub fn message_help(msg: &str, help: &str) -> Self {
        Self {
            message: msg.into(),
            help: Some(help.into()),
            number: 0,
        }
    }
}

impl From<TreatyGenerateContractError> for TreatyError {
    fn from(error: TreatyGenerateContractError) -> Self {
        Self {
            message: error.to_string(),
            help: None,
            number: 0,
        }
    }
}

impl From<TreatyDbError> for TreatyGenerateContractError {
    fn from(error: TreatyDbError) -> Self {
        TreatyGenerateContractError::General(error.to_string())
    }
}

impl From<serde_json::Error> for TreatyError {
    fn from(value: serde_json::Error) -> Self {
        Self {
            message: value.to_string(),
            help: None,
            number: 0,
        }
    }
}

impl From<TreatyDbError> for TreatyError {
    fn from(error: TreatyDbError) -> Self {
        let kind: TreatyErrorKind = match error {
            TreatyDbError::General(_) => TreatyErrorKind::General,
            TreatyDbError::DbNotFound(_) => TreatyErrorKind::DbNotFound,
            TreatyDbError::TableNotFoundInDatabase(_, _) => {
                TreatyErrorKind::TableNotFoundInDatabase
            }
            TreatyDbError::LogicalStoragePolicyNotSet(_) => {
                TreatyErrorKind::LogicalStoragePolicyNotSet
            }
            TreatyDbError::DbAlreadyExists(_) => TreatyErrorKind::DbAlreadyExists,
            TreatyDbError::MultipleParticipantAlias(_) => TreatyErrorKind::MultipleParticipantAlias,
            TreatyDbError::NoRowsFound(_) => TreatyErrorKind::NoRowsFound,
        };

        Self {
            message: error.to_string(),
            help: None,
            number: kind.to_u32().unwrap(),
        }
    }
}

pub trait LogErrorActions {
    fn log_any_err(&self, function_name: &str);
}

impl LogErrorActions for SaveContractResult {
    fn log_any_err(&self, function_name: &str) {
        if self.is_error {
            if let Some(e) = &self.error {
                error!("[{}]: from remote treaty: {e:?}", function_name);
            }
        }
    }
}

impl LogErrorActions for NotifyHostOfRemovedRowResult {
    fn log_any_err(&self, function_name: &str) {
        if self.error.is_some() {
            let e = self.error.as_ref().unwrap();
            error!("[{}]: from remote treaty: {e:?}", function_name);
        }
    }
}

impl LogErrorActions for DeleteDataResult {
    fn log_any_err(&self, function_name: &str) {
        if self.is_error {
            if let Some(e) = &self.error {
                error!("[{}]: from remote treaty: {e:?}", function_name);
            }
        }
    }
}

impl LogErrorActions for UpdateDataResult {
    fn log_any_err(&self, function_name: &str) {
        if self.is_error {
            if let Some(e) = &self.error {
                error!("[{}]: from remote treaty: {e:?}", function_name);
            }
        }
    }
}

impl LogErrorActions for InsertDataResult {
    fn log_any_err(&self, function_name: &str) {
        if self.is_error {
            if let Some(e) = &self.error {
                error!("[{}]: from remote treaty: {e:?}", function_name);
            }
        }
    }
}

impl LogErrorActions for GetRowFromPartialDatabaseResult {
    fn log_any_err(&self, function_name: &str) {
        if self.error.is_some() {
            let e = self.error.as_ref().unwrap();
            error!("[{}]: from remote treaty: {e:?}", function_name);
        }
    }
}

impl LogErrorActions for UpdateRowDataHashForHostResult {
    fn log_any_err(&self, function_name: &str) {
        if self.error.is_some() {
            let e = self.error.as_ref().unwrap();
            error!("[{}]: from remote treaty: {e:?}", function_name);
        }
    }
}

impl LogErrorActions for ParticipantAcceptsContractResult {
    fn log_any_err(&self, function_name: &str) {
        if self.error.is_some() {
            let e = self.error.as_ref().unwrap();
            error!("[{}]: from remote treaty: {e:?}", function_name);
        }
    }
}
