use async_trait::async_trait;
use tonic::{Request, Response, Status};
use tracing::trace;

use crate::{
    db_interface::dbi_actions::DbiActions,
    treaty_proto::{
        data_service_server::DataService, CreatePartialDatabaseRequest,
        CreatePartialDatabaseResult, CreateTableRequest, CreateTableResult, DeleteDataRequest,
        DeleteDataResult, GetRowFromPartialDatabaseRequest, GetRowFromPartialDatabaseResult,
        InsertDataRequest, InsertDataResult, NotifyHostOfRemovedRowRequest,
        NotifyHostOfRemovedRowResult, ParticipantAcceptsContractRequest,
        ParticipantAcceptsContractResult, SaveContractRequest, SaveContractResult, TestReply,
        TestRequest, TryAuthRequest, TryAuthResult, UpdateDataRequest, UpdateDataResult,
        UpdateRowDataHashForHostRequest, UpdateRowDataHashForHostResult,
    },
};

use super::DataServiceHandler;

#[async_trait]
impl<D: DbiActions + Clone + Sync + Send + 'static> DataService for DataServiceHandler<D> {
    async fn is_online(
        &self,
        request: Request<TestRequest>,
    ) -> Result<Response<TestReply>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.is_online(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn create_partial_database(
        &self,
        request: Request<CreatePartialDatabaseRequest>,
    ) -> Result<Response<CreatePartialDatabaseResult>, Status> {
        let response = self.create_partial_database(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn create_table_in_database(
        &self,
        request: Request<CreateTableRequest>,
    ) -> Result<Response<CreateTableResult>, Status> {
        let response = self.create_table_in_database(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn insert_command_into_table(
        &self,
        request: Request<InsertDataRequest>,
    ) -> Result<Response<InsertDataResult>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.insert_command_into_table(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn update_command_into_table(
        &self,
        request: Request<UpdateDataRequest>,
    ) -> Result<Response<UpdateDataResult>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.update_command_into_table(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn delete_command_into_table(
        &self,
        request: Request<DeleteDataRequest>,
    ) -> Result<Response<DeleteDataResult>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.delete_command_into_table(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn get_row_from_partial_database(
        &self,
        request: Request<GetRowFromPartialDatabaseRequest>,
    ) -> Result<Response<GetRowFromPartialDatabaseResult>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self
            .get_row_from_partial_database(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn save_contract(
        &self,
        request: Request<SaveContractRequest>,
    ) -> Result<Response<SaveContractResult>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.save_contract(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn accept_contract(
        &self,
        request: Request<ParticipantAcceptsContractRequest>,
    ) -> Result<Response<ParticipantAcceptsContractResult>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.accept_contract(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn update_row_data_hash_for_host(
        &self,
        request: Request<UpdateRowDataHashForHostRequest>,
    ) -> Result<Response<UpdateRowDataHashForHostResult>, Status> {
        trace!(
            "update_row_data_hash_for_host: Request from {:?}",
            request.remote_addr()
        );
        trace! {"{request:?}"};
        let response = self
            .update_row_data_hash_for_host(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn notify_host_of_removed_row(
        &self,
        request: Request<NotifyHostOfRemovedRowRequest>,
    ) -> Result<Response<NotifyHostOfRemovedRowResult>, Status> {
        trace!(
            "notify_host_of_removed_row: Request from {:?}",
            request.remote_addr()
        );
        let response = self.notify_host_of_removed_row(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn try_auth(
        &self,
        request: Request<TryAuthRequest>,
    ) -> Result<Response<TryAuthResult>, Status> {
        trace!("Request from {:?}", request.remote_addr());
        let response = self.try_auth(request.into_inner()).await;
        Ok(Response::new(response))
    }
}
