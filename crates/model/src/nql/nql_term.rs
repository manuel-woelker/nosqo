use serde::{Deserialize, Serialize};

use crate::{NodeId, Value};

use super::NqlVariable;

/// A term in an NQL triple pattern.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NqlTerm {
    /// A query variable.
    Variable(NqlVariable),
    /// A concrete identifier.
    Id(NodeId),
    /// A concrete literal or identifier value.
    Value(Value),
}

impl NqlTerm {
    /// Creates a variable term.
    pub fn variable(name: impl Into<NqlVariable>) -> Self {
        Self::Variable(name.into())
    }

    /// Creates an identifier term.
    pub fn id(id: impl Into<NodeId>) -> Self {
        Self::Id(id.into())
    }

    /// Creates a value term.
    pub fn value(value: impl Into<Value>) -> Self {
        Self::Value(value.into())
    }
}
