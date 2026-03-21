use nosqo_base::shared_string::SharedString;
use serde::{Deserialize, Serialize};

/// An ISO-8601 date-time literal as represented by the nosqo text format.
///
/// The model stores the original textual representation instead of eagerly
/// parsing into a timestamp type so that formatting remains lossless.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DateTimeValue {
    /// The date-time value without the text-format `t` prefix.
    value: SharedString,
}

impl DateTimeValue {
    /// Creates a new date-time value from its canonical text representation.
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the canonical date-time text.
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

impl From<&str> for DateTimeValue {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for DateTimeValue {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::DateTimeValue;

    #[test]
    fn stores_date_time_text_without_modification() {
        let value = DateTimeValue::new("2026-03-21T12:00:00Z");

        assert_eq!(value.as_str(), "2026-03-21T12:00:00Z");
    }
}
