mod api;
mod html;

use axum::{
    body::Body,
    extract::{OriginalUri, Path, State},
    http::{header::ACCEPT, Request, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_htmx::HxRequest;

async fn emails(
    State(crate::AppState { event_store, .. }): State<crate::AppState>,
    uri: OriginalUri,
    req: Request<Body>,
) -> impl IntoResponse {
    if let Some(accept) = req.headers().get(ACCEPT) {
        return match accept.to_str().unwrap() {
            "text/event-stream" => html::emails_sse(&event_store).await.into_response(),
            "application/json" => api::emails_json(&event_store).await.into_response(),
            _ => html::emails_page(&event_store, None, uri)
                .await
                .into_response(),
        };
    }
    (StatusCode::NOT_FOUND).into_response()
}

async fn email(
    State(crate::AppState { event_store, .. }): State<crate::AppState>,
    Path(id): Path<String>,
    HxRequest(hx_request): HxRequest,
    uri: OriginalUri,
    req: Request<Body>,
) -> impl IntoResponse {
    if let Some(accept) = req.headers().get(ACCEPT) {
        return match accept.to_str().unwrap() {
            "application/json" => api::email_json(&event_store, &id).await.into_response(),
            _ => html::email_page(&event_store, &id, hx_request, uri)
                .await
                .into_response(),
        };
    }
    (StatusCode::NOT_FOUND).into_response()
}

async fn email_content(
    State(crate::AppState { event_store, .. }): State<crate::AppState>,
    Path(id): Path<String>,
    // req: Request<Body>,
) -> impl IntoResponse {
    html::email_content(&event_store, &id).await.into_response()
}

pub fn create() -> crate::AppStateRouter {
    Router::new().nest(
        "/emails",
        Router::new()
            .route("/", get(emails))
            .route("/{id}", get(email))
            .route("/{id}/content", get(email_content)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_store::send_email::SendEmail;
    use crate::event_store::{Event, EventContent, EventStore};
    use crate::AppState;
    use axum::{
        body::{to_bytes, Body},
        http::{self, Request},
    };
    use eventsource_stream::{Event as ESEvent, Eventsource};
    use futures::StreamExt;
    use maud::html;
    use ses_serde::{
        operations::send_email::SendEmailInput,
        types::{Destination, EmailContent},
    };
    use std::sync::Arc;
    use tokio::pin;
    use tokio::{net::TcpListener, sync::RwLock};
    use tower::{Service, ServiceExt};
    use uuid::Uuid;

    fn create_send_email_input(to: Option<String>) -> SendEmailInput {
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
    async fn emails_json() {
        let router = create();
        let es = Arc::new(RwLock::new(EventStore::new()));
        {
            let mut esw = es.write().await;
            _ = esw
                .push(Event::new(EventContent::SendEmail(SendEmail::new(
                    create_send_email_input(Some(String::from("a@example.com"))),
                ))))
                .await;
            _ = esw
                .push(Event::new(EventContent::SendEmail(SendEmail::new(
                    create_send_email_input(Some(String::from("b@example.com"))),
                ))))
                .await;
        }
        let response = router
            .with_state(AppState {
                event_store: es.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "application/json")
                    .uri("/emails")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: Vec<SendEmail> = serde_json::from_slice(&body_bytes).unwrap();
        for (e, a) in es
            .read()
            .await
            .get_all_emails()
            .into_iter()
            .zip(resp)
            .collect::<Vec<(&SendEmail, SendEmail)>>()
        {
            assert_eq!(&a, e);
        }
    }

    #[tokio::test]
    async fn emails_html() {
        let router = create();
        let es = Arc::new(RwLock::new(EventStore::new()));
        {
            let mut esw = es.write().await;
            _ = esw
                .push(Event::new(EventContent::SendEmail(SendEmail::new(
                    create_send_email_input(Some(String::from("a@example.com"))),
                ))))
                .await;
            _ = esw
                .push(Event::new(EventContent::SendEmail(SendEmail::new(
                    create_send_email_input(Some(String::from("b@example.com"))),
                ))))
                .await;
        }
        let response = router
            .with_state(AppState {
                event_store: es.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "text/html")
                    .uri("/emails")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp = String::from_utf8(body_bytes.into_iter().collect()).unwrap();
        let esr = es.read().await;
        assert_eq!(
            resp,
            crate::routes::local::emails::html::templates::emails::build(
                &esr.get_all_emails(),
                None
            )
            .into_string()
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn emails_sse() {
        // setup app with routes running in tokio thread
        async fn start_app(host: impl Into<String>, app_state: AppState) -> String {
            let host = host.into();
            // Bind to localhost at the port 0, which will let the OS assign an available port to us
            let listener = TcpListener::bind(format!("{host}:0")).await.unwrap();
            // Retrieve the port assigned to us by the OS
            let port = listener.local_addr().unwrap().port();
            tokio::spawn(async {
                axum::serve(listener, create().with_state(app_state))
                    .await
                    .unwrap();
            });
            // Returns address (e.g. http://127.0.0.1{random_port})
            format!("http://{host}:{port}")
        }
        let app_state = AppState {
            event_store: Arc::new(RwLock::new(EventStore::new())),
        };
        let server_url = start_app("127.0.0.1", app_state.clone()).await;

        // Make request
        let event_stream = reqwest::Client::new()
            .get(format!("{}/emails", server_url))
            .header(http::header::ACCEPT, "text/event-stream")
            .send()
            .await
            .unwrap()
            .bytes_stream()
            .eventsource()
            .take(2);

        // Spawn task to aggregate events
        let events_gatherer = tokio::spawn(async move {
            let mut events: Vec<ESEvent> = vec![];
            pin!(event_stream);
            while let Some(event) = event_stream.next().await {
                match event {
                    Ok(event) => {
                        if event.data == "[DONE]" {
                            break;
                        }
                        events.push(event);
                    }
                    Err(_) => {
                        panic!("Error in event stream");
                    }
                }
            }
            events
        });

        // add events to email_store to trgger server sent events
        let mut esw = app_state.event_store.write().await;
        let se1 = SendEmail::new(create_send_email_input(Some("a@example.com".to_string())));
        let se2 = SendEmail::new(create_send_email_input(Some("b@example.com".to_string())));
        let rec_email1 = Event::new(EventContent::SendEmail(se1.clone()));
        let rec_email2 = Event::new(EventContent::SendEmail(se2.clone()));
        _ = esw.push(rec_email1.clone()).await;
        _ = esw.push(rec_email2.clone()).await;

        let events = events_gatherer.await.unwrap();

        let expected = vec![
            ESEvent {
                event: "email".to_string(),
                data: crate::routes::local::emails::html::templates::email_row::build(&se1)
                    .into_string(),
                id: "".to_string(),
                retry: None,
            },
            ESEvent {
                event: "email".to_string(),
                data: crate::routes::local::emails::html::templates::email_row::build(&se2)
                    .into_string(),
                id: "".to_string(),
                retry: None,
            },
        ];
        for item in expected {
            let found = events.iter().find(|e| e.data == item.data);
            assert_eq!(found, Some(&item));
        }
    }

    #[tokio::test]
    async fn emails_no_accept_header_404() {
        let router = create();
        let evs = Arc::new(RwLock::new(EventStore::new()));
        let response = router
            .with_state(AppState {
                event_store: evs.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/emails")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn email_json() {
        let router = create();
        let es = Arc::new(RwLock::new(EventStore::new()));
        let message_id = {
            let mut esw = es.write().await;
            let se = SendEmail::new(create_send_email_input(Some(String::from("a@example.com"))));
            _ = esw
                .push(Event::new(EventContent::SendEmail(se.clone())))
                .await;
            se.response.message_id.unwrap()
        };
        let response = router
            .with_state(AppState {
                event_store: es.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "application/json")
                    .uri(format!("/emails/{}", message_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: SendEmail = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(
            &resp,
            es.read()
                .await
                .get_email_by_message_id(&message_id)
                .unwrap()
        );
    }

    #[tokio::test]
    async fn email_json_email_not_found_404() {
        let router = create();
        let es = Arc::new(RwLock::new(EventStore::new()));
        let response = router
            .with_state(AppState {
                event_store: es.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "application/json")
                    .uri(format!("/emails/{}", Uuid::new_v4().to_string()))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn email_html() {
        let router = create();
        let es = Arc::new(RwLock::new(EventStore::new()));
        let message_id = {
            let mut esw = es.write().await;
            let se = SendEmail::new(create_send_email_input(Some(String::from("a@example.com"))));
            _ = esw
                .push(Event::new(EventContent::SendEmail(se.clone())))
                .await;
            se.response.message_id.unwrap()
        };
        let response = router
            .with_state(AppState {
                event_store: es.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "text/html")
                    .uri(format!("/emails/{}", message_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp = String::from_utf8(body_bytes.into_iter().collect()).unwrap();
        let esr = es.read().await;
        assert_eq!(
            resp,
            crate::routes::local::emails::html::templates::emails::build(
                &esr.get_all_emails(),
                Some(crate::routes::local::emails::html::templates::email::build(
                    esr.get_email_by_message_id(&message_id).unwrap()
                )),
            )
            .into_string()
        );
    }

    #[tokio::test]
    async fn email_html_email_not_found_contains_email_not_found_content() {
        let router = create();
        let es = Arc::new(RwLock::new(EventStore::new()));
        let message_id = Uuid::new_v4().to_string();
        let response = router
            .with_state(AppState {
                event_store: es.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "text/html")
                    .uri(format!("/emails/{}", message_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp = String::from_utf8(body_bytes.into_iter().collect()).unwrap();
        assert_eq!(
            resp,
            crate::routes::local::emails::html::templates::emails::build(
                &es.read().await.get_all_emails(),
                Some(html! { (format!("Email Not Found: {}", message_id))}),
            )
            .into_string()
        );
    }

    #[tokio::test]
    async fn email_html_htmx_fragment() {
        let router = create();
        let es = Arc::new(RwLock::new(EventStore::new()));
        let message_id = {
            let mut esw = es.write().await;
            let se = SendEmail::new(create_send_email_input(Some(String::from("a@example.com"))));
            _ = esw
                .push(Event::new(EventContent::SendEmail(se.clone())))
                .await;
            se.response.message_id.unwrap()
        };
        let response = router
            .with_state(AppState {
                event_store: es.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "text/html")
                    .header("HX-Request", "true")
                    .uri(format!("/emails/{}", message_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp = String::from_utf8(body_bytes.into_iter().collect()).unwrap();
        let esr = es.read().await;
        assert_eq!(
            resp,
            crate::routes::local::emails::html::templates::email::build(
                esr.get_email_by_message_id(&message_id).unwrap()
            )
            .into_string()
        );
    }

    #[tokio::test]
    async fn email_html_htmx_fragment_email_not_found_contains_email_not_found_content() {
        let router = create();
        let es = Arc::new(RwLock::new(EventStore::new()));
        let message_id = Uuid::new_v4().to_string();
        let response = router
            .with_state(AppState {
                event_store: es.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "text/html")
                    .header("HX-Request", "true")
                    .uri(format!("/emails/{}", message_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp = String::from_utf8(body_bytes.into_iter().collect()).unwrap();
        assert_eq!(
            resp,
            html! { (format!("Email Not Found: {}", message_id))}.into_string()
        );
    }

    #[tokio::test]
    async fn email_no_accept_header_404() {
        let router = create();
        let es = Arc::new(RwLock::new(EventStore::new()));
        let message_id = {
            let mut esw = es.write().await;
            let se = SendEmail::new(create_send_email_input(Some(String::from("a@example.com"))));
            _ = esw
                .push(Event::new(EventContent::SendEmail(se.clone())))
                .await;
            se.response.message_id.unwrap()
        };
        let response = router
            .with_state(AppState {
                event_store: es.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/emails/{}", message_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
