use serde::{Deserialize, Serialize};
use ses_serde::{operations::send_email::SendEmailInput, types::Destination};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReceivedEmail {
    pub email: Email,
    pub message_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display)]
pub enum Email {
    Simple(SendEmailInput),
    Template(SendEmailInput),
    Raw(SendEmailInput),
    Unknown(SendEmailInput),
}

impl ReceivedEmail {
    pub fn new(email: SendEmailInput) -> Self {
        let email = if let Some(content) = &email.content {
            if content.simple.is_some() {
                Email::Simple(email)
            } else if content.template.is_some() {
                Email::Template(email)
            } else {
                Email::Raw(email)
            }
        } else {
            Email::Unknown(email)
        };
        ReceivedEmail {
            email,
            message_id: Uuid::new_v4().to_string(),
        }
    }
    pub fn get_subject(&self) -> Option<&String> {
        match &self.email {
            Email::Simple(e) => e
                .content
                .as_ref()?
                .simple
                .as_ref()?
                .subject
                .as_ref()
                .map(|x| &x.data),
            _ => None,
        }
    }

    fn get_email(&self) -> &SendEmailInput {
        match &self.email {
            Email::Simple(e) => e,
            Email::Template(e) => e,
            Email::Raw(e) => e,
            Email::Unknown(e) => e,
        }
    }

    pub fn get_destination(&self) -> Option<&Destination> {
        self.get_email().destination.as_ref().map(|x| x)
    }
}
