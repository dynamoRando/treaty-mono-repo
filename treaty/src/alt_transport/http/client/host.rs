use crate::treaty_proto::{
    GenerateHostInfoReply, GenerateHostInfoRequest, GetCooperativeHostsReply, HostInfoReply,
};
use rocket::State;
use rocket::{http::Status, post, serde::json::Json};

use crate::alt_transport::http::HttpServer;

#[post(
    "/client/host/generate",
    format = "application/json",
    data = "<request>"
)]
pub async fn generate_host_info(
    request: Json<GenerateHostInfoRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GenerateHostInfoReply>) {
    let core = x.user().await;
    let result = core.generate_host_info(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post("/client/host/get", format = "application/json")]
pub async fn get_host_info(x: &State<HttpServer>) -> (Status, Json<HostInfoReply>) {
    let core = x.user().await;

    let result = core.get_host_info().await;

    (Status::Ok, Json(result))
}

#[post("/client/host/get-coop-hosts", format = "application/json")]
pub async fn get_cooperative_hosts(
    x: &State<HttpServer>,
) -> (Status, Json<GetCooperativeHostsReply>) {
    let core = x.user().await;
    let result = core.get_cooperative_hosts().await;

    (Status::Ok, Json(result))
}
