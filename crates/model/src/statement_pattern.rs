use serde::{Deserialize, Serialize};

use crate::{NodeId, Statement, StatementPatternValue, Value};

/// A pattern for matching statements.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StatementPattern {
    /// The subject match pattern.
    pub subject: StatementPatternValue<NodeId>,
    /// The predicate match pattern.
    pub predicate: StatementPatternValue<NodeId>,
    /// The object match pattern.
    pub object: StatementPatternValue<Value>,
}

impl StatementPattern {
    /// Creates a new statement pattern.
    pub fn new(
        subject: StatementPatternValue<NodeId>,
        predicate: StatementPatternValue<NodeId>,
        object: StatementPatternValue<Value>,
    ) -> Self {
        Self {
            subject,
            predicate,
            object,
        }
    }

    /// Creates a pattern that matches any statement.
    pub fn any() -> Self {
        Self::new(
            StatementPatternValue::Any,
            StatementPatternValue::Any,
            StatementPatternValue::Any,
        )
    }

    /// Returns true if the pattern matches the provided statement.
    pub fn matches(&self, statement: &Statement) -> bool {
        self.subject.matches(&statement.subject)
            && self.predicate.matches(&statement.predicate)
            && self.object.matches(&statement.object)
    }
}

#[cfg(test)]
mod tests {
    use crate::{NodeId, Statement, StatementPattern, StatementPatternValue, Value};

    #[test]
    fn matches_statements_with_exact_and_any_fields() {
        let statement = Statement::value(
            "berlin",
            NodeId::predicate_id("~label").unwrap(),
            Value::text("Berlin"),
        );

        let matching_pattern = StatementPattern::new(
            StatementPatternValue::Exact(NodeId::entity("berlin")),
            StatementPatternValue::Any,
            StatementPatternValue::Exact(Value::text("Berlin")),
        );
        let non_matching_pattern = StatementPattern::new(
            StatementPatternValue::Exact(NodeId::entity("paris")),
            StatementPatternValue::Any,
            StatementPatternValue::Any,
        );

        assert!(matching_pattern.matches(&statement));
        assert!(!non_matching_pattern.matches(&statement));
    }
}
