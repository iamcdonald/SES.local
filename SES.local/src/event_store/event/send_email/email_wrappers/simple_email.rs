use super::EmailWrapper;
use ses_serde::{operations::send_email::SendEmailInput, types::Destination};

#[derive(Debug)]
pub struct SimpleEmail {}

impl EmailWrapper for SimpleEmail {
    fn get_subject(email: &SendEmailInput) -> Option<&String> {
        email
            .content
            .as_ref()?
            .simple
            .as_ref()?
            .subject
            .as_ref()
            .map(|x| &x.data)
    }
    fn get_to(email: &SendEmailInput) -> Option<&Destination> {
        email.destination.as_ref().map(|x| x)
    }
    fn get_from(email: &SendEmailInput) -> Option<&String> {
        email.from_email_address.as_ref().map(|x| x)
    }
    fn get_body(email: &SendEmailInput) -> Option<&String> {
        email
            .content
            .as_ref()?
            .simple
            .as_ref()?
            .body
            .as_ref()
            .and_then(|x| {
                if let Some(_) = &x.text {
                    x.text.as_ref().map(|x| &x.data)
                } else if let Some(_) = &x.html {
                    x.html.as_ref().map(|x| &x.data)
                } else {
                    None
                }
            })
    }
}
