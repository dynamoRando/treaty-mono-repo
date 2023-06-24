use client_actions::ClientActions;
use treaty_types::enums::*;
use grpc::GrpcClient;
use http::HttpClient;
use treaty::{
  
    treaty_proto::{
        AcceptPendingActionReply, Contract, GetActiveContractReply, GetCooperativeHostsReply,
        GetDatabasesReply, GetDeletesFromHostBehaviorReply, GetDeletesToHostBehaviorReply,
        GetParticipantsReply, GetPendingActionsReply, GetUpdatesFromHostBehaviorReply,
        GetUpdatesToHostBehaviorReply, HostInfoReply, RevokeReply, StatementResultset, TokenReply,
        TreatyError,
    },
};


pub mod client_actions;
pub mod error;

#[doc(hidden)]
pub mod grpc;
#[doc(hidden)]
pub mod http;

#[derive(Debug, Clone)]
pub enum TreatyClientType {
    Grpc,
    Http,
}

#[derive(Debug, Clone)]
pub struct Auth {
    pub user_name: String,
    pub pw: String,
    pub jwt: String,
}

#[derive(Debug, Clone)]
pub struct TreatyClient<C: ClientActions> {
    client: C,
    client_type: TreatyClientType,
}

impl<C: ClientActions> TreatyClient<C> {
    pub fn client_type(&self) -> TreatyClientType {
        self.client_type.clone()
    }

    /// Create a new TreatyClient using gRPC + Protobuf transport
    /// # Arguments
    /// * `addr_port` - The IP address and user port for Treaty, i.e. `127.0.0.1:50051`
    /// * `timeout_in_seconds` - The gRPC connection timeout, in seconds
    /// * `auth` - The Auth struct for passing credentials to Treaty
    /// * `send_jwt_if_available` - Send a Json Web Token in place of Auth credentials if we have recieved one from Treaty
    /// * `host_id` - Optional. Sends the Treaty's `host_id` to denote a specific Treaty instance we wish to talk to. This is used mainly by `treaty-proxy.`
    pub async fn new_grpc(
        addr_port: &str,
        timeout_in_seconds: u32,
        auth: Auth,
        send_jwt_if_available: bool,
        host_id: Option<String>,
    ) -> TreatyClient<GrpcClient> {
        let grpc = GrpcClient::new(
            addr_port,
            timeout_in_seconds,
            auth,
            send_jwt_if_available,
            host_id,
        )
        .await;
        TreatyClient::_new_gprc(grpc).await
    }

    /// Create a new TreatyClient using HTTP + JSON transport
    /// # Arguments
    /// * `addr` - The IP addr or URL for Treaty
    /// * `port` - The port number for Treaty
    /// * `auth` - The Auth struct for passing credentials to Treaty 
    /// * `timeout_in_seconds` - The HTTP timeout in seconds
    /// * `send_jwt_if_available` - Send a Json Web Token in place of Auth credentials if we have recieved one from Treaty
    /// * `host_id` - Optional. Sends the Treaty's `host_id` to denote a specific Treaty instance we wish to talk to. This is used mainly by `treaty-proxy.`
    pub async fn new_http(
        addr: &str,
        port: u32,
        auth: Auth,
        timeout_in_seconds: u32,
        send_jwt_if_available: bool,
        host_id: Option<String>,
    ) -> TreatyClient<HttpClient> {
        let http = HttpClient::new(
            addr,
            port,
            auth,
            timeout_in_seconds,
            send_jwt_if_available,
            host_id,
        )
        .await;
        TreatyClient::_new_http(http).await
    }

    async fn _new_gprc(client: C) -> Self {
        Self {
            client,
            client_type: TreatyClientType::Grpc,
        }
    }

    async fn _new_http(client: C) -> Self {
        Self {
            client,
            client_type: TreatyClientType::Http,
        }
    }
}

impl<C: ClientActions> ClientActions for TreatyClient<C> {
    fn debug(&self) -> String {
        self.client.debug()
    }

    fn get_host_info<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<HostInfoReply, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.client.get_host_info()
    }

    fn is_online<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.client.is_online()
    }

    fn get_active_contract<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<GetActiveContractReply, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.client.get_active_contract(db_name)
    }

    fn revoke_token<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<RevokeReply, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.client.revoke_token()
    }

    fn auth_for_token<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<TokenReply, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.client.auth_for_token()
    }

    fn accept_pending_action_at_participant<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
        row_id: u32,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<AcceptPendingActionReply, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .accept_pending_action_at_participant(db_name, table_name, row_id)
    }

    fn get_settings<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<
                    Output = Result<treaty::treaty_proto::GetSettingsReply, TreatyError>,
                > + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.client.get_settings()
    }

    fn get_cooperative_hosts<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<GetCooperativeHostsReply, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.client.get_cooperative_hosts()
    }

    fn get_participants_for_database<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<GetParticipantsReply, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.client.get_participants_for_database(db_name)
    }

    fn get_pending_actions_at_participant<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
        action: &'life3 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<GetPendingActionsReply, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .get_pending_actions_at_participant(db_name, table_name, action)
    }

    fn get_row_id_at_participant<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
        where_clause: &'life3 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Vec<u32>, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .get_row_id_at_participant(db_name, table_name, where_clause)
    }

    fn get_data_hash_at_participant<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
        row_id: u32,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<u64, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .get_data_hash_at_participant(db_name, table_name, row_id)
    }

    fn get_data_hash_at_host<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
        row_id: u32,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<u64, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .get_data_hash_at_host(db_name, table_name, row_id)
    }

    fn get_deletes_to_host_behavior<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<GetDeletesToHostBehaviorReply, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .get_deletes_to_host_behavior(db_name, table_name)
    }

    fn change_deletes_to_host_behavior<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
        behavior: DeletesToHostBehavior,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .change_deletes_to_host_behavior(db_name, table_name, behavior)
    }

    fn get_updates_to_host_behavior<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<GetUpdatesToHostBehaviorReply, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .get_updates_to_host_behavior(db_name, table_name)
    }

    fn change_updates_to_host_behavior<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
        behavior: UpdatesToHostBehavior,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .change_updates_to_host_behavior(db_name, table_name, behavior)
    }

    fn get_deletes_from_host_behavior<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<GetDeletesFromHostBehaviorReply, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .get_deletes_from_host_behavior(db_name, table_name)
    }

    fn change_deletes_from_host_behavior<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
        behavior: DeletesFromHostBehavior,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .change_deletes_from_host_behavior(db_name, table_name, behavior)
    }

    fn get_updates_from_host_behavior<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<GetUpdatesFromHostBehaviorReply, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .get_updates_from_host_behavior(db_name, table_name)
    }

    fn change_updates_from_host_behavior<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
        behavior: UpdatesFromHostBehavior,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .change_updates_from_host_behavior(db_name, table_name, behavior)
    }

    fn change_host_status_by_id<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        host_id: &'life1 str,
        status: u32,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.client.change_host_status_by_id(host_id, status)
    }

    fn change_host_status_by_name<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        host_name: &'life1 str,
        status: u32,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.client.change_host_status_by_name(host_name, status)
    }

    fn generate_host_info<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        host_name: &'life1 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.client.generate_host_info(host_name)
    }

    fn get_databases<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<GetDatabasesReply, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.client.get_databases()
    }

    fn execute_cooperative_write_at_host<'life0, 'life1, 'life2, 'life3, 'life4, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        cmd: &'life2 str,
        participant_alias: &'life3 str,
        where_clause: &'life4 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        'life4: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .execute_cooperative_write_at_host(db_name, cmd, participant_alias, where_clause)
    }

    fn view_pending_contracts<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Vec<Contract>, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.client.view_pending_contracts()
    }

    fn accept_pending_contract<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        host_alias: &'life1 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.client.accept_pending_contract(host_alias)
    }

    fn send_participant_contract<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        participant_alias: &'life2 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .send_participant_contract(db_name, participant_alias)
    }

    fn add_participant<'life0, 'life1, 'life2, 'life3, 'life4, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        participant_alias: &'life2 str,
        participant_ip4addr: &'life3 str,
        participant_db_port: u32,
        participant_http_addr: &'life4 str,
        participant_http_port: u16,
        participant_id: Option<String>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        'life4: 'async_trait,
        Self: 'async_trait,
    {
        self.client.add_participant(
            db_name,
            participant_alias,
            participant_ip4addr,
            participant_db_port,
            participant_http_addr,
            participant_http_port,
            participant_id,
        )
    }

    fn generate_contract<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        host_name: &'life2 str,
        desc: &'life3 str,
        remote_delete_behavior: RemoteDeleteBehavior,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .generate_contract(db_name, host_name, desc, remote_delete_behavior)
    }

    fn has_table<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client.has_table(db_name, table_name)
    }

    fn get_logical_storage_policy<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<LogicalStoragePolicy, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client.get_logical_storage_policy(db_name, table_name)
    }

    fn set_logical_storage_policy<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        table_name: &'life2 str,
        policy: LogicalStoragePolicy,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .set_logical_storage_policy(db_name, table_name, policy)
    }

    fn execute_write_at_host<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        sql_statement: &'life2 str,
        db_type: u32,
        where_clause: &'life3 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .execute_write_at_host(db_name, sql_statement, db_type, where_clause)
    }

    fn execute_write_at_participant<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        sql_statement: &'life2 str,
        db_type: u32,
        where_clause: &'life3 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .execute_write_at_participant(db_name, sql_statement, db_type, where_clause)
    }

    fn try_auth_at_participant<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 mut self,
        alias: &'life1 str,
        id: &'life2 str,
        db_name: &'life3 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: 'async_trait,
    {
        self.client.try_auth_at_participant(alias, id, db_name)
    }

    fn execute_read_at_participant<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        sql_statement: &'life2 str,
        db_type: u32,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<StatementResultset, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .execute_read_at_participant(db_name, sql_statement, db_type)
    }

    fn execute_read_at_host<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
        sql_statement: &'life2 str,
        db_type: u32,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<StatementResultset, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.client
            .execute_read_at_host(db_name, sql_statement, db_type)
    }

    fn enable_cooperative_features<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.client.enable_cooperative_features(db_name)
    }

    fn create_user_database<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        db_name: &'life1 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<bool, TreatyError>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.client.create_user_database(db_name)
    }
}
