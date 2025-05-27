use super::static_content;
use crate::event_store::Event;
use maud::{html, Markup};

pub fn build(event: &Event) -> Markup {
    let dl = vec![
        ("name", event.get_name()),
        ("timestamp", event.timestamp.clone()),
    ];
    html! {
        div class="
            flex
            items-start
            m-1
            rounded-md
            shadow-sm
            items-center
            mb-2
            snap-center
            is-enabled:hover:shadow-xl
            is-enabled:hover:bg-indigo-50
            is-disabled:shadow-lg
            is-disabled:bg-indigo-100" {
            a hx-get=(format!("/events/{}", event.id))
                hx-push-url="true"
                hx-target=(format!("#{}", static_content::EVENT_DETAIL_ID))
                hx-swap="innerHTML" {
                div class="p-3 shrink-0 w-full" {
                    dl class="grid grid-cols-2 gap-2" {
                        @for (dt, dd) in dl {
                            dt { (dt) }
                            dd { (dd) }
                        }
                    }
                }
            }
            button class="h-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" hx-delete=(format!("/events/{}", event.id)) hx-target="closest div" {
                "delete"
            }
        }
    }
}
