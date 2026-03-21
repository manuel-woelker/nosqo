use nosqo_base::shared_string::SharedString;
use serde::{Deserialize, Serialize};

use crate::{DateTimeValue, DateValue, DecimalValue, NodeId};

/// A value in the nosqo statement model.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Value {
    /// Another graph node identifier.
    Id(NodeId),
    /// A human-readable text value.
    Text(SharedString),
    /// An identifier-like symbol literal.
    Symbol(SharedString),
    /// A whole number value.
    Integer(i64),
    /// A decimal number stored losslessly as canonical text.
    Decimal(DecimalValue),
    /// A calendar date stored as canonical text.
    Date(DateValue),
    /// A date-time stored as canonical text.
    DateTime(DateTimeValue),
    /// A true or false value.
    Boolean(bool),
}

impl Value {
    /// Creates an identifier value.
    pub fn id(value: impl Into<NodeId>) -> Self {
        Self::Id(value.into())
    }

    /// Creates a text literal.
    pub fn text(value: impl Into<SharedString>) -> Self {
        Self::Text(value.into())
    }

    /// Creates a symbol literal.
    pub fn symbol(value: impl Into<SharedString>) -> Self {
        Self::Symbol(value.into())
    }
}

impl From<NodeId> for Value {
    fn from(value: NodeId) -> Self {
        Self::Id(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Value;
    use crate::{DateTimeValue, DateValue, DecimalValue, NodeId};

    #[test]
    fn models_the_supported_literal_kinds() {
        assert_eq!(Value::id("berlin"), Value::Id(NodeId::entity("berlin")));
        assert_eq!(Value::text("Berlin"), Value::Text("Berlin".into()));
        assert_eq!(
            Value::symbol("capital_of"),
            Value::Symbol("capital_of".into())
        );
        assert_eq!(Value::Integer(42), Value::Integer(42));
        assert_eq!(
            Value::Decimal(DecimalValue::new("3.14")),
            Value::Decimal(DecimalValue::new("3.14"))
        );
        assert_eq!(
            Value::Date(DateValue::new("2026-03-21")),
            Value::Date(DateValue::new("2026-03-21"))
        );
        assert_eq!(
            Value::DateTime(DateTimeValue::new("2026-03-21T12:00:00Z")),
            Value::DateTime(DateTimeValue::new("2026-03-21T12:00:00Z"))
        );
        assert_eq!(Value::Boolean(true), Value::Boolean(true));
    }
}
