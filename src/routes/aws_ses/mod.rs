use axum::Router;

mod v2;

pub fn get() -> Router {
    Router::new().route("/v2", Router::new().merge(v2::get()))
}
