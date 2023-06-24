#[allow(dead_code)]
pub const EMPTY_GUID: &str = "00000000-0000-0000-0000-000000000000";
#[allow(dead_code)]
pub const DATETIME_STRING_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.6f UTC";
#[allow(dead_code)]
pub const DATA_LOG_TABLE_SUFFIX: &str = "_COOP_DATA_LOG";
#[allow(dead_code)]
pub const DATA_QUEUE_TABLE_SUFFIX: &str = "_COOP_DATA_QUEUE";
#[allow(dead_code)]
pub const METADATA_TABLE_SUFFIX: &str = "_COOP_METADATA";
#[allow(dead_code)]
pub const SHOWS_TABLE_SUFFIX: &str = "_COOP_SHADOWS";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
