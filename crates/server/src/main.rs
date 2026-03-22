mod error_response;
mod query_request_error;
mod query_response;
mod read_knowledge;
mod server_state;

use crate::{query_request_error::QueryRequestError, query_response::QueryResponse};
use axum::Json;
use axum::Router;
use axum::extract::{Query, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::routing::post;
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

    let app = create_app(state);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("server should bind to a local development port");

    tracing::info!("nosqo server listening on http://{address}");
    axum::serve(listener, app)
        .await
        .expect("server should run until it is stopped");
}

fn create_app(state: ServerState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/v1/info", get(info))
        .route("/api/v1/statements", get(get_statements))
        .route("/api/v1/query", post(post_query))
        .with_state(state)
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

async fn post_query(
    State(state): State<ServerState>,
    body: String,
) -> Result<Json<QueryResponse>, QueryRequestError> {
    let response = state.execute_nql_query(body.as_str())?;
    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use nosqo_engine::{InMemoryStatementStore, StatementStore};
    use nosqo_model::{Statement, StatementSet};
    use nosqo_pal::pal::PalHandle;
    use nosqo_pal::pal_mock::PalMock;
    use tower::ServiceExt;

    use super::create_app;
    use crate::server_state::ServerState;

    #[tokio::test]
    async fn post_query_returns_row_oriented_results() {
        let store = std::sync::Arc::new(InMemoryStatementStore::new(StatementSet::default()));
        store
            .assert_statements(StatementSet::from(vec![
                Statement::from_strings("berlin", "label", "Berlin"),
                Statement::from_strings("paris", "label", "Paris"),
            ]))
            .expect("test store should accept seed statements");
        let app = create_app(ServerState::new(PalHandle::new(PalMock::new()), store));

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/query")
                    .header("content-type", "text/plain")
                    .body(Body::from(
                        "match\n?city ~label ?label\nreturn\n?city ?label\n",
                    ))
                    .expect("request should build"),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body should read");
        let json: serde_json::Value =
            serde_json::from_slice(body.as_ref()).expect("body should be valid json");

        assert_eq!(
            json,
            serde_json::json!({
                "columns": ["?city", "?label"],
                "rows": [
                    ["@berlin", "\"Berlin\""],
                    ["@paris", "\"Paris\""]
                ]
            })
        );
    }

    #[tokio::test]
    async fn post_query_returns_bad_request_for_invalid_queries() {
        let app = create_app(ServerState::new(
            PalHandle::new(PalMock::new()),
            std::sync::Arc::new(InMemoryStatementStore::default()),
        ));

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/query")
                    .header("content-type", "text/plain")
                    .body(Body::from("match\nreturn\n*\n"))
                    .expect("request should build"),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body should read");
        let json: serde_json::Value =
            serde_json::from_slice(body.as_ref()).expect("body should be valid json");

        assert_eq!(
            json,
            serde_json::json!({
                "error": "query must contain at least one pattern"
            })
        );
    }

    #[tokio::test]
    async fn post_query_returns_empty_row_sets() {
        let store = std::sync::Arc::new(InMemoryStatementStore::new(StatementSet::default()));
        store
            .assert_statements(StatementSet::from(vec![Statement::from_strings(
                "berlin", "label", "Berlin",
            )]))
            .expect("test store should accept seed statements");
        let app = create_app(ServerState::new(PalHandle::new(PalMock::new()), store));

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/query")
                    .header("content-type", "text/plain")
                    .body(Body::from("match\n?city ~label \"Rome\"\nreturn\n?city\n"))
                    .expect("request should build"),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body should read");
        let json: serde_json::Value =
            serde_json::from_slice(body.as_ref()).expect("body should be valid json");

        assert_eq!(
            json,
            serde_json::json!({
                "columns": ["?city"],
                "rows": []
            })
        );
    }
}
