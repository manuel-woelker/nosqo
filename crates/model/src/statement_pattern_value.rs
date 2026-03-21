use serde::{Deserialize, Serialize};

/// A pattern field that either matches a specific value or any value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum StatementPatternValue<T> {
    /// Matches any value.
    Any,
    /// Matches an exact value.
    Exact(T),
}

impl<T> StatementPatternValue<T> {
    /// Returns true if the pattern value matches the provided value.
    pub fn matches(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        match self {
            Self::Any => true,
            Self::Exact(expected) => expected == value,
        }
    }
}
