use maud::{html, Markup};
use ses_serde::types::Destination;

use super::static_content;
use crate::email_store::{Email, ReceivedEmail};

pub fn build(email: &ReceivedEmail) -> Markup {
    if let Some(content) = match email.email {
        Email::Simple(_) => simple(email),
        Email::Raw(_) => raw(email),
        _ => None,
    } {
        return html! {
            a hx-get=(format!("/emails/{}", email.message_id)) hx-push-url="true" hx-target=(format!("#{}", static_content::EMAIL_DETAIL_ID)) hx-swap="innerHTML" class="flex flex-row" {
                div class="p-3" { (content) }
                div class="p-3 flex-shrink" {
                    (tag(&email.email))
                }
            }
        };
    }
    html! {}
}

fn simple(email: &ReceivedEmail) -> Option<Markup> {
    let subject = email.get_subject()?;
    Some(html! {
        dl class="grid grid-cols-2 gap-2" {
            dt { "to" }
            dd { (destination(email.get_destination())?) }
            dt { "subject" }
            dd { (subject) }
        }
    })
}

fn raw(email: &ReceivedEmail) -> Option<Markup> {
    Some(html! {
        div {
            (destination(email.get_destination())?)
        }
        div { "raw" }
    })
}

fn tag(tag: &Email) -> Markup {
    html! {
        div class="flex items-center justify-center rounded-full w-[50px] h-[50px] bg-red-50" { (tag.to_string().remove(0).to_uppercase()) }
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
