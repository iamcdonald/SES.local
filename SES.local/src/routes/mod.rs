mod aws_ses;
mod local;

pub fn create() -> crate::AppStateRouter {
    aws_ses::create().merge(local::create())
}
