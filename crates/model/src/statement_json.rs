use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{NodeId, StatementSet, Value};

/// Versioned compact JSON transport for nosqo statements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatementJsonDocument {
    /// Stable format identifier for the transport.
    pub format: String,
    /// Indexed value table shared across all statement rows.
    pub values: Vec<StatementJsonValue>,
    /// Statement rows containing subject, predicate, and one or more objects.
    pub statements: Vec<Vec<usize>>,
}

impl StatementJsonDocument {
    pub const FORMAT: &str = "nosqo-statement-json-v1";

    /// Creates a document with the v1 format identifier.
    pub fn new(values: Vec<StatementJsonValue>, statements: Vec<Vec<usize>>) -> Self {
        Self {
            format: Self::FORMAT.to_owned(),
            values,
            statements,
        }
    }
}

/// A single value table entry in the compact JSON transport.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatementJsonValue {
    /// A nosqo-syntax token such as `#Type`, `~label`, `i42`, or `T`.
    NosqoToken(String),
    /// Plain text encoded explicitly to avoid ambiguity with nosqo tokens.
    Text([String; 1]),
}

impl StatementJsonValue {
    fn from_subject_or_predicate(id: &NodeId) -> Self {
        Self::NosqoToken(id.to_nosqo_string())
    }

    fn from_object(value: &Value) -> Self {
        match value {
            Value::Text(text) => Self::Text([text.as_str().to_owned()]),
            other => Self::NosqoToken(other.to_nosqo_string()),
        }
    }
}

impl StatementSet {
    /// Emits the statement set as compact indexed JSON transport.
    pub fn to_statement_json(&self) -> StatementJsonDocument {
        let mut sorted_statements = self.statements.clone();
        sorted_statements.sort();

        let mut values = Vec::new();
        let mut indexes = BTreeMap::new();
        let mut rows: Vec<Vec<usize>> = Vec::new();

        for statement in &sorted_statements {
            let subject_index = intern_value(
                StatementJsonValue::from_subject_or_predicate(&statement.subject),
                &mut values,
                &mut indexes,
            );
            let predicate_index = intern_value(
                StatementJsonValue::from_subject_or_predicate(&statement.predicate),
                &mut values,
                &mut indexes,
            );
            let object_index = intern_value(
                StatementJsonValue::from_object(&statement.object),
                &mut values,
                &mut indexes,
            );

            let should_append_to_current_row = rows.last().is_some_and(|current_row| {
                current_row.first() == Some(&subject_index)
                    && current_row.get(1) == Some(&predicate_index)
            });

            if should_append_to_current_row {
                rows.last_mut()
                    .expect("checked current row presence")
                    .push(object_index);
            } else {
                rows.push(vec![subject_index, predicate_index, object_index]);
            }
        }

        StatementJsonDocument::new(values, rows)
    }
}

fn intern_value(
    value: StatementJsonValue,
    values: &mut Vec<StatementJsonValue>,
    indexes: &mut BTreeMap<StatementJsonValue, usize>,
) -> usize {
    if let Some(index) = indexes.get(&value) {
        return *index;
    }

    let index = values.len();
    values.push(value.clone());
    indexes.insert(value, index);
    index
}

#[cfg(test)]
mod tests {
    use crate::{Statement, StatementSet};

    use super::{StatementJsonDocument, StatementJsonValue};

    #[test]
    fn emits_compact_indexed_statement_json() {
        let statement_set = StatementSet::from(vec![
            Statement::from_strings("#Person", "isA", "#Type"),
            Statement::from_strings("#Person", "label", "Person"),
            Statement::from_strings("#Person", "attribute", "~description"),
            Statement::from_strings("#Person", "attribute", "~label"),
            Statement::from_strings("~label", "targetType", "#String"),
            Statement::from_strings("~label", "isA", "#Predicate"),
        ]);

        let emitted = statement_set.to_statement_json();

        assert_eq!(
            emitted,
            StatementJsonDocument::new(
                vec![
                    StatementJsonValue::NosqoToken("#Person".to_owned()),
                    StatementJsonValue::NosqoToken("~attribute".to_owned()),
                    StatementJsonValue::NosqoToken("~description".to_owned()),
                    StatementJsonValue::NosqoToken("~label".to_owned()),
                    StatementJsonValue::NosqoToken("~isA".to_owned()),
                    StatementJsonValue::NosqoToken("#Type".to_owned()),
                    StatementJsonValue::Text(["Person".to_owned()]),
                    StatementJsonValue::NosqoToken("#Predicate".to_owned()),
                    StatementJsonValue::NosqoToken("~targetType".to_owned()),
                    StatementJsonValue::NosqoToken("#String".to_owned()),
                ],
                vec![
                    vec![0, 1, 2, 3],
                    vec![0, 4, 5],
                    vec![0, 3, 6],
                    vec![3, 4, 7],
                    vec![3, 8, 9],
                ],
            )
        );
    }

    #[test]
    fn wraps_plain_text_values_but_keeps_nosqo_literals_as_tokens() {
        let statement_set = StatementSet::from(vec![
            Statement::from_strings("berlin", "label", "Berlin"),
            Statement::from_strings("berlin", "population", "i3769000"),
            Statement::from_strings("berlin", "isCapital", "T"),
        ]);

        let emitted = statement_set.to_statement_json();

        assert!(
            emitted
                .values
                .contains(&StatementJsonValue::Text(["Berlin".to_owned()]))
        );
        assert!(
            emitted
                .values
                .contains(&StatementJsonValue::NosqoToken("i3769000".to_owned()))
        );
        assert!(
            emitted
                .values
                .contains(&StatementJsonValue::NosqoToken("T".to_owned()))
        );
    }
}
