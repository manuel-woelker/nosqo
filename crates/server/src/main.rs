use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::routing::get;
use nosqo_base::logging::init_logging;
use nosqo_pal::pal::PalHandle;
use nosqo_pal::pal_real::PalReal;
use serde_json::{Value, json};
use std::net::SocketAddr;

#[derive(Clone)]
struct AppState {
    #[allow(dead_code)]
    pal: PalHandle,
}

#[tokio::main]
async fn main() {
    init_logging();

    let state = AppState {
        pal: PalReal::new_handle(),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/info", get(info))
        .with_state(state);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("server should bind to a local development port");

    tracing::info!("nosqo server listening on http://{address}");
    axum::serve(listener, app)
        .await
        .expect("server should run until it is stopped");
}

async fn health() -> &'static str {
    "ok"
}

async fn info(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({
        "name": "nosqo",
        "model": "statement-triple",
        "status": "bootstrap"
    }))
}
