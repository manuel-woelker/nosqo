use serde::Deserialize;
use std::collections::BTreeMap;

/// Structured entity search criteria from the browser UI.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EntitySearchRequest {
    /// Canonical nosqo type id such as `#Person`.
    #[serde(rename = "type")]
    pub entity_type: String,
    /// Exact-match attribute filters keyed by canonical predicate id such as
    /// `~label`.
    pub filters: BTreeMap<String, String>,
}
