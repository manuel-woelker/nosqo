use nosqo_base::shared_string::SharedString;
use serde::{Deserialize, Serialize};

use crate::NodeId;

/// A predicate identifier in canonical form.
///
/// The stored value always includes the leading `~` used by the nosqo text
/// format for predicate nodes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PredicateId {
    /// The canonical predicate identifier without the leading `@`.
    value: SharedString,
}

impl PredicateId {
    /// Creates a predicate identifier from canonical text or a bare predicate
    /// name. Bare names are normalized to the `~name` form.
    pub fn new(value: impl Into<SharedString>) -> Self {
        let value: SharedString = value.into();

        if value.starts_with('~') {
            return Self { value };
        }

        Self {
            value: format!("~{}", value).into(),
        }
    }

    /// Returns the canonical predicate identifier text.
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    /// Returns the predicate name without the leading `~`.
    pub fn local_name(&self) -> &str {
        self.value.as_str().trim_start_matches('~')
    }

    /// Returns this predicate as a regular node identifier.
    pub fn as_node_id(&self) -> NodeId {
        NodeId::new(self.value.clone())
    }
}

impl From<&str> for PredicateId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for PredicateId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<SharedString> for PredicateId {
    fn from(value: SharedString) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::PredicateId;

    #[test]
    fn normalizes_bare_predicate_names() {
        let predicate = PredicateId::new("label");

        assert_eq!(predicate.as_str(), "~label");
        assert_eq!(predicate.local_name(), "label");
    }

    #[test]
    fn preserves_canonical_predicate_names() {
        let predicate = PredicateId::new("~isA");

        assert_eq!(predicate.as_str(), "~isA");
        assert_eq!(predicate.local_name(), "isA");
    }
}
