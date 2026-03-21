use nosqo_base::shared_string::SharedString;
use serde::{Deserialize, Serialize};

use crate::{DateTimeValue, DateValue, DecimalValue, NodeId, Value};

/// A single knowledge statement expressed as a subject, predicate, and object.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Statement {
    /// The statement subject.
    pub subject: NodeId,
    /// The statement predicate.
    pub predicate: NodeId,
    /// The statement object.
    pub object: Value,
}

impl Statement {
    /// Creates a new statement from canonical subject, predicate, and object
    /// values.
    pub fn new(
        subject: impl Into<NodeId>,
        predicate: impl Into<NodeId>,
        object: impl Into<Value>,
    ) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
        }
    }

    /// Creates a statement whose object is another node identifier.
    pub fn id(
        subject: impl Into<NodeId>,
        predicate: impl Into<NodeId>,
        object: impl Into<NodeId>,
    ) -> Self {
        Self::new(subject, predicate, Value::Id(object.into()))
    }

    /// Creates a statement whose object is any supported value.
    pub fn value(
        subject: impl Into<NodeId>,
        predicate: impl Into<NodeId>,
        object: impl Into<Value>,
    ) -> Self {
        Self::new(subject, predicate, object)
    }

    /// Creates a statement from string-like inputs.
    ///
    /// Values are normalized using the nosqo statement conventions for
    /// subjects, predicates, and objects.
    pub fn from_strings(
        subject: impl Into<SharedString>,
        predicate: impl Into<SharedString>,
        object: impl Into<SharedString>,
    ) -> Self {
        let subject: SharedString = subject.into();
        let predicate: SharedString = predicate.into();
        let object: SharedString = object.into();

        Self::new(
            parse_node_id(subject, StatementPosition::Subject),
            parse_node_id(predicate, StatementPosition::Predicate),
            parse_value(object),
        )
    }
}

enum StatementPosition {
    Subject,
    Predicate,
}

fn parse_node_id(value: SharedString, position: StatementPosition) -> NodeId {
    if let Some(id) = value.as_str().strip_prefix('@') {
        return NodeId::new(id);
    }

    if value.starts_with('#') || value.starts_with('~') {
        return NodeId::new(value);
    }

    match position {
        StatementPosition::Subject => NodeId::entity(value),
        StatementPosition::Predicate => NodeId::predicate_name(value),
    }
}

fn parse_value(value: SharedString) -> Value {
    if let Some(text) = parse_quoted_value(value.as_str(), '"') {
        return Value::text(text);
    }

    if let Some(symbol) = parse_quoted_value(value.as_str(), '\'') {
        return Value::symbol(symbol);
    }

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

fn parse_quoted_value(value: &str, quote: char) -> Option<String> {
    if !(value.starts_with(quote) && value.ends_with(quote) && value.len() >= 2) {
        return None;
    }

    let inner = &value[quote.len_utf8()..value.len() - quote.len_utf8()];
    let mut unescaped = String::new();
    let mut chars = inner.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            unescaped.push(ch);
            continue;
        }

        let escaped = chars.next()?;
        unescaped.push(match escaped {
            '\\' => '\\',
            '"' => '"',
            '\'' => '\'',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            other => other,
        });
    }

    Some(unescaped)
}

#[cfg(test)]
mod tests {
    use super::Statement;
    use crate::{NodeId, Value};

    #[test]
    fn creates_id_statements() {
        let statement = Statement::id(
            "berlin",
            NodeId::predicate_id("~isA").unwrap(),
            NodeId::type_name("City"),
        );

        assert_eq!(statement.subject.as_str(), "berlin");
        assert_eq!(statement.predicate.as_str(), "~isA");
        assert_eq!(statement.object, Value::Id(NodeId::type_name("City")));
    }

    #[test]
    fn creates_literal_statements() {
        let statement = Statement::value(
            "berlin",
            NodeId::predicate_id("~label").unwrap(),
            Value::text("Berlin"),
        );

        assert_eq!(statement.subject.as_str(), "berlin");
        assert_eq!(statement.predicate.as_str(), "~label");
        assert_eq!(statement.object, Value::text("Berlin"));
    }

    #[test]
    fn creates_statements_from_strings() {
        let statement = Statement::from_strings("berlin", "label", "@germany");
        let literal_statement = Statement::from_strings("berlin", "population", "i42");
        let symbol_statement = Statement::from_strings("berlin", "speaks", "'de'");

        assert_eq!(statement.subject, NodeId::entity("berlin"));
        assert_eq!(statement.predicate, NodeId::predicate_name("label"));
        assert_eq!(statement.object, Value::id("germany"));

        assert_eq!(literal_statement.subject, NodeId::entity("berlin"));
        assert_eq!(
            literal_statement.predicate,
            NodeId::predicate_name("population")
        );
        assert_eq!(literal_statement.object, Value::Integer(42));
        assert_eq!(symbol_statement.object, Value::symbol("de"));
    }
}
