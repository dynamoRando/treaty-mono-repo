use treaty_types::enums::*;

pub(crate) mod sqlite;

/// Takes a SQL statement and returns a list of tables involved in that SQL statement
pub fn get_table_names(cmd: &str, db_type: DatabaseType) -> Vec<String> {
    match db_type {
        DatabaseType::Unknown => todo!(),
        DatabaseType::Sqlite => sqlite::get_table_names(cmd),
        DatabaseType::Mysql => todo!(),
        DatabaseType::Postgres => todo!(),
        DatabaseType::Sqlserver => todo!(),
    }
}

pub fn get_values_from_insert_statement(
    insert_statement: &str,
    db_type: DatabaseType,
) -> Vec<String> {
    match db_type {
        DatabaseType::Unknown => todo!(),
        DatabaseType::Sqlite => sqlite::get_values_from_insert_statement(insert_statement),
        DatabaseType::Mysql => todo!(),
        DatabaseType::Postgres => todo!(),
        DatabaseType::Sqlserver => todo!(),
    }
}

#[allow(dead_code)]
pub fn get_table_name(cmd: &str, db_type: DatabaseType) -> String {
    match db_type {
        DatabaseType::Unknown => todo!(),
        DatabaseType::Sqlite => sqlite::get_table_name(cmd),
        DatabaseType::Mysql => todo!(),
        DatabaseType::Postgres => todo!(),
        DatabaseType::Sqlserver => todo!(),
    }
}

#[allow(dead_code)]
pub fn determine_dml_type(cmd: &str, db_type: DatabaseType) -> DmlType {
    match db_type {
        DatabaseType::Mysql => unimplemented!(),
        DatabaseType::Unknown => unimplemented!(),
        DatabaseType::Sqlite => sqlite::determine_statement_type(cmd.to_string()),
        DatabaseType::Postgres => unimplemented!(),
        DatabaseType::Sqlserver => unimplemented!(),
    }
}
