use maud::{html, Markup};

use crate::event_store::Event;

pub fn build(event: &Event) -> Markup {
    html! {
        pre class="text-xs" {
            (serde_json::to_string_pretty(event).unwrap())
        }
    }
}
