use super::ReceivedEmail;
use futures::Stream;
use thiserror::Error;
use tokio::{sync::broadcast, task::JoinHandle};

#[derive(Error, Debug)]
pub enum StoreEmailError {
    #[error("Storing email failed")]
    Failed,
}

#[derive(Clone)]
pub struct EmailStore {
    emails: Vec<ReceivedEmail>,
    stream: broadcast::Sender<ReceivedEmail>,
}

impl EmailStore {
    pub fn new() -> Self {
        let (stream, mut rx) = broadcast::channel(2);
        let emails = Vec::new();
        let mut emails_clone = emails.clone();
        tokio::spawn(async move {
            while let Ok(email) = rx.recv().await {
                emails_clone.push(email);
            }
        });
        EmailStore { emails, stream }
    }

    pub async fn push(&mut self, email: ReceivedEmail) -> Result<ReceivedEmail, StoreEmailError> {
        self.emails.push(email.clone());
        let saved = self.wait_for_message_id(&email.message_id);
        match self.stream.send(email.clone()) {
            Ok(_) => saved.await.or(Err(StoreEmailError::Failed)),
            _ => Err(StoreEmailError::Failed),
        }
    }

    pub fn get_all(&self) -> &Vec<ReceivedEmail> {
        &self.emails
    }

    pub fn get_by_message_id(&self, message_id: &String) -> Option<&ReceivedEmail> {
        self.emails.iter().find(|e| e.message_id == *message_id)
    }

    fn wait_for_message_id(&self, message_id: &String) -> JoinHandle<ReceivedEmail> {
        let m_id = message_id.clone();
        let mut rx = self.stream.subscribe();
        tokio::spawn(async move {
            let mut em: Option<ReceivedEmail> = None;
            while let Ok(email) = rx.recv().await {
                if email.message_id == m_id {
                    em = Some(email.clone());
                    break;
                }
            }
            em.unwrap()
        })
    }

    pub fn clear(&mut self) {
        self.emails.clear();
    }

    pub fn get_stream(&self) -> impl Stream<Item = ReceivedEmail> + use<> {
        let mut rx = self.stream.subscribe();
        async_stream::stream! {
            while let Ok(email) = rx.recv().await {
                yield email;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;
    use ses_serde::{
        operations::send_email::SendEmailInput,
        types::{Destination, EmailContent},
    };

    use super::*;

    fn create(to: Option<String>) -> SendEmailInput {
        SendEmailInput {
            destination: Some(Destination {
                to_addresses: to.map(|x| vec![x]),
                cc_addresses: None,
                bcc_addresses: None,
            }),
            from_email_address: None,
            content: Some(EmailContent {
                simple: None,
                template: None,
                raw: None,
            }),
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

    #[tokio::test]
    async fn push() {
        let mut es = EmailStore::new();
        let re = ReceivedEmail::new(create(None));
        let e = es.push(re.clone()).await.unwrap();
        assert_eq!(e.email, re.email);
        assert_eq!(es.emails, vec![re]);
    }

    #[tokio::test]
    async fn get_all() {
        let mut es = EmailStore::new();
        let re1 = ReceivedEmail::new(create(Some("a@example.com".to_string())));
        let re2 = ReceivedEmail::new(create(Some("b@example.com".to_string())));
        _ = es.push(re1.clone()).await.unwrap();
        _ = es.push(re2.clone()).await.unwrap();
        assert_eq!(es.get_all(), &vec![re1, re2]);
    }

    #[tokio::test]
    async fn get_by_message_id() {
        let mut es = EmailStore::new();
        let re1 = ReceivedEmail::new(create(Some("a@example.com".to_string())));
        let re2 = ReceivedEmail::new(create(Some("b@example.com".to_string())));
        let e1 = es.push(re1.clone()).await.unwrap();
        let e2 = es.push(re2.clone()).await.unwrap();
        assert_eq!(es.get_by_message_id(&e1.message_id), Some(&re1));
        assert_eq!(es.get_by_message_id(&e2.message_id), Some(&re2));
    }

    #[tokio::test]
    async fn clear() {
        let mut es = EmailStore::new();
        let re1 = ReceivedEmail::new(create(Some("a@example.com".to_string())));
        let re2 = ReceivedEmail::new(create(Some("b@example.com".to_string())));
        _ = es.push(re1.clone()).await.unwrap();
        _ = es.push(re2.clone()).await.unwrap();
        assert_eq!(es.emails, vec![re1, re2]);
        es.clear();
        assert_eq!(es.emails, vec![]);
    }

    #[tokio::test]
    async fn get_stream() {
        let mut es = EmailStore::new();
        let stream = es.get_stream();
        let re1 = ReceivedEmail::new(create(Some("a@example.com".to_string())));
        let re2 = ReceivedEmail::new(create(Some("b@example.com".to_string())));
        _ = es.push(re1.clone()).await.unwrap();
        _ = es.push(re2.clone()).await.unwrap();
        let c = stream.take(2).collect::<Vec<ReceivedEmail>>().await;
        assert_eq!(c, vec![re1, re2])
    }
}
