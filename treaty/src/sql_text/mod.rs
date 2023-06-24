/// Anything in CDS is in the Cooperative Data Store.
pub struct Cds {}

/// Anything in COOP are tables stored in the user database and are used
/// to enable cooperative functions with participants.
pub struct Coop {}

impl Cds {
    pub fn text_create_user_tokens_table() -> String {
        String::from(
            "
        CREATE TABLE IF NOT EXISTS CDS_USER_TOKENS
        (
            USERNAME VARCHAR(25) NOT NULL,
            TOKEN TEXT NOT NULL,
            ISSUED_UTC DATETIME,
            EXPIRATION_UTC DATETIME
        );",
        )
    }

    /// Returns create table statement for storing users of the CDS.
    pub fn text_create_user_table() -> String {
        String::from(
            "CREATE TABLE IF NOT EXISTS CDS_USER
        (
            USERNAME VARCHAR(25) UNIQUE,
            HASH BLOB NOT NULL
        );",
        )
    }

    pub fn text_add_user() -> String {
        String::from("INSERT INTO CDS_USER (USERNAME, HASH) VALUES (:username, :hash);")
    }

    pub fn text_get_user() -> String {
        String::from("SELECT USERNAME, HASH FROM CDS_USER WHERE USERNAME = :un ;")
    }

    pub fn text_get_user_role() -> String {
        String::from("SELECT count(*) AS TOTALCOUNT FROM CDS_USER_ROLE WHERE USERNAME = :username AND ROLENAME = :rolename;")
    }

    pub fn text_add_user_role() -> String {
        String::from(
            "INSERT INTO CDS_USER_ROLE (USERNAME, ROLENAME) VALUES (:username, :rolename);",
        )
    }

    pub fn text_get_role() -> String {
        String::from("SELECT count(*) AS ROLECOUNT FROM CDS_ROLE WHERE ROLENAME = :rolename;")
    }

    /// Returns create table statement for storing roles of the CDS.
    pub fn text_create_role_table() -> String {
        String::from(
            "CREATE TABLE IF NOT EXISTS CDS_ROLE
                (
                    ROLENAME VARCHAR(25) UNIQUE
                );",
        )
    }

    /// Returns create table statement for xref users to roles.
    pub fn text_create_user_role_table() -> String {
        String::from(
            "CREATE TABLE IF NOT EXISTS CDS_USER_ROLE
            (
                USERNAME VARCHAR(25) NOT NULL,
                ROLENAME VARCHAR(25) NOT NULL   
            );",
        )
    }

    /// Returns create table statement for storing unique identifier to participants.
    pub fn text_create_host_info_table() -> String {
        String::from(
            "CREATE TABLE IF NOT EXISTS CDS_HOST_INFO
         (
             HOST_ID CHAR(36) NOT NULL,
             HOST_NAME VARCHAR(50) NOT NULL,
             TOKEN BLOB NOT NULL
         );",
        )
    }

    /// Returns create table statement for hosts that this CDS is cooperating with.
    /// This is used for partial databases and their contracts.
    pub fn text_create_cds_hosts_table() -> String {
        String::from(
            "CREATE TABLE IF NOT EXISTS CDS_HOSTS
        (
            HOST_ID CHAR(36) NOT NULL,
            HOST_NAME VARCHAR(50),
            TOKEN BLOB,
            IP4ADDRESS VARCHAR(25),
            IP6ADDRESS VARCHAR(25),
            PORT INT,
            LAST_COMMUNICATION_UTC DATETIME,
            HOST_STATUS INT,
            HTTP_ADDR VARCHAR(50),
            HTTP_PORT INT
        );",
        )
    }

    /// Returns create table statement for holding schema information for partial databases participating with a remote host.
    /// This is used for partial databases and their contracts.
    pub fn text_create_cds_contracts_table() -> String {
        String::from(
            "CREATE TABLE IF NOT EXISTS CDS_CONTRACTS
        (
            HOST_ID CHAR(36) NOT NULL,
            CONTRACT_ID CHAR(36) NOT NULL,
            CONTRACT_VERSION_ID CHAR(36) NOT NULL,
            DATABASE_NAME VARCHAR(50) NOT NULL,
            DATABASE_ID CHAR(36) NOT NULL,
            DESCRIPTION VARCHAR(255),
            GENERATED_DATE_UTC DATETIME,
            CONTRACT_STATUS INT
        );",
        )
    }

    /// Returns create table statement for holding the tables in the partial database.
    /// This is used for partial databases and their contracts.
    pub fn text_create_cds_contracts_tables_table() -> String {
        String::from(
            "CREATE TABLE IF NOT EXISTS CDS_CONTRACTS_TABLES
        (
            DATABASE_ID CHAR(36) NOT NULL,
            DATABASE_NAME VARCHAR(50) NOT NULL,
            TABLE_ID CHAR(36) NOT NULL,
            TABLE_NAME VARCHAR(50) NOT NULL,
            LOGICAL_STORAGE_POLICY INT,
            UPDATES_FROM_HOST_BEHAVIOR INT,
            DELETES_FROM_HOST_BEHAVIOR INT,
            UPDATES_TO_HOST_BEHAVIOR INT,
            DELETES_TO_HOST_BEHAVIOR INT,
            USE_DATA_LOG_TABLE INT
        );",
        )
    }

    /// Returns create table statement for holding the schema for the tables in the partial database.
    /// This is used for partial databases and their contracts.
    pub fn text_create_cds_contracts_tables_schemas_table() -> String {
        String::from(
            "CREATE TABLE IF NOT EXISTS CDS_CONTRACTS_TABLE_SCHEMAS
        (
            TABLE_ID CHAR(36) NOT NULL,
            COLUMN_ID CHAR(36) NOT NULL,
            COLUMN_NAME VARCHAR(50) NOT NULL,
            COLUMN_TYPE INT NOT NULL,
            COLUMN_LENGTH INT NOT NULL,
            COLUMN_ORDINAL INT NOT NULL,
            IS_NULLABLE INT
        );",
        )
    }
}

impl Coop {
    pub fn text_create_data_queue_table() -> String {
        String::from(
            "
        CREATE TABLE IF NOT EXISTS :table_name
        (
            ID INT NOT NULL,
            STATEMENT TEXT NOT NULL,
            WHERE_CLAUSE TEXT,
            REQUESTED_TS_UTC DATETIME NOT NULL,
            HOST_ID CHAR(36) NOT NULL,
            ACTION VARCHAR(10) NOT NULL
        )
        ;",
        )
    }

    pub fn text_create_data_log_table() -> String {
        String::from(
            "
        CREATE TABLE IF NOT EXISTS :table_name
        (
            :column_list,
            ROW_ID INT,
            ACTION VARCHAR(20),
            TS_UTC DATETIME
        )
        ;",
        )
    }

    pub fn text_delete_row_metadata_table() -> String {
        String::from(
            "DELETE FROM 
                :table_name
            WHERE 
                ROW_ID = :row 
            AND 
                INTERNAL_PARTICIPANT_ID = :pid
        ;",
        )
    }

    pub fn text_update_row_metadata_table() -> String {
        String::from(
            "UPDATE 
                :table_name
            SET 
                HASH = :hash
            WHERE 
                ROW_ID = :row 
            AND 
                INTERNAL_PARTICIPANT_ID = :pid
        ;",
        )
    }

    pub fn text_insert_row_metadata_table() -> String {
        String::from(
            "INSERT INTO :table_name
        (
            ROW_ID,
            HASH,
            INTERNAL_PARTICIPANT_ID
        )
        VALUES
        (
            :row,
            :hash,
            :pid
        )
        ;",
        )
    }

    pub fn text_create_metadata_table() -> String {
        String::from(
            "
        CREATE TABLE IF NOT EXISTS :table_name
        (
            ROW_ID INT,
            HASH BLOB,
            INTERNAL_PARTICIPANT_ID CHAR(36)
        );
        ",
        )
    }

    /// Returns create table statement for storing the database id when we 1st enable cooperative features
    pub fn text_create_data_host_table() -> String {
        String::from(
            "CREATE TABLE IF NOT EXISTS COOP_DATA_HOST
        (
           DATABASE_ID CHAR(36) NOT NULL,
           DATABASE_NAME VARCHAR(500) NOT NULL
        );
        ",
        )
    }

    /// Returns create table statement for storing the table ids generated when we start setting logical
    /// storage policies on tables. This should align with COOP_REMOTES.
    pub fn text_create_data_host_tables_table() -> String {
        String::from(
            "CREATE TABLE IF NOT EXISTS COOP_DATA_TABLES
            (
                TABLE_ID CHAR(36) NOT NULL,
                TABLE_NAME VARCHAR(500) NOT NULL
            );
            ",
        )
    }

    /// Returns create table statement for storing the column ids generated when we start setting logical
    /// storage policies on tables. This should align with the actual schema of the table in the datbase.
    pub fn text_create_data_host_tables_columns_table() -> String {
        String::from(
            "CREATE TABLE IF NOT EXISTS COOP_DATA_HOST_TABLE_COLUMNS
            (
                TABLE_ID CHAR(36) NOT NULL,
                COLUMN_ID CHAR(36) NOT NULL,
                COLUMN_NAME VARCHAR(500) NOT NULL
            )
            ",
        )
    }

    /// Returns SQL statement for getting the count of tables in the cooperative data table tables (this is for contracts)
    /// for the specified table name
    /// # Params:
    /// - ":table_name"
    pub fn text_get_count_from_data_host_tables_for_table(table_name: &str) -> String {
        let mut statement = String::from(
            "SELECT count(*) tablecount FROM COOP_DATA_TABLES WHERE TABLE_NAME = ':table_name'",
        );
        statement = statement.replace(&String::from(":table_name"), table_name);
        statement
    }

    /// Returns SQL statement for adding a table name and id to the cooperative data table.
    /// # Params:
    /// - ":table_id"
    /// - ":table_name"
    pub fn text_add_table_to_data_host_table(table_name: String, table_id: String) -> String {
        let mut statement = String::from("INSERT INTO COOP_DATA_TABLES ( TABLE_ID, TABLE_NAME ) VALUES (':table_id', ':table_name');");
        statement = statement.replace(&String::from(":table_id"), &table_id);
        statement = statement.replace(&String::from(":table_name"), &table_name);
        statement
    }

    pub fn text_get_count_from_data_host() -> String {
        String::from("SELECT COUNT(*) COUNT FROM COOP_DATA_HOST")
    }

    /// adds the generated database_id and database_name to the COOP_DATA_HOST table
    /// # Params:
    /// - ":database_id"
    /// - ":database_name"
    pub fn text_add_database_id_to_host() -> String {
        String::from(
            "INSERT INTO COOP_DATA_HOST
        (DATABASE_ID, DATABASE_NAME) VALUES (:database_id, :database_name);",
        )
    }

    /// Returns create table statement for storing the logcial storage policy for each table
    pub fn text_create_data_remotes_table() -> String {
        String::from(
            "CREATE TABLE IF NOT EXISTS COOP_REMOTES
        (
            TABLENAME VARCHAR(255) NOT NULL,
            LOGICAL_STORAGE_POLICY INT NOT NULL
        );
        ;",
        )
    }

    pub fn text_get_logical_storage_policy_tables() -> String {
        String::from(
            "
        SELECT
            TABLENAME,
            LOGICAL_STORAGE_POLICY  
        FROM
            COOP_REMOTES
            ;
        ",
        )
    }
}
