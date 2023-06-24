use indexmap::IndexMap;

use crate::{types::treaty_proto::TableSchema, formatter::build_table};



/// returns a markdown table with column name, column data type
pub fn table_schema_to_markdown_table(table: &TableSchema) -> String {
    let mut kv: IndexMap<String, String> = IndexMap::new();

    for column in &table.columns {
        let data_type = get_datatype_for_column_num(column.column_type, column.column_length);
        kv.insert(column.column_name.clone().to_string(), data_type);
    }

    build_table(kv)
}

fn get_datatype_for_column_num(data_type: u32, length: u32) -> String {
    match data_type {
        0 => "Unknown".to_string(),
        1 => "Int".to_string(),
        2 => "Bit".to_string(),
        3 => "Char".to_string() + " - " + &length.to_string(),
        4 => "DateTime".to_string(),
        5 => "Decimal".to_string(),
        6 => "Varchar".to_string() + " - " + &length.to_string(),
        7 => "Binary".to_string(),
        8 => "Varbinary".to_string() + " - " + &length.to_string(),
        9 => "Text".to_string(),
        _ => "Unknown".to_string(),
    }
}
