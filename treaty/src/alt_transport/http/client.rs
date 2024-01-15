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
        AuthRequestBasic, AuthRequestWebToken, ChangeHostStatusReply, ChangeHostStatusRequest,
        GetBackingDatabaseConfigReply, GetSettingsReply, RevokeReply, TestReply, TestRequest,
        TokenReply, TryAuthAtParticipantRequest, TryAuthAtPartipantReply,
    },
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

#[post("/client/db-type", format = "application/json")]
pub async fn db_type(x: &State<HttpServer>) -> (Status, Json<GetBackingDatabaseConfigReply>) {
    let core = x.user().await;
    let response = core.get_backing_database_config().await;

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
    let core = x.user().await;

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
    let core = x.user().await;
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
    let core = x.user().await;

    let response = core.try_auth_at_participant(request.into_inner()).await;

    (Status::Ok, Json(response))
}

#[post("/client/token", format = "application/json", data = "<request>")]
pub async fn auth_for_token(
    request: Json<AuthRequestBasic>,
    x: &State<HttpServer>,
) -> (Status, Json<TokenReply>) {
    let core = x.user().await;
    let response = core.auth_for_token(request.into_inner()).await;

    trace!("{response:?}");

    (Status::Ok, Json(response))
}

#[post("/client/settings", format = "application/json")]
pub async fn get_settings(x: &State<HttpServer>) -> (Status, Json<GetSettingsReply>) {
    let core = x.user().await;
    let response = core.get_settings().await;

    (Status::Ok, Json(response))
}

#[post(
    "/client/token-revoke",
    format = "application/json",
    data = "<request>"
)]
pub async fn revoke_token(
    request: Json<AuthRequestWebToken>,
    x: &State<HttpServer>,
) -> (Status, Json<RevokeReply>) {
    let core = x.user().await;
    let response = core.revoke_token(request.into_inner()).await;

    (Status::Ok, Json(response))
}
