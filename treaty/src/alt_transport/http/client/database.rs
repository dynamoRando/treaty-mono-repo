use crate::treaty_proto::{
    CreateUserDatabaseReply, CreateUserDatabaseRequest, DeleteUserDatabaseReply,
    DeleteUserDatabaseRequest, EnableCoooperativeFeaturesReply, EnableCoooperativeFeaturesRequest,
    GenerateContractReply, GenerateContractRequest, GetActiveContractReply,
    GetActiveContractRequest, GetDataHashReply, GetDataHashRequest, GetDatabasesReply,
    GetLogicalStoragePolicyReply, GetLogicalStoragePolicyRequest, GetReadRowIdsReply,
    GetReadRowIdsRequest, HasTableReply, HasTableRequest, SetLogicalStoragePolicyReply,
    SetLogicalStoragePolicyRequest,
};
use rocket::{http::Status, post, serde::json::Json, State};

use crate::alt_transport::http::HttpServer;

pub mod actions;
pub mod behavior;
pub mod participant;

#[post("/client/databases")]
pub async fn post_get_databases(x: &State<HttpServer>) -> (Status, Json<GetDatabasesReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST
    let core = x.user().await;
    let result = core.get_databases().await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/table/policy/get",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_logical_storage_policy(
    request: Json<GetLogicalStoragePolicyRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetLogicalStoragePolicyReply>) {
    let core = x.user().await;
    let result = core.get_logical_storage_policy(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/table/policy/set",
    format = "application/json",
    data = "<request>"
)]
pub async fn set_logical_storage_policy(
    request: Json<SetLogicalStoragePolicyRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<SetLogicalStoragePolicyReply>) {
    let core = x.user().await;
    let result = core.set_logical_storage_policy(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/contract/generate",
    format = "application/json",
    data = "<request>"
)]
pub async fn generate_contract(
    request: Json<GenerateContractRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GenerateContractReply>) {
    let core = x.user().await;
    let result = core.generate_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/new",
    format = "application/json",
    data = "<request>"
)]
pub async fn new_database(
    request: Json<CreateUserDatabaseRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<CreateUserDatabaseReply>) {
    let core = x.user().await;
    let result = core.create_user_database(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/delete-force",
    format = "application/json",
    data = "<request>"
)]
pub async fn delete_database_forcefully(
    request: Json<DeleteUserDatabaseRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<DeleteUserDatabaseReply>) {
    let core = x.user().await;
    let result = core
        .delete_user_database_forcefully(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/contract/get",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_active_contact(
    request: Json<GetActiveContractRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetActiveContractReply>) {
    let core = x.user().await;
    let result = core.get_active_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/enable-cooperative-features",
    format = "application/json",
    data = "<request>"
)]
pub async fn enable_coooperative_features(
    request: Json<EnableCoooperativeFeaturesRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<EnableCoooperativeFeaturesReply>) {
    let core = x.user().await;
    let result = core
        .enable_coooperative_features(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/participant/io/get",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_row_id_at_participant(
    request: Json<GetReadRowIdsRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetReadRowIdsReply>) {
    let core = x.user().await;
    let result = core.read_row_id_at_participant(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/participant/io/get-hash",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_data_hash_at_participant(
    request: Json<GetDataHashRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetDataHashReply>) {
    let core = x.user().await;
    let result = core
        .get_data_hash_at_participant(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/host/io/get-hash",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_data_hash_at_host(
    request: Json<GetDataHashRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetDataHashReply>) {
    let core = x.user().await;
    let result = core.get_data_hash_at_host(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/has_table",
    format = "application/json",
    data = "<request>"
)]
pub async fn has_table(
    request: Json<HasTableRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<HasTableReply>) {
    let core = x.user().await;
    let result = core.has_table(request.into_inner()).await;

    (Status::Ok, Json(result))
}
