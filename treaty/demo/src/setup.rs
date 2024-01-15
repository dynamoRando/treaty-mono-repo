use stdext::function_name;
use treaty_client::{client_actions::ClientActions, grpc::GrpcClient, Auth, TreatyClient};
use treaty_types::enums::{DatabaseType, LogicalStoragePolicy, RemoteDeleteBehavior};

use crate::models::user::User;

pub const DB_NAME: &str = "items.db";

/// a simple helper enum for this demo to denote which Treaty container we want to connect to
pub enum TreatyInstance {
    Host,
    Participant1,
    Participant2,
}

/// A helper struct for this demo that builds us a TreatyClient
pub struct DemoClientBuilder {}

impl DemoClientBuilder {
    /// returns a TreatyClient using gRPC for transport
    pub async fn build(instance: TreatyInstance) -> TreatyClient<GrpcClient> {
        let addr_port = match instance {
            TreatyInstance::Host => "http://127.0.0.1:50051",
            TreatyInstance::Participant1 => "http://127.0.0.1:50061",
            TreatyInstance::Participant2 => "http://127.0.0.1:50071",
        };

        let timeout_in_seconds = 30;

        // Use the default creds in Treaty. This should be changed if you want to use Treaty and
        // can be configured in the Settings.toml.
        let auth = Auth {
            user_name: String::from("tester"),
            pw: String::from("123456"),
            jwt: String::from(""),
        };

        let send_jwt_if_available = true;
        let host_id: Option<String> = None;

        /*
        Create a new TreatyClient, leveraging gRPC + Protobuf as the transport mechanism.

        Treaty also supports using HTTP + JSON; implemented in support for communicating with web browsers.
        */

        TreatyClient::<GrpcClient>::new_grpc(
            addr_port,
            timeout_in_seconds,
            auth,
            send_jwt_if_available,
            host_id,
        )
        .await
    }
}

/// Connects to Treaty and is responsible for creating our initial database schema. For more information on
/// many of the concepts introducted here, see the manual in the "docs" folder.
pub async fn init_host() {
    // let's get a client to Treaty
    let mut client = DemoClientBuilder::build(TreatyInstance::Host).await;

    let db_name = DB_NAME;
    let db_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    // Create the "items.db" database, which is just a Sqlite database.
    let result = client.create_user_database(db_name).await.unwrap();
    println!("create user database result is: {result:?}");

    /*
    Enable cooperative features.

    Cooperative features instructs Treaty to create additional meta-data tables in our database as needed.
    These tables support tracking things such as who is participanting with our database, the location of "remote rows" from
    the hosts perspective, and so on.

    A "remote row" is a record in the hosts database table that stores the row-id
    and data hash of the actual row that is saved at the participant.
    */

    let result = client.enable_cooperative_features(db_name).await.unwrap();
    println!("enable cooperative features result is: {result:?}");

    // Drop any tables if they already exist
    let result = client
        .execute_write_at_host(db_name, "DROP TABLE IF EXISTS ITEMS", db_type, "")
        .await
        .unwrap();

    println!("execute write result is: {result:?}");

    let result = client
        .execute_write_at_host(db_name, "DROP TABLE IF EXISTS USERS", db_type, "")
        .await
        .unwrap();

    println!("execute write result is: {result:?}");

    let result = client
        .execute_write_at_host(db_name, "DROP TABLE IF EXISTS LISTS", db_type, "")
        .await
        .unwrap();

    println!("execute write result is: {result:?}");

    // Setup our SQL create table statements
    let create_items_table = r#"
    CREATE TABLE ITEMS 
    (
        id int,
        name text
    );
    "#;

    let create_users_table = r#"
    CREATE TABLE USERS 
    (
        id int,
        alias text
    );
    "#;

    let create_lists_table = r#"
    CREATE TABLE LISTS 
    (
        user_id int,
        item_id int,
        item_name text
    );
    "#;

    let result = client
        .execute_write_at_host(db_name, create_items_table, db_type, "")
        .await
        .unwrap();

    println!("execute write result is: {result:?}");

    let result = client
        .execute_write_at_host(db_name, create_users_table, db_type, "")
        .await
        .unwrap();

    println!("execute write result is: {result:?}");

    let result = client
        .execute_write_at_host(db_name, create_lists_table, db_type, "")
        .await
        .unwrap();

    println!("execute write result is: {result:?}");

    /*
    Set Logical Storage Policies for each table.

    A logical storage policy denotes to Treaty where data should be stored when inserted: at the host, at the participant,
    or some other variation. It is a foundational concept to Treaty. When you later define a _database contract_ for your
    host database and participants, you MUST set a logical storage policy on every table, or the contract generation will fail.
    */

    let logical_storage_policy = LogicalStoragePolicy::HostOnly;

    let result = client
        .set_logical_storage_policy(db_name, "ITEMS", logical_storage_policy)
        .await
        .unwrap();

    println!("set policy result is: {result:?}");

    let logical_storage_policy = LogicalStoragePolicy::HostOnly;

    let result = client
        .set_logical_storage_policy(db_name, "USERS", logical_storage_policy)
        .await
        .unwrap();

    println!("set policy result is: {result:?}");

    let logical_storage_policy = LogicalStoragePolicy::ParticipantOwned;

    let result = client
        .set_logical_storage_policy(db_name, "LISTS", logical_storage_policy)
        .await
        .unwrap();

    println!("set policy result is: {result:?}");

    /*
    Generate a database contract for our database.

    A database contract is a foundational concept to Treaty. It is how data cooperation happens between the host and the participants
    in the database system.

    A database contract gives the participant a copy of the schema of _all_ tables in the database system, including data types and so on.
    This is to ensure transparency. The schema includes the Logical Storage Policy of each table.

    A database contract is versioned, and every database contract for each host database has a unique UUID.

    Any time you make a change to your database schema, you must re-generate a new database contract.

    Contracts must be _accepted_ or _rejected_ by our database participants first before cooperation can take place.

    When a participant accepts a database contract, Treaty will create a mirror schema for all tables that are participant owned,
    along with any supporting meta-data structures in that participant's Treaty instance.

    A contract also defines how write actions (Insert/Update/Delete) taken place at the participant will be communicated back to the host.
    */

    // How deletions at the participant will be communicated back to the host.
    let behavior = RemoteDeleteBehavior::Ignore;

    // A description of our database contract that will be viewable by our participants.
    let contract_desc = "A grocery tracking application. You have the ability to edit your grocery list items at the risk of breaking app functionality.";

    // A user friendly name of our Treaty instance used to identify ourselves to our participants.
    let host_name = "lists-app";

    // Actually generate the contract.
    let result = client
        .generate_contract(db_name, host_name, contract_desc, behavior)
        .await
        .unwrap();

    println!("generate contract result is: {result:?}");

    println!("{}: finished", function_name!());
}

pub async fn init_part1() {
    let mut client = DemoClientBuilder::build(TreatyInstance::Participant1).await;
    client.generate_host_info("participant1").await.unwrap();
    println!("{}: finished", function_name!());
}

pub async fn init_part2() {
    let mut client = DemoClientBuilder::build(TreatyInstance::Participant2).await;
    client.generate_host_info("participant2").await.unwrap();
    println!("{}: finished", function_name!());
}

pub async fn part1_accept_contract() {
    let mut client = DemoClientBuilder::build(TreatyInstance::Participant1).await;
    client.accept_pending_contract("lists-app").await.unwrap();
    println!("{}: finished", function_name!());
}

pub async fn part2_accept_contract() {
    let mut client = DemoClientBuilder::build(TreatyInstance::Participant2).await;
    client.accept_pending_contract("lists-app").await.unwrap();
    println!("{}: finished", function_name!());
}

/// Add our first participant to our database. In a normal application, this process would be akin
/// to a sign-up process from a user.
pub async fn add_participant1() {
    let mut client = DemoClientBuilder::build(TreatyInstance::Host).await;

    let participant_ip4addr = "treaty-part-1";
    let participant_db_port = 50052;

    // add our participant (user) to treaty
    client
        .add_participant(
            DB_NAME,
            "participant1",
            participant_ip4addr,
            participant_db_port,
            "",
            0,
            None,
        )
        .await
        .unwrap();

    // add our participant (user) to our users table by way of the model we created in `user.rs`
    let user1 = User::new("participant1");
    user1.add().await;

    println!("{}: finished", function_name!());
}

/// Add our second participant to our database. In a normal application, this process would be akin
/// to a sign-up process from a user.
pub async fn add_participant2() {
    let mut client = DemoClientBuilder::build(TreatyInstance::Host).await;

    let participant_ip4addr = "treaty-part-2";
    let participant_db_port = 50052;

    client
        .add_participant(
            DB_NAME,
            "participant2",
            participant_ip4addr,
            participant_db_port,
            "",
            0,
            None,
        )
        .await
        .unwrap();

    let user2 = User::new("participant2");
    user2.add().await;

    println!("{}: finished", function_name!());
}

/// Send a copy of our database contract to our first participant.
pub async fn send_contract_to_participant1() {
    let mut client = DemoClientBuilder::build(TreatyInstance::Host).await;

    client
        .send_participant_contract(DB_NAME, "participant1")
        .await
        .unwrap();

    println!("{}: finished", function_name!());
}

/// Send a copy of our database contract to our second participant.
pub async fn send_contract_to_participant2() {
    let mut client = DemoClientBuilder::build(TreatyInstance::Host).await;

    client
        .send_participant_contract(DB_NAME, "participant2")
        .await
        .unwrap();

    println!("{}: finished", function_name!());
}

pub async fn part1_update_manual(){
    let mut client = DemoClientBuilder::build(TreatyInstance::Participant1).await;
    let db_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    client.change_updates_to_host_behavior(DB_NAME, "LISTS", treaty_types::enums::UpdatesToHostBehavior::DoNothing)
    .await.unwrap();

    let sql_statement = "UPDATE LISTS SET ITEM_NAME = 'yogurt' WHERE ITEM_NAME = 'milk';";
    let where_clause = "ITEM_NAME = 'milk'";

    client.execute_write_at_participant(DB_NAME, sql_statement, db_type, where_clause).await.unwrap();
}