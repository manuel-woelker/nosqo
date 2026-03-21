use serde::{Deserialize, Serialize};

use crate::{NodeId, Value};

/// A single knowledge statement expressed as a subject, predicate, and object.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Statement {
    /// The statement subject.
    pub subject: NodeId,
    /// The statement predicate.
    pub predicate: NodeId,
    /// The statement object.
    pub object: Value,
}

impl Statement {
    /// Creates a new statement from canonical subject, predicate, and object
    /// values.
    pub fn new(
        subject: impl Into<NodeId>,
        predicate: impl Into<NodeId>,
        object: impl Into<Value>,
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
        predicate: impl Into<NodeId>,
        object: impl Into<NodeId>,
    ) -> Self {
        Self::new(subject, predicate, Value::Id(object.into()))
    }

    /// Creates a statement whose object is any supported value.
    pub fn value(
        subject: impl Into<NodeId>,
        predicate: impl Into<NodeId>,
        object: impl Into<Value>,
    ) -> Self {
        Self::new(subject, predicate, object)
    }
}

#[cfg(test)]
mod tests {
    use super::Statement;
    use crate::{NodeId, Value};

    #[test]
    fn creates_id_statements() {
        let statement = Statement::id(
            "berlin",
            NodeId::predicate_id("~isA").unwrap(),
            NodeId::type_name("City"),
        );

        assert_eq!(statement.subject.as_str(), "berlin");
        assert_eq!(statement.predicate.as_str(), "~isA");
        assert_eq!(statement.object, Value::Id(NodeId::type_name("City")));
    }

    #[test]
    fn creates_literal_statements() {
        let statement = Statement::value(
            "berlin",
            NodeId::predicate_id("~label").unwrap(),
            Value::text("Berlin"),
        );

        assert_eq!(statement.subject.as_str(), "berlin");
        assert_eq!(statement.predicate.as_str(), "~label");
        assert_eq!(statement.object, Value::text("Berlin"));
    }
}
