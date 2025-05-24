mod emails;

pub fn create() -> crate::AppStateRouter {
    emails::create()
}
