use indexmap::IndexMap;

use crate::{formatter::build_table, types::treaty_proto::Host};

/// takes a Host and returns a markdown table in a key/value format
pub fn host_to_markdown_table(host: &Host) -> String {
    let mut kv: IndexMap<String, String> = IndexMap::new();

    let guid_label = "GUID: ";
    let name_label = "Host Name: ";
    let ip4_label = "IP 4: ";
    let ip6_label = "IP 6: ";
    let db_port_label = "Db Port: ";
    let token_label = "Token: ";
    let http_addr = "HTTP Addr: ";
    let http_port = "HTTP Port: ";

    let token_string = String::from_utf8_lossy(&host.token).to_owned().to_string();

    kv.insert(guid_label.to_string(), host.host_guid.clone().to_string());
    kv.insert(name_label.to_string(), host.host_name.clone().to_string());

    match &host.network {
        Some(network) => {
            kv.insert(
                ip4_label.to_string(),
                network
                    .ip4_address
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .to_string(),
            );
            kv.insert(
                ip6_label.to_string(),
                network
                    .ip6_address
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .to_string(),
            );
            kv.insert(
                db_port_label.to_string(),
                network
                    .database_port_number
                    .as_ref()
                    .unwrap_or(&0)
                    .to_string(),
            );
            kv.insert(
                http_addr.to_string(),
                network
                    .http_addr
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .to_string(),
            );
            kv.insert(
                http_port.to_string(),
                network.http_port.as_ref().unwrap_or(&0).to_string(),
            );
        }
        None => {
            kv.insert(ip4_label.to_string(), "".to_string());
            kv.insert(ip6_label.to_string(), "".to_string());
            kv.insert(db_port_label.to_string(), "".to_string());
            kv.insert(http_addr.to_string(), "".to_string());
            kv.insert(http_port.to_string(), "".to_string());
        }
    }

    if !token_string.is_empty() {
        kv.insert(token_label.to_string(), "(REDACTED)".to_string());
    } else {
        kv.insert(token_label.to_string(), "(MISSING)".to_string());
    }

    build_table(kv)
}
