use async_trait::async_trait;
use treaty_types::auth::AuthRequestData;

use crate::{
    settings::Settings,
    treaty_proto::{
        AcceptPendingActionReply, AcceptPendingActionRequest, AcceptPendingContractReply,
        AcceptPendingContractRequest, AddParticipantReply, AddParticipantRequest, AuthRequestBasic,
        AuthRequestWebToken, ChangeDeletesFromHostBehaviorReply,
        ChangeDeletesFromHostBehaviorRequest, ChangeDeletesToHostBehaviorReply,
        ChangeDeletesToHostBehaviorRequest, ChangeHostStatusReply, ChangeHostStatusRequest,
        ChangeUpdatesFromHostBehaviorRequest, ChangeUpdatesToHostBehaviorReply,
        ChangeUpdatesToHostBehaviorRequest, ChangesUpdatesFromHostBehaviorReply,
        CreateUserDatabaseReply, CreateUserDatabaseRequest, DeleteUserDatabaseReply,
        DeleteUserDatabaseRequest, EnableCoooperativeFeaturesReply,
        EnableCoooperativeFeaturesRequest, ExecuteCooperativeWriteReply,
        ExecuteCooperativeWriteRequest, ExecuteReadReply, ExecuteReadRequest, ExecuteWriteReply,
        ExecuteWriteRequest, GenerateContractReply, GenerateContractRequest, GenerateHostInfoReply,
        GenerateHostInfoRequest, GetActiveContractReply, GetActiveContractRequest,
        GetBackingDatabaseConfigReply, GetCooperativeHostsReply, GetDataHashReply,
        GetDataHashRequest, GetDatabasesReply, GetDeletesFromHostBehaviorReply,
        GetDeletesFromHostBehaviorRequest, GetDeletesToHostBehaviorReply,
        GetDeletesToHostBehaviorRequest, GetLogicalStoragePolicyReply,
        GetLogicalStoragePolicyRequest, GetParticipantsReply, GetParticipantsRequest,
        GetPendingActionsReply, GetPendingActionsRequest, GetReadRowIdsReply, GetReadRowIdsRequest,
        GetSettingsReply, GetUpdatesFromHostBehaviorReply, GetUpdatesFromHostBehaviorRequest,
        GetUpdatesToHostBehaviorReply, GetUpdatesToHostBehaviorRequest, HasTableReply,
        HasTableRequest, HostInfoReply, RevokeReply, SendParticipantContractReply,
        SendParticipantContractRequest, SetLogicalStoragePolicyReply,
        SetLogicalStoragePolicyRequest, TestReply, TestRequest, TokenReply,
        TryAuthAtParticipantRequest, TryAuthAtPartipantReply, ViewPendingContractsReply,
    },
};

#[async_trait]
pub trait UserServiceHandlerActions {
    async fn change_host_status(&self, request: ChangeHostStatusRequest) -> ChangeHostStatusReply;
    async fn is_online(&self, request: TestRequest) -> TestReply;
    async fn get_cooperative_hosts(&self) -> GetCooperativeHostsReply;
    async fn get_settings(&self) -> GetSettingsReply;
    async fn get_deletes_from_host_behavior(
        &self,
        request: GetDeletesFromHostBehaviorRequest,
    ) -> GetDeletesFromHostBehaviorReply;
    async fn get_deletes_to_host_behavior(
        &self,
        request: GetDeletesToHostBehaviorRequest,
    ) -> GetDeletesToHostBehaviorReply;
    async fn get_updates_from_host_behavior(
        &self,
        request: GetUpdatesFromHostBehaviorRequest,
    ) -> GetUpdatesFromHostBehaviorReply;
    async fn execute_write_at_host(&self, request: ExecuteWriteRequest) -> ExecuteWriteReply;
    async fn execute_read_at_host(&self, request: ExecuteReadRequest) -> ExecuteReadReply;
    async fn execute_read_at_participant(&self, request: ExecuteReadRequest) -> ExecuteReadReply;
    async fn enable_coooperative_features(
        &self,
        request: EnableCoooperativeFeaturesRequest,
    ) -> EnableCoooperativeFeaturesReply;
    async fn create_user_database(
        &self,
        request: CreateUserDatabaseRequest,
    ) -> CreateUserDatabaseReply;
    async fn delete_user_database(
        &self,
        request: DeleteUserDatabaseRequest,
    ) -> DeleteUserDatabaseReply;
    async fn delete_user_database_forcefully(
        &self,
        request: DeleteUserDatabaseRequest,
    ) -> DeleteUserDatabaseReply;
    async fn generate_host_info(&self, request: GenerateHostInfoRequest) -> GenerateHostInfoReply;
    async fn get_active_contract(
        &self,
        request: GetActiveContractRequest,
    ) -> GetActiveContractReply;
    async fn accept_pending_contract(
        &self,
        request: AcceptPendingContractRequest,
    ) -> AcceptPendingContractReply;
    async fn get_data_hash_at_host(&self, request: GetDataHashRequest) -> GetDataHashReply;
    async fn get_data_hash_at_participant(&self, request: GetDataHashRequest) -> GetDataHashReply;
    async fn read_row_id_at_participant(&self, request: GetReadRowIdsRequest)
        -> GetReadRowIdsReply;
    async fn change_deletes_to_host_behavior(
        &self,
        request: ChangeDeletesToHostBehaviorRequest,
    ) -> ChangeDeletesToHostBehaviorReply;
    async fn change_updates_to_host_behavior(
        &self,
        request: ChangeUpdatesToHostBehaviorRequest,
    ) -> ChangeUpdatesToHostBehaviorReply;
    async fn change_deletes_from_host_behavior(
        &self,
        request: ChangeDeletesFromHostBehaviorRequest,
    ) -> ChangeDeletesFromHostBehaviorReply;
    async fn change_updates_from_host_behavior(
        &self,
        request: ChangeUpdatesFromHostBehaviorRequest,
    ) -> ChangesUpdatesFromHostBehaviorReply;
    async fn review_pending_contracts(&self) -> ViewPendingContractsReply;
    async fn get_backing_database_config(&self) -> GetBackingDatabaseConfigReply;
    async fn send_participant_contract(
        &self,
        request: SendParticipantContractRequest,
    ) -> SendParticipantContractReply;
    async fn add_participant(&self, request: AddParticipantRequest) -> AddParticipantReply;
    async fn get_databases(&self) -> GetDatabasesReply;
    async fn get_pending_actions_at_participant(
        &self,
        request: GetPendingActionsRequest,
    ) -> GetPendingActionsReply;
    async fn accept_pending_action_at_participant(
        &self,
        request: AcceptPendingActionRequest,
    ) -> AcceptPendingActionReply;
    async fn get_participants(&self, request: GetParticipantsRequest) -> GetParticipantsReply;
    async fn execute_write_at_participant(&self, request: ExecuteWriteRequest)
        -> ExecuteWriteReply;
    async fn get_updates_to_host_behavior(
        &self,
        request: GetUpdatesToHostBehaviorRequest,
    ) -> GetUpdatesToHostBehaviorReply;
    async fn revoke_token(&self, request: AuthRequestWebToken) -> RevokeReply;
    async fn auth_for_token(&self, request: AuthRequestBasic) -> TokenReply;
    async fn set_logical_storage_policy(
        &self,
        request: SetLogicalStoragePolicyRequest,
    ) -> SetLogicalStoragePolicyReply;
    async fn get_logical_storage_policy(
        &self,
        request: GetLogicalStoragePolicyRequest,
    ) -> GetLogicalStoragePolicyReply;
    async fn try_auth_at_participant(
        &self,
        request: TryAuthAtParticipantRequest,
    ) -> TryAuthAtPartipantReply;
    async fn get_host_info(&self) -> HostInfoReply;
    async fn generate_contract(&self, request: GenerateContractRequest) -> GenerateContractReply;
    async fn has_table(&self, request: HasTableRequest) -> HasTableReply;
    async fn execute_cooperative_write_at_host(
        &self,
        request: ExecuteCooperativeWriteRequest,
    ) -> ExecuteCooperativeWriteReply;
    async fn authenticate_request(&self, auth_data: &AuthRequestData) -> bool;
    fn debug_settings(&self) -> Settings;
    fn debug_root_dir(&self) -> Option<String>;
}
