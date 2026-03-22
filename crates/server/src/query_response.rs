use nosqo_model::NqlQueryResult;
use serde::Serialize;

/// The HTTP response payload for an NQL query.
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct QueryResponse {
    /// The projected variables in output order.
    pub columns: Vec<String>,
    /// The projected rows in output order.
    pub rows: Vec<Vec<String>>,
}

impl QueryResponse {
    /// Creates a response payload from an executed query result.
    pub fn from_query_result(result: NqlQueryResult) -> Self {
        Self {
            columns: result
                .columns
                .into_iter()
                .map(|column| format!("?{}", column.as_str()))
                .collect(),
            rows: result
                .rows
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|value| value.to_nosqo_string())
                        .collect()
                })
                .collect(),
        }
    }
}
