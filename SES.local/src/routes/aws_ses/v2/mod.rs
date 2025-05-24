use axum::Router;

mod email;

pub fn create() -> crate::AppStateRouter {
    Router::new().nest("/v2", email::create())
}
