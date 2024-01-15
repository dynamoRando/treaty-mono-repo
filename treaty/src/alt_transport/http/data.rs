pub mod io;
use rocket::{get, http::Status, post, serde::json::Json, State};

use crate::{
    alt_transport::http::HttpServer,
    defaults,
    treaty_proto::{TestReply, TestRequest, TryAuthResult},
};

#[get("/data/status")]
pub async fn status() -> &'static str {
    "Status From Rocket"
}

#[post("/data/version", format = "application/json", data = "<request>")]
pub fn version(request: Json<TestRequest>) -> (Status, Json<TestReply>) {
    let response = TestReply {
        reply_time_utc: "".to_string(),
        reply_echo_message: request.request_echo_message.clone(),
        treaty_version: defaults::VERSION.to_string(),
    };

    (Status::Ok, Json(response))
}

#[post("/data/try-auth", format = "application/json")]
pub async fn try_auth(x: &State<HttpServer>) -> (Status, Json<TryAuthResult>) {
    let core = x.data().await;
    let result = core.try_auth().await;

    (Status::Ok, Json(result))
}
