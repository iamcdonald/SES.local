use axum::Router;

mod outbound_emails;

pub fn get() -> Router {
    Router::new().merge(outbound_emails::handler())
}
