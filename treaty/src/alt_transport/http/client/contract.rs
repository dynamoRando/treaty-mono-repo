use crate::treaty_proto::{
    AcceptPendingContractReply, AcceptPendingContractRequest, ViewPendingContractsReply,
    ViewPendingContractsRequest,
};
use rocket::{http::Status, post, serde::json::Json, State};

use crate::alt_transport::http::HttpServer;
use crate::user_service_handler::user_service_handler_actions::UserServiceHandlerActions;

#[post(
    "/client/contract/review",
    format = "application/json",
    data = "<request>"
)]
pub async fn review_pending_contracts(
    request: Json<ViewPendingContractsRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ViewPendingContractsReply>) {
    let core = x.user();
    let result = core.review_pending_contracts(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/contract/accept",
    format = "application/json",
    data = "<request>"
)]
pub async fn accept_pending_contract(
    request: Json<AcceptPendingContractRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<AcceptPendingContractReply>) {
    let core = x.user();
    let result = core.accept_pending_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}
