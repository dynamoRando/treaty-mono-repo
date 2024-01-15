use crate::alt_transport::http::HttpServer;
use crate::treaty_proto::{
    AddParticipantReply, AddParticipantRequest, GetParticipantsReply, GetParticipantsRequest,
    SendParticipantContractReply, SendParticipantContractRequest,
};
use rocket::State;
use rocket::{http::Status, post, serde::json::Json};
use tracing::trace;

#[post(
    "/client/databases/participant/add",
    format = "application/json",
    data = "<request>"
)]
pub async fn add_participant(
    request: Json<AddParticipantRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<AddParticipantReply>) {
    let core = x.user().await;
    let result = core.add_participant(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/participant/send-contract",
    format = "application/json",
    data = "<request>"
)]
pub async fn send_contract_to_participant(
    request: Json<SendParticipantContractRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<SendParticipantContractReply>) {
    trace!("{request:?}");

    let core = x.user().await;

    let result = core.send_participant_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/participant/get",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_participants(
    request: Json<GetParticipantsRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetParticipantsReply>) {
    let core = x.user().await;
    let result = core.get_participants(request.into_inner()).await;

    (Status::Ok, Json(result))
}
