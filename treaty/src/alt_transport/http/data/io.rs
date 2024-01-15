use crate::{
    alt_transport::http::HttpServer,
    treaty_proto::{
        DeleteDataRequest, DeleteDataResult, GetRowFromPartialDatabaseRequest,
        GetRowFromPartialDatabaseResult, InsertDataRequest, InsertDataResult,
        NotifyHostOfRemovedRowRequest, NotifyHostOfRemovedRowResult, UpdateDataRequest,
        UpdateDataResult, UpdateRowDataHashForHostRequest, UpdateRowDataHashForHostResult,
    },
};
use rocket::{http::Status, post, serde::json::Json, State};
use stdext::function_name;
use tracing::trace;

#[post("/data/io/remove-row", format = "application/json", data = "<request>")]
pub async fn remove_row_at_participant(
    request: Json<DeleteDataRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<DeleteDataResult>) {
    let core = x.data().await;
    let result = core.delete_command_into_table(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/data/io/notify-host-removed-row",
    format = "application/json",
    data = "<request>"
)]
pub async fn notify_host_of_removed_row(
    request: Json<NotifyHostOfRemovedRowRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<NotifyHostOfRemovedRowResult>) {
    let core = x.data().await;
    let result = core.notify_host_of_removed_row(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post("/data/io/update-row", format = "application/json", data = "<request>")]
pub async fn update_row_at_participant(
    request: Json<UpdateDataRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<UpdateDataResult>) {
    let core = x.data().await;
    let result = core.update_command_into_table(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post("/data/io/insert-row", format = "application/json", data = "<request>")]
pub async fn insert_row_at_participant(
    request: Json<InsertDataRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<InsertDataResult>) {
    let core = x.data().await;

    trace!("[{}]: {request:?}", function_name!());

    let result = core.insert_command_into_table(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post("/data/io/get-row", format = "application/json", data = "<request>")]
pub async fn get_row_at_participant(
    request: Json<GetRowFromPartialDatabaseRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetRowFromPartialDatabaseResult>) {
    let core = x.data().await;
    let result = core
        .get_row_from_partial_database(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/data/io/notify-host-updated-hash",
    format = "application/json",
    data = "<request>"
)]
pub async fn notify_host_of_updated_hash(
    request: Json<UpdateRowDataHashForHostRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<UpdateRowDataHashForHostResult>) {
    let core = x.data().await;
    let result = core
        .update_row_data_hash_for_host(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}
