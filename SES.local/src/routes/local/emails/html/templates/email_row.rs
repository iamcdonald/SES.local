use maud::{html, Markup};
use ses_serde::types::Destination;

use crate::event_store::send_email::SendEmail;

use super::static_content;
use super::tag;

pub fn build(email: &SendEmail) -> Markup {
    let summary = email.request.get_summary();
    let info = vec![
        destination(summary.to).unwrap_or(html! { "unknown" }),
        html! { (summary.subject.unwrap_or(&String::from("unknown"))) },
    ];
    html! {
        a hx-get=(format!("/emails/{}", email.response.message_id.clone().unwrap_or("".to_string())))
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
                    (tag::build(&email.request.get_tag()))
                }
                div class="p-3 flex flex-col" {
                    @for item in info {
                        (item)
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
