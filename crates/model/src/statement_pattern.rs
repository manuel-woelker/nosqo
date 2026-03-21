use nosqo_base::shared_string::SharedString;
use serde::{Deserialize, Serialize};

use crate::{
    DateTimeValue, DateValue, DecimalValue, NodeId, Statement, StatementPatternValue, Value,
};

/// A pattern for matching statements.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StatementPattern {
    /// The subject match pattern.
    pub subject: StatementPatternValue<NodeId>,
    /// The predicate match pattern.
    pub predicate: StatementPatternValue<NodeId>,
    /// The object match pattern.
    pub object: StatementPatternValue<Value>,
}

impl StatementPattern {
    /// Creates a new statement pattern.
    pub fn new(
        subject: StatementPatternValue<NodeId>,
        predicate: StatementPatternValue<NodeId>,
        object: StatementPatternValue<Value>,
    ) -> Self {
        Self {
            subject,
            predicate,
            object,
        }
    }

    /// Creates a pattern that matches any statement.
    pub fn any() -> Self {
        Self::new(
            StatementPatternValue::Any,
            StatementPatternValue::Any,
            StatementPatternValue::Any,
        )
    }

    /// Creates a statement pattern from string-like inputs.
    ///
    /// The `*` token maps to `Any`. Other values are normalized using the
    /// nosqo statement conventions for subjects, predicates, and objects.
    pub fn from_strings(
        subject: impl Into<SharedString>,
        predicate: impl Into<SharedString>,
        object: impl Into<SharedString>,
    ) -> Self {
        let subject: SharedString = subject.into();
        let predicate: SharedString = predicate.into();
        let object: SharedString = object.into();

        Self::new(
            parse_subject_pattern_value(subject),
            parse_predicate_pattern_value(predicate),
            parse_object_pattern_value(object),
        )
    }

    /// Returns true if the pattern matches the provided statement.
    pub fn matches(&self, statement: &Statement) -> bool {
        self.subject.matches(&statement.subject)
            && self.predicate.matches(&statement.predicate)
            && self.object.matches(&statement.object)
    }
}

fn parse_subject_pattern_value(value: SharedString) -> StatementPatternValue<NodeId> {
    if value == "*" {
        return StatementPatternValue::Any;
    }

    StatementPatternValue::Exact(parse_node_id(value, PatternPosition::Subject))
}

fn parse_predicate_pattern_value(value: SharedString) -> StatementPatternValue<NodeId> {
    if value == "*" {
        return StatementPatternValue::Any;
    }

    StatementPatternValue::Exact(parse_node_id(value, PatternPosition::Predicate))
}

fn parse_object_pattern_value(value: SharedString) -> StatementPatternValue<Value> {
    if value == "*" {
        return StatementPatternValue::Any;
    }

    StatementPatternValue::Exact(parse_value(value))
}

enum PatternPosition {
    Subject,
    Predicate,
}

fn parse_node_id(value: SharedString, position: PatternPosition) -> NodeId {
    if let Some(id) = value.as_str().strip_prefix('@') {
        return NodeId::new(id);
    }

    if value.starts_with('#') || value.starts_with('~') {
        return NodeId::new(value);
    }

    match position {
        PatternPosition::Subject => NodeId::entity(value),
        PatternPosition::Predicate => NodeId::predicate_name(value),
    }
}

fn parse_value(value: SharedString) -> Value {
    if value == "T" {
        return Value::Boolean(true);
    }

    if value == "F" {
        return Value::Boolean(false);
    }

    if let Some(id) = value.as_str().strip_prefix('@') {
        return Value::id(NodeId::entity(id));
    }

    if value.starts_with('#') || value.starts_with('~') {
        return Value::id(NodeId::new(value));
    }

    if is_integer_literal(value.as_str()) {
        return Value::Integer(
            value.as_str()[1..]
                .parse()
                .expect("validated integer literal"),
        );
    }

    if is_decimal_literal(value.as_str()) {
        return Value::Decimal(DecimalValue::new(&value.as_str()[1..]));
    }

    if is_date_literal(value.as_str()) {
        return Value::Date(DateValue::new(&value.as_str()[1..]));
    }

    if is_date_time_literal(value.as_str()) {
        return Value::DateTime(DateTimeValue::new(&value.as_str()[1..]));
    }

    Value::text(value)
}

fn is_integer_literal(value: &str) -> bool {
    value.starts_with('i')
        && value.len() > 1
        && value[1..]
            .chars()
            .enumerate()
            .all(|(index, ch)| ch.is_ascii_digit() || (index == 0 && ch == '-'))
}

fn is_decimal_literal(value: &str) -> bool {
    value.starts_with('n')
        && value.len() > 1
        && value[1..]
            .chars()
            .enumerate()
            .all(|(index, ch)| ch.is_ascii_digit() || ch == '.' || (index == 0 && ch == '-'))
}

fn is_date_literal(value: &str) -> bool {
    value.starts_with('d')
        && value.len() > 1
        && value[1..]
            .chars()
            .all(|ch| ch.is_ascii_digit() || ch == '-')
}

fn is_date_time_literal(value: &str) -> bool {
    value.starts_with('t') && value.len() > 1
}

#[cfg(test)]
mod tests {
    use crate::{NodeId, Statement, StatementPattern, StatementPatternValue, Value};

    #[test]
    fn matches_statements_with_exact_and_any_fields() {
        let statement = Statement::value(
            "berlin",
            NodeId::predicate_id("~label").unwrap(),
            Value::text("Berlin"),
        );

        let matching_pattern = StatementPattern::new(
            StatementPatternValue::Exact(NodeId::entity("berlin")),
            StatementPatternValue::Any,
            StatementPatternValue::Exact(Value::text("Berlin")),
        );
        let non_matching_pattern = StatementPattern::new(
            StatementPatternValue::Exact(NodeId::entity("paris")),
            StatementPatternValue::Any,
            StatementPatternValue::Any,
        );

        assert!(matching_pattern.matches(&statement));
        assert!(!non_matching_pattern.matches(&statement));
    }

    #[test]
    fn creates_patterns_from_strings_and_maps_star_to_any() {
        let pattern = StatementPattern::from_strings("berlin", "label", "*");

        assert_eq!(
            pattern,
            StatementPattern::new(
                StatementPatternValue::Exact(NodeId::entity("berlin")),
                StatementPatternValue::Exact(NodeId::predicate_name("label")),
                StatementPatternValue::Any,
            )
        );
    }

    #[test]
    fn parses_object_strings_using_statement_conventions() {
        let id_pattern = StatementPattern::from_strings("*", "*", "@germany");
        let text_pattern = StatementPattern::from_strings("*", "*", "Berlin");

        assert_eq!(
            id_pattern.object,
            StatementPatternValue::Exact(Value::id("germany"))
        );
        assert_eq!(
            text_pattern.object,
            StatementPatternValue::Exact(Value::text("Berlin"))
        );
    }
}
