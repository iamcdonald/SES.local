use axum::body::Bytes;
use jiff::Timestamp;
use send_email::SendEmail;
use serde::{Deserialize, Serialize};
use ses_serde::operations::send_email::SendEmailInput;
use uuid::Uuid;

pub mod send_email;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Event {
    pub id: String,
    pub timestamp: String,
    pub content: Option<EventContent>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, strum_macros::Display)]
pub enum EventContent {
    SendEmail(SendEmail),
}

impl Event {
    pub fn from_body(body: Bytes, uri: &str) -> Option<Self> {
        if let Some(content) = match uri.as_ref() {
            "/v2/email/outbound-emails" => {
                let sei: SendEmailInput = serde_json::from_slice(&body).unwrap();
                Some(EventContent::SendEmail(SendEmail::new(sei)))
            }
            _ => None,
        } {
            Some(Event::new(content))
        } else {
            None
        }
    }

    pub fn new(content: EventContent) -> Self {
        Event {
            id: Uuid::new_v4().to_string(),
            timestamp: Timestamp::now().to_string(),
            content: Some(content),
        }
    }

    pub fn get_json_response(&self) -> Option<String> {
        if let Some(content) = &self.content {
            match &content {
                EventContent::SendEmail(ev) => serde_json::to_string(&ev.response).ok(),
            }
        } else {
            None
        }
    }

    pub fn empty() -> Self {
        Event {
            id: Uuid::new_v4().to_string(),
            timestamp: Timestamp::now().to_string(),
            content: None,
        }
    }

    pub fn get_name(&self) -> String {
        self.content
            .as_ref()
            .map(|t| t.to_string())
            .unwrap_or("None".to_string())
    }
}
