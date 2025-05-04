use axum::Router;

mod aws_ses;

pub fn get() -> Router {
    Router::new().merge(aws_ses::get())
}
