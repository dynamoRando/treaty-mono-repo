use async_trait::async_trait;
use treaty_types::enums::*;
use treaty::{
    treaty_proto::{
        AcceptPendingActionReply, Contract, GetActiveContractReply, GetCooperativeHostsReply,
        GetDatabasesReply, GetDeletesFromHostBehaviorReply, GetDeletesToHostBehaviorReply,
        GetParticipantsReply, GetPendingActionsReply, GetUpdatesFromHostBehaviorReply,
        GetUpdatesToHostBehaviorReply, HostInfoReply, RevokeReply, StatementResultset, TokenReply,
        TreatyError, GetSettingsReply,
    },
};

/// The types of actions that can be performed by a Treaty client. Note: This is an async trait, which may lead to some auto-generated code.
#[async_trait]
pub trait ClientActions {

    /// Checks to see if the configured Treaty instance is available.
    async fn is_online(&mut self) -> Result<bool, TreatyError>;

    /// Requests information about the host.
    async fn get_host_info(&mut self) -> Result<HostInfoReply, TreatyError>;

    /// Returns the active contract for the specified database.
    async fn get_active_contract(
        &mut self,
        db_name: &str,
    ) -> Result<GetActiveContractReply, TreatyError>;

    /// Instructs treaty to revoke the Json Web Token the client currently has.
    async fn revoke_token(&mut self) -> Result<RevokeReply, TreatyError>;

    /// Requests Treaty to supply a Json Web Token based on the credentials the client has.
    async fn auth_for_token(&mut self) -> Result<TokenReply, TreatyError>;

    /// Accepts a pending action at a partial database. 
    /// 
    /// Example: A Host may have sent a DELETE to a Participant; but the Participant
    /// may have requested to queue all pending actions for review. 
    async fn accept_pending_action_at_participant(
        &mut self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<AcceptPendingActionReply, TreatyError>;

    /// Get a list of Hosts that we have participants and partial databases for. 
    async fn get_cooperative_hosts(&mut self) -> Result<GetCooperativeHostsReply, TreatyError>;

    /// Get a list of Participants that are part of a database we have.
    async fn get_participants_for_database(
        &mut self,
        db_name: &str,
    ) -> Result<GetParticipantsReply, TreatyError>;

    /// Get a list of all pending actions at a partial database.
    async fn get_pending_actions_at_participant(
        &mut self,
        db_name: &str,
        table_name: &str,
        action: &str,
    ) -> Result<GetPendingActionsReply, TreatyError>;

    /// Gets a specified row id at a partial database for the specified WHERE clause.
    async fn get_row_id_at_participant(
        &mut self,
        db_name: &str,
        table_name: &str,
        where_clause: &str,
    ) -> Result<Vec<u32>, TreatyError>;

    /// Gets a row hash from the partial database meta-table for the specified row id.
    async fn get_data_hash_at_participant(
        &mut self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<u64, TreatyError>;

    /// Gets a row hash from the host database for the specified row id.
    async fn get_data_hash_at_host(
        &mut self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<u64, TreatyError>;

    /// Gets the currently configured DeleteToHost behavior.
    async fn get_deletes_to_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
    ) -> Result<GetDeletesToHostBehaviorReply, TreatyError>;

    /// Changes the currently specified DeletesToHost behavior.
    async fn change_deletes_to_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
        behavior: DeletesToHostBehavior,
    ) -> Result<bool, TreatyError>;

    /// Gets the currently configured UpdatesToHost behavior.
    async fn get_updates_to_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
    ) -> Result<GetUpdatesToHostBehaviorReply, TreatyError>;

    /// Changes the currently configured UpdatesToHost behavior.
    async fn change_updates_to_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
        behavior: UpdatesToHostBehavior,
    ) -> Result<bool, TreatyError>;

    /// Gets the currently configured DeletesFromHost behavior.
    async fn get_deletes_from_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
    ) -> Result<GetDeletesFromHostBehaviorReply, TreatyError>;

    /// Changes the currently configured DeletesFromHost behavior.
    async fn change_deletes_from_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
        behavior: DeletesFromHostBehavior,
    ) -> Result<bool, TreatyError>;

    /// Gets the currently configured UpdatesFromHost behavior.
    async fn get_updates_from_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
    ) -> Result<GetUpdatesFromHostBehaviorReply, TreatyError>;

    /// Changes the currently configured UpdatesFromHost behavior.
    async fn change_updates_from_host_behavior(
        &mut self,
        db_name: &str,
        table_name: &str,
        behavior: UpdatesFromHostBehavior,
    ) -> Result<bool, TreatyError>;

    /// Changes the permission status of the specified host to either allow or deny connecting to us.
    async fn change_host_status_by_id(
        &mut self,
        host_id: &str,
        status: u32,
    ) -> Result<bool, TreatyError>;

    /// Changes the permission status of the specified host to either allow or deny connecting to us.
    async fn change_host_status_by_name(
        &mut self,
        host_name: &str,
        status: u32,
    ) -> Result<bool, TreatyError>;

    /// Generates a host id and assigns Treaty the specified host name.
    async fn generate_host_info(&mut self, host_name: &str) -> Result<bool, TreatyError>;

    /// Gets a list of databases currently at Treaty.
    async fn get_databases(&mut self) -> Result<GetDatabasesReply, TreatyError>;

    /// Executes a cooperative INSERT/UPDATE/DELETE against the specified Host database. This takes a SQL statement and a WHERE clause
    /// and tries to hash the data and saves the hash at the Host while sending the actual data to the specified Participant.
    /// 
    /// Note: The WHERE clause can be empty if the INSERT/UPDATE/DELETE statement is not targeting specific rows.
    async fn execute_cooperative_write_at_host(
        &mut self,
        db_name: &str,
        cmd: &str,
        participant_alias: &str,
        where_clause: &str,
    ) -> Result<bool, TreatyError>;

    /// Gets a list of contracts from Treaty that are pending acceptance or rejection.
    async fn view_pending_contracts(&mut self) -> Result<Vec<Contract>, TreatyError>;

    /// Accepts a pending contract sent to us by the specified host alias.
    async fn accept_pending_contract(&mut self, host_alias: &str) -> Result<bool, TreatyError>;

    /// Sends a copy of the active database contract to the specified participant for acceptance or rejection.
    async fn send_participant_contract(
        &mut self,
        db_name: &str,
        participant_alias: &str,
    ) -> Result<bool, TreatyError>;

    /// Adds a participant to our Treaty database with the specified values.
    /// # Arguments:
    /// * `db_name` - The database we want to add the participant for.
    /// * `participant_alias` - A friendly name for the participant that we can reference.
    /// * `participant_ip4addr` - The IP address for the participant in IP4 format.
    /// * `participant_db_port` - The database port Treaty will use for cooperation of data. Default is usually 50052.
    /// * `participant_http_addr` - The HTTP address for the participant if applicable.
    /// * `participant_http_port` - The HTTP port for the participant if applicable. Default is usually 50055.
    /// * `participant_id` - The host_id of the participant. This is used only if the Treaty instance we are connecting to is actually being
    /// hosted by a `treaty-proxy` instance, which is a SaaS version of Treaty. 
    async fn add_participant(
        &mut self,
        db_name: &str,
        participant_alias: &str,
        participant_ip4addr: &str,
        participant_db_port: u32,
        participant_http_addr: &str,
        participant_http_port: u16,
        participant_id: Option<String>,
    ) -> Result<bool, TreatyError>;

    /// Generates a database contract with the provided values.
    /// 
    /// Note: In order to generate a contract, you must make sure that all tables in your database have a Logical Storage Policy defined. 
    /// For more information, see the manual.
    async fn generate_contract(
        &mut self,
        db_name: &str,
        host_name: &str,
        desc: &str,
        remote_delete_behavior: RemoteDeleteBehavior,
    ) -> Result<bool, TreatyError>;

    /// Checks to see if the specified table in the specified database exists.
    async fn has_table(&mut self, db_name: &str, table_name: &str) -> Result<bool, TreatyError>;

    /// Gets the Logical Storage Policy for the specified table.
    async fn get_logical_storage_policy(
        &mut self,
        db_name: &str,
        table_name: &str,
    ) -> Result<LogicalStoragePolicy, TreatyError>;

    /// Sets the Logical Storage Policy for the specified table.
    async fn set_logical_storage_policy(
        &mut self,
        db_name: &str,
        table_name: &str,
        policy: LogicalStoragePolicy,
    ) -> Result<bool, TreatyError>;

    /// Executes the specified INSERT/UPDATE/DELETE statement at the specified Host database. The WHERE clause can be empty 
    /// if the actual INSERT/UPDATE/DELETE isn't targeting a specific set of rows.
    async fn execute_write_at_host(
        &mut self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
        where_clause: &str,
    ) -> Result<bool, TreatyError>;

    /// Executes the specified INSERT/UPDATE/DELETE statement at the specified Partial database. The WHERE clause can be empty 
    /// if the actual INSERT/UPDATE/DELETE isn't targeting a specific set of rows.
    async fn execute_write_at_participant(
        &mut self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
        where_clause: &str,
    ) -> Result<bool, TreatyError>;

    /// Attempts to authenticate this Treaty instance at the specified Participant. 
    async fn try_auth_at_participant(
        &mut self,
        alias: &str,
        id: &str,
        db_name: &str,
    ) -> Result<bool, TreatyError>;

    /// Executes the specified SELECT statement against the specified Partial database.
    async fn execute_read_at_participant(
        &mut self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
    ) -> Result<StatementResultset, TreatyError>;

    /// Executes the specified SELECT statement against the specified Host database.
    async fn execute_read_at_host(
        &mut self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
    ) -> Result<StatementResultset, TreatyError>;

    /// Enables cooperative features for the specified database. This instructs Treaty to create additional meta-data structures 
    /// to support cooperation.
    async fn enable_cooperative_features(&mut self, db_name: &str) -> Result<bool, TreatyError>;

    /// Creates the specified database.
    async fn create_user_database(&mut self, db_name: &str) -> Result<bool, TreatyError>;

    /// Gets the settings for the Treaty instance.
    async fn get_settings(&mut self) -> Result<GetSettingsReply, TreatyError>;

    /// Returns a debug formatted version of our client.
    fn debug(&self) -> String;
}
