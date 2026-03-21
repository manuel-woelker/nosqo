mod read_knowledge;

use axum::Json;
use axum::Router;
use axum::extract::{Query, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use nosqo_base::logging::init_logging;
use nosqo_engine::{InMemoryStatementStore, StatementStore};
use nosqo_model::StatementPattern;
use nosqo_pal::pal::PalHandle;
use nosqo_pal::pal_real::PalReal;
use read_knowledge::read_knowledge;
use serde::Deserialize;
use serde_json::{Value, json};
use std::net::SocketAddr;
use std::sync::Arc;

const NOSQO_MIME_TYPE: &str = "text/plain";

#[derive(Clone)]
struct AppState {
    #[allow(dead_code)]
    pal: PalHandle,
    store: Arc<InMemoryStatementStore>,
}

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

async fn info(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({
        "name": "nosqo",
        "model": "statement-triple",
        "status": "bootstrap"
    }))
}

async fn get_statements(
    State(state): State<AppState>,
    Query(query): Query<StatementQuery>,
) -> Result<Response, StatusCode> {
    let pattern = StatementPattern::from_strings(
        query.subject.unwrap_or_else(|| "*".to_owned()),
        query.predicate.unwrap_or_else(|| "*".to_owned()),
        query.object.unwrap_or_else(|| "*".to_owned()),
    );
    let statement_set = state.store.find_statements(&pattern).map_err(|error| {
        tracing::error!("failed to query statements from the in-memory store: {error:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static(NOSQO_MIME_TYPE),
        )],
        statement_set.to_nosqo_string(),
    )
        .into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body;
    use nosqo_model::{Statement, StatementSet};
    use nosqo_pal::pal_mock::PalMock;

    #[tokio::test]
    async fn get_statements_returns_pretty_printed_nosqo_with_custom_mime_type() {
        let store = Arc::new(InMemoryStatementStore::new(StatementSet::default()));
        store
            .assert_statements(StatementSet::from(vec![
                Statement::from_strings("berlin", "label", "\"Berlin\""),
                Statement::from_strings("berlin", "isA", "#City"),
                Statement::from_strings("paris", "label", "\"Paris\""),
            ]))
            .expect("test store should accept seed statements");

        let state = AppState {
            pal: PalHandle::new(PalMock::new()),
            store,
        };
        let response = get_statements(
            State(state),
            Query(StatementQuery {
                subject: Some("berlin".to_owned()),
                predicate: Some("label".to_owned()),
                object: Some("*".to_owned()),
            }),
        )
        .await
        .expect("statement query should succeed");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(NOSQO_MIME_TYPE))
        );
        assert_eq!(
            String::from_utf8(
                body::to_bytes(response.into_body(), usize::MAX)
                    .await
                    .expect("response body should be readable")
                    .to_vec()
            )
            .expect("response body should be valid utf-8"),
            "berlin {\n  label \"Berlin\"\n}"
        );
    }
}
