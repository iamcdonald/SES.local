use maud::{html, Markup};

use crate::event_store::Event;

pub fn build(event: &Event) -> Markup {
    let c = vec![(
        "id",
        html! {
            (event.id)
        },
    )];
    html! {
        dl {
            @for (dt, dd) in c {
                div class="flex" {
                    dt class="text-gray-500 flex-[80px] grow-0 shrink-0" { (dt) ":" }
                    dd class="flex-grow" { (dd) }
                }
            }
        }
    }
}
