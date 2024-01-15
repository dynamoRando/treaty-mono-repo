use crate::treaty_proto::{
    AcceptPendingContractReply, AcceptPendingContractRequest, ViewPendingContractsReply,
};
use rocket::{http::Status, post, serde::json::Json, State};

use crate::alt_transport::http::HttpServer;

#[post("/client/contract/review", format = "application/json")]
pub async fn review_pending_contracts(
    x: &State<HttpServer>,
) -> (Status, Json<ViewPendingContractsReply>) {
    let core = x.user().await;
    let result = core.review_pending_contracts().await;

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
    let core = x.user().await;
    let result = core.accept_pending_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}
