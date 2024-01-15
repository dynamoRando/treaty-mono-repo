use async_trait::async_trait;

use crate::treaty_proto::{
    CreatePartialDatabaseRequest, CreatePartialDatabaseResult, CreateTableRequest,
    CreateTableResult, DeleteDataRequest, DeleteDataResult, GetRowFromPartialDatabaseRequest,
    GetRowFromPartialDatabaseResult, InsertDataRequest, InsertDataResult,
    NotifyHostOfRemovedRowRequest, NotifyHostOfRemovedRowResult, TestReply, TestRequest,
    TryAuthResult, UpdateDataRequest, UpdateDataResult, UpdateRowDataHashForHostRequest,
    UpdateRowDataHashForHostResult,
};

#[async_trait]
pub trait DataServiceHandlerActions {
    async fn delete_command_into_table(&self, request: DeleteDataRequest) -> DeleteDataResult;
    async fn get_row_from_partial_database(
        &self,
        request: GetRowFromPartialDatabaseRequest,
    ) -> GetRowFromPartialDatabaseResult;
    async fn update_command_into_table(&self, request: UpdateDataRequest) -> UpdateDataResult;
    async fn insert_command_into_table(&self, request: InsertDataRequest) -> InsertDataResult;
    async fn create_table_in_database(&self, request: CreateTableRequest) -> CreateTableResult;
    async fn create_partial_database(
        &self,
        request: CreatePartialDatabaseRequest,
    ) -> CreatePartialDatabaseResult;
    async fn update_row_data_hash_for_host(
        &self,
        request: UpdateRowDataHashForHostRequest,
    ) -> UpdateRowDataHashForHostResult;
    async fn notify_host_of_removed_row(
        &self,
        request: NotifyHostOfRemovedRowRequest,
    ) -> NotifyHostOfRemovedRowResult;
    async fn try_auth(&self) -> TryAuthResult;
    async fn is_online(&self, request: TestRequest) -> TestReply;
}
