use axum::Router;
mod events;

pub fn create() -> crate::AppStateRouter {
    Router::new().merge(events::create())
}
