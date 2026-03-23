use crate::{
    entity_detail_attribute::EntityDetailAttribute, entity_detail_response::EntityDetailResponse,
    entity_search_request::EntitySearchRequest, entity_search_response::EntitySearchResponse,
    entity_search_result::EntitySearchResult, query_request_error::QueryRequestError,
    query_response::QueryResponse,
};
use nosqo_base::result::NosqoResult;
use nosqo_engine::{InMemoryStatementStore, StatementStore, execute_nql_query, validate_nql_query};
use nosqo_model::{Statement, StatementJsonDocument, StatementPattern, Value};
use nosqo_pal::pal::PalHandle;
use nosqo_parser::NqlParser;
use serde_json::{Value as JsonValue, json};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

/// Shared server state plus the non-HTTP business logic that handlers forward
/// to.
#[derive(Clone)]
pub struct ServerState {
    /// Platform access used by the server process.
    #[allow(dead_code)]
    pub pal: PalHandle,
    /// In-memory statement store loaded at startup.
    pub store: Arc<InMemoryStatementStore>,
}

impl ServerState {
    /// Creates a new server state value.
    pub fn new(pal: PalHandle, store: Arc<InMemoryStatementStore>) -> Self {
        Self { pal, store }
    }

    /// Returns the number of loaded statements.
    pub fn statement_count(&self) -> NosqoResult<usize> {
        Ok(self
            .store
            .find_statements(&StatementPattern::any())?
            .as_slice()
            .len())
    }

    /// Returns the static server info payload.
    pub fn info(&self) -> JsonValue {
        json!({
            "name": "nosqo",
            "model": "statement-triple",
            "status": "bootstrap"
        })
    }

    /// Finds statements matching the provided query strings and returns them as
    /// pretty-printed nosqo text.
    pub fn find_statements_nosqo(
        &self,
        subject: Option<String>,
        predicate: Option<String>,
        object: Option<String>,
    ) -> NosqoResult<String> {
        let pattern = StatementPattern::from_strings(
            subject.unwrap_or_else(|| "*".to_owned()),
            predicate.unwrap_or_else(|| "*".to_owned()),
            object.unwrap_or_else(|| "*".to_owned()),
        );
        let statement_set = self.store.find_statements(&pattern)?;
        Ok(statement_set.to_nosqo_string())
    }

    /// Returns ontology statements using the indexed JSON transport.
    pub fn ontology_statement_json(&self) -> NosqoResult<StatementJsonDocument> {
        let ontology_statements = self
            .store
            .find_statements(&StatementPattern::any())?
            .as_slice()
            .iter()
            .filter(|statement| {
                let subject = statement.subject.as_str();
                subject.starts_with('#') || subject.starts_with('~')
            })
            .cloned()
            .collect();

        Ok(nosqo_model::StatementSet::new(ontology_statements).to_statement_json())
    }

    /// Parses, validates, and executes a raw NQL query string.
    pub fn execute_nql_query(&self, query_text: &str) -> Result<QueryResponse, QueryRequestError> {
        let query = NqlParser::parse_str(query_text)
            .map_err(|error| QueryRequestError::InvalidQuery(error.kind().to_string()))?;
        validate_nql_query(&query)
            .map_err(|error| QueryRequestError::InvalidQuery(error.kind().to_string()))?;
        let result =
            execute_nql_query(&*self.store, &query).map_err(QueryRequestError::Internal)?;

        Ok(QueryResponse::from_query_result(result))
    }

    /// Searches non-ontology entities by exact type and exact attribute values.
    pub fn search_entities(
        &self,
        request: &EntitySearchRequest,
    ) -> NosqoResult<EntitySearchResponse> {
        let statements = self.store.find_statements(&StatementPattern::any())?;
        let grouped = group_statements_by_subject(statements.as_slice());

        let mut results = grouped
            .iter()
            .filter(|(subject, _)| !is_ontology_subject(subject))
            .filter_map(|(subject, entity_statements)| {
                let type_ids = read_type_ids(entity_statements);

                if !type_ids
                    .iter()
                    .any(|type_id| type_id == &request.entity_type)
                {
                    return None;
                }

                let matches_all_filters =
                    request.filters.iter().all(|(predicate_id, filter_value)| {
                        entity_statements.iter().any(|statement| {
                            statement.predicate.as_str() == predicate_id
                                && display_value(&statement.object) == *filter_value
                        })
                    });

                if !matches_all_filters {
                    return None;
                }

                Some(EntitySearchResult {
                    id: subject.clone(),
                    nosqo_id: format!("@{subject}"),
                    label: preferred_label(entity_statements),
                    type_ids,
                })
            })
            .collect::<Vec<_>>();

        results.sort_by(|left, right| {
            left.label
                .cmp(&right.label)
                .then_with(|| left.nosqo_id.cmp(&right.nosqo_id))
        });

        Ok(EntitySearchResponse { results })
    }

    /// Returns grouped attributes for one non-ontology entity if it exists.
    pub fn get_entity_detail(&self, entity_id: &str) -> NosqoResult<Option<EntityDetailResponse>> {
        let statements = self.store.find_statements(&StatementPattern::any())?;
        let grouped = group_statements_by_subject(statements.as_slice());
        let ontology_labels = build_predicate_label_map(&grouped);
        let Some(entity_statements) = grouped.get(entity_id) else {
            return Ok(None);
        };

        if is_ontology_subject(entity_id) {
            return Ok(None);
        }

        let mut grouped_attributes = BTreeMap::<String, Vec<String>>::new();

        for statement in entity_statements {
            grouped_attributes
                .entry(statement.predicate.as_str().to_owned())
                .or_default()
                .push(display_value(&statement.object));
        }

        let attributes = grouped_attributes
            .into_iter()
            .map(|(predicate_id, values)| EntityDetailAttribute {
                label: ontology_labels
                    .get(predicate_id.as_str())
                    .cloned()
                    .unwrap_or_else(|| predicate_id.trim_start_matches('~').to_owned()),
                predicate_id,
                values,
            })
            .collect();

        Ok(Some(EntityDetailResponse {
            id: entity_id.to_owned(),
            nosqo_id: format!("@{entity_id}"),
            label: preferred_label(entity_statements),
            type_ids: read_type_ids(entity_statements),
            attributes,
        }))
    }
}

fn group_statements_by_subject(statements: &[Statement]) -> HashMap<String, Vec<Statement>> {
    let mut grouped = HashMap::<String, Vec<Statement>>::new();

    for statement in statements {
        grouped
            .entry(statement.subject.as_str().to_owned())
            .or_default()
            .push(statement.clone());
    }

    grouped
}

fn build_predicate_label_map(
    grouped_statements: &HashMap<String, Vec<Statement>>,
) -> HashMap<String, String> {
    grouped_statements
        .iter()
        .filter(|(subject, _)| subject.starts_with('~'))
        .map(|(subject, statements)| (subject.clone(), preferred_label(statements)))
        .collect()
}

fn is_ontology_subject(subject: &str) -> bool {
    subject.starts_with('#') || subject.starts_with('~')
}

fn read_type_ids(statements: &[Statement]) -> Vec<String> {
    let mut type_ids = statements
        .iter()
        .filter(|statement| statement.predicate.as_str() == "~isA")
        .map(|statement| display_value(&statement.object))
        .collect::<Vec<_>>();

    type_ids.sort();
    type_ids.dedup();
    type_ids
}

fn preferred_label(statements: &[Statement]) -> String {
    find_statement_value(statements, "~label")
        .or_else(|| find_statement_value(statements, "~name"))
        .unwrap_or_else(|| format!("@{}", statements[0].subject.as_str()))
}

fn find_statement_value(statements: &[Statement], predicate_id: &str) -> Option<String> {
    statements
        .iter()
        .find(|statement| statement.predicate.as_str() == predicate_id)
        .map(|statement| display_value(&statement.object))
}

fn display_value(value: &Value) -> String {
    match value {
        Value::Text(text) => text.as_str().to_owned(),
        Value::Symbol(symbol) => symbol.as_str().to_owned(),
        Value::Id(id) => id.to_nosqo_string(),
        Value::Integer(integer) => format!("i{integer}"),
        Value::Decimal(decimal) => format!("n{}", decimal.as_str()),
        Value::Date(date) => format!("d{}", date.as_str()),
        Value::DateTime(date_time) => format!("t{}", date_time.as_str()),
        Value::Boolean(true) => "T".to_owned(),
        Value::Boolean(false) => "F".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nosqo_model::{Statement, StatementSet};
    use nosqo_pal::pal_mock::PalMock;

    #[test]
    fn find_statements_nosqo_returns_pretty_printed_matches() {
        let store = Arc::new(InMemoryStatementStore::new(StatementSet::default()));
        store
            .assert_statements(StatementSet::from(vec![
                Statement::from_strings("berlin", "label", "\"Berlin\""),
                Statement::from_strings("berlin", "isA", "#City"),
                Statement::from_strings("paris", "label", "\"Paris\""),
            ]))
            .expect("test store should accept seed statements");
        let state = ServerState::new(PalHandle::new(PalMock::new()), store);

        let rendered = state
            .find_statements_nosqo(Some("berlin".to_owned()), Some("label".to_owned()), None)
            .expect("statement query should succeed");

        assert_eq!(rendered, "berlin {\n  label \"Berlin\"\n}");
    }

    #[test]
    fn omitted_query_values_default_to_any() {
        let store = Arc::new(InMemoryStatementStore::new(StatementSet::default()));
        store
            .assert_statements(StatementSet::from(vec![
                Statement::from_strings("berlin", "label", "\"Berlin\""),
                Statement::from_strings("paris", "label", "\"Paris\""),
            ]))
            .expect("test store should accept seed statements");
        let state = ServerState::new(PalHandle::new(PalMock::new()), store);

        let rendered = state
            .find_statements_nosqo(None, Some("label".to_owned()), None)
            .expect("statement query should succeed");

        assert_eq!(
            rendered,
            "berlin {\n  label \"Berlin\"\n}\n\nparis {\n  label \"Paris\"\n}"
        );
    }

    #[test]
    fn execute_nql_query_returns_row_oriented_json_payload_data() {
        let store = Arc::new(InMemoryStatementStore::new(StatementSet::default()));
        store
            .assert_statements(StatementSet::from(vec![
                Statement::from_strings("berlin", "label", "Berlin"),
                Statement::from_strings("berlin", "isA", "#City"),
            ]))
            .expect("test store should accept seed statements");
        let state = ServerState::new(PalHandle::new(PalMock::new()), store);

        let response = state
            .execute_nql_query("match\n?city ~label ?label\nreturn\n?city ?label\n")
            .expect("query should succeed");

        assert_eq!(
            response,
            QueryResponse {
                columns: vec!["?city".to_string(), "?label".to_string()],
                rows: vec![vec!["@berlin".to_string(), "\"Berlin\"".to_string()]],
            }
        );
    }

    #[test]
    fn execute_nql_query_returns_invalid_query_errors_for_bad_input() {
        let state = ServerState::new(
            PalHandle::new(PalMock::new()),
            Arc::new(InMemoryStatementStore::default()),
        );

        let error = state
            .execute_nql_query("match\nreturn\n*\n")
            .expect_err("query should fail");

        let QueryRequestError::InvalidQuery(message) = error else {
            panic!("expected invalid query error");
        };

        assert!(message.contains("query must contain at least one pattern"));
    }

    #[test]
    fn ontology_statement_json_returns_only_ontology_subjects() {
        let store = Arc::new(InMemoryStatementStore::new(StatementSet::default()));
        store
            .assert_statements(StatementSet::from(vec![
                Statement::from_strings("#Person", "isA", "#Type"),
                Statement::from_strings("#Person", "name", "Person"),
                Statement::from_strings("frodo_baggins", "isA", "#Person"),
                Statement::from_strings("frodo_baggins", "name", "Frodo Baggins"),
                Statement::from_strings("~name", "isA", "#Predicate"),
                Statement::from_strings("~name", "targetType", "#String"),
            ]))
            .expect("test store should accept seed statements");
        let state = ServerState::new(PalHandle::new(PalMock::new()), store);

        let emitted = state
            .ontology_statement_json()
            .expect("ontology emission should succeed");

        assert_eq!(emitted.format, "nosqo-statement-json-v1");
        assert_eq!(emitted.statements.len(), 4);
        assert!(emitted.values.iter().any(|value| {
            matches!(
                value,
                nosqo_model::StatementJsonValue::NosqoToken(token) if token == "#Person"
            )
        }));
        assert!(emitted.values.iter().any(|value| {
            matches!(
                value,
                nosqo_model::StatementJsonValue::Text(text) if text[0] == "Person"
            )
        }));
    }

    #[test]
    fn search_entities_filters_by_exact_type_and_attribute_values() {
        let store = Arc::new(InMemoryStatementStore::new(StatementSet::default()));
        store
            .assert_statements(StatementSet::from(vec![
                Statement::from_strings("alice", "isA", "#Person"),
                Statement::from_strings("alice", "label", "Alice"),
                Statement::from_strings("alice", "alias", "A"),
                Statement::from_strings("bob", "isA", "#Person"),
                Statement::from_strings("bob", "label", "Bob"),
                Statement::from_strings("bob", "alias", "B"),
                Statement::from_strings("~alias", "label", "Alias"),
            ]))
            .expect("test store should accept seed statements");
        let state = ServerState::new(PalHandle::new(PalMock::new()), store);

        let response = state
            .search_entities(&EntitySearchRequest {
                entity_type: "#Person".to_owned(),
                filters: BTreeMap::from([("~alias".to_owned(), "A".to_owned())]),
            })
            .expect("search should succeed");

        assert_eq!(
            response,
            EntitySearchResponse {
                results: vec![EntitySearchResult {
                    id: "alice".to_owned(),
                    nosqo_id: "@alice".to_owned(),
                    label: "Alice".to_owned(),
                    type_ids: vec!["#Person".to_owned()],
                }],
            }
        );
    }

    #[test]
    fn get_entity_detail_groups_repeated_values_and_uses_predicate_labels() {
        let store = Arc::new(InMemoryStatementStore::new(StatementSet::default()));
        store
            .assert_statements(StatementSet::from(vec![
                Statement::from_strings("~alias", "label", "Alias"),
                Statement::from_strings("samwise_gamgee", "isA", "#Person"),
                Statement::from_strings("samwise_gamgee", "label", "Samwise Gamgee"),
                Statement::from_strings("samwise_gamgee", "alias", "Sam"),
                Statement::from_strings("samwise_gamgee", "alias", "Samwise"),
            ]))
            .expect("test store should accept seed statements");
        let state = ServerState::new(PalHandle::new(PalMock::new()), store);

        let response = state
            .get_entity_detail("samwise_gamgee")
            .expect("detail lookup should succeed")
            .expect("entity should exist");

        assert_eq!(
            response,
            EntityDetailResponse {
                id: "samwise_gamgee".to_owned(),
                nosqo_id: "@samwise_gamgee".to_owned(),
                label: "Samwise Gamgee".to_owned(),
                type_ids: vec!["#Person".to_owned()],
                attributes: vec![
                    EntityDetailAttribute {
                        predicate_id: "~alias".to_owned(),
                        label: "Alias".to_owned(),
                        values: vec!["Sam".to_owned(), "Samwise".to_owned()],
                    },
                    EntityDetailAttribute {
                        predicate_id: "~isA".to_owned(),
                        label: "isA".to_owned(),
                        values: vec!["#Person".to_owned()],
                    },
                    EntityDetailAttribute {
                        predicate_id: "~label".to_owned(),
                        label: "label".to_owned(),
                        values: vec!["Samwise Gamgee".to_owned()],
                    },
                ],
            }
        );
    }
}
