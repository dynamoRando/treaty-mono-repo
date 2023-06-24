use indexmap::IndexMap;

use crate::{formatter::build_table, types::treaty_proto::Contract};

use super::{db::full_database_schema_to_tables, host::host_to_markdown_table};

pub fn contract_to_markdown_table(contract: &Contract) -> String {
    let mut markdown_table = String::new();
    let mut contract_kv: IndexMap<String, String> = IndexMap::new();

    contract_kv.insert(
        "GUID".to_string(),
        contract.contract_guid.clone().to_string(),
    );
    contract_kv.insert(
        "Description".to_string(),
        contract.description.clone().to_string(),
    );
    contract_kv.insert(
        "Status".to_string(),
        contract_value_to_status(contract.status),
    );
    contract_kv.insert(
        "Version".to_string(),
        contract.contract_version.clone().to_string(),
    );

    markdown_table += "Contract Details: ";
    markdown_table += "\n";
    markdown_table = markdown_table + &build_table(contract_kv);
    markdown_table += "\n";
    markdown_table += "Database Schema: ";
    markdown_table += "\n";
    markdown_table =
        markdown_table + &full_database_schema_to_tables(contract.schema.as_ref().unwrap());
    markdown_table += "\n";
    markdown_table += "Host: ";
    markdown_table += "\n";
    markdown_table = markdown_table + &host_to_markdown_table(contract.host_info.as_ref().unwrap());
    markdown_table += "\n";

    markdown_table
}

fn contract_value_to_status(status: u32) -> String {
    match status {
        0 => "Unknown".to_string(),
        1 => "NotSent".to_string(),
        2 => "Pending".to_string(),
        3 => "Accepted".to_string(),
        4 => "Rejected".to_string(),
        _ => "Unknown".to_string(),
    }
}
