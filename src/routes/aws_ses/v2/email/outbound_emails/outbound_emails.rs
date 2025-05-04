// use aws_sdk_sesv2::operation::send_email::{SendEmailInput, SendEmailOutput};
use axum::{response::IntoResponse, routing::put, Json, Router};
use serde_json::json;
use tracing::debug;

use super::request::Request;

#[axum::debug_handler]
async fn outbound_emails(Json(req): Json<Request>) -> impl IntoResponse {
    debug!("{:?}", req);
    Json(json!({ "token": "hih" }))
}

pub fn handler() -> Router {
    Router::new().route("/outbound-emails", put(outbound_emails))
}
