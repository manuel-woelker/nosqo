use nosqo_base::{Mutex, result::NosqoResult};
use nosqo_model::{StatementPattern, StatementSet};

use super::StatementStore;

/// An in-memory statement store backed by a mutex-protected statement set.
pub struct InMemoryStatementStore {
    statement_set: Mutex<StatementSet>,
}

impl InMemoryStatementStore {
    /// Creates a new in-memory store with an initial statement set.
    pub fn new(statement_set: StatementSet) -> Self {
        Self {
            statement_set: Mutex::new(statement_set),
        }
    }
}

impl Default for InMemoryStatementStore {
    fn default() -> Self {
        Self::new(StatementSet::default())
    }
}

impl StatementStore for InMemoryStatementStore {
    fn assert_statements(&self, statement_set: StatementSet) -> NosqoResult<()> {
        let mut stored_statement_set = self.statement_set.lock();

        for statement in statement_set.statements {
            if !stored_statement_set.statements.contains(&statement) {
                stored_statement_set.statements.push(statement);
            }
        }

        Ok(())
    }

    fn find_statements(&self, pattern: &StatementPattern) -> NosqoResult<StatementSet> {
        let statements = self
            .statement_set
            .lock()
            .statements
            .iter()
            .filter(|statement| pattern.matches(statement))
            .cloned()
            .collect();

        Ok(StatementSet::new(statements))
    }
}

#[cfg(test)]
mod tests {
    use nosqo_model::{
        NodeId, Statement, StatementPattern, StatementPatternValue, StatementSet, Value,
    };

    use super::InMemoryStatementStore;
    use crate::store::StatementStore;

    #[test]
    fn finds_matching_statements() {
        let store = InMemoryStatementStore::new(StatementSet::new(vec![Statement::value(
            "berlin",
            NodeId::predicate_id("~label").unwrap(),
            Value::text("Berlin"),
        )]));

        let statement_set = store
            .find_statements(&StatementPattern::new(
                StatementPatternValue::Exact(NodeId::entity("berlin")),
                StatementPatternValue::Any,
                StatementPatternValue::Any,
            ))
            .unwrap();

        assert_eq!(
            statement_set,
            StatementSet::new(vec![Statement::value(
                "berlin",
                NodeId::predicate_id("~label").unwrap(),
                Value::text("Berlin"),
            )])
        );
    }

    #[test]
    fn asserts_new_statements_without_duplicate_inserts() {
        let store = InMemoryStatementStore::default();
        let statement_set = StatementSet::new(vec![Statement::id(
            "berlin",
            NodeId::predicate_id("~isA").unwrap(),
            NodeId::type_name("City"),
        )]);

        store.assert_statements(statement_set.clone()).unwrap();
        store.assert_statements(statement_set.clone()).unwrap();

        assert_eq!(
            store.find_statements(&StatementPattern::any()).unwrap(),
            statement_set
        );
    }
}
