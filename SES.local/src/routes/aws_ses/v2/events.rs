use crate::event_store::Event;
use axum::{
    body::Bytes,
    extract::{OriginalUri, State},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use reqwest::StatusCode;

async fn handler(
    State(crate::AppState { event_store, .. }): State<crate::AppState>,
    OriginalUri(original_uri): OriginalUri,
    body: Bytes,
) -> impl IntoResponse {
    if let Some(ev) = Event::from_body(body, &original_uri.to_string()) {
        tracing::debug!("{:?}", ev);
        match event_store.write().await.push(ev).await.ok() {
            Some(ev) => Json(ev.get_json_response()).into_response(),
            None => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}

pub fn create() -> crate::AppStateRouter {
    Router::new().route("/{*wildcard}", post(handler))
}
