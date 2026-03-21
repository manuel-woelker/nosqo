use nosqo_base::shared_string::SharedString;
use serde::{Deserialize, Serialize};

/// A decimal literal as represented by the nosqo text format.
///
/// Decimals are stored textually to avoid lossy floating-point conversions in
/// the core model.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DecimalValue {
    /// The decimal value without the text-format `n` prefix.
    value: SharedString,
}

impl DecimalValue {
    /// Creates a new decimal value from its canonical text representation.
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the canonical decimal text.
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

impl From<&str> for DecimalValue {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for DecimalValue {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::DecimalValue;

    #[test]
    fn stores_decimal_text_without_modification() {
        let value = DecimalValue::new("3.14");

        assert_eq!(value.as_str(), "3.14");
    }
}
