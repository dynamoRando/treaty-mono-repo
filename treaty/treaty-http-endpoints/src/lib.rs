/// Represents endpoints for an treaty Client Service
pub mod client {
    pub const DB_TYPE: &str = "/client/db-type";
    pub const READ_SQL_AT_HOST: &str = "/client/sql/host/read/";
    pub const SEND_CONTRACT_TO_PARTICIPANT: &str = "/client/databases/participant/send-contract";
    pub const READ_SQL_AT_PARTICIPANT: &str = "/client/sql/participant/read/";
    pub const WRITE_SQL_AT_HOST: &str = "/client/sql/host/write/";
    pub const COOPERATIVE_WRITE_SQL_AT_HOST: &str = "/client/sql/host/write/cooperative";
    pub const WRITE_SQL_AT_PARTICIPANT: &str = "/client/sql/participant/write/";
    pub const TRY_AUTH_PARTICIPANT: &str = "/client/try-auth-participant";
    pub const GENERATE_CONTRACT: &str = "/client/databases/contract/generate/";
    pub const ADD_PARTICIPANT: &str = "/client/databases/participant/add/";
    pub const GET_PARTICIPANTS: &str = "/client/databases/participant/get";
    pub const GET_ACTIVE_CONTRACT: &str = "/client/databases/contract/get/";
    pub const GET_ROW_AT_PARTICIPANT: &str = "/client/databases/participant/io/get";
    pub const GET_DATA_HASH_AT_PARTICIPANT: &str = "/client/databases/participant/io/get-hash";
    pub const GET_DATA_HASH_AT_HOST: &str = "/client/databases/host/io/get-hash";
    pub const IS_ONLINE: &str = "/client/version";
    pub const AUTH_FOR_TOKEN: &str = "/client/token";
    pub const REVOKE_TOKEN: &str = "/client/token-revoke";
    pub const NEW_DATABASE: &str = "/client/databases/new/";
    pub const DELETE_DATABASE_FORCE: &str = "/client/databases/delete-force/";
    pub const ENABLE_COOPERATIVE_FEATURES: &str = "/client/databases/enable-cooperative-features";
    pub const GET_POLICY: &str = "/client/databases/table/policy/get";
    pub const SET_POLICY: &str = "/client/databases/table/policy/set/";
    pub const GENERATE_HOST_INFO: &str = "/client/host/generate";
    pub const GET_COOP_HOSTS: &str = "/client/host/get-coop-hosts";
    pub const GET_HOST_INFO: &str = "/client/host/get";
    pub const VIEW_PENDING_CONTRACTS: &str = "/client/contract/review";
    pub const ACCEPT_PENDING_CONTRACT: &str = "/client/contract/accept/";
    pub const ACCEPT_PENDING_ACTION: &str = "/client/databases/actions/accept-pending";
    pub const GET_PENDING_ACTIONS: &str = "/client/databases/actions/get-pending";
    pub const CHANGE_DELETES_TO_HOST_BEHAVIOR: &str =
        "/client/databases/behavior/change-deletes-to-host";
    pub const GET_DELETES_TO_HOST_BEHAVIOR: &str = "/client/databases/behavior/get-deletes-to-host";
    pub const CHANGE_UPDATES_TO_HOST_BEHAVIOR: &str =
        "/client/databases/behavior/change-updates-to-host";
    pub const GET_UPDATES_TO_HOST_BEHAVIOR: &str = "/client/databases/behavior/get-updates-to-host";
    pub const CHANGE_DELETES_FROM_HOST_BEHAVIOR: &str =
        "/client/databases/behavior/change-deletes-from-host";
    pub const GET_DELETES_FROM_HOST_BEHAVIOR: &str =
        "/client/databases/behavior/get-deletes-from-host";
    pub const CHANGE_UPDATES_FROM_HOST_BEHAVIOR: &str =
        "/client/databases/behavior/change-updates-from-host";
    pub const GET_UPDATES_FROM_HOST_BEHAVIOR: &str =
        "/client/databases/behavior/get-updates-from-host";
    pub const CHANGE_HOST_STATUS_ID: &str = "/client/change-host-status-id";
    pub const CHANGE_HOST_STATUS_NAME: &str = "/client/change-host-status-name";
    pub const GET_DATABASES: &str = "/client/databases";
    pub const HAS_TABLE: &str = "/client/databases/has_table";
    pub const GET_SETTINGS: &str = "/client/settings";
    pub const GET_LAST_LOGS: &str = "/client/logs/by-last-entries";
}

/// Represents endpoints for an treaty Data Service
pub mod data {
    pub const REMOVE_ROW_AT_PARTICIPANT: &str = "/data/io/remove-row";
    pub const UPDATE_ROW_AT_PARTICIPANT: &str = "/data/io/update-row";
    pub const INSERT_ROW_AT_PARTICIPANT: &str = "/data/io/insert-row";
    pub const GET_ROW_AT_PARTICIPANT: &str = "/data/io/get-row";
    pub const NOTIFY_HOST_OF_REMOVED_ROW: &str = "/data/io/notify-host-removed-row";
    pub const NOTIFY_HOST_OF_UPDATED_HASH: &str = "/data/io/notify-host-updated-hash";
    pub const TRY_AUTH: &str = "/data/try-auth";
}

/// Represents endpoints for a treay Info Service
pub mod info {
    pub const SAVE_CONTRACT: &str = "/info/contract/save/";
    pub const PARTICIPANT_ACCEPTS_CONTRACT: &str = "/info/contract/accepted-by-participant";
    pub const INFO_AUTH_FOR_TOKEN: &str = "/info/token";
    pub const INFO_TRY_AUTH: &str = "/info/try-auth";
}

/// Represents the types of headers we expect to evaluate for in Treaty
pub mod headers {
    // bin types means that we should deseralize the message

    // plain username and pw
    pub const TREATY_AUTH_HEADER_BASIC_BIN: &str = "x-auth-treaty-basic-bin";
    pub const TREATY_AUTH_HEADER_BASIC: &str = "x-auth-treaty-basic";
    // username and json web token
    pub const TREATY_AUTH_HEADER_WEB_TOKEN_BIN: &str = "x-auth-treaty-web-token-bin";
    pub const TREATY_AUTH_HEADER_WEB_TOKEN: &str = "x-auth-treaty-web-token";
    // username and token byte array
    pub const TREATY_AUTH_HEADER_BIN: &str = "x-auth-treaty-bin";
    pub const TREATY_AUTH_HEADER: &str = "x-auth-treaty";
    // additional metadata such as host_id and/or database name
    pub const TREATY_AUTH_HEADER_METADATA_BIN: &str = "x-auth-treaty-metadata-bin";
    pub const TREATY_AUTH_HEADER_METADATA: &str = "x-auth-treaty-metadata";
    // the author type of the message: a user, or treaty data instance or participant
    pub const TREATY_AUTH_HEADER_AUTHOR_BIN: &str = "x-auth-treaty-author-bin";
    pub const TREATY_AUTH_HEADER_AUTHOR: &str = "x-auth-treaty-author";
}
