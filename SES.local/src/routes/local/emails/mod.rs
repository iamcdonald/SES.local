mod api;
mod html;

use axum::{
    body::Body,
    extract::{Path, State},
    http::{header::ACCEPT, Request, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_htmx::HxRequest;

async fn emails(
    State(crate::AppState { email_store, .. }): State<crate::AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    if let Some(accept) = req.headers().get(ACCEPT) {
        return match accept.to_str().unwrap() {
            "text/event-stream" => html::emails_sse(&email_store).await.into_response(),
            "application/json" => api::emails_json(&email_store).await.into_response(),
            _ => html::emails_page(&email_store, None).await.into_response(),
        };
    }
    (StatusCode::NOT_FOUND).into_response()
}

async fn email(
    State(crate::AppState { email_store, .. }): State<crate::AppState>,
    Path(id): Path<String>,
    HxRequest(hx_request): HxRequest,
    req: Request<Body>,
) -> impl IntoResponse {
    if let Some(accept) = req.headers().get(ACCEPT) {
        return match accept.to_str().unwrap() {
            "application/json" => api::email_json(&email_store, &id).await.into_response(),
            _ => html::email_page(&email_store, &id, hx_request)
                .await
                .into_response(),
        };
    }
    (StatusCode::NOT_FOUND).into_response()
}

pub fn create() -> crate::AppStateRouter {
    Router::new().nest(
        "/emails",
        Router::new()
            .route("/", get(emails))
            .route("/{id}", get(email)),
    )
}
