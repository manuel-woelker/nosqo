use nosqo_base::shared_string::SharedString;
use serde::{Deserialize, Serialize};

/// An ISO-8601 calendar date literal as represented by the nosqo text format.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DateValue {
    /// The date value without the text-format `d` prefix.
    value: SharedString,
}

impl DateValue {
    /// Creates a new date value from its canonical text representation.
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the canonical date text.
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

impl From<&str> for DateValue {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for DateValue {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::DateValue;

    #[test]
    fn stores_date_text_without_modification() {
        let value = DateValue::new("2026-03-21");

        assert_eq!(value.as_str(), "2026-03-21");
    }
}
