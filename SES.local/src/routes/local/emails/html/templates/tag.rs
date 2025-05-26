use maud::{html, Markup};

use crate::event_store::send_email::EmailTag;

pub fn build(email_tag: &EmailTag) -> Markup {
    let color = match email_tag {
        EmailTag::Raw => "bg-teal-200",
        EmailTag::Template => "bg-indigo-200",
        EmailTag::Simple => "bg-lime-200",
        EmailTag::Unknown => "bg-zinc-100",
    };
    html! {
        span class=(format!("rounded-full w-[40px] h-[40px] flex items-center justify-center {}", color)) {
              (email_tag.to_string().remove(0).to_uppercase())
        }
    }
}
