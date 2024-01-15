use stdext::function_name;
use tracing::{debug, trace, warn};
use treaty_types::enums::{DatabaseType, LogicalStoragePolicy, RemoteDeleteBehavior};

use crate::harness::{get_treaty_client, CoreTestConfig, ServiceAddr, TreatyClientConfig};

/// has a main and participant establish a new contract and verifies that the main has read/write
/// to the participant
pub async fn main_and_participant_setup(config: CoreTestConfig) -> bool {
    debug!("[{}]: {config:?}", function_name!());

    let mc = config.main_client.clone();
    let pc = config.participant_client.as_ref().unwrap().clone();
    let pdb = config.participant_db_addr.as_ref().unwrap().clone();
    let pi = config.participant_info_addr.as_ref().unwrap().clone();
    let db = config.test_db_name.clone();
    let contract = config.contract_desc.as_ref().unwrap().clone();
    let participant_id = config.participant_id;

    let client_sent_contract = client(&db, &mc, &pdb, &pi, &contract, participant_id).await;

    assert!(client_sent_contract);

    let accepted_contract = participant(&pc, &contract).await;

    assert!(accepted_contract);

    let has_io = io(&db, &mc).await;

    assert!(has_io);

    true
}

pub async fn client(
    db_name: &str,
    config: &TreatyClientConfig,
    participant_db_addr: &ServiceAddr,
    participant_info_addr: &ServiceAddr,
    contract_desc: &str,
    participant_id: Option<String>,
) -> bool {
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    let mut client = get_treaty_client(config).await;

    let db_config = client.get_backing_db_config().await.unwrap();

    let db_type = db_config.database_type;
    let db_type = DatabaseType::from_u32(db_type);
    let use_schema = db_config.use_schema;

    debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Test backing database type: {db_type:?} ########",
        function_name!()
    );
    if (db_type == DatabaseType::Postgres && use_schema == false) || db_type == DatabaseType::Sqlite
    {
        let response = client.create_user_database(db_name).await.unwrap();
        debug!(
            "[{}]: ######## COMMON CONTRACT SETUP: Creating User Database: Should be true ########",
            function_name!()
        );
        assert!(response);

        // this should be false
        let response = client.create_user_database(db_name).await.unwrap();
        debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Creating User Database: Should be false ########",
        function_name!()
    );
        assert!(!response);
    } else {
        debug!(
            "[{}]: ######## COMMON CONTRACT SETUP: Skipping create user database ########",
            function_name!()
        );
        /*
        We skip testing for creating a user database because of the difference in implementation for a database system
        that can hold multiple schemas and one that cannot. In a database system that can not hold multiple schemas (MySQL, Sqlite),
        the `Treaty` schema and the user database we're testing against are two different databases, and we want to make sure
        that we can create the user database successfully, and that if we try to re-create the user database we don't overwrite
        blindly that database.

        In a database system that can have multiple schemas in one database (Postgres, MS SQL Server), we keep the `Treaty` schema
        in the actual user database itself, and keep the user defined tables in the default schema (`public` for Postgres, `dbo` for MS SQL Server).
        */
    }

    let response = client.enable_cooperative_features(db_name).await.unwrap();
    debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Enable cooperative features ########",
        function_name!()
    );
    assert!(response);

    let response = client
        .execute_write_at_host(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type, "")
        .await
        .unwrap();

    assert!(response);

    debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Create EMPLOYEE TABLE ########",
        function_name!()
    );
    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    let result = client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    assert!(result);

    let logical_storage_policy = LogicalStoragePolicy::ParticipantOwned;

    debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Set logical storage policy for EMPLOYEE ########",
        function_name!()
    );
    let result = client
        .set_logical_storage_policy(db_name, "EMPLOYEE", logical_storage_policy)
        .await
        .unwrap();

    assert!(result);

    debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Create SHARED_ENTRIES TABLE ########",
        function_name!()
    );

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS SHARED_ENTRIES (Id INT, Notes TEXT);");

    let result = client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    assert!(result);

    let logical_storage_policy = LogicalStoragePolicy::Shared;

    debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Set logical storage policy for SHARED_ENTRIES ########",
        function_name!()
    );

    let result = client
        .set_logical_storage_policy(db_name, "SHARED_ENTRIES", logical_storage_policy)
        .await
        .unwrap();

    assert!(result);

    debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Create MIRRORED_ENTRIES TABLE ########",
        function_name!()
    );

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS MIRRORED_ENTRIES (Id INT, Notes TEXT);");

    let result = client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    assert!(result);

    let logical_storage_policy = LogicalStoragePolicy::Mirror;

    debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Set logical storage policy for MIRRORED_ENTRIES ########",
        function_name!()
    );

    let result = client
        .set_logical_storage_policy(db_name, "MIRRORED_ENTRIES", logical_storage_policy)
        .await
        .unwrap();

    assert!(result);

    let behavior = RemoteDeleteBehavior::Ignore;

    let result = client
        .generate_contract(db_name, "tester", contract_desc, behavior)
        .await
        .unwrap();

    assert!(result);

    let contract = client.get_active_contract(db_name).await.unwrap();
    let cd = contract.contract.unwrap();
    let schema = cd.schema.unwrap();

    assert!(schema.database_name == db_name);

    debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Add Participant ########",
        function_name!()
    );
    let result = client
        .add_participant(
            db_name,
            "participant",
            &participant_db_addr.ip4_addr,
            Some(participant_db_addr.port),
            participant_info_addr.port,
            &participant_db_addr.ip4_addr.clone(),
            participant_db_addr.port as u16,
            participant_id,
        )
        .await
        .unwrap();

    assert!(result);

    debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Send Participant Contract ########",
        function_name!()
    );
    let result = client
        .send_participant_contract(db_name, "participant")
        .await
        .unwrap();

    assert!(result);

    result
}

pub async fn participant(config: &TreatyClientConfig, contract_desc: &str) -> bool {
    let mut has_contract = false;

    let mut client = get_treaty_client(config).await;

    trace!("{}", client.debug());

    let result = client.generate_host_info("participant").await.unwrap();
    assert!(result);

    let pending_contracts = client.view_pending_contracts().await.unwrap();

    if pending_contracts.is_empty() {
        warn!("no contracts found");
    }

    for contract in &pending_contracts {
        if contract.description == contract_desc {
            has_contract = true;
            break;
        }
    }

    assert!(has_contract);

    let mut accepted_contract = false;

    debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Accept Contract ########",
        function_name!()
    );
    if has_contract {
        accepted_contract = client.accept_pending_contract("tester").await.unwrap();
        assert!(accepted_contract);
    }

    accepted_contract
}

pub async fn io(db_name: &str, config: &TreatyClientConfig) -> bool {
    let mut client = get_treaty_client(config).await;

    debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Execute Cooperative Write At Host ########",
        function_name!()
    );
    let result = client
        .execute_cooperative_write_at_host(
            db_name,
            "INSERT INTO EMPLOYEE ( Id, Name ) VALUES ( 999, 'ASDF');",
            "participant",
            "",
        )
        .await
        .unwrap();

    assert!(result);

    debug!(
        "[{}]: ######## COMMON CONTRACT SETUP: Read data back ########",
        function_name!()
    );
    let data = client
        .execute_read_at_host(
            db_name,
            "SELECT ID FROM EMPLOYEE",
            DatabaseType::to_u32(DatabaseType::Sqlite),
        )
        .await
        .unwrap();

    trace!("{data:#?}");

    let value = data
        .rows
        .first()
        .unwrap()
        .values
        .first()
        .unwrap()
        .value
        .clone();

    trace!("assert left: {value:?}");

    let expected_value = "999".as_bytes().to_vec();

    trace!("assert right: {expected_value:?}");

    value == expected_value
}
