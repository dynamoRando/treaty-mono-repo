use async_trait::async_trait;
use tonic::{Request, Response, Status};
use tracing::trace;

use crate::{
    db_interface::dbi_actions::DbiActions,
    remote::remote_actions::RemoteActions,
    treaty_proto::{
        user_service_server::UserService, AcceptPendingActionReply, AcceptPendingActionRequest,
        AcceptPendingContractReply, AcceptPendingContractRequest, AddParticipantReply,
        AddParticipantRequest, AuthRequest, ChangeDeletesFromHostBehaviorReply,
        ChangeDeletesFromHostBehaviorRequest, ChangeDeletesToHostBehaviorReply,
        ChangeDeletesToHostBehaviorRequest, ChangeHostStatusReply, ChangeHostStatusRequest,
        ChangeUpdatesFromHostBehaviorRequest, ChangeUpdatesToHostBehaviorReply,
        ChangeUpdatesToHostBehaviorRequest, ChangesUpdatesFromHostBehaviorReply,
        CreateUserDatabaseReply, CreateUserDatabaseRequest, EnableCoooperativeFeaturesReply,
        EnableCoooperativeFeaturesRequest, ExecuteCooperativeWriteReply,
        ExecuteCooperativeWriteRequest, ExecuteReadReply, ExecuteReadRequest, ExecuteWriteReply,
        ExecuteWriteRequest, GenerateContractReply, GenerateContractRequest, GenerateHostInfoReply,
        GenerateHostInfoRequest, GetActiveContractReply, GetActiveContractRequest,
        GetCooperativeHostsReply, GetCooperativeHostsRequest, GetDataHashReply, GetDataHashRequest,
        GetDataLogTableStatusReply, GetDataLogTableStatusRequest, GetDatabasesReply,
        GetDatabasesRequest, GetDeletesFromHostBehaviorReply, GetDeletesFromHostBehaviorRequest,
        GetDeletesToHostBehaviorReply, GetDeletesToHostBehaviorRequest,
        GetLogicalStoragePolicyReply, GetLogicalStoragePolicyRequest, GetLogsByLastNumberReply,
        GetLogsByLastNumberRequest, GetParticipantsReply, GetParticipantsRequest,
        GetPendingActionsReply, GetPendingActionsRequest, GetReadRowIdsReply, GetReadRowIdsRequest,
        GetSettingsReply, GetSettingsRequest, GetUpdatesFromHostBehaviorReply,
        GetUpdatesFromHostBehaviorRequest, GetUpdatesToHostBehaviorReply,
        GetUpdatesToHostBehaviorRequest, HasTableReply, HasTableRequest, HostInfoReply,
        RejectPendingContractReply, RejectPendingContractRequest, RevokeReply,
        SendParticipantContractReply, SendParticipantContractRequest, SetDataLogTableStatusReply,
        SetDataLogTableStatusRequest, SetLogicalStoragePolicyReply, SetLogicalStoragePolicyRequest,
        TestReply, TestRequest, TokenReply, TryAuthAtParticipantRequest, TryAuthAtPartipantReply,
        VersionReply, ViewPendingContractsReply, ViewPendingContractsRequest, DeleteUserDatabaseReply, DeleteUserDatabaseRequest
    },
};

use super::UserServiceHandler;

#[async_trait]
impl<
        D: DbiActions + Clone + Sync + Send + 'static,
        R: RemoteActions + Clone + Sync + Send + 'static,
    > UserService for UserServiceHandler<D, R>
{
    async fn is_online(
        &self,
        request: Request<TestRequest>,
    ) -> Result<Response<TestReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.is_online(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn get_logs_by_last_number(
        &self,
        request: Request<GetLogsByLastNumberRequest>,
    ) -> Result<Response<GetLogsByLastNumberReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        todo!();
    }

    async fn get_settings(
        &self,
        request: Request<GetSettingsRequest>,
    ) -> Result<Response<GetSettingsReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.get_settings(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn get_cooperative_hosts(
        &self,
        request: Request<GetCooperativeHostsRequest>,
    ) -> Result<Response<GetCooperativeHostsReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.get_cooperative_hosts(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn get_updates_from_host_behavior(
        &self,
        request: Request<GetUpdatesFromHostBehaviorRequest>,
    ) -> Result<Response<GetUpdatesFromHostBehaviorReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .get_updates_from_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn get_updates_to_host_behavior(
        &self,
        request: Request<GetUpdatesToHostBehaviorRequest>,
    ) -> Result<Response<GetUpdatesToHostBehaviorReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .get_updates_to_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn get_deletes_from_host_behavior(
        &self,
        request: Request<GetDeletesFromHostBehaviorRequest>,
    ) -> Result<Response<GetDeletesFromHostBehaviorReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .get_deletes_from_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn get_deletes_to_host_behavior(
        &self,
        request: Request<GetDeletesToHostBehaviorRequest>,
    ) -> Result<Response<GetDeletesToHostBehaviorReply>, Status> {
        let response = self
            .get_deletes_to_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn get_versions(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<VersionReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        // need to write an HTTP version as well
        todo!()
    }

    async fn get_host_info(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<HostInfoReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.get_host_info(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn revoke_token(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<RevokeReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.revoke_token(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn auth_for_token(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<TokenReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.auth_for_token(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn get_active_contract(
        &self,
        request: Request<GetActiveContractRequest>,
    ) -> Result<Response<GetActiveContractReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.get_active_contract(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn get_participants(
        &self,
        request: Request<GetParticipantsRequest>,
    ) -> Result<Response<GetParticipantsReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.get_participants(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn get_databases(
        &self,
        request: Request<GetDatabasesRequest>,
    ) -> Result<Response<GetDatabasesReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.get_databases(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn accept_pending_action_at_participant(
        &self,
        request: Request<AcceptPendingActionRequest>,
    ) -> Result<Response<AcceptPendingActionReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .accept_pending_action_at_participant(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn get_pending_actions_at_participant(
        &self,
        request: Request<GetPendingActionsRequest>,
    ) -> Result<Response<GetPendingActionsReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .get_pending_updates_at_participant(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn get_data_log_table_status_at_participant(
        &self,
        request: Request<GetDataLogTableStatusRequest>,
    ) -> Result<Response<GetDataLogTableStatusReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        unimplemented!();
    }

    async fn set_data_log_table_status_at_participant(
        &self,
        request: Request<SetDataLogTableStatusRequest>,
    ) -> Result<Response<SetDataLogTableStatusReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        unimplemented!();
    }

    async fn generate_host_info(
        &self,
        request: Request<GenerateHostInfoRequest>,
    ) -> Result<Response<GenerateHostInfoReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.generate_host_info(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn create_user_database(
        &self,
        request: Request<CreateUserDatabaseRequest>,
    ) -> Result<Response<CreateUserDatabaseReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.create_user_database(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn delete_user_database(
        &self,
        request: Request<DeleteUserDatabaseRequest>,
    ) -> Result<Response<DeleteUserDatabaseReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.delete_user_database(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn enable_coooperative_features(
        &self,
        request: Request<EnableCoooperativeFeaturesRequest>,
    ) -> Result<Response<EnableCoooperativeFeaturesReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .enable_coooperative_features(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn execute_read_at_host(
        &self,
        request: Request<ExecuteReadRequest>,
    ) -> Result<Response<ExecuteReadReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.execute_read_at_host(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn execute_read_at_participant(
        &self,
        request: Request<ExecuteReadRequest>,
    ) -> Result<Response<ExecuteReadReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.execute_read_at_participant(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn execute_write_at_host(
        &self,
        request: Request<ExecuteWriteRequest>,
    ) -> Result<Response<ExecuteWriteReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.execute_write_at_host(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn execute_write_at_participant(
        &self,
        request: Request<ExecuteWriteRequest>,
    ) -> Result<Response<ExecuteWriteReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .execute_write_at_participant(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    #[allow(unused_assignments)]
    async fn execute_cooperative_write_at_host(
        &self,
        request: Request<ExecuteCooperativeWriteRequest>,
    ) -> Result<Response<ExecuteCooperativeWriteReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .execute_cooperative_write_at_host(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn has_table(
        &self,
        request: Request<HasTableRequest>,
    ) -> Result<Response<HasTableReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.has_table(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn set_logical_storage_policy(
        &self,
        request: Request<SetLogicalStoragePolicyRequest>,
    ) -> Result<Response<SetLogicalStoragePolicyReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.set_logical_storage_policy(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn get_logical_storage_policy(
        &self,
        request: Request<GetLogicalStoragePolicyRequest>,
    ) -> Result<Response<GetLogicalStoragePolicyReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.get_logical_storage_policy(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn generate_contract(
        &self,
        request: Request<GenerateContractRequest>,
    ) -> Result<Response<GenerateContractReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.generate_contract(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn add_participant(
        &self,
        request: Request<AddParticipantRequest>,
    ) -> Result<Response<AddParticipantReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.add_participant(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn send_participant_contract(
        &self,
        request: Request<SendParticipantContractRequest>,
    ) -> Result<Response<SendParticipantContractReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.send_participant_contract(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn review_pending_contracts(
        &self,
        request: Request<ViewPendingContractsRequest>,
    ) -> Result<Response<ViewPendingContractsReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.review_pending_contracts(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn accept_pending_contract(
        &self,
        request: Request<AcceptPendingContractRequest>,
    ) -> Result<Response<AcceptPendingContractReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.accept_pending_contract(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn reject_pending_contract(
        &self,
        _request: tonic::Request<RejectPendingContractRequest>,
    ) -> Result<tonic::Response<RejectPendingContractReply>, tonic::Status> {
        unimplemented!();
    }

    async fn change_host_status(
        &self,
        request: tonic::Request<ChangeHostStatusRequest>,
    ) -> Result<tonic::Response<ChangeHostStatusReply>, tonic::Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.change_host_status(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn try_auth_at_participant(
        &self,
        request: tonic::Request<TryAuthAtParticipantRequest>,
    ) -> Result<tonic::Response<TryAuthAtPartipantReply>, tonic::Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.try_auth_at_participant(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn change_updates_from_host_behavior(
        &self,
        request: Request<ChangeUpdatesFromHostBehaviorRequest>,
    ) -> Result<Response<ChangesUpdatesFromHostBehaviorReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .change_updates_from_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn change_deletes_from_host_behavior(
        &self,
        request: Request<ChangeDeletesFromHostBehaviorRequest>,
    ) -> Result<Response<ChangeDeletesFromHostBehaviorReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .change_deletes_from_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn change_updates_to_host_behavior(
        &self,
        request: Request<ChangeUpdatesToHostBehaviorRequest>,
    ) -> Result<Response<ChangeUpdatesToHostBehaviorReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .change_updates_to_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn change_deletes_to_host_behavior(
        &self,
        request: Request<ChangeDeletesToHostBehaviorRequest>,
    ) -> Result<Response<ChangeDeletesToHostBehaviorReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .change_deletes_to_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn read_row_id_at_participant(
        &self,
        request: Request<GetReadRowIdsRequest>,
    ) -> Result<Response<GetReadRowIdsReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.read_row_id_at_participant(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn get_data_hash_at_host(
        &self,
        request: Request<GetDataHashRequest>,
    ) -> Result<Response<GetDataHashReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.get_data_hash_at_host(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn get_data_hash_at_participant(
        &self,
        request: Request<GetDataHashRequest>,
    ) -> Result<Response<GetDataHashReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .get_data_hash_at_participant(request.into_inner())
            .await;
        Ok(Response::new(response))
    }
}
