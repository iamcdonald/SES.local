use crate::{event_store::EventContent, AppEventStore};
use axum::{
    extract::OriginalUri,
    response::{sse::Event, Html, IntoResponse, Sse},
};
use futures::{Stream, StreamExt};
use maud::{html, Markup};
use std::{io::Error, time::Duration};

pub mod templates;

pub async fn emails_sse(
    event_store: &AppEventStore,
) -> Sse<impl Stream<Item = Result<Event, Error>>> {
    let stream = event_store.read().await.get_stream();
    let events = stream.filter_map(|ev| async move {
        match ev.content {
            Some(EventContent::SendEmail(se)) => Some(Ok(Event::default()
                .event("email")
                .data(templates::email_row::build(&se).into_string()))),
            _ => None,
        }
    });
    Sse::new(events).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-stream-alive"),
    )
}

pub async fn emails_page(
    event_store: &AppEventStore,
    email: Option<Markup>,
    uri: OriginalUri,
) -> impl IntoResponse {
    let esr = event_store.read().await;
    let ems = esr.get_all_emails();
    Html(templates::emails::build(&ems, email, uri.path()).into_string())
}

pub async fn email_page(
    event_store: &AppEventStore,
    id: &str,
    hx_request: bool,
    uri: OriginalUri,
) -> impl IntoResponse {
    let esr = event_store.read().await;
    match esr.get_email_by_message_id(id) {
        Some(em) => {
            let email_content = templates::email::build(em);
            match hx_request {
                true => Html(email_content.into_string()).into_response(),
                false => emails_page(event_store, Some(email_content), uri)
                    .await
                    .into_response(),
            }
        }
        None => {
            let not_found = html! { (format!("Email Not Found: {}", id))};
            match hx_request {
                true => Html(not_found.into_string()).into_response(),
                false => emails_page(event_store, Some(not_found), uri)
                    .await
                    .into_response(),
            }
        }
    }
}

pub async fn email_content(event_store: &AppEventStore, id: &str) -> impl IntoResponse {
    // let esr = event_store.read().await;
    // if let Some(email) = esr.get_email_by_message_id(id) {
    //     let content = email.request.get_email_content();
    //     let body = content.body.as_ref().and_then(|x| x.content).unwrap();
    //     let x = body.to_owned();
    //     Html(x).into_response()
    // } else {
    //     Html("").into_response()
    // }

    let esr = event_store.read().await;
    match esr.get_email_by_message_id(id) {
        Some(email) => {
            let content = email.request.get_email_content();
            let body = content.body.as_ref().and_then(|x| x.content).unwrap();
            let x = body.to_owned();
            Html(x).into_response()
        }
        None => Html("").into_response(),
    }
    // match email {
    //     Some(email) => {
    //         let email = &email.request.get_email_content();
    //         // let email_content = templates::email::build(em);
    //         let content = email.body.as_ref().and_then(|x| x.content);
    //         Html(content.unwrap()).into_response()
    //         // match content {
    //         //     Some(c) => Html(c).into_response()
    //         //     None => Html("").into_response()
    //         // }
    //         // let email_content = body.and_then(|x| x.content).unwrap_or(&String::from(""));
    //         // .and_then(|x| x);
    //         // .unwrap_or(String::from(""));
    //         // .unwrap_or(String::from(""));
    //         // .and_then(|x| x.content.clone())
    //         // .unwrap_or(&String::from(""));
    //         // Html(email_content.clone()).into_response()
    //     }
    //     None => {
    //         let not_found = html! { (format!("Email Not Found: {}", id))};
    //         Html(not_found.into_string()).into_response()
    //     }
    // }
}
