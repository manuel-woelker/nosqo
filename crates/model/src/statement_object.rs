use serde::{Deserialize, Serialize};

use crate::{NodeId, Value};

/// The object position of a statement.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum StatementObject {
    /// Another graph node identifier.
    Id(NodeId),
    /// A typed literal value.
    Value(Value),
}

impl From<NodeId> for StatementObject {
    fn from(value: NodeId) -> Self {
        Self::Id(value)
    }
}

impl From<Value> for StatementObject {
    fn from(value: Value) -> Self {
        Self::Value(value)
    }
}
