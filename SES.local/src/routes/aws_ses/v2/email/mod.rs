use axum::Router;

mod outbound_emails;

pub fn create() -> crate::AppStateRouter {
    Router::new().nest("/email", outbound_emails::create())
}
