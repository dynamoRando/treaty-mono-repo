use crate::{
    alt_transport::http::HttpServer,
    treaty_proto::{
        AuthRequestBasic, AuthRequestWebToken, ParticipantAcceptsContractRequest,
        ParticipantAcceptsContractResult, SaveContractRequest, SaveContractResult, TokenReply,
        TryAuthResult,
    },
};
use rocket::{http::Status, post, serde::json::Json, State};

#[post("/info/contract/save", format = "application/json", data = "<request>")]
pub async fn save_contract(
    request: Json<SaveContractRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<SaveContractResult>) {
    let core = x.info().await;
    let result = core.save_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/info/contract/accepted-by-participant",
    format = "application/json",
    data = "<request>"
)]
pub async fn participant_accepts_contract(
    request: Json<ParticipantAcceptsContractRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ParticipantAcceptsContractResult>) {
    let core = x.info().await;
    let result = core.accept_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post("/info/try-auth", format = "application/json", data = "<request>")]
pub async fn info_try_auth_web_token(
    request: Json<AuthRequestWebToken>,
    x: &State<HttpServer>,
) -> (Status, Json<TryAuthResult>) {
    let core = x.info().await;
    let result = core.try_auth_web_token(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post("/info/token", format = "application/json", data = "<request>")]
pub async fn auth_for_token(
    request: Json<AuthRequestBasic>,
    x: &State<HttpServer>,
) -> (Status, Json<TokenReply>) {
    let core = x.info().await;
    let result = core.auth_for_token(request.into_inner()).await;

    (Status::Ok, Json(result))
}
