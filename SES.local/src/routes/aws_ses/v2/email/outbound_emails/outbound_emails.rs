use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde_json::json;
use ses_serde::operations::send_email::{SendEmailInput, SendEmailOutput};

use crate::email_store::ReceivedEmail;

async fn outbound_emails(
    State(crate::AppState { email_store, .. }): State<crate::AppState>,
    Json(email): Json<SendEmailInput>,
) -> impl IntoResponse {
    let rec_email = ReceivedEmail::new(email);
    let emsc = email_store.clone();
    let mut ems = emsc.write().await;
    match ems.push(rec_email).await {
        Ok(saved) => Json(json!(SendEmailOutput {
            message_id: Some(saved.message_id),
        }))
        .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub fn create() -> crate::AppStateRouter {
    Router::new().route("/outbound-emails", post(outbound_emails))
}
