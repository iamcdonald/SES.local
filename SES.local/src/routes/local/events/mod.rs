mod api;
mod html;

use axum::{
    body::Body,
    extract::{Path, State},
    http::{header::ACCEPT, Request, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_htmx::HxRequest;

async fn events(
    State(crate::AppState { event_store, .. }): State<crate::AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    if let Some(accept) = req.headers().get(ACCEPT) {
        return match accept.to_str().unwrap() {
            "text/event-stream" => html::events_sse(&event_store).await.into_response(),
            "application/json" => api::events_json(&event_store).await.into_response(),
            _ => html::events_page(&event_store, None).await.into_response(),
        };
    }
    (StatusCode::NOT_FOUND).into_response()
}

async fn event(
    State(crate::AppState { event_store, .. }): State<crate::AppState>,
    Path(id): Path<String>,
    HxRequest(hx_request): HxRequest,
    req: Request<Body>,
) -> impl IntoResponse {
    if let Some(accept) = req.headers().get(ACCEPT) {
        return match accept.to_str().unwrap() {
            "application/json" => api::event_json(&event_store, &id).await.into_response(),
            _ => html::event_page(&event_store, &id, hx_request)
                .await
                .into_response(),
        };
    }
    (StatusCode::NOT_FOUND).into_response()
}

pub fn create() -> crate::AppStateRouter {
    Router::new().nest(
        "/events",
        Router::new()
            .route("/", get(events))
            .route("/{id}", get(event)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        event_store::{Event, EventStore},
        AppState,
    };
    use axum::{
        body::{to_bytes, Body},
        http::{self, Request},
    };
    use eventsource_stream::{Event as ESEvent, Eventsource};
    use futures::StreamExt;
    use maud::html;
    use std::sync::Arc;
    use tokio::pin;
    use tokio::{net::TcpListener, sync::RwLock};
    use tower::{Service, ServiceExt};
    use uuid::Uuid;

    fn create_event() -> Event {
        Event::empty()
    }

    #[tokio::test]
    async fn events_json() {
        let router = create();
        let evs = Arc::new(RwLock::new(EventStore::new()));
        {
            let mut evsw = evs.write().await;
            _ = evsw.push(create_event()).await;
            _ = evsw.push(create_event()).await;
        }
        let response = router
            .with_state(AppState {
                event_store: evs.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "application/json")
                    .uri("/events")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: Vec<Event> = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(&resp, evs.read().await.get_all());
    }

    #[tokio::test]
    async fn emails_html() {
        let router = create();
        let evs = Arc::new(RwLock::new(EventStore::new()));
        {
            let mut evsw = evs.write().await;
            _ = evsw.push(create_event()).await;
            _ = evsw.push(create_event()).await;
        }
        let response = router
            .with_state(AppState {
                event_store: evs.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "text/html")
                    .uri("/events")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp = String::from_utf8(body_bytes.into_iter().collect()).unwrap();
        let evsr = evs.read().await;
        assert_eq!(
            resp,
            crate::routes::local::events::html::templates::events::build(evsr.get_all(), None)
                .into_string()
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn events_sse() {
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
            .get(format!("{}/events", server_url))
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
        let mut evsr = app_state.event_store.write().await;
        let ev1 = create_event();
        let ev2 = create_event();
        _ = evsr.push(ev1.clone()).await;
        _ = evsr.push(ev2.clone()).await;

        let events = events_gatherer.await.unwrap();

        let expected = vec![
            ESEvent {
                event: "event".to_string(),
                data: crate::routes::local::events::html::templates::event_row::build(&ev1)
                    .into_string(),
                id: "".to_string(),
                retry: None,
            },
            ESEvent {
                event: "event".to_string(),
                data: crate::routes::local::events::html::templates::event_row::build(&ev2)
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
    async fn events_no_accept_header_404() {
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
    async fn event_json() {
        let router = create();
        let evs = Arc::new(RwLock::new(EventStore::new()));
        let id = {
            let mut evsw = evs.write().await;
            let ev = evsw.push(create_event()).await;
            ev.unwrap().id
        };
        let response = router
            .with_state(AppState {
                event_store: evs.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "application/json")
                    .uri(format!("/events/{}", id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: Event = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(&resp, evs.read().await.get_by_event_id(&id).unwrap());
    }

    #[tokio::test]
    async fn event_json_email_not_found_404() {
        let router = create();
        let evs = Arc::new(RwLock::new(EventStore::new()));
        let response = router
            .with_state(AppState {
                event_store: evs.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "application/json")
                    .uri(format!("/events/{}", Uuid::new_v4().to_string()))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn event_html() {
        let router = create();
        let evs = Arc::new(RwLock::new(EventStore::new()));
        let id = {
            let mut evsw = evs.write().await;
            let ev = evsw.push(create_event()).await;
            ev.unwrap().id
        };
        let response = router
            .with_state(AppState {
                event_store: evs.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "text/html")
                    .uri(format!("/events/{}", id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp = String::from_utf8(body_bytes.into_iter().collect()).unwrap();
        let evsr = evs.read().await;
        assert_eq!(
            resp,
            crate::routes::local::events::html::templates::events::build(
                evsr.get_all(),
                Some(crate::routes::local::events::html::templates::event::build(
                    evsr.get_by_event_id(&id).unwrap()
                )),
            )
            .into_string()
        );
    }

    #[tokio::test]
    async fn event_html_email_not_found_contains_event_not_found_content() {
        let router = create();
        let evs = Arc::new(RwLock::new(EventStore::new()));
        let id = Uuid::new_v4().to_string();
        let response = router
            .with_state(AppState {
                event_store: evs.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "text/html")
                    .uri(format!("/events/{}", id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp = String::from_utf8(body_bytes.into_iter().collect()).unwrap();
        let evsr = evs.read().await;
        assert_eq!(
            resp,
            crate::routes::local::events::html::templates::events::build(
                evsr.get_all(),
                Some(html! { (format!("Event Not Found: {}", id))}),
            )
            .into_string()
        );
    }

    #[tokio::test]
    async fn event_html_htmx_fragment() {
        let router = create();
        let evs = Arc::new(RwLock::new(EventStore::new()));
        let id = {
            let mut evsw = evs.write().await;
            let ev = evsw.push(create_event()).await;
            ev.unwrap().id
        };
        let response = router
            .with_state(AppState {
                event_store: evs.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "text/html")
                    .header("HX-Request", "true")
                    .uri(format!("/events/{}", id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp = String::from_utf8(body_bytes.into_iter().collect()).unwrap();
        let evsr = evs.read().await;
        assert_eq!(
            resp,
            crate::routes::local::events::html::templates::event::build(
                evsr.get_by_event_id(&id).unwrap()
            )
            .into_string()
        );
    }

    #[tokio::test]
    async fn event_html_htmx_fragment_event_not_found_contains_event_not_found_content() {
        let router = create();
        let evs = Arc::new(RwLock::new(EventStore::new()));
        let id = Uuid::new_v4().to_string();
        let response = router
            .with_state(AppState {
                event_store: evs.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .header(http::header::ACCEPT, "text/html")
                    .header("HX-Request", "true")
                    .uri(format!("/events/{}", id))
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
            html! { (format!("Event Not Found: {}", id))}.into_string()
        );
    }

    #[tokio::test]
    async fn event_no_accept_header_404() {
        let router = create();
        let evs = Arc::new(RwLock::new(EventStore::new()));
        let id = {
            let mut evsw = evs.write().await;
            let ev = evsw.push(create_event()).await;
            ev.unwrap().id
        };
        let response = router
            .with_state(AppState {
                event_store: evs.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/events/{}", id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
