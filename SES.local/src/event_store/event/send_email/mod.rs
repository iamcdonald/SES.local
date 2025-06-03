use email_wrappers::{Body, EmailWrapper, RawEmail, SimpleEmail, TemplateEmail, UnknownEmail};
use serde::{Deserialize, Serialize};
use ses_serde::{
    operations::send_email::{SendEmailInput, SendEmailOutput},
    types::Destination,
};
use uuid::Uuid;

mod email_wrappers;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SendEmail {
    pub request: EmailRequest,
    pub response: SendEmailOutput,
}

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display)]
pub enum EmailTag {
    Simple,
    Template,
    Raw,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display, PartialEq)]
pub enum EmailRequest {
    Simple(SendEmailInput),
    Template(SendEmailInput),
    Raw(SendEmailInput),
    Unknown(SendEmailInput),
}

pub struct Summary<'a> {
    pub subject: Option<&'a str>,
    pub to: Option<&'a Destination>,
}

pub struct EmailContent<'a> {
    pub subject: Option<&'a str>,
    pub from: Option<&'a str>,
    pub to: Option<&'a Destination>,
    pub body: Option<Body<'a>>,
}

impl SendEmail {
    pub fn new(email: SendEmailInput) -> Self {
        SendEmail {
            request: EmailRequest::new(email),
            response: SendEmailOutput {
                message_id: Some(Uuid::new_v4().to_string()),
            },
        }
    }
}

impl EmailRequest {
    pub fn new(email: SendEmailInput) -> Self {
        if let Some(content) = &email.content {
            if content.simple.is_some() {
                EmailRequest::Simple(email)
            } else if content.template.is_some() {
                EmailRequest::Template(email)
            } else if content.raw.is_some() {
                EmailRequest::Raw(email)
            } else {
                EmailRequest::Unknown(email)
            }
        } else {
            EmailRequest::Unknown(email)
        }
    }

    pub fn get_tag(&self) -> EmailTag {
        match &self {
            EmailRequest::Simple(_) => EmailTag::Simple,
            EmailRequest::Template(_) => EmailTag::Template,
            EmailRequest::Raw(_) => EmailTag::Raw,
            EmailRequest::Unknown(_) => EmailTag::Unknown,
        }
    }

    pub fn get_subject(&self) -> Option<&str> {
        match &self {
            EmailRequest::Simple(e) => SimpleEmail::get_subject(e),
            EmailRequest::Template(e) => TemplateEmail::get_subject(e),
            EmailRequest::Raw(e) => RawEmail::get_subject(e),
            EmailRequest::Unknown(e) => UnknownEmail::get_subject(e),
        }
    }

    pub fn get_to(&self) -> Option<&Destination> {
        match &self {
            EmailRequest::Simple(e) => SimpleEmail::get_to(e),
            EmailRequest::Template(e) => TemplateEmail::get_to(e),
            EmailRequest::Raw(e) => RawEmail::get_to(e),
            EmailRequest::Unknown(e) => UnknownEmail::get_to(e),
        }
    }

    pub fn get_from(&self) -> Option<&str> {
        match &self {
            EmailRequest::Simple(e) => SimpleEmail::get_from(e),
            EmailRequest::Template(e) => TemplateEmail::get_from(e),
            EmailRequest::Raw(e) => RawEmail::get_from(e),
            EmailRequest::Unknown(e) => UnknownEmail::get_from(e),
        }
    }

    pub fn get_body(&self) -> Option<Body> {
        match &self {
            EmailRequest::Simple(e) => SimpleEmail::get_body(e),
            EmailRequest::Template(e) => TemplateEmail::get_body(e),
            EmailRequest::Raw(e) => RawEmail::get_body(e),
            EmailRequest::Unknown(e) => UnknownEmail::get_body(e),
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

    use ses_serde::types::{
        Body as SESBody, Content, EmailContent, EmailTemplateContent, Message, Template,
    };

    use super::*;

    struct ContentWrapper {
        text: Option<String>,
        html: Option<String>,
    }

    struct SendEmailInputWrapper {
        email_tag: EmailTag,
        to: Option<String>,
        from: Option<String>,
        subject: Option<String>,
        content: Option<ContentWrapper>,
    }

    fn create_email(config: SendEmailInputWrapper) -> SendEmailInput {
        let email_content = match config.email_tag {
            EmailTag::Simple => EmailContent {
                simple: Some(Message {
                    subject: config.subject.map(|data| Content {
                        data,
                        charset: None,
                    }),
                    body: config.content.map(|x| SESBody {
                        text: x.text.map(|data| Content {
                            data,
                            charset: None,
                        }),
                        html: x.html.map(|data| Content {
                            data,
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
                        text: config.content.as_ref().and_then(|x| x.text.clone()),
                        html: config.content.as_ref().and_then(|x| x.html.clone()),
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
    fn simple_email_text() {
        let sei = create_email(SendEmailInputWrapper {
            email_tag: EmailTag::Simple,
            to: Some("to@example.com".to_string()),
            from: Some("from@example.com".to_string()),
            subject: Some("Email Subject!".to_string()),
            content: Some(ContentWrapper {
                html: Some("Email Content ... html".to_string()),
                text: None, //Some("Email Content ... text".to_string()),
            }),
        });

        let re = EmailRequest::new(sei);
        assert_eq!(
            re.get_to().unwrap().to_addresses,
            Some(vec!["to@example.com".to_string()])
        );
        assert_eq!(re.get_from().unwrap(), "from@example.com");
        assert_eq!(re.get_subject().unwrap(), "Email Subject!");
        assert_eq!(
            re.get_body().unwrap(),
            Body {
                content: Some(&"Email Content ... html".to_string()),
                is_html: true
            }
        );
    }

    #[test]
    fn simple_email_html() {
        let sei = create_email(SendEmailInputWrapper {
            email_tag: EmailTag::Simple,
            to: Some("to@example.com".to_string()),
            from: Some("from@example.com".to_string()),
            subject: Some("Email Subject!".to_string()),
            content: Some(ContentWrapper {
                html: None,
                text: Some("Email Content ... text".to_string()),
            }),
        });

        let re = EmailRequest::new(sei);
        assert_eq!(
            re.get_to().unwrap().to_addresses,
            Some(vec!["to@example.com".to_string()])
        );
        assert_eq!(re.get_from().unwrap(), "from@example.com");
        assert_eq!(re.get_subject().unwrap(), "Email Subject!");
        assert_eq!(
            re.get_body().unwrap(),
            Body {
                content: Some(&"Email Content ... text".to_string()),
                is_html: false
            }
        );
    }

    #[test]
    fn template_email_html() {
        let sei = create_email(SendEmailInputWrapper {
            email_tag: EmailTag::Template,
            to: Some("to@example.com".to_string()),
            from: Some("from@example.com".to_string()),
            subject: Some("Email Subject!".to_string()),
            content: Some(ContentWrapper {
                html: Some("Email Content ... html".to_string()),
                text: None,
            }),
        });

        let re = EmailRequest::new(sei);
        assert_eq!(
            re.get_to().unwrap().to_addresses,
            Some(vec!["to@example.com".to_string()])
        );
        assert_eq!(re.get_from().unwrap(), "from@example.com");
        assert_eq!(re.get_subject().unwrap(), "Email Subject!");
        assert_eq!(
            re.get_body().unwrap(),
            Body {
                content: Some(&"Email Content ... html".to_string()),
                is_html: true
            }
        );
    }

    #[test]
    fn template_email_text() {
        let sei = create_email(SendEmailInputWrapper {
            email_tag: EmailTag::Template,
            to: Some("to@example.com".to_string()),
            from: Some("from@example.com".to_string()),
            subject: Some("Email Subject!".to_string()),
            content: Some(ContentWrapper {
                text: Some("Email Content ... text".to_string()),
                html: None, //Some("Email Content ... text".to_string()),
            }),
        });

        let re = EmailRequest::new(sei);
        assert_eq!(
            re.get_to().unwrap().to_addresses,
            Some(vec!["to@example.com".to_string()])
        );
        assert_eq!(re.get_from().unwrap(), "from@example.com");
        assert_eq!(re.get_subject().unwrap(), "Email Subject!");
        assert_eq!(
            re.get_body().unwrap(),
            Body {
                content: Some(&"Email Content ... text".to_string()),
                is_html: false
            }
        );
    }
}
