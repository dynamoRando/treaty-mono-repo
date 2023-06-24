use treaty_types::enums::DmlType;
use antlr_rust::{
    common_token_stream::CommonTokenStream, token_factory::CommonTokenFactory, InputStream,
};
use tracing::trace;

use self::{
    sqlitelexer::SQLiteLexer,
    sqliteparser::SQLiteParser,
    treaty_insert_sqlite_listener::{InsertData, TreatyInsertSqliteListener},
    treaty_sqlite_listener::{DmlData, TreatySqliteListener},
};

mod sqlitelexer;
mod sqlitelistener;
mod sqliteparser;
mod treaty_insert_sqlite_listener;
mod treaty_sqlite_listener;

pub fn get_table_names(cmd: &str) -> Vec<String> {
    let tf = CommonTokenFactory::default();
    let input = InputStream::new(cmd);
    let lexer = SQLiteLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = SQLiteParser::new(token_source);

    let treaty_listener = TreatySqliteListener {
        statement_type: Box::new(DmlData {
            data: DmlType::Unknown,
            table_name: String::from(""),
            table_name_collection: Vec::new(),
        }),
    };

    let listener_id = parser.add_parse_listener(Box::new(treaty_listener));
    let _ = parser.parse();
    let item = parser.remove_parse_listener(listener_id);

    item.statement_type.table_name_collection
}

#[allow(dead_code)]
pub fn get_table_name(cmd: &str) -> String {
    let tf = CommonTokenFactory::default();
    let input = InputStream::new(cmd);
    let lexer = SQLiteLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = SQLiteParser::new(token_source);

    let treaty_listener = TreatySqliteListener {
        statement_type: Box::new(DmlData {
            data: DmlType::Unknown,
            table_name: String::from(""),
            table_name_collection: Vec::new(),
        }),
    };

    let listener_id = parser.add_parse_listener(Box::new(treaty_listener));
    let _ = parser.parse();
    let item = parser.remove_parse_listener(listener_id);

    item.statement_type.table_name
}

#[allow(dead_code)]
pub fn determine_statement_type(sql_text: String) -> DmlType {
    let text = sql_text.as_str();

    trace!("{}", sql_text);

    let tf = CommonTokenFactory::default();
    let input = InputStream::new(text);
    let lexer = SQLiteLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = SQLiteParser::new(token_source);

    let treaty_listener = TreatySqliteListener {
        statement_type: Box::new(DmlData {
            data: DmlType::Unknown,
            table_name: String::from(""),
            table_name_collection: Vec::new(),
        }),
    };

    let listener_id = parser.add_parse_listener(Box::new(treaty_listener));
    let _ = parser.parse();
    let item = parser.remove_parse_listener(listener_id);

    item.statement_type.data
}

pub fn get_values_from_insert_statement(insert_statement: &str) -> Vec<String> {
    let tf = CommonTokenFactory::default();
    let input = InputStream::new(insert_statement);
    let lexer = SQLiteLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = SQLiteParser::new(token_source);

    let treaty_listener = TreatyInsertSqliteListener {
        insert_data: Box::new(InsertData {
            table_name: String::from(""),
            column_names: Vec::new(),
            values: Vec::new(),
            col_and_vals: Vec::new(),
        }),
    };

    let listener_id = parser.add_parse_listener(Box::new(treaty_listener));
    let _ = parser.parse();
    let item = parser.remove_parse_listener(listener_id);

    item.insert_data.values
}

pub mod tests {

    pub mod parse_insert_for_values {
        #[cfg(test)]
        use treaty_types::enums::DatabaseType;
        #[cfg(test)]
        use crate::query_parser::get_values_from_insert_statement;

        #[test]
        pub fn test() {
            let insert_statement = "INSERT INTO test_table ( col1, col2 ) VALUES (1, 'abcd');";
            let values = get_values_from_insert_statement(insert_statement, DatabaseType::Sqlite);

            let test_values: Vec<String> = vec!["1".to_string(), "'abcd'".to_string()];

            assert_eq!(test_values, values);
        }
    }

    pub mod determine_statement_type {
        #[cfg(test)]
        use treaty_types::enums::DmlType;
        #[cfg(test)]
        use crate::query_parser::sqlite::determine_statement_type;

        #[test]
        pub fn unknown() {
            let example = "ABCD";
            let statement_type = determine_statement_type(example.to_string());
            assert_eq!(statement_type, DmlType::Unknown);
        }

        #[test]
        pub fn select() {
            let example = "SELECT col1, col2 FROM asdf;";
            let statement_type = determine_statement_type(example.to_string());
            assert_eq!(statement_type, DmlType::Select);
        }

        #[test]
        pub fn insert() {
            let example = "INSERT INTO asdf ( col1, col2 ) VALUES ( 1, 'abcd');";
            let statement_type = determine_statement_type(example.to_string());
            assert_eq!(statement_type, DmlType::Insert);
        }

        #[test]
        pub fn update() {
            let example = "UPDATE asdf SET col1 = 'foo' WHERE col2 = 3;";
            let statement_type = determine_statement_type(example.to_string());
            assert_eq!(statement_type, DmlType::Update);
        }

        #[test]
        pub fn delete() {
            let example = "DELETE FROM asdf WHERE col1 = 'a';";
            let statement_type = determine_statement_type(example.to_string());
            assert_eq!(statement_type, DmlType::Delete);
        }
    }

    pub mod determine_table_name {
        #[cfg(test)]
        use treaty_types::enums::DatabaseType;
        #[cfg(test)]
        use crate::query_parser::get_table_name;

        #[test]
        pub fn unknown() {
            let example = "ABCD";
            let table_name = get_table_name(example, DatabaseType::Sqlite);
            assert_eq!(table_name, String::from(""));
        }

        #[test]
        pub fn select() {
            let example = "SELECT col1, col2 FROM asdf;";
            let table_name = get_table_name(example, DatabaseType::Sqlite);
            assert_eq!(table_name, "asdf");
        }

        #[test]
        pub fn insert() {
            let example = "INSERT INTO asdf ( col1, col2 ) VALUES ( 1, 'abcd');";
            let table_name = get_table_name(example, DatabaseType::Sqlite);
            assert_eq!(table_name, "asdf");
        }

        #[test]
        pub fn update() {
            let example = "UPDATE asdf SET col1 = 'foo' WHERE col2 = 3;";
            let table_name = get_table_name(example, DatabaseType::Sqlite);
            assert_eq!(table_name, "asdf");
        }

        #[test]
        pub fn delete() {
            let example = "DELETE FROM asdf WHERE col1 = 'a';";
            let table_name = get_table_name(example, DatabaseType::Sqlite);
            assert_eq!(table_name, "asdf");
        }
    }
}
