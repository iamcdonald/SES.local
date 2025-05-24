use maud::{html, Markup};

use crate::email_store::ReceivedEmail;

pub fn build(email: &ReceivedEmail) -> Markup {
    html! {
        div {
            (email.message_id)
        }
    }
}
