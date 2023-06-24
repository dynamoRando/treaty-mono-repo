use crate::treaty_proto::{
    AcceptPendingActionReply, AcceptPendingActionRequest, GetPendingActionsReply,
    GetPendingActionsRequest,
};
use crate::user_service_handler::user_service_handler_actions::UserServiceHandlerActions;
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
    let core = x.user();
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
    let core = x.user();
    let result = core
        .get_pending_actions_at_participant(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}
