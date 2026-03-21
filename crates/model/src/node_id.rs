use nosqo_base::{err, result::NosqoResult, shared_string::SharedString};
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

    /// Creates a predicate identifier from canonical predicate-id text.
    ///
    /// Valid predicate identifiers must start with `~` and contain at least one
    /// character after the sigil.
    pub fn predicate_id(value: impl Into<SharedString>) -> NosqoResult<Self> {
        let value: SharedString = value.into();

        if !value.starts_with('~') {
            return Err(err!(
                "invalid predicate id `{}`: predicate ids must start with `~`",
                value
            ));
        }

        if value.len() == 1 {
            return Err(err!(
                "invalid predicate id `{}`: predicate ids must include a name after `~`",
                value
            ));
        }

        Ok(Self::new(value))
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

    #[test]
    fn validates_canonical_predicate_ids() {
        let id = NodeId::predicate_id("~label").unwrap();

        assert_eq!(id.as_str(), "~label");
    }

    #[test]
    fn rejects_predicate_ids_without_tilde_prefix() {
        let error = NodeId::predicate_id("label").unwrap_err();

        assert!(
            error
                .to_test_string()
                .contains("predicate ids must start with `~`")
        );
    }

    #[test]
    fn rejects_empty_predicate_ids() {
        let error = NodeId::predicate_id("~").unwrap_err();

        assert!(
            error
                .to_test_string()
                .contains("predicate ids must include a name after `~`")
        );
    }
}
