use crate::AppEmailStore;
use axum::response::{sse::Event, Html, IntoResponse, Sse};
use futures::{Stream, StreamExt};
use maud::{html, Markup};
use std::{io::Error, time::Duration};

pub mod templates;

pub async fn emails_sse(
    email_store: &AppEmailStore,
) -> Sse<impl Stream<Item = Result<Event, Error>>> {
    let stream = email_store.read().await.get_stream();
    let events = stream.map(|re| {
        tracing::debug!("{:?}", re);
        Ok(Event::default()
            .event("email")
            .data(templates::email_row::build(&re).into_string()))
    });
    Sse::new(events).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-stream-alive"),
    )
}

pub async fn emails_page(email_store: &AppEmailStore, email: Option<Markup>) -> impl IntoResponse {
    let esr = email_store.read().await;
    let ems = esr.get_all();
    Html(templates::emails::build(&ems, email).into_string())
}

pub async fn email_page(
    email_store: &AppEmailStore,
    id: &String,
    hx_request: bool,
) -> impl IntoResponse {
    let esr = email_store.read().await;
    match esr.get_by_message_id(id) {
        Some(em) => {
            let email_content = templates::email::build(em);
            match hx_request {
                true => Html(email_content.into_string()).into_response(),
                false => emails_page(email_store, Some(email_content))
                    .await
                    .into_response(),
            }
        }
        None => {
            let not_found = html! { (format!("Email Not Found: {}", id))};
            match hx_request {
                true => Html(not_found.into_string()).into_response(),
                false => emails_page(email_store, Some(not_found))
                    .await
                    .into_response(),
            }
        }
    }
}
