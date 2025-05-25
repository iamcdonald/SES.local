use email_wrappers::{EmailWrapper, RawEmail, SimpleEmail, TemplateEmail, UnknownEmail};
use serde::{Deserialize, Serialize};
use ses_serde::{operations::send_email::SendEmailInput, types::Destination};
use uuid::Uuid;

mod email_wrappers;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReceivedEmail {
    pub email: Email,
    pub message_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display)]
pub enum EmailTag {
    Simple,
    Template,
    Raw,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display)]
pub enum Email {
    Simple(SendEmailInput),
    Template(SendEmailInput),
    Raw(SendEmailInput),
    Unknown(SendEmailInput),
}

pub struct Summary<'a> {
    pub subject: Option<&'a String>,
    pub to: Option<&'a Destination>,
}

pub struct EmailContent<'a> {
    pub subject: Option<&'a String>,
    pub from: Option<&'a String>,
    pub to: Option<&'a Destination>,
    pub body: Option<&'a String>,
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

    pub fn get_tag(&self) -> EmailTag {
        match &self.email {
            Email::Simple(_) => EmailTag::Simple,
            Email::Template(_) => EmailTag::Template,
            Email::Raw(_) => EmailTag::Raw,
            Email::Unknown(_) => EmailTag::Unknown,
        }
    }

    // fn get_email(&self) -> &SendEmailInput {
    //     match &self.email {
    //         Email::Simple(e) => e,
    //         Email::Template(e) => e,
    //         Email::Raw(e) => e,
    //         Email::Unknown(e) => e,
    //     }
    // }

    pub fn get_subject(&self) -> Option<&String> {
        match &self.email {
            Email::Simple(e) => SimpleEmail::get_subject(e),
            Email::Template(e) => TemplateEmail::get_subject(e),
            Email::Raw(e) => RawEmail::get_subject(e),
            Email::Unknown(e) => UnknownEmail::get_subject(e),
        }
    }

    pub fn get_to(&self) -> Option<&Destination> {
        match &self.email {
            Email::Simple(e) => SimpleEmail::get_to(e),
            Email::Template(e) => TemplateEmail::get_to(e),
            Email::Raw(e) => RawEmail::get_to(e),
            Email::Unknown(e) => UnknownEmail::get_to(e),
        }
    }

    pub fn get_from(&self) -> Option<&String> {
        match &self.email {
            Email::Simple(e) => SimpleEmail::get_from(e),
            Email::Template(e) => TemplateEmail::get_from(e),
            Email::Raw(e) => RawEmail::get_from(e),
            Email::Unknown(e) => UnknownEmail::get_from(e),
        }
    }

    pub fn get_body(&self) -> Option<&String> {
        match &self.email {
            Email::Simple(e) => SimpleEmail::get_body(e),
            Email::Template(e) => TemplateEmail::get_body(e),
            Email::Raw(e) => RawEmail::get_body(e),
            Email::Unknown(e) => UnknownEmail::get_body(e),
        }
    }

    pub fn get_summary(&self) -> Summary {
        Summary {
            subject: self.get_subject(),
            to: self.get_to(),
        }
    }

    pub fn get_email_content(&self) -> EmailContent {
        EmailContent {
            subject: self.get_subject(),
            from: self.get_from(),
            to: self.get_to(),
            body: self.get_body(),
        }
    }
}
