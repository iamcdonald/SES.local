use std::sync::Arc;

use axum::{serve, Router};
use event_store::EventStore;
use tokio::sync::RwLock;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod conf;
mod page_template;
use conf::Conf;
mod event_store;
mod routes;

pub type AppEventStore = Arc<RwLock<EventStore>>;
#[derive(Clone)]
pub struct AppState {
    // that holds some api specific state
    event_store: AppEventStore,
}

pub type AppStateRouter = Router<AppState>;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "authentication=trace,{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = routes::create()
        .nest_service("/assets", ServeDir::new(&Conf::get().server.assets.path))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState {
            event_store: Arc::new(RwLock::new(EventStore::new())),
        });

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{0}", Conf::get().server.port))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    serve(listener, app).await.unwrap();
}
