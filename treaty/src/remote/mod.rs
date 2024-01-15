use async_trait::async_trait;

use crate::treaty_proto::TreatyPorts;

use self::remote_actions::RemoteActions;

#[allow(dead_code, unused_variables)]
pub(crate) mod remote_actions;
#[allow(dead_code, unused_variables)]
pub(crate) mod remote_grpc;
#[allow(dead_code, unused_variables)]
pub(crate) mod remote_http;

/// A remote client for connecting to another Treaty instance
#[derive(Debug, Clone)]
pub(crate) struct Remote<R: RemoteActions> {
    r: R,
}

impl<R: RemoteActions> Remote<R> {
    pub fn new(remote: R) -> Self {
        Self { r: remote }
    }
}

#[async_trait]
impl<R: RemoteActions + Clone> RemoteActions for Remote<R> {
    fn try_auth_at_participant<'life0, 'life1, 'async_trait>(
        &'life0 self,
        participant: crate::models::CoopDatabaseParticipant,
        own_host_info: &'life1 crate::models::HostInfo,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = bool> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.r.try_auth_at_participant(participant, own_host_info)
    }

    fn send_participant_contract<'life0, 'async_trait>(
        &'life0 self,
        participant: crate::models::CoopDatabaseParticipant,
        host_info: crate::models::HostInfo,
        contract: crate::models::CoopDatabaseContract,
        db_schema: crate::treaty_proto::DatabaseSchema,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = crate::models::TreatySaveContractResult>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.r
            .send_participant_contract(participant, host_info, contract, db_schema)
    }

    fn notify_host_of_removed_row<'life0, 'life1, 'life2, 'life3, 'life4, 'async_trait>(
        &'life0 self,
        host: &'life1 crate::models::CdsHosts,
        own_host_info: &'life2 crate::models::HostInfo,
        db_name: &'life3 str,
        table_name: &'life4 str,
        row_id: u32,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = bool> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        'life4: 'async_trait,
        Self: 'async_trait,
    {
        self.r
            .notify_host_of_removed_row(host, own_host_info, db_name, table_name, row_id)
    }

    fn remove_row_at_participant<'life0, 'life1, 'life2, 'life3, 'life4, 'life5, 'async_trait>(
        &'life0 self,
        participant: crate::models::CoopDatabaseParticipant,
        own_host_info: &'life1 crate::models::HostInfo,
        db_name: &'life2 str,
        table_name: &'life3 str,
        sql: &'life4 str,
        where_clause: &'life5 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = crate::treaty_proto::DeleteDataResult>
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
        'life5: 'async_trait,
        Self: 'async_trait,
    {
        self.r.remove_row_at_participant(
            participant,
            own_host_info,
            db_name,
            table_name,
            sql,
            where_clause,
        )
    }

    fn update_row_at_participant<'life0, 'life1, 'life2, 'life3, 'life4, 'life5, 'async_trait>(
        &'life0 self,
        participant: crate::models::CoopDatabaseParticipant,
        own_host_info: &'life1 crate::models::HostInfo,
        db_name: &'life2 str,
        table_name: &'life3 str,
        sql: &'life4 str,
        where_clause: &'life5 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = crate::treaty_proto::UpdateDataResult>
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
        'life5: 'async_trait,
        Self: 'async_trait,
    {
        self.r.update_row_at_participant(
            participant,
            own_host_info,
            db_name,
            table_name,
            sql,
            where_clause,
        )
    }

    fn insert_row_at_participant<'life0, 'life1, 'life2, 'life3, 'life4, 'async_trait>(
        &'life0 self,
        participant: crate::models::CoopDatabaseParticipant,
        own_host_info: &'life1 crate::models::HostInfo,
        db_name: &'life2 str,
        table_name: &'life3 str,
        sql: &'life4 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = crate::treaty_proto::InsertDataResult>
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
        self.r
            .insert_row_at_participant(participant, own_host_info, db_name, table_name, sql)
    }

    fn get_row_from_participant<'life0, 'async_trait>(
        &'life0 self,
        participant: crate::models::CoopDatabaseParticipantData,
        own_host_info: crate::models::HostInfo,
        row_id: u32,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = crate::treaty_proto::GetRowFromPartialDatabaseResult>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.r
            .get_row_from_participant(participant, own_host_info, row_id)
    }

    fn notify_host_of_updated_hash<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 self,
        host: &'life1 crate::models::CdsHosts,
        own_host_info: &'life2 crate::models::HostInfo,
        data_info: &'life3 crate::models::DataInfo,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = bool> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: 'async_trait,
    {
        self.r
            .notify_host_of_updated_hash(host, own_host_info, data_info)
    }

    fn notify_host_of_acceptance_of_contract<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        accepted_contract: &'life1 crate::treaty_proto::Contract,
        own_host_info: &'life2 crate::models::HostInfo,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = bool> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.r
            .notify_host_of_acceptance_of_contract(accepted_contract, own_host_info)
    }

    fn get_remote_ports<'life0, 'life1, 'async_trait>(
        &'life0 self,
        ip4addr: &'life1 str,
        info_port: u32,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = TreatyPorts> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.r.get_remote_ports(ip4addr, info_port)
    }
}
