use serde::{Deserialize, Serialize};

use super::{NqlPattern, NqlReturn};

/// An NQL query AST.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NqlQuery {
    /// The triple patterns in the match block.
    pub patterns: Vec<NqlPattern>,
    /// The return projection.
    pub return_spec: NqlReturn,
}

impl NqlQuery {
    /// Creates a new NQL query.
    pub fn new(patterns: Vec<NqlPattern>, return_spec: NqlReturn) -> Self {
        Self {
            patterns,
            return_spec,
        }
    }
}
