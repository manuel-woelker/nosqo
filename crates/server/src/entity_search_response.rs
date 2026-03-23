use crate::entity_search_result::EntitySearchResult;
use serde::Serialize;

/// Entity search response payload.
#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EntitySearchResponse {
    /// Matching entities in scan-friendly order.
    pub results: Vec<EntitySearchResult>,
}
