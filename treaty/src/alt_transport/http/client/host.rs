use crate::treaty_proto::{
    AuthRequest, GenerateHostInfoReply, GenerateHostInfoRequest, GetCooperativeHostsReply,
    GetCooperativeHostsRequest, HostInfoReply,
};
use rocket::State;
use rocket::{http::Status, post, serde::json::Json};

use crate::alt_transport::http::HttpServer;
use crate::user_service_handler::user_service_handler_actions::UserServiceHandlerActions;

#[post(
    "/client/host/generate",
    format = "application/json",
    data = "<request>"
)]
pub async fn generate_host_info(
    request: Json<GenerateHostInfoRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GenerateHostInfoReply>) {
    let core = x.user();
    let result = core.generate_host_info(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post("/client/host/get", format = "application/json", data = "<request>")]
pub async fn get_host_info(
    request: Json<AuthRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<HostInfoReply>) {
    let core = x.user();

    let result = core.get_host_info(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/host/get-coop-hosts",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_cooperative_hosts(
    request: Json<GetCooperativeHostsRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetCooperativeHostsReply>) {
    let core = x.user();
    let result = core.get_cooperative_hosts(request.into_inner()).await;

    (Status::Ok, Json(result))
}
