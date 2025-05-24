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
        // let email = email_.clone();
        // let stream = self.stream.clone();
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

    pub fn wait_for_message_id(&self, message_id: &String) -> JoinHandle<ReceivedEmail> {
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

    pub fn subscribe(&self) -> broadcast::Receiver<ReceivedEmail> {
        self.stream.subscribe()
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
