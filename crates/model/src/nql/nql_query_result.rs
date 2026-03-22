use serde::{Deserialize, Serialize};

use super::{NqlBindingValue, NqlVariable};

/// A projected NQL query result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NqlQueryResult {
    /// The projected variables in output order.
    pub columns: Vec<NqlVariable>,
    /// The projected rows in row order.
    pub rows: Vec<Vec<NqlBindingValue>>,
}

impl NqlQueryResult {
    /// Creates a projected NQL query result.
    pub fn new(columns: Vec<NqlVariable>, rows: Vec<Vec<NqlBindingValue>>) -> Self {
        Self { columns, rows }
    }
}
