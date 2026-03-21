mod read_knowledge;

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::routing::get;
use nosqo_base::logging::init_logging;
use nosqo_engine::{InMemoryStatementStore, StatementStore};
use nosqo_model::StatementPattern;
use nosqo_pal::pal::PalHandle;
use nosqo_pal::pal_real::PalReal;
use read_knowledge::read_knowledge;
use serde_json::{Value, json};
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    #[allow(dead_code)]
    pal: PalHandle,
    #[allow(dead_code)]
    store: Arc<InMemoryStatementStore>,
}

#[tokio::main]
async fn main() {
    init_logging();

    let pal = PalReal::new_handle();
    let store = Arc::new(read_knowledge(&*pal).expect("server should load knowledge at startup"));
    let statement_count = store
        .find_statements(&StatementPattern::any())
        .expect("server should be able to inspect the loaded knowledge")
        .as_slice()
        .len();
    tracing::info!(
        "loaded {} statements from knowledge/ into the in-memory store",
        statement_count
    );

    let state = AppState { pal, store };

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
