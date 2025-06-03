use std::collections::VecDeque;

use super::{send_email::SendEmail, Event, EventContent};
use futures::Stream;
use thiserror::Error;
use tokio::{sync::broadcast, task::JoinHandle};

#[derive(Error, Debug)]
pub enum EventStoreError {
    #[error("Storing email failed")]
    Failed,
}

#[derive(Clone)]
pub struct EventStore {
    events: VecDeque<Event>,
    stream: broadcast::Sender<Event>,
}

impl EventStore {
    pub fn new() -> Self {
        let (stream, mut rx) = broadcast::channel(2);
        let events = VecDeque::new();
        let mut events_clone = events.clone();
        tokio::spawn(async move {
            while let Ok(event) = rx.recv().await {
                events_clone.push_front(event);
            }
        });
        EventStore { events, stream }
    }

    pub async fn push(&mut self, event: Event) -> Result<Event, EventStoreError> {
        self.events.push_front(event.clone());
        let saved = self.wait_for_event_id(&event.id);
        match self.stream.send(event.clone()) {
            Ok(_) => saved.await.or(Err(EventStoreError::Failed)),
            _ => Err(EventStoreError::Failed),
        }
    }

    pub fn get_all(&self) -> Vec<&Event> {
        (&self.events).into_iter().collect::<Vec<&Event>>()
    }

    pub fn get_by_event_id(&self, id: &str) -> Option<&Event> {
        self.events.iter().find(|e| e.id == *id)
    }

    fn wait_for_event_id(&self, id_: &str) -> JoinHandle<Event> {
        let id = String::from(id_);
        let mut rx = self.stream.subscribe();
        tokio::spawn(async move {
            let mut em: Option<Event> = None;
            while let Ok(event) = rx.recv().await {
                if event.id == id {
                    em = Some(event.clone());
                    break;
                }
            }
            em.unwrap()
        })
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn delete_event(&mut self, id: &str) {
        if let Some(index) = self.events.iter().position(|e| *e.id == *id) {
            self.events.remove(index);
        }
    }

    pub fn get_stream(&self) -> impl Stream<Item = Event> + use<> {
        let mut rx = self.stream.subscribe();
        async_stream::stream! {
            while let Ok(event) = rx.recv().await {
                yield event;
            }
        }
    }

    pub fn get_all_emails(&self) -> Vec<&SendEmail> {
        self.get_all()
            .into_iter()
            .filter_map(|ev| match &ev.content {
                Some(EventContent::SendEmail(ev)) => Some(ev),
                _ => None,
            })
            .collect::<Vec<&SendEmail>>()
    }

    pub fn get_email_by_message_id(&self, message_id: &str) -> Option<&SendEmail> {
        self.get_all().into_iter().find_map(|ev| match &ev.content {
            Some(EventContent::SendEmail(ev)) => {
                if ev.response.message_id == Some(message_id.to_string()) {
                    Some(ev)
                } else {
                    None
                }
            }
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;

    use super::*;

    #[tokio::test]
    async fn push() {
        let mut es = EventStore::new();
        let event = Event::empty();
        let e = es.push(event.clone()).await.unwrap();
        assert_eq!(e.id, event.id);
        assert_eq!(es.events, vec![event]);
    }

    #[tokio::test]
    async fn get_all() {
        let mut es = EventStore::new();
        let ev1 = Event::empty();
        let ev2 = Event::empty();
        _ = es.push(ev1.clone()).await.unwrap();
        _ = es.push(ev2.clone()).await.unwrap();
        assert_eq!(es.get_all(), vec![&ev2, &ev1]);
    }

    #[tokio::test]
    async fn get_by_message_id() {
        let mut es = EventStore::new();
        let ev1 = Event::empty();
        let ev2 = Event::empty();
        let e1 = es.push(ev1.clone()).await.unwrap();
        let e2 = es.push(ev2.clone()).await.unwrap();
        assert_eq!(es.get_by_event_id(&e1.id), Some(&ev1));
        assert_eq!(es.get_by_event_id(&e2.id), Some(&ev2));
    }

    #[tokio::test]
    async fn clear() {
        let mut es = EventStore::new();
        let ev1 = Event::empty();
        let ev2 = Event::empty();
        _ = es.push(ev1.clone()).await.unwrap();
        _ = es.push(ev2.clone()).await.unwrap();
        assert_eq!(
            es.events.clone().into_iter().collect::<Vec<Event>>(),
            vec![ev2, ev1]
        );
        es.clear();
        assert_eq!(es.events.into_iter().collect::<Vec<Event>>(), vec![]);
    }

    #[tokio::test]
    async fn delete_event() {
        let mut es = EventStore::new();
        let ev1 = Event::empty();
        let ev2 = Event::empty();
        _ = es.push(ev1.clone()).await.unwrap();
        _ = es.push(ev2.clone()).await.unwrap();
        assert_eq!(
            es.events.clone().into_iter().collect::<Vec<Event>>(),
            vec![ev2.clone(), ev1.clone()]
        );
        es.delete_event(&ev2.id);
        assert_eq!(es.events.into_iter().collect::<Vec<Event>>(), vec![ev1]);
    }

    #[tokio::test]
    async fn get_stream() {
        let mut es = EventStore::new();
        let stream = es.get_stream();
        let ev1 = Event::empty();
        let ev2 = Event::empty();
        _ = es.push(ev1.clone()).await.unwrap();
        _ = es.push(ev2.clone()).await.unwrap();
        let c = stream.take(2).collect::<Vec<Event>>().await;
        assert_eq!(c, vec![ev1, ev2])
    }
}
