use crate::alt_transport::http::HttpServer;
use crate::treaty_proto::{
    ChangeDeletesFromHostBehaviorReply, ChangeDeletesFromHostBehaviorRequest,
    ChangeDeletesToHostBehaviorReply, ChangeDeletesToHostBehaviorRequest,
    ChangeUpdatesFromHostBehaviorRequest, ChangeUpdatesToHostBehaviorReply,
    ChangeUpdatesToHostBehaviorRequest, ChangesUpdatesFromHostBehaviorReply,
    GetDeletesFromHostBehaviorReply, GetDeletesFromHostBehaviorRequest,
    GetDeletesToHostBehaviorReply, GetDeletesToHostBehaviorRequest,
    GetUpdatesFromHostBehaviorReply, GetUpdatesFromHostBehaviorRequest,
    GetUpdatesToHostBehaviorReply, GetUpdatesToHostBehaviorRequest,
};
use rocket::{http::Status, post, serde::json::Json, State};

#[post(
    "/client/databases/behavior/change-deletes-to-host",
    format = "application/json",
    data = "<request>"
)]
pub async fn change_deletes_to_host_behavior(
    request: Json<ChangeDeletesToHostBehaviorRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ChangeDeletesToHostBehaviorReply>) {
    let core = x.user().await;
    let result = core
        .change_deletes_to_host_behavior(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/behavior/change-updates-to-host",
    format = "application/json",
    data = "<request>"
)]
pub async fn change_updates_to_host_behavior(
    request: Json<ChangeUpdatesToHostBehaviorRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ChangeUpdatesToHostBehaviorReply>) {
    let core = x.user().await;
    let result = core
        .change_updates_to_host_behavior(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/behavior/change-deletes-from-host",
    format = "application/json",
    data = "<request>"
)]
pub async fn change_deletes_from_host_behavior(
    request: Json<ChangeDeletesFromHostBehaviorRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ChangeDeletesFromHostBehaviorReply>) {
    let core = x.user().await;
    let result = core
        .change_deletes_from_host_behavior(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/behavior/change-updates-from-host",
    format = "application/json",
    data = "<request>"
)]
pub async fn change_updates_from_host_behavior(
    request: Json<ChangeUpdatesFromHostBehaviorRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<ChangesUpdatesFromHostBehaviorReply>) {
    let core = x.user().await;
    let result = core
        .change_updates_from_host_behavior(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/behavior/get-updates-from-host",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_updates_from_host_behavior(
    request: Json<GetUpdatesFromHostBehaviorRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetUpdatesFromHostBehaviorReply>) {
    let core = x.user().await;
    let result = core
        .get_updates_from_host_behavior(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/behavior/get-deletes-from-host",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_deletes_from_host_behavior(
    request: Json<GetDeletesFromHostBehaviorRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetDeletesFromHostBehaviorReply>) {
    let core = x.user().await;
    let result = core
        .get_deletes_from_host_behavior(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/behavior/get-updates-to-host",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_updates_to_host_behavior(
    request: Json<GetUpdatesToHostBehaviorRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetUpdatesToHostBehaviorReply>) {
    let core = x.user().await;
    let result = core
        .get_updates_to_host_behavior(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/behavior/get-deletes-to-host",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_deletes_to_host_behavior(
    request: Json<GetDeletesToHostBehaviorRequest>,
    x: &State<HttpServer>,
) -> (Status, Json<GetDeletesToHostBehaviorReply>) {
    let core = x.user().await;
    let result = core
        .get_deletes_to_host_behavior(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}
