use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::AppEmailStore;

pub async fn emails_json(email_store: &AppEmailStore) -> impl IntoResponse {
    Json(json!(email_store.read().await.get_all()))
}

pub async fn email_json(email_store: &AppEmailStore, id: &String) -> impl IntoResponse {
    Json(json!(email_store.read().await.get_by_message_id(id)))
}
