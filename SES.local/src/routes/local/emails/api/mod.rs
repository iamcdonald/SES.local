use crate::AppEventStore;
use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

pub async fn emails_json(event_store: &AppEventStore) -> impl IntoResponse {
    Json(json!(event_store.read().await.get_all_emails()))
}

pub async fn email_json(event_store: &AppEventStore, id: &String) -> impl IntoResponse {
    if let Some(found) = event_store.read().await.get_email_by_message_id(id) {
        Json(json!(found)).into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}
