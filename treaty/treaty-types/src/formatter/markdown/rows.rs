use crate::{formatter::build_max_lengths_for_columns, types::treaty_proto::Row};
use tracing::trace;

/// takes a Vec of rows and formats a table similiar to Markdown
pub fn rows_to_string_markdown_table(rows: &[Row]) -> String {
    let max_lengths = build_max_lengths_for_columns(rows);

    let mut markdown_table = String::new();

    let mut total_length_of_table = 0;

    for length in max_lengths.values() {
        // for every column, we also want to pad both sides with a | divider plus a space
        total_length_of_table = total_length_of_table + length + 3;
    }

    // add the column names
    for lengths in &max_lengths {
        markdown_table += "|";
        let pad_length = lengths.1 - lengths.0.len() as u32 - 1;
        markdown_table = markdown_table + " " + lengths.0;
        markdown_table += format!(" {:<width$}", "", width = pad_length as usize).as_str();
    }

    markdown_table += "|";
    markdown_table += "\n";

    // add the header seperator
    for lengths in &max_lengths {
        markdown_table += "|";
        let pad_length = lengths.1 - 1;
        markdown_table += format!(" {:-<width$} ", "", width = pad_length as usize).as_str();
    }

    markdown_table += "|";
    markdown_table += "\n";

    for row in rows {
        markdown_table += "|";
        for value in &row.values {
            let col_name: String = value.column.as_ref().unwrap().column_name.clone().into();
            let col_max = max_lengths.get_key_value(&col_name).unwrap();
            let max_length = *col_max.1;
            markdown_table = markdown_table + " " + &value.string_value.clone();

            trace!("max_length: {:?}", max_length);
            trace!("max_length: {:?}", value.string_value.len());

            // let pad_length = max_length - *&value.string_value.len() as u32 - 1;
            let pad_length = max_length - value.string_value.len() as u32;

            markdown_table += format!("{:<width$}", "", width = pad_length as usize).as_str();
            markdown_table += "|";
        }

        markdown_table += "\n";
    }

    markdown_table
}
