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
    let db = config.test_db_name.clone();
    let contract = config.contract_desc.as_ref().unwrap().clone();
    let participant_id = config.participant_id;

    let client_sent_contract = client(&db, &mc, &pdb, &contract, participant_id).await;

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
    contract_desc: &str,
    participant_id: Option<String>,
) -> bool {
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    let mut client = get_treaty_client(config).await;

    let response = client.create_user_database(db_name).await.unwrap();

    assert!(response);

    // this should be false
    let response = client.create_user_database(db_name).await.unwrap();
    assert!(!response);

    let response = client.enable_cooperative_features(db_name).await.unwrap();

    assert!(response);

    let response = client
        .execute_write_at_host(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type, "")
        .await
        .unwrap();

    assert!(response);

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    let result = client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    assert!(result);

    let logical_storage_policy = LogicalStoragePolicy::ParticpantOwned;

    let result = client
        .set_logical_storage_policy(db_name, "EMPLOYEE", logical_storage_policy)
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

    let result = client
        .add_participant(
            db_name,
            "participant",
            &participant_db_addr.ip4_addr,
            participant_db_addr.port,
            &participant_db_addr.ip4_addr.clone(),
            participant_db_addr.port as u16,
            participant_id,
        )
        .await
        .unwrap();

    assert!(result);

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

    if has_contract {
        accepted_contract = client.accept_pending_contract("tester").await.unwrap();
        assert!(accepted_contract);
    }

    accepted_contract
}

pub async fn io(db_name: &str, config: &TreatyClientConfig) -> bool {
    let mut client = get_treaty_client(config).await;

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

    let data = client
        .execute_read_at_host(
            db_name,
            "SELECT ID FROM EMPLOYEE",
            DatabaseType::to_u32(DatabaseType::Sqlite),
        )
        .await
        .unwrap();

    trace!("{data:?}");

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
