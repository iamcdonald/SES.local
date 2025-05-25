use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde_json::json;
use ses_serde::operations::send_email::{SendEmailInput, SendEmailOutput};

use crate::email_store::ReceivedEmail;

async fn outbound_emails(
    State(crate::AppState { email_store, .. }): State<crate::AppState>,
    Json(email): Json<SendEmailInput>,
) -> impl IntoResponse {
    let rec_email = ReceivedEmail::new(email);
    let mut ems = email_store.write().await;
    match ems.push(rec_email).await {
        Ok(saved) => Json(json!(SendEmailOutput {
            message_id: Some(saved.message_id),
        }))
        .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub fn create() -> crate::AppStateRouter {
    Router::new().route("/outbound-emails", post(outbound_emails))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{email_store::EmailStore, AppState};

    use super::*;
    use axum::{
        body::{to_bytes, Body},
        http::{self, Request},
    };
    use ses_serde::types::{Destination, EmailContent};
    use tokio::sync::RwLock;
    use tower::{Service, ServiceExt};

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
    async fn outbound_emails() {
        let router = create();
        let es = Arc::new(RwLock::new(EmailStore::new()));
        let email = create_send_email_input(None);
        let response = router
            .with_state(AppState {
                email_store: es.clone(),
            })
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .uri("/outbound-emails")
                    .body(Body::from(serde_json::to_string(&email).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: SendEmailOutput = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(
            resp.message_id.unwrap(),
            es.read().await.get_all()[0].message_id
        );
    }
}
