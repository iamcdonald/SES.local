use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

use crate::AppEmailStore;

pub async fn emails_json(email_store: &AppEmailStore) -> impl IntoResponse {
    Json(json!(email_store.read().await.get_all()))
}

pub async fn email_json(email_store: &AppEmailStore, id: &String) -> impl IntoResponse {
    if let Some(found) = email_store.read().await.get_by_message_id(id) {
        Json(json!(found)).into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}
