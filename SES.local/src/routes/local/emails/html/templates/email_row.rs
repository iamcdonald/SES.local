use maud::{html, Markup};
use ses_serde::types::Destination;

use super::static_content;
use super::tag;
use crate::email_store::ReceivedEmail;

pub fn build(email: &ReceivedEmail) -> Markup {
    let summary = email.get_summary();
    let dl = vec![
        ("to", destination(summary.to).unwrap_or(html! { "unknown" })),
        (
            "subject",
            html! { (summary.subject.unwrap_or(&String::from("unknown"))) },
        ),
    ];
    html! {
        a hx-get=(format!("/emails/{}", email.message_id))
            hx-push-url="true"
            hx-target=(format!("#{}", static_content::EMAIL_DETAIL_ID))
            hx-swap="innerHTML"
            class="
            m-1
            rounded-md
            shadow-sm
            flex
            flex-row
            items-center
            mb-2
            snap-center
            is-enabled:hover:shadow-xl
            is-enabled:hover:bg-indigo-50
            is-disabled:shadow-lg
            is-disabled:bg-indigo-100" {
                div class="p-3" {
                    (tag::build(&email.get_tag()))
                }
                div class="p-3 shrink-0" {
                    dl class="grid grid-cols-2 gap-2" {
                        @for (dt, dd) in dl {
                            dt { (dt) }
                            dd { (dd) }
                        }
                    }
                }
        }
    }
}

fn destination(dest: Option<&Destination>) -> Option<Markup> {
    match dest {
        Some(d) => Some(html! {
            div {
                @for to in d.clone().to_addresses? {
                    span { (to) }
                }
            }
        }),
        _ => None,
    }
}
