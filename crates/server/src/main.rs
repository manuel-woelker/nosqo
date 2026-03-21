mod read_knowledge;
mod server_state;

use axum::Json;
use axum::Router;
use axum::extract::{Query, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use nosqo_base::logging::init_logging;
use nosqo_pal::pal_real::PalReal;
use read_knowledge::read_knowledge;
use serde::Deserialize;
use serde_json::Value;
use server_state::ServerState;
use std::net::SocketAddr;

const NOSQO_MIME_TYPE: &str = "text/plain";

#[derive(Debug, Deserialize)]
struct StatementQuery {
    subject: Option<String>,
    predicate: Option<String>,
    object: Option<String>,
}

#[tokio::main]
async fn main() {
    init_logging();

    let pal = PalReal::new_handle();
    let store = std::sync::Arc::new(
        read_knowledge(&*pal).expect("server should load knowledge at startup"),
    );
    let state = ServerState::new(pal, store);
    let statement_count = state
        .statement_count()
        .expect("server should be able to inspect the loaded knowledge");
    tracing::info!(
        "loaded {} statements from knowledge/ into the in-memory store",
        statement_count
    );

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/info", get(info))
        .route("/api/v1/statements", get(get_statements))
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

// Keep HTTP handlers thin. Forward all server logic to `ServerState` methods so
// it stays easy to test without Axum plumbing.
async fn info(State(state): State<ServerState>) -> Json<Value> {
    Json(state.info())
}

async fn get_statements(
    State(state): State<ServerState>,
    Query(query): Query<StatementQuery>,
) -> Result<Response, StatusCode> {
    let rendered = state
        .find_statements_nosqo(query.subject, query.predicate, query.object)
        .map_err(|error| {
            tracing::error!("failed to query statements from the in-memory store: {error:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static(NOSQO_MIME_TYPE),
        )],
        rendered,
    )
        .into_response())
}
