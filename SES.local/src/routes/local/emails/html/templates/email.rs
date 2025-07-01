use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
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
            "to",
            html! {
                (
                    content.from.unwrap_or(&String::from(""))
                )
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
            "subject",
            html! {
                (
                    content.subject.unwrap_or(&String::from(""))
                )
            },
        ),
    ];
    let email = match content.body {
        Some(b) => match b.content {
            Some(c) => {
                if b.is_html {
                    let uri = format!(
                        "/emails/{}/content",
                        email.response.message_id.clone().unwrap_or("".to_string())
                    );
                    // let src = format!("data:text/html;base64,{}", STANDARD_NO_PAD.encode(c));
                    html! { iframe class="grow" src=(uri) {} }
                } else {
                    html! { (c) }
                }
            }
            None => html! { "" },
        },
        None => html! {
            ""
        },
    };
    html! {
        dl class="flex flex-col min-h-full" {
            div class="p-4 pb-0" {
                @for (dt, dd) in c {
                    div class="flex" {
                        dt class="text-gray-500 flex-[80px] grow-0 shrink-0" { (dt) ":" }
                        dd class="flex-grow wrap-anywhere" { (dd) }
                    }
                }
            }
            div class="flex grow" {
                (email)
            }

        }
    }
}
