use serde::Serialize;

/// Grouped attribute values for an entity detail view.
#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EntityDetailAttribute {
    /// Canonical predicate id such as `~label`.
    pub predicate_id: String,
    /// Human-readable predicate label.
    pub label: String,
    /// All stored values for this predicate.
    pub values: Vec<String>,
}
