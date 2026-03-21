use serde::{Deserialize, Serialize};

use crate::Statement;

/// A collection of statements.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StatementSet {
    /// The statements in the set.
    pub statements: Vec<Statement>,
}

impl StatementSet {
    /// Creates a statement set from a vector of statements.
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    /// Returns the statements as a slice.
    pub fn as_slice(&self) -> &[Statement] {
        &self.statements
    }

    /// Appends a statement to the set.
    pub fn push(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

impl From<Vec<Statement>> for StatementSet {
    fn from(statements: Vec<Statement>) -> Self {
        Self::new(statements)
    }
}

#[cfg(test)]
mod tests {
    use super::StatementSet;
    use crate::{NodeId, Statement, Value};

    #[test]
    fn creates_statement_sets_from_vectors() {
        let statements = vec![Statement::value(
            "berlin",
            NodeId::predicate_id("~label").unwrap(),
            Value::text("Berlin"),
        )];

        let statement_set = StatementSet::new(statements.clone());

        assert_eq!(statement_set.as_slice(), statements.as_slice());
    }

    #[test]
    fn appends_statements() {
        let mut statement_set = StatementSet::default();
        let statement = Statement::id(
            "berlin",
            NodeId::predicate_id("~isA").unwrap(),
            NodeId::type_name("City"),
        );

        statement_set.push(statement.clone());

        assert_eq!(statement_set.as_slice(), &[statement]);
    }
}
