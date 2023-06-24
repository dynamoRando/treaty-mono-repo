use rocket::{get, http::Status, post, serde::json::Json, State};

pub mod contract;
pub mod database;
pub mod host;
pub mod logs;
pub mod sql;
use tracing::trace;

use crate::{
    alt_transport::http::HttpServer,
    defaults,
    treaty_proto::{
        AuthRequest, ChangeHostStatusReply, ChangeHostStatusRequest, GetSettingsReply,
        GetSettingsRequest, RevokeReply, TestReply, TestRequest, TokenReply,
        TryAuthAtParticipantRequest, TryAuthAtPartipantReply,
    },
    user_service_handler::user_service_handler_actions::UserServiceHandlerActions,
};

#[get("/client/status")]
pub async fn status() -> &'static str {
    "Status From Rocket"
}

#[post("/client/version", format = "application/json", data = "<request>")]
pub fn version(request: Json<TestRequest>) -> (Status, Json<TestReply>) {
    let response = TestReply {
        reply_time_utc: "".to_string(),
        reply_echo_message: request.request_echo_message.clone(),
        treaty_version: defaults::VERSION.to_string(),
    };

    (Status::Ok, Json(response))
}

#[post(
    "/client/change-host-status-id",
    format = "application/json",
    data = "<request>"
)]
pub async fn change_host_status_id(
    request: Json<ChangeHostStatusRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ChangeHostStatusReply>) {
    let core = x.user();

    let response = core.change_host_status(request.into_inner()).await;

    (Status::Ok, Json(response))
}

#[post(
    "/client/change-host-status-name",
    format = "application/json",
    data = "<request>"
)]
pub async fn change_host_status_name(
    request: Json<ChangeHostStatusRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ChangeHostStatusReply>) {
    let core = x.user();
    let response = core.change_host_status(request.into_inner()).await;

    (Status::Ok, Json(response))
}

#[post(
    "/client/try-auth-participant",
    format = "application/json",
    data = "<request>"
)]
pub async fn try_auth_at_participant(
    request: Json<TryAuthAtParticipantRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<TryAuthAtPartipantReply>) {
    let core = x.user();

    let response = core.try_auth_at_participant(request.into_inner()).await;

    (Status::Ok, Json(response))
}

#[post("/client/token", format = "application/json", data = "<request>")]
pub async fn auth_for_token(
    request: Json<AuthRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<TokenReply>) {
    let core = x.user();

    let response = core.auth_for_token(request.into_inner()).await;

    trace!("{response:?}");

    (Status::Ok, Json(response))
}

#[post("/client/settings", format = "application/json", data = "<request>")]
pub async fn get_settings(
    request: Json<GetSettingsRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetSettingsReply>) {
    let core = x.user();

    let response = core.get_settings(request.into_inner()).await;

    (Status::Ok, Json(response))
}

#[post(
    "/client/token-revoke",
    format = "application/json",
    data = "<request>"
)]
pub async fn revoke_token(
    request: Json<AuthRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<RevokeReply>) {
    let core = x.user();

    let response = core.revoke_token(request.into_inner()).await;

    (Status::Ok, Json(response))
}
