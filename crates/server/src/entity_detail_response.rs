use crate::entity_detail_attribute::EntityDetailAttribute;
use serde::Serialize;

/// Detailed read model for a single entity.
#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EntityDetailResponse {
    /// Stable subject id without the `@` prefix.
    pub id: String,
    /// Stable nosqo subject id such as `@alice`.
    pub nosqo_id: String,
    /// Human-readable label for the entity.
    pub label: String,
    /// Exact type ids asserted for the entity.
    pub type_ids: Vec<String>,
    /// Grouped attributes for the detail pane.
    pub attributes: Vec<EntityDetailAttribute>,
}
