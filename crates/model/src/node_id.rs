use nosqo_base::shared_string::SharedString;
use serde::{Deserialize, Serialize};

/// A canonical graph node identifier.
///
/// The stored value matches the text after the `@` sigil in the text format.
/// Examples: `berlin`, `#City`, `~label`, `_k9x2`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NodeId {
    /// The canonical identifier text without the leading `@`.
    value: SharedString,
}

impl NodeId {
    /// Creates a new node identifier from canonical identifier text.
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Creates a subject or object identifier for a regular entity name.
    pub fn entity(name: impl AsRef<str>) -> Self {
        Self::new(name.as_ref())
    }

    /// Creates an identifier for a type such as `#City`.
    pub fn type_name(name: impl AsRef<str>) -> Self {
        Self::new(format!("#{}", name.as_ref()))
    }

    /// Creates an identifier for a predicate node such as `~label`.
    pub fn predicate_name(name: impl AsRef<str>) -> Self {
        Self::new(format!("~{}", name.as_ref()))
    }

    /// Returns the canonical identifier text.
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

impl From<&str> for NodeId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for NodeId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<SharedString> for NodeId {
    fn from(value: SharedString) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::NodeId;

    #[test]
    fn creates_entity_ids() {
        let id = NodeId::entity("berlin");

        assert_eq!(id.as_str(), "berlin");
    }

    #[test]
    fn creates_type_ids() {
        let id = NodeId::type_name("City");

        assert_eq!(id.as_str(), "#City");
    }

    #[test]
    fn creates_predicate_ids() {
        let id = NodeId::predicate_name("label");

        assert_eq!(id.as_str(), "~label");
    }
}
