use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

use crate::AppEventStore;

pub async fn events_json(event_store: &AppEventStore) -> impl IntoResponse {
    Json(json!(event_store.read().await.get_all()))
}

pub async fn event_json(event_store: &AppEventStore, id: &String) -> impl IntoResponse {
    if let Some(found) = event_store.read().await.get_by_event_id(id) {
        Json(json!(found)).into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}
