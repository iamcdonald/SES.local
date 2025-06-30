mod emails;
mod events;
use axum::{response::Redirect, routing::get, Router};

pub fn create() -> crate::AppStateRouter {
    Router::new()
        .route("/", get(|| async { Redirect::permanent("/emails") }))
        .merge(emails::create())
        .merge(events::create())
}
