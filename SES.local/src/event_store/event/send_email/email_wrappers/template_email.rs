use super::{Body, EmailWrapper};
use ses_serde::{operations::send_email::SendEmailInput, types::Destination};

#[derive(Debug)]
pub struct TemplateEmail {}

impl EmailWrapper for TemplateEmail {
    fn get_subject(email: &SendEmailInput) -> Option<&String> {
        email
            .content
            .as_ref()?
            .template
            .as_ref()?
            .template_content
            .as_ref()?
            .subject
            .as_ref()
            .map(|x| x)
    }
    fn get_to(email: &SendEmailInput) -> Option<&Destination> {
        email.destination.as_ref().map(|x| x)
    }
    fn get_from(email: &SendEmailInput) -> Option<&String> {
        email.from_email_address.as_ref().map(|x| x)
    }
    fn get_body(email: &SendEmailInput) -> Option<Body> {
        email
            .content
            .as_ref()?
            .template
            .as_ref()?
            .template_content
            .as_ref()
            .map(|x| match &x.html {
                Some(html) => Body {
                    content: Some(html),
                    is_html: true,
                },
                None => Body {
                    content: x.text.as_ref(),
                    is_html: false,
                },
            })
        // .and_then(|x| {
        //     if let Some(_) = &x.text {
        //         x.text.as_ref()
        //     } else if let Some(_) = &x.html {
        //         x.html.as_ref()
        //     } else {
        //         None
        //     }
        // })
    }
}
