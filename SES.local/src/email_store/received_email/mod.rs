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

#[cfg(test)]
mod tests {

    use maud::html;
    use ses_serde::types::{Body, Content, EmailContent, EmailTemplateContent, Message, Template};

    use super::*;

    struct SendEmailInputWrapper {
        email_tag: EmailTag,
        to: Option<String>,
        from: Option<String>,
        subject: Option<String>,
        content: Option<String>,
    }

    fn create_email(config: SendEmailInputWrapper) -> SendEmailInput {
        let email_content = match config.email_tag {
            EmailTag::Simple => EmailContent {
                simple: Some(Message {
                    subject: config.subject.map(|data| Content {
                        data,
                        charset: None,
                    }),
                    body: Some(Body {
                        text: config.content.map(|data| Content {
                            data,
                            charset: None,
                        }),
                        html: Some(Content {
                            data: (html! { div { "Some Html" } }).into_string(),
                            charset: None,
                        }),
                    }),
                    headers: None,
                    attachments: None,
                }),
                raw: None,
                template: None,
            },
            EmailTag::Template => EmailContent {
                template: Some(Template {
                    template_content: Some(EmailTemplateContent {
                        subject: config.subject,
                        text: config.content,
                        html: None,
                    }),
                    template_name: None,
                    template_arn: None,
                    template_data: None,
                    headers: None,
                    attachments: None,
                }),
                simple: None,
                raw: None,
            },
            _ => EmailContent {
                simple: None,
                raw: None,
                template: None,
            },
        };
        SendEmailInput {
            destination: Some(Destination {
                to_addresses: config.to.map(|x| vec![x]),
                cc_addresses: None,
                bcc_addresses: None,
            }),
            from_email_address: config.from,
            content: Some(email_content),
            from_email_address_identity_arn: None,
            reply_to_addresses: None,
            feedback_forwarding_email_address: None,
            feedback_forwarding_email_address_identity_arn: None,
            email_tags: None,
            configuration_set_name: None,
            endpoint_id: None,
            list_management_options: None,
        }
    }

    #[test]
    fn simple_email() {
        let sei = create_email(SendEmailInputWrapper {
            email_tag: EmailTag::Simple,
            to: Some("to@example.com".to_string()),
            from: Some("from@example.com".to_string()),
            subject: Some("Email Subject!".to_string()),
            content: Some("Email Content".to_string()),
        });

        let re = ReceivedEmail::new(sei);
        assert_eq!(
            re.get_to().unwrap().to_addresses,
            Some(vec!["to@example.com".to_string()])
        );
        assert_eq!(re.get_from().unwrap(), "from@example.com");
        assert_eq!(re.get_subject().unwrap(), "Email Subject!");
        assert_eq!(re.get_body().unwrap(), "Email Content");
    }

    #[test]
    fn template_email() {
        let sei = create_email(SendEmailInputWrapper {
            email_tag: EmailTag::Template,
            to: Some("to@example.com".to_string()),
            from: Some("from@example.com".to_string()),
            subject: Some("Email Subject!".to_string()),
            content: Some("Email Content".to_string()),
        });

        let re = ReceivedEmail::new(sei);
        assert_eq!(
            re.get_to().unwrap().to_addresses,
            Some(vec!["to@example.com".to_string()])
        );
        assert_eq!(re.get_from().unwrap(), "from@example.com");
        assert_eq!(re.get_subject().unwrap(), "Email Subject!");
        assert_eq!(re.get_body().unwrap(), "Email Content");
    }
}
