use nosqo_base::shared_string::SharedString;
use serde::{Deserialize, Serialize};

/// A variable in an NQL query.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NqlVariable {
    /// The variable name without the leading `?`.
    pub name: SharedString,
}

impl NqlVariable {
    /// Creates a new variable from its local name.
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self { name: name.into() }
    }

    /// Returns the variable name without the leading `?`.
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}

impl From<&str> for NqlVariable {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
