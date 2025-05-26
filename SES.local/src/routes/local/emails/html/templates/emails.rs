use super::email_row;
use super::static_content;
use super::tag;
use crate::event_store::send_email::{EmailTag, SendEmail};
use crate::page_template;
use maud::{html, Markup};

pub fn build(emails: &Vec<&SendEmail>, email: Option<Markup>) -> Markup {
    page_template::build(html! {
        script {
            "document.addEventListener('htmx:pushedIntoHistory', function ({ detail: { path }}) {
                let email_rows = document.querySelectorAll('#emails [hx-get][disabled]');
                for (let row of email_rows) {
                    row.removeAttribute('disabled');
                }
                let active_row = document.querySelector(`#emails [hx-get='${path}']`);
                if (active_row) {
                    active_row.setAttribute('disabled', true);
                }
            })"
        }
        div class="flex flex-col h-screen" {
            div class="flex flex-row flex-grow" {
                div class="shrink-0 py-2 flex-[360px] grow-0 max-h-screen min-h-screen flex flex-col overflow-hidden" {
                    div class="border-b-1 border-stone-100 pb-2 shadow-md" {
                        (email_type_key())
                    }
                    div id="emails" class="overflow-auto flex-grow snap-y snap-mandatory snap-center" {
                        div hx-ext="sse" sse-connect="/emails" {
                            div sse-swap="email" hx-swap="afterbegin" {}
                        }
                        @for em in emails {
                            (email_row::build(&em))
                        }
                    }
                }
                div id=(static_content::EMAIL_DETAIL_ID) class="p-3 border-l-1 border-stone-100 grow shrink max-h-screen min-h-screen overflow-auto" {
                    (email.unwrap_or(html! { "email" }))
                }
            }
        }
    })
}

pub fn email_type_key() -> Markup {
    let tags = vec![
        (EmailTag::Simple, "simple"),
        (EmailTag::Template, "template"),
        (EmailTag::Raw, "raw"),
    ];
    html! {
        div class="flex justify-between mb-2 mx-4" {
            @for (tag, text) in tags {
                div class="flex flex-col items-center" {
                    (tag::build(&tag))
                    span { (text) }
                }
            }
        }
    }
}
