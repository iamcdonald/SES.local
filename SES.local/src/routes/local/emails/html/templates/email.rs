use maud::{html, Markup};

use crate::event_store::send_email::SendEmail;

pub fn build(email: &SendEmail) -> Markup {
    let content = email.request.get_email_content();
    let c = vec![
        (
            "id",
            html! {
                (email.response.message_id.clone().unwrap_or("".to_string()))
            },
        ),
        (
            "from",
            html! {
                (
                    content.to.map(
                        |x| x.to_addresses.as_ref().map(|a| a.join(", ")).unwrap_or("".to_string())
                    ).unwrap_or("".to_string())
                )
            },
        ),
        (
            "to",
            html! {
                (
                    content.from.unwrap_or(&String::from(""))
                )
            },
        ),
        (
            "subject",
            html! {
                (
                    content.subject.unwrap_or(&String::from(""))
                )
            },
        ),
        (
            "content",
            html! {
                (
                    content.body.and_then(|x| x.content).unwrap_or(&String::from(""))
                )
            },
        ),
    ];
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
