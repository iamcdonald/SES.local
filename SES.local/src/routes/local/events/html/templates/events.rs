use super::event_row;
use super::static_content;
use crate::{event_store::Event, page_template};
use maud::{html, Markup};

pub fn build(events: &Vec<Event>, event: Option<Markup>) -> Markup {
    page_template::build(html! {
        script {
            (format!("document.addEventListener('htmx:pushedIntoHistory', function ({{ detail: {{ path }}}}) {{
                let event_rows = document.querySelectorAll('#{0} [hx-get][disabled]');
                for (let row of event_rows) {{
                    row.removeAttribute('disabled');
                }}
                let active_row = document.querySelector(`#{0} [hx-get='${{path}}']`);
                if (active_row) {{
                    active_row.setAttribute('disabled', true);
                }}
            }})", static_content::EVENTS_DETAIL_ID))
        }
        div class="flex flex-col h-screen" {
            div class="flex flex-row flex-grow" {
                div class="shrink-0 flex-[360px] grow-0 max-h-screen min-h-screen flex flex-col overflow-hidden" {
                    div hx-ext="sse" sse-connect="/events" sse-swap="event" hx-swap="afterbegin" hx-target=(format!("#{}", static_content::EVENTS_DETAIL_ID)) {
                    }
                    div id=(static_content::EVENTS_DETAIL_ID) class="overflow-auto flex-grow snap-y snap-mandatory inset-shadow-sm" {
                        @for ev in events {
                            (event_row::build(&ev))
                        }
                    }
                    div class="p-4 flex justify-end border-t-1 border-stone-100 pb-2" {
                        button
                          class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                          hx-delete="/events"
                          hx-target=(format!("#{}", static_content::EVENTS_DETAIL_ID)) {
                            "Clear All Events"
                        }
                    }
                }
                div id=(static_content::EVENT_DETAIL_ID) class="p-3 border-l-1 border-stone-100 grow shrink max-h-screen min-h-screen overflow-auto" {
                    (event.unwrap_or(html! { "event" }))
                }
            }
        }
    })
}
