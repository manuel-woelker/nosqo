use nosqo_base::{err, result::NosqoResult};
use nosqo_model::{
    DateValue, DecimalValue, NodeId, NqlPattern, NqlQuery, NqlReturn, NqlTerm, NqlVariable, Value,
};

use super::{nql_token::NqlToken, nql_token_kind::NqlTokenKind};

/// A recursive descent parser for NQL v1.
pub struct NqlParser {
    tokens: Vec<NqlToken>,
    cursor: usize,
}

enum TermPosition {
    Subject,
    Predicate,
    Object,
}

impl NqlParser {
    /// Parses an NQL query string.
    pub fn parse_str(input: &str) -> NosqoResult<NqlQuery> {
        let tokens = lex_nql(input)?;
        let mut parser = Self { tokens, cursor: 0 };
        parser.parse_query()
    }

    fn parse_query(&mut self) -> NosqoResult<NqlQuery> {
        self.consume_newlines();
        self.expect_keyword("match")?;
        self.consume_newlines();

        let mut patterns = Vec::new();
        while !self.is_keyword("return") {
            if self.is_eof() {
                return Err(err!("expected `return` block"));
            }
            patterns.push(self.parse_pattern()?);
        }

        if patterns.is_empty() {
            return Err(err!("query must contain at least one pattern"));
        }

        self.expect_keyword("return")?;
        self.consume_newlines();
        let return_spec = self.parse_return_spec()?;
        self.consume_newlines();

        if !self.is_eof() {
            return Err(err!(
                "unexpected trailing token at byte offset {}",
                self.tokens[self.cursor].offset
            ));
        }

        Ok(NqlQuery::new(patterns, return_spec))
    }

    fn parse_pattern(&mut self) -> NosqoResult<NqlPattern> {
        let subject = self.parse_term(TermPosition::Subject)?;
        let predicate = self.parse_term(TermPosition::Predicate)?;
        let object = self.parse_term(TermPosition::Object)?;

        if !matches!(self.peek(), NqlTokenKind::Newline | NqlTokenKind::Eof)
            && !self.is_keyword("return")
        {
            return Err(err!(
                "expected end of pattern at byte offset {}, found {:?}",
                self.tokens[self.cursor].offset,
                self.peek()
            ));
        }

        self.consume_newlines();
        Ok(NqlPattern::new(subject, predicate, object))
    }

    fn parse_return_spec(&mut self) -> NosqoResult<NqlReturn> {
        if self.matches(&NqlTokenKind::Star) {
            self.bump();
            return Ok(NqlReturn::All);
        }

        let mut variables = Vec::new();
        while !self.is_eof() {
            match self.peek() {
                NqlTokenKind::Newline => {
                    self.bump();
                }
                NqlTokenKind::Variable(name) => {
                    variables.push(NqlVariable::new(name.clone()));
                    self.bump();
                }
                other => {
                    return Err(err!(
                        "return block may only contain variables or `*`, found {:?} at byte offset {}",
                        other,
                        self.tokens[self.cursor].offset
                    ));
                }
            }
        }

        if variables.is_empty() {
            return Err(err!(
                "return block must contain at least one variable or `*`"
            ));
        }

        Ok(NqlReturn::Variables(variables))
    }

    fn parse_term(&mut self, position: TermPosition) -> NosqoResult<NqlTerm> {
        let token = self.bump().clone();
        match token.kind {
            NqlTokenKind::Variable(name) => Ok(NqlTerm::variable(NqlVariable::new(name))),
            NqlTokenKind::DoubleQuotedString(text) => Ok(NqlTerm::value(Value::text(text))),
            NqlTokenKind::Word(word) => self.parse_word_term(&word, position),
            other => Err(err!(
                "expected a term at byte offset {}, found {:?}",
                token.offset,
                other
            )),
        }
    }

    fn parse_word_term(&self, word: &str, position: TermPosition) -> NosqoResult<NqlTerm> {
        if word == "T" {
            return Ok(NqlTerm::value(Value::Boolean(true)));
        }

        if word == "F" {
            return Ok(NqlTerm::value(Value::Boolean(false)));
        }

        if let Some(id) = word.strip_prefix('@') {
            return Ok(NqlTerm::id(NodeId::new(id)));
        }

        if word.starts_with('#') {
            return Ok(NqlTerm::id(NodeId::new(word)));
        }

        if word.starts_with('~') {
            return Ok(NqlTerm::id(NodeId::predicate_id(word)?));
        }

        if is_integer_literal(word) {
            let integer = word[1..]
                .parse::<i64>()
                .map_err(|error| err!("invalid integer literal `{}`: {}", word, error))?;
            return Ok(NqlTerm::value(Value::Integer(integer)));
        }

        if is_decimal_literal(word) {
            return Ok(NqlTerm::value(Value::Decimal(DecimalValue::new(
                &word[1..],
            ))));
        }

        if is_date_literal(word) {
            return Ok(NqlTerm::value(Value::Date(DateValue::new(&word[1..]))));
        }

        match position {
            TermPosition::Subject => Ok(NqlTerm::id(NodeId::entity(word))),
            TermPosition::Predicate => Ok(NqlTerm::id(NodeId::predicate_name(word))),
            TermPosition::Object => Err(err!(
                "invalid NQL term `{}`; identifiers must use @, #, or ~ and variables must use ?",
                word
            )),
        }
    }

    fn consume_newlines(&mut self) {
        while matches!(self.peek(), NqlTokenKind::Newline) {
            self.bump();
        }
    }

    fn expect_keyword(&mut self, expected: &str) -> NosqoResult<()> {
        let token = self.bump().clone();
        match token.kind {
            NqlTokenKind::Word(word) if word == expected => Ok(()),
            other => Err(err!(
                "expected keyword `{}` at byte offset {}, found {:?}",
                expected,
                token.offset,
                other
            )),
        }
    }

    fn is_keyword(&self, expected: &str) -> bool {
        matches!(self.peek(), NqlTokenKind::Word(word) if word == expected)
    }

    fn matches(&self, expected: &NqlTokenKind) -> bool {
        self.peek() == expected
    }

    fn is_eof(&self) -> bool {
        matches!(self.peek(), NqlTokenKind::Eof)
    }

    fn peek(&self) -> &NqlTokenKind {
        &self.tokens[self.cursor].kind
    }

    fn bump(&mut self) -> &NqlToken {
        let token = &self.tokens[self.cursor];
        self.cursor += 1;
        token
    }
}

fn is_integer_literal(word: &str) -> bool {
    word.starts_with('i')
        && word.len() > 1
        && word[1..]
            .chars()
            .enumerate()
            .all(|(index, ch)| ch.is_ascii_digit() || (index == 0 && ch == '-'))
}

fn is_decimal_literal(word: &str) -> bool {
    word.starts_with('n')
        && word.len() > 1
        && word[1..]
            .chars()
            .enumerate()
            .all(|(index, ch)| ch.is_ascii_digit() || ch == '.' || (index == 0 && ch == '-'))
}

fn is_date_literal(word: &str) -> bool {
    word.starts_with('d')
        && word.len() > 1
        && word[1..].chars().all(|ch| ch.is_ascii_digit() || ch == '-')
}

fn lex_nql(input: &str) -> NosqoResult<Vec<NqlToken>> {
    let mut tokens = Vec::new();
    let mut offset = 0;

    while let Some(ch) = input[offset..].chars().next() {
        if ch == '\n' {
            tokens.push(NqlToken::new(NqlTokenKind::Newline, offset));
            offset += 1;
            continue;
        }

        if ch.is_whitespace() {
            offset += ch.len_utf8();
            continue;
        }

        let token_offset = offset;
        match ch {
            '"' => {
                let (text, next_offset) = lex_quoted_string(input, offset, '"')?;
                offset = next_offset;
                tokens.push(NqlToken::new(
                    NqlTokenKind::DoubleQuotedString(text),
                    token_offset,
                ));
            }
            '?' => {
                let (name, next_offset) = lex_prefixed_name(input, offset, '?');
                offset = next_offset;
                tokens.push(NqlToken::new(NqlTokenKind::Variable(name), token_offset));
            }
            '*' => {
                offset += 1;
                tokens.push(NqlToken::new(NqlTokenKind::Star, token_offset));
            }
            _ => {
                let (word, next_offset) = lex_word(input, offset);
                offset = next_offset;
                tokens.push(NqlToken::new(NqlTokenKind::Word(word), token_offset));
            }
        }
    }

    tokens.push(NqlToken::new(NqlTokenKind::Eof, offset));
    Ok(tokens)
}

fn lex_quoted_string(
    input: &str,
    start_offset: usize,
    quote: char,
) -> NosqoResult<(String, usize)> {
    let mut offset = start_offset + quote.len_utf8();
    let mut value = String::new();

    while let Some(ch) = input[offset..].chars().next() {
        if ch == quote {
            return Ok((value, offset + quote.len_utf8()));
        }

        if ch == '\\' {
            offset += 1;
            let escaped = input[offset..].chars().next().ok_or_else(|| {
                err!(
                    "unterminated escape sequence starting at byte offset {}",
                    offset
                )
            })?;
            offset += escaped.len_utf8();
            value.push(match escaped {
                '\\' => '\\',
                '"' => '"',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                other => other,
            });
            continue;
        }

        offset += ch.len_utf8();
        value.push(ch);
    }

    Err(err!(
        "unterminated string literal starting at byte offset {}",
        start_offset
    ))
}

fn lex_prefixed_name(input: &str, start_offset: usize, prefix: char) -> (String, usize) {
    let mut offset = start_offset + prefix.len_utf8();
    let mut name = String::new();

    while let Some(ch) = input[offset..].chars().next() {
        if ch.is_whitespace() {
            break;
        }

        name.push(ch);
        offset += ch.len_utf8();
    }

    (name, offset)
}

fn lex_word(input: &str, start_offset: usize) -> (String, usize) {
    let mut offset = start_offset;
    let mut word = String::new();

    while let Some(ch) = input[offset..].chars().next() {
        if ch.is_whitespace() {
            break;
        }

        word.push(ch);
        offset += ch.len_utf8();
    }

    (word, offset)
}

#[cfg(test)]
mod tests {
    use nosqo_model::{NodeId, NqlPattern, NqlQuery, NqlReturn, NqlTerm, NqlVariable, Value};

    use super::NqlParser;

    #[test]
    fn parses_city_label_query() {
        let query = NqlParser::parse_str(
            r#"
            match
            ?city isA #City
            ?city label ?label
            return
            ?city ?label
            "#,
        )
        .unwrap();

        assert_eq!(
            query,
            NqlQuery::new(
                vec![
                    NqlPattern::new(
                        NqlTerm::variable("city"),
                        NqlTerm::id(NodeId::predicate_name("isA")),
                        NqlTerm::id(NodeId::type_name("City")),
                    ),
                    NqlPattern::new(
                        NqlTerm::variable("city"),
                        NqlTerm::id(NodeId::predicate_name("label")),
                        NqlTerm::variable("label"),
                    ),
                ],
                NqlReturn::Variables(vec![NqlVariable::new("city"), NqlVariable::new("label")]),
            )
        );
    }

    #[test]
    fn parses_return_all_queries() {
        let query = NqlParser::parse_str(
            r#"
            match
            ?city label "Berlin"
            return
            *
            "#,
        )
        .unwrap();

        assert_eq!(
            query,
            NqlQuery::new(
                vec![NqlPattern::new(
                    NqlTerm::variable("city"),
                    NqlTerm::id(NodeId::predicate_name("label")),
                    NqlTerm::value(Value::text("Berlin")),
                )],
                NqlReturn::All,
            )
        );
    }

    #[test]
    fn rejects_returning_non_variables() {
        let error = NqlParser::parse_str(
            r#"
            match
            ?city isA #City
            return
            #City
            "#,
        )
        .unwrap_err();

        assert!(
            error
                .to_test_string()
                .contains("return block may only contain variables or `*`")
        );
    }

    #[test]
    fn requires_at_least_one_pattern() {
        let error = NqlParser::parse_str(
            r#"
            match
            return
            *
            "#,
        )
        .unwrap_err();

        assert!(error.to_test_string().contains("at least one pattern"));
    }
}
