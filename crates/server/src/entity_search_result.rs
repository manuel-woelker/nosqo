use serde::Serialize;

/// One row in an entity search result set.
#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EntitySearchResult {
    /// Stable subject id without the `@` prefix.
    pub id: String,
    /// Stable nosqo subject id such as `@alice`.
    pub nosqo_id: String,
    /// Human-readable label for scanning.
    pub label: String,
    /// Exact type ids asserted for the entity.
    pub type_ids: Vec<String>,
}
