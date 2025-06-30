use crate::AppEventStore;
use axum::extract::OriginalUri;
use axum::response::{sse::Event, Html, IntoResponse, Sse};
use futures::{Stream, StreamExt};
use maud::{html, Markup};
use std::{io::Error, time::Duration};

pub mod templates;

pub async fn events_sse(
    event_store: &AppEventStore,
) -> Sse<impl Stream<Item = Result<Event, Error>>> {
    let stream = event_store.read().await.get_stream();
    let events = stream.map(|re| {
        tracing::debug!("{:?}", re);
        Ok(Event::default()
            .event("event")
            .data(templates::event_row::build(&re).into_string()))
    });
    Sse::new(events).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-stream-alive"),
    )
}

pub async fn events_page(
    event_store: &AppEventStore,
    event: Option<Markup>,
    uri: OriginalUri,
) -> impl IntoResponse {
    let esr = event_store.read().await;
    let evs = esr.get_all();
    Html(templates::events::build(&evs, event, uri.path()).into_string())
}

pub async fn event_page(
    event_store: &AppEventStore,
    id: &str,
    hx_request: bool,
    uri: OriginalUri,
) -> impl IntoResponse {
    let esr = event_store.read().await;
    match esr.get_by_event_id(id) {
        Some(em) => {
            let event_content = templates::event::build(em);
            match hx_request {
                true => Html(event_content.into_string()).into_response(),
                false => events_page(event_store, Some(event_content), uri)
                    .await
                    .into_response(),
            }
        }
        None => {
            let not_found = html! { (format!("Event Not Found: {}", id))};
            match hx_request {
                true => Html(not_found.into_string()).into_response(),
                false => events_page(event_store, Some(not_found), uri)
                    .await
                    .into_response(),
            }
        }
    }
}
