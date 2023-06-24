use crate::{
    alt_transport::http::HttpServer,
    data_service_handler::data_service_handler_actions::DataServiceHandlerActions,
    treaty_proto::{
        ParticipantAcceptsContractRequest, ParticipantAcceptsContractResult, SaveContractRequest,
        SaveContractResult,
    },
};
use rocket::{http::Status, post, serde::json::Json, State};

#[post("/data/contract/save", format = "application/json", data = "<request>")]
pub async fn save_contract(
    request: Json<SaveContractRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<SaveContractResult>) {
    let core = x.data();
    let result = core.save_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/data/contract/accepted-by-participant",
    format = "application/json",
    data = "<request>"
)]
pub async fn participant_accepts_contract(
    request: Json<ParticipantAcceptsContractRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ParticipantAcceptsContractResult>) {
    let core = x.data();
    let result = core.accept_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}
