use serde::{Deserialize, Serialize};

use crate::{NodeId, Value};

/// A bound value for an NQL variable.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NqlBindingValue {
    /// An identifier binding.
    Id(NodeId),
    /// A scalar or object value binding.
    Value(Value),
}

impl NqlBindingValue {
    /// Renders the binding using nosqo/NQL term syntax.
    pub fn to_nosqo_string(&self) -> String {
        match self {
            Self::Id(id) => id.to_nosqo_string(),
            Self::Value(value) => value.to_nosqo_string(),
        }
    }
}

impl From<NodeId> for NqlBindingValue {
    fn from(value: NodeId) -> Self {
        Self::Id(value)
    }
}

impl From<Value> for NqlBindingValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Id(id) => Self::Id(id),
            other => Self::Value(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NqlBindingValue;
    use crate::{NodeId, Value};

    #[test]
    fn normalizes_identifier_values_into_id_bindings() {
        assert_eq!(
            NqlBindingValue::from(Value::id("berlin")),
            NqlBindingValue::Id(NodeId::entity("berlin"))
        );
    }

    #[test]
    fn renders_binding_values_with_nosqo_syntax() {
        assert_eq!(
            NqlBindingValue::Id(NodeId::entity("berlin")).to_nosqo_string(),
            "@berlin"
        );
        assert_eq!(
            NqlBindingValue::Value(Value::text("Berlin")).to_nosqo_string(),
            "\"Berlin\""
        );
    }
}
