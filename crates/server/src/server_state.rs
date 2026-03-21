use nosqo_base::result::NosqoResult;
use nosqo_engine::{InMemoryStatementStore, StatementStore};
use nosqo_model::StatementPattern;
use nosqo_pal::pal::PalHandle;
use serde_json::{Value, json};
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
    pub fn info(&self) -> Value {
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
}
