use super::email_row;
use super::static_content;
use crate::{email_store::ReceivedEmail, page_template};
use maud::{html, Markup};

pub fn build(emails: &Vec<ReceivedEmail>, email: Option<Markup>) -> Markup {
    page_template::build(html! {
        div class="flex flex-col h-screen overflow-hidden" {
            div class="text-red-400" { "Emails!" }
            div class="flex flex-row flex-grow" {
                div class="h-full w-[400px]" {
                    div hx-ext="sse" sse-connect="/emails" {
                        div sse-swap="email" hx-swap="afterbegin" {}
                    }
                    @for em in emails {
                        (email_row::build(&em))
                    }
                }
                div id=(static_content::EMAIL_DETAIL_ID) class="bg-red-100 flex-grow h-full" {
                    (email.unwrap_or(html! { "email" }))
                }
            }
        }
    })
}
