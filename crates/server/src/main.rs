mod entity_detail_attribute;
mod entity_detail_response;
mod entity_search_request;
mod entity_search_response;
mod entity_search_result;
mod error_response;
mod query_request_error;
mod query_response;
mod read_knowledge;
mod server_state;

use crate::{
    entity_detail_response::EntityDetailResponse, entity_search_request::EntitySearchRequest,
    entity_search_response::EntitySearchResponse, error_response::ErrorResponse,
    query_request_error::QueryRequestError, query_response::QueryResponse,
};
use axum::Json;
use axum::Router;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::routing::post;
use nosqo_base::logging::init_logging;
use nosqo_model::StatementJsonDocument;
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
        .route("/api/v1/entities/search", post(post_entity_search))
        .route("/api/v1/entities/{entity_id}", get(get_entity_detail))
        .route("/api/v1/ontology", get(get_ontology))
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

async fn get_ontology(
    State(state): State<ServerState>,
) -> Result<Json<StatementJsonDocument>, StatusCode> {
    let emitted = state.ontology_statement_json().map_err(|error| {
        tracing::error!("failed to retrieve ontology from the in-memory store: {error:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(emitted))
}

async fn post_query(
    State(state): State<ServerState>,
    body: String,
) -> Result<Json<QueryResponse>, QueryRequestError> {
    let response = state.execute_nql_query(body.as_str())?;
    Ok(Json(response))
}

async fn post_entity_search(
    State(state): State<ServerState>,
    Json(request): Json<EntitySearchRequest>,
) -> Result<Json<EntitySearchResponse>, StatusCode> {
    let response = state.search_entities(&request).map_err(|error| {
        tracing::error!("failed to search entities from the in-memory store: {error:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(response))
}

async fn get_entity_detail(
    State(state): State<ServerState>,
    Path(entity_id): Path<String>,
) -> Result<Json<EntityDetailResponse>, (StatusCode, Json<ErrorResponse>)> {
    let entity = state
        .get_entity_detail(entity_id.as_str())
        .map_err(|error| {
            tracing::error!(
                "failed to retrieve entity details from the in-memory store: {error:?}"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("failed to retrieve entity details")),
            )
        })?;

    let Some(entity) = entity else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(format!(
                "entity `{entity_id}` was not found"
            ))),
        ));
    };

    Ok(Json(entity))
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
    use serde_json::json;
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

    #[tokio::test]
    async fn get_ontology_returns_statement_json_for_ontology_subjects() {
        let store = std::sync::Arc::new(InMemoryStatementStore::new(StatementSet::default()));
        store
            .assert_statements(StatementSet::from(vec![
                Statement::from_strings("#Person", "isA", "#Type"),
                Statement::from_strings("#Person", "name", "Person"),
                Statement::from_strings("frodo_baggins", "isA", "#Person"),
                Statement::from_strings("~name", "isA", "#Predicate"),
            ]))
            .expect("test store should accept seed statements");
        let app = create_app(ServerState::new(PalHandle::new(PalMock::new()), store));

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/ontology")
                    .body(Body::empty())
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
                "format": "nosqo-statement-json-v1",
                "values": [
                    "#Person",
                    "~isA",
                    "#Type",
                    "~name",
                    ["Person"],
                    "#Predicate"
                ],
                "statements": [
                    [0, 1, 2],
                    [0, 3, 4],
                    [3, 1, 5]
                ]
            })
        );
    }

    #[tokio::test]
    async fn post_entity_search_returns_matching_entities() {
        let store = std::sync::Arc::new(InMemoryStatementStore::new(StatementSet::default()));
        store
            .assert_statements(StatementSet::from(vec![
                Statement::from_strings("#Person", "attribute", "~email"),
                Statement::from_strings("alice", "isA", "#Person"),
                Statement::from_strings("alice", "label", "Alice"),
                Statement::from_strings("alice", "email", "alice@example.com"),
                Statement::from_strings("bob", "isA", "#Person"),
                Statement::from_strings("bob", "label", "Bob"),
                Statement::from_strings("bob", "email", "bob@example.com"),
            ]))
            .expect("test store should accept seed statements");
        let app = create_app(ServerState::new(PalHandle::new(PalMock::new()), store));

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/entities/search")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        json!({
                            "type": "#Person",
                            "filters": {
                                "~email": "alice@example.com"
                            }
                        })
                        .to_string(),
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
            json!({
                "results": [
                    {
                        "id": "alice",
                        "nosqoId": "@alice",
                        "label": "Alice",
                        "typeIds": ["#Person"]
                    }
                ]
            })
        );
    }

    #[tokio::test]
    async fn get_entity_detail_returns_grouped_attributes() {
        let store = std::sync::Arc::new(InMemoryStatementStore::new(StatementSet::default()));
        store
            .assert_statements(StatementSet::from(vec![
                Statement::from_strings("~label", "label", "Label"),
                Statement::from_strings("~alias", "label", "Alias"),
                Statement::from_strings("frodo_baggins", "isA", "#Person"),
                Statement::from_strings("frodo_baggins", "label", "Frodo Baggins"),
                Statement::from_strings("frodo_baggins", "alias", "Ring-bearer"),
                Statement::from_strings("frodo_baggins", "alias", "Mr. Underhill"),
            ]))
            .expect("test store should accept seed statements");
        let app = create_app(ServerState::new(PalHandle::new(PalMock::new()), store));

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/entities/frodo_baggins")
                    .body(Body::empty())
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
            json!({
                "id": "frodo_baggins",
                "nosqoId": "@frodo_baggins",
                "label": "Frodo Baggins",
                "typeIds": ["#Person"],
                "attributes": [
                    {
                        "predicateId": "~alias",
                        "label": "Alias",
                        "values": ["Ring-bearer", "Mr. Underhill"]
                    },
                    {
                        "predicateId": "~isA",
                        "label": "isA",
                        "values": ["#Person"]
                    },
                    {
                        "predicateId": "~label",
                        "label": "Label",
                        "values": ["Frodo Baggins"]
                    }
                ]
            })
        );
    }

    #[tokio::test]
    async fn get_entity_detail_returns_not_found_for_unknown_ids() {
        let app = create_app(ServerState::new(
            PalHandle::new(PalMock::new()),
            std::sync::Arc::new(InMemoryStatementStore::default()),
        ));

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/entities/missing")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
