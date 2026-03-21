use serde::{Deserialize, Serialize};

use crate::{NodeId, PredicateId, StatementObject, Value};

/// A single knowledge statement expressed as a subject, predicate, and object.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Statement {
    /// The statement subject.
    pub subject: NodeId,
    /// The statement predicate.
    pub predicate: PredicateId,
    /// The statement object.
    pub object: StatementObject,
}

impl Statement {
    /// Creates a new statement from canonical subject, predicate, and object
    /// values.
    pub fn new(
        subject: impl Into<NodeId>,
        predicate: impl Into<PredicateId>,
        object: impl Into<StatementObject>,
    ) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
        }
    }

    /// Creates a statement whose object is another node identifier.
    pub fn id(
        subject: impl Into<NodeId>,
        predicate: impl Into<PredicateId>,
        object: impl Into<NodeId>,
    ) -> Self {
        Self::new(subject, predicate, StatementObject::Id(object.into()))
    }

    /// Creates a statement whose object is a literal value.
    pub fn value(
        subject: impl Into<NodeId>,
        predicate: impl Into<PredicateId>,
        object: impl Into<Value>,
    ) -> Self {
        Self::new(subject, predicate, StatementObject::Value(object.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::Statement;
    use crate::{NodeId, StatementObject, Value};

    #[test]
    fn creates_id_statements() {
        let statement = Statement::id("berlin", "isA", NodeId::type_name("City"));

        assert_eq!(statement.subject.as_str(), "berlin");
        assert_eq!(statement.predicate.as_str(), "~isA");
        assert_eq!(
            statement.object,
            StatementObject::Id(NodeId::type_name("City"))
        );
    }

    #[test]
    fn creates_literal_statements() {
        let statement = Statement::value("berlin", "label", Value::text("Berlin"));

        assert_eq!(statement.subject.as_str(), "berlin");
        assert_eq!(statement.predicate.as_str(), "~label");
        assert_eq!(
            statement.object,
            StatementObject::Value(Value::text("Berlin"))
        );
    }
}
