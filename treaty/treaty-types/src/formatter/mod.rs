use indexmap::IndexMap;

use crate::types::treaty_proto::Row;

pub mod markdown;

/// takes a Vec of rows and formats a table similar to MySQL

pub fn rows_to_string_table(_rows: Vec<Row>) -> String {
    todo!()
}

/// takes a Vec of rows and formats a table similiar to Markdown
pub fn rows_to_string_markdown_table(rows: &[Row]) -> String {
    markdown::rows::rows_to_string_markdown_table(rows)
}

pub fn build_max_lengths_for_columns(rows: &[Row]) -> IndexMap<String, u32> {
    let mut max_lengths: IndexMap<String, u32> = IndexMap::new();

    for row in rows {
        for value in &row.values {
            let col_name: String = value.column.as_ref().unwrap().column_name.clone().into();
            let col_length = col_name.len() + 4;

            if !max_lengths.contains_key(&col_name) {
                max_lengths.insert(col_name.clone(), col_length.try_into().unwrap());
            }

            if max_lengths.contains_key(&col_name) {
                let kv = max_lengths.get_key_value(&col_name).unwrap();
                let value_length = value.string_value.len() as u32;
                if value_length > *kv.1 {
                    // max_lengths.remove(&col_name);
                    // max_lengths.insert(col_name.clone(), value_length);
                    *max_lengths.get_mut(&col_name).unwrap() = value_length + 4;
                }
            }
        }
    }

    max_lengths
}

pub fn build_table(key_value: IndexMap<String, String>) -> String {
    let mut markdown_table = String::new();
    let mut keys: Vec<&str> = Vec::new();
    let mut values: Vec<&str> = Vec::new();

    for item in key_value.keys() {
        keys.push(item.as_str());
    }

    keys.push("Key");

    let max_length_key_strings = get_max_length_for_vec_strings(keys);

    for item in key_value.values() {
        values.push(item);
    }

    values.push("Value");

    let max_length_key_values = get_max_length_for_vec_strings(values);

    markdown_table = markdown_table
        + &build_markdown_row(
            &"Key".to_string(),
            &"Value".to_string(),
            max_length_key_strings,
            max_length_key_values,
        );

    markdown_table += "\n";

    markdown_table += "|";

    markdown_table = markdown_table + &build_markdown_seperator(max_length_key_strings + 1);

    markdown_table += "|";

    markdown_table = markdown_table + &build_markdown_seperator(max_length_key_values + 1);

    markdown_table += "|";

    markdown_table += "\n";

    for (k, v) in &key_value {
        markdown_table = markdown_table
            + &build_markdown_row(k, v, max_length_key_strings, max_length_key_values);
        markdown_table += "\n";
    }

    markdown_table
}

/// builds a markdown seperator without the "|" between
fn build_markdown_seperator(max_length: u32) -> String {
    let mut return_row = String::new();
    let pad_length = max_length;
    return_row += format!(" {:-<width$} ", "", width = pad_length as usize).as_str();

    return_row
}

/// takes a vec of strings and returns the max length found + 4 (to account for space and | for column)
fn get_max_length_for_vec_strings(items: Vec<&str>) -> u32 {
    let mut max_length: u32 = 0;

    for item in items {
        let item_length = item.len() as u32;

        if item_length >= max_length {
            max_length = item_length;
        }
    }

    max_length
}

/*
# String Padding In Rust
- https://stackoverflow.com/questions/50458144/what-is-the-easiest-way-to-pad-a-string-with-0-to-the-left
- https://stackoverflow.com/questions/64810657/how-to-pad-left-in-rust
- https://stackoverflow.com/questions/69067436/how-do-i-make-the-fill-padding-in-stdformat-dynamic
 */

fn build_markdown_row(
    key_string: &String,
    key_value: &String,
    key_string_max_length: u32,
    key_value_max_length: u32,
) -> String {
    let mut string_row: String = "".to_string();

    string_row += "| ";
    string_row = string_row + key_string;

    let mut pad_length: u32;
    pad_length = key_string_max_length - key_string.len() as u32;

    string_row += format!(" {:<width$} ", "", width = pad_length as usize).as_str();
    string_row += "| ";
    string_row = string_row + key_value;

    pad_length = key_value_max_length - key_value.len() as u32;

    string_row += format!(" {:<width$} ", "", width = pad_length as usize).as_str();
    string_row += "|";

    string_row
}
