use crate::alt_transport::http::HttpServer;
use crate::treaty_proto::{
    ExecuteCooperativeWriteReply, ExecuteCooperativeWriteRequest, ExecuteReadReply,
    ExecuteReadRequest, ExecuteWriteReply, ExecuteWriteRequest,
};
use rocket::State;
use rocket::{http::Status, post, serde::json::Json};

use crate::user_service_handler::user_service_handler_actions::UserServiceHandlerActions;

#[post(
    "/client/sql/host/read",
    format = "application/json",
    data = "<request>"
)]
pub async fn read_at_host(
    request: Json<ExecuteReadRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ExecuteReadReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST
    let core = x.user();
    let result = core.execute_read_at_host(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/sql/host/write",
    format = "application/json",
    data = "<request>"
)]
pub async fn write_at_host(
    request: Json<ExecuteWriteRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ExecuteWriteReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST

    let core = x.user();
    let result = core.execute_write_at_host(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/sql/host/write/cooperative",
    format = "application/json",
    data = "<request>"
)]
pub async fn cooperative_write_at_host(
    request: Json<ExecuteCooperativeWriteRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ExecuteCooperativeWriteReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST

    let core = x.user();
    let result = core
        .execute_cooperative_write_at_host(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/sql/participant/write",
    format = "application/json",
    data = "<request>"
)]
pub async fn write_at_participant(
    request: Json<ExecuteWriteRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ExecuteWriteReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST

    let core = x.user();
    let result = core
        .execute_write_at_participant(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/sql/participant/read",
    format = "application/json",
    data = "<request>"
)]
pub async fn read_at_participant(
    request: Json<ExecuteReadRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ExecuteReadReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST

    let core = x.user();
    let result = core.execute_read_at_participant(request.into_inner()).await;

    (Status::Ok, Json(result))
}
