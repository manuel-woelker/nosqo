use serde::{Deserialize, Serialize};

use super::NqlTerm;

/// A triple pattern in an NQL match block.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NqlPattern {
    /// The subject term.
    pub subject: NqlTerm,
    /// The predicate term.
    pub predicate: NqlTerm,
    /// The object term.
    pub object: NqlTerm,
}

impl NqlPattern {
    /// Creates a new NQL triple pattern.
    pub fn new(subject: NqlTerm, predicate: NqlTerm, object: NqlTerm) -> Self {
        Self {
            subject,
            predicate,
            object,
        }
    }
}
