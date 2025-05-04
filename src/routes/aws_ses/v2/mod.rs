use axum::Router;

mod email;

pub fn get() -> Router {
    Router::new().route("/email", email::get())
}
