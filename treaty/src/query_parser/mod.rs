use stdext::function_name;
use tracing::warn;
use treaty_types::enums::*;

pub(crate) mod sqlite;

/// Takes a SQL statement and returns a list of tables involved in that SQL statement
pub fn get_table_names(cmd: &str, db_type: DatabaseType) -> Vec<String> {
    match db_type {
        DatabaseType::Unknown => todo!(),
        DatabaseType::Sqlite => sqlite::get_table_names(cmd),
        DatabaseType::Postgres => {
            warn!("[{}]: Using Sqlite parser for Postgres", function_name!());
            sqlite::get_table_names(cmd)
        }
    }
}

pub fn get_values_from_insert_statement(
    insert_statement: &str,
    db_type: DatabaseType,
) -> Vec<String> {
    match db_type {
        DatabaseType::Unknown => todo!(),
        DatabaseType::Sqlite => sqlite::get_values_from_insert_statement(insert_statement),
        DatabaseType::Postgres => {
            warn!("[{}]: Using Sqlite parser for Postgres", function_name!());
            sqlite::get_values_from_insert_statement(insert_statement)
        }
    }
}

#[allow(dead_code)]
pub fn get_table_name(cmd: &str, db_type: DatabaseType) -> String {
    match db_type {
        DatabaseType::Unknown => todo!(),
        DatabaseType::Sqlite => sqlite::get_table_name(cmd),
        DatabaseType::Postgres => {
            warn!("[{}]: Using Sqlite parser for Postgres", function_name!());
            sqlite::get_table_name(cmd)
        }
    }
}

#[allow(dead_code)]
pub fn determine_dml_type(cmd: &str, db_type: DatabaseType) -> DmlType {
    match db_type {
        DatabaseType::Unknown => unimplemented!(),
        DatabaseType::Sqlite => sqlite::determine_statement_type(cmd.to_string()),
        DatabaseType::Postgres => {
            warn!("[{}]: Using Sqlite parser for Postgres", function_name!());
            sqlite::determine_statement_type(cmd.to_string())
        }
    }
}
