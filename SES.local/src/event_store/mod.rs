mod event;
mod event_store;
pub use event::{send_email, Event, EventContent};
pub use event_store::EventStore;
