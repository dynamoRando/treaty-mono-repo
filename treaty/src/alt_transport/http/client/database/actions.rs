use crate::treaty_proto::{
    AcceptPendingActionReply, AcceptPendingActionRequest, GetPendingActionsReply,
    GetPendingActionsRequest,
};
use rocket::{http::Status, post, serde::json::Json, State};

use crate::alt_transport::http::HttpServer;

#[post(
    "/client/databases/actions/accept-pending",
    format = "application/json",
    data = "<request>"
)]
pub async fn accept_pending_action_at_participant(
    request: Json<AcceptPendingActionRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<AcceptPendingActionReply>) {
    let core = x.user().await;
    let result = core
        .accept_pending_action_at_participant(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/actions/get-pending",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_pending_actions_at_participant(
    request: Json<GetPendingActionsRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetPendingActionsReply>) {
    let core = x.user().await;
    let result = core
        .get_pending_actions_at_participant(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}
