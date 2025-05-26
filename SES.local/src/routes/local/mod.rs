mod emails;
mod events;

pub fn create() -> crate::AppStateRouter {
    emails::create().merge(events::create())
}
