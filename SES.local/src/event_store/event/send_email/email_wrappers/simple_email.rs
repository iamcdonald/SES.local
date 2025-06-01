use super::{Body, EmailWrapper};
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
        email.destination.as_ref()
    }
    fn get_from(email: &SendEmailInput) -> Option<&String> {
        email.from_email_address.as_ref()
    }
    fn get_body(email: &SendEmailInput) -> Option<Body> {
        email
            .content
            .as_ref()?
            .simple
            .as_ref()?
            .body
            .as_ref()
            .map(|x| match &x.html {
                Some(html) => Body {
                    content: Some(&html.data),
                    is_html: true,
                },
                None => Body {
                    content: x.text.as_ref().map(|x| &x.data),
                    is_html: false,
                },
            })
    }
}
