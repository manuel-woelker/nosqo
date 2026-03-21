use nosqo_base::{err, result::NosqoResult};
use nosqo_model::{DateTimeValue, DateValue, DecimalValue, NodeId, Statement, StatementSet, Value};

use crate::{lexer::Lexer, token::Token, token_kind::TokenKind};

/// A recursive descent parser for the nosqo text format.
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    /// Parses a nosqo text document into a flat statement set.
    pub fn parse_str(input: &str) -> NosqoResult<StatementSet> {
        let tokens = Lexer::new(input).lex()?;
        let mut parser = Self { tokens, cursor: 0 };

        parser.parse_document()
    }

    fn parse_document(&mut self) -> NosqoResult<StatementSet> {
        let mut statements = Vec::new();

        while !self.is_eof() {
            statements.extend(self.parse_item()?);
        }

        Ok(StatementSet::new(statements))
    }

    fn parse_item(&mut self) -> NosqoResult<Vec<Statement>> {
        let header = self.parse_word_token()?;

        if self.matches(&TokenKind::LeftBrace) {
            self.parse_block(header)
        } else {
            self.parse_statement_from_header(header)
        }
    }

    fn parse_block(&mut self, header: String) -> NosqoResult<Vec<Statement>> {
        self.expect(&TokenKind::LeftBrace)?;

        if self.matches(&TokenKind::RightBrace) {
            self.expect(&TokenKind::RightBrace)?;
            return Ok(Vec::new());
        }

        let first_inner = self.parse_word_token()?;
        let mut statements = if self.matches(&TokenKind::Arrow) {
            self.parse_predicate_block_entries(header, first_inner)?
        } else {
            self.parse_subject_block_entries(header, first_inner)?
        };

        self.expect(&TokenKind::RightBrace)?;
        Ok(std::mem::take(&mut statements))
    }

    fn parse_subject_block_entries(
        &mut self,
        header: String,
        first_predicate: String,
    ) -> NosqoResult<Vec<Statement>> {
        let subject = self.parse_subject_id(&header)?;
        let mut statements = Vec::new();
        statements.extend(self.parse_subject_block_entry(subject.clone(), first_predicate)?);

        while !self.matches(&TokenKind::RightBrace) {
            let predicate = self.parse_word_token()?;
            statements.extend(self.parse_subject_block_entry(subject.clone(), predicate)?);
        }

        Ok(statements)
    }

    fn parse_subject_block_entry(
        &mut self,
        subject: NodeId,
        predicate: String,
    ) -> NosqoResult<Vec<Statement>> {
        let predicate = self.parse_predicate_id(&predicate)?;
        let objects = self.parse_object_list()?;

        Ok(objects
            .into_iter()
            .map(|object| Statement::new(subject.clone(), predicate.clone(), object))
            .collect())
    }

    fn parse_predicate_block_entries(
        &mut self,
        header: String,
        first_subject: String,
    ) -> NosqoResult<Vec<Statement>> {
        let predicate = self.parse_predicate_id(&header)?;
        let mut statements = Vec::new();
        statements.extend(self.parse_predicate_block_entry(predicate.clone(), first_subject)?);

        while !self.matches(&TokenKind::RightBrace) {
            let subject = self.parse_word_token()?;
            statements.extend(self.parse_predicate_block_entry(predicate.clone(), subject)?);
        }

        Ok(statements)
    }

    fn parse_predicate_block_entry(
        &mut self,
        predicate: NodeId,
        subject: String,
    ) -> NosqoResult<Vec<Statement>> {
        let subject = self.parse_subject_id(&subject)?;
        self.expect(&TokenKind::Arrow)?;
        let objects = self.parse_object_list()?;

        Ok(objects
            .into_iter()
            .map(|object| Statement::new(subject.clone(), predicate.clone(), object))
            .collect())
    }

    fn parse_statement_from_header(&mut self, header: String) -> NosqoResult<Vec<Statement>> {
        let subject = self.parse_subject_id(&header)?;
        let predicate_word = self.parse_word_token()?;
        let predicate = self.parse_predicate_id(&predicate_word)?;
        let objects = self.parse_object_list()?;

        Ok(objects
            .into_iter()
            .map(|object| Statement::new(subject.clone(), predicate.clone(), object))
            .collect())
    }

    fn parse_object_list(&mut self) -> NosqoResult<Vec<Value>> {
        let mut objects = vec![self.parse_object_value()?];

        while self.matches(&TokenKind::Comma) {
            self.expect(&TokenKind::Comma)?;
            objects.push(self.parse_object_value()?);
        }

        Ok(objects)
    }

    fn parse_object_value(&mut self) -> NosqoResult<Value> {
        let token = self.bump().clone();
        match token.kind {
            TokenKind::DoubleQuotedString(text) => Ok(Value::text(text)),
            TokenKind::SingleQuotedString(text) => Ok(Value::symbol(text)),
            TokenKind::Word(word) => self.parse_word_object_value(&word),
            other => Err(err!(
                "expected an object value at byte offset {}, found {:?}",
                token.offset,
                other
            )),
        }
    }

    fn parse_word_object_value(&self, word: &str) -> NosqoResult<Value> {
        if word == "T" {
            return Ok(Value::Boolean(true));
        }

        if word == "F" {
            return Ok(Value::Boolean(false));
        }

        if let Some(id) = word.strip_prefix('@') {
            return Ok(Value::Id(NodeId::new(id)));
        }

        if word.starts_with('#') {
            return Ok(Value::Id(NodeId::new(word)));
        }

        if word.starts_with('~') {
            return Ok(Value::Id(NodeId::predicate_id(word)?));
        }

        if let Some(integer) = word.strip_prefix('i') {
            let integer = integer
                .parse::<i64>()
                .map_err(|error| err!("invalid integer literal `{}`: {}", word, error))?;
            return Ok(Value::Integer(integer));
        }

        if let Some(decimal) = word.strip_prefix('n') {
            if decimal.is_empty() {
                return Err(err!("invalid decimal literal `{}`", word));
            }
            return Ok(Value::Decimal(DecimalValue::new(decimal)));
        }

        if let Some(date) = word.strip_prefix('d') {
            if date.is_empty() {
                return Err(err!("invalid date literal `{}`", word));
            }
            return Ok(Value::Date(DateValue::new(date)));
        }

        if let Some(date_time) = word.strip_prefix('t') {
            if date_time.is_empty() {
                return Err(err!("invalid datetime literal `{}`", word));
            }
            return Ok(Value::DateTime(DateTimeValue::new(date_time)));
        }

        Err(err!(
            "bare object identifier `{}` is not allowed; use @name, #Type, or ~predicate",
            word
        ))
    }

    fn parse_subject_id(&self, word: &str) -> NosqoResult<NodeId> {
        if let Some(id) = word.strip_prefix('@') {
            return Ok(NodeId::new(id));
        }

        if word.starts_with('#') {
            return Ok(NodeId::new(word));
        }

        if word.starts_with('~') {
            return NodeId::predicate_id(word);
        }

        if word.is_empty() {
            return Err(err!("subject identifier cannot be empty"));
        }

        Ok(NodeId::entity(word))
    }

    fn parse_predicate_id(&self, word: &str) -> NosqoResult<NodeId> {
        if let Some(id) = word.strip_prefix('@') {
            return NodeId::predicate_id(id);
        }

        if word.starts_with('~') {
            return NodeId::predicate_id(word);
        }

        if word.is_empty() {
            return Err(err!("predicate identifier cannot be empty"));
        }

        Ok(NodeId::predicate_name(word))
    }

    fn parse_word_token(&mut self) -> NosqoResult<String> {
        let token = self.bump().clone();
        match token.kind {
            TokenKind::Word(word) => Ok(word),
            other => Err(err!(
                "expected a word token at byte offset {}, found {:?}",
                token.offset,
                other
            )),
        }
    }

    fn expect(&mut self, expected: &TokenKind) -> NosqoResult<()> {
        let token = self.bump().clone();
        if &token.kind == expected {
            return Ok(());
        }

        Err(err!(
            "expected {:?} at byte offset {}, found {:?}",
            expected,
            token.offset,
            token.kind
        ))
    }

    fn matches(&self, expected: &TokenKind) -> bool {
        self.peek() == expected
    }

    fn is_eof(&self) -> bool {
        matches!(self.peek(), TokenKind::Eof)
    }

    fn peek(&self) -> &TokenKind {
        &self.tokens[self.cursor].kind
    }

    fn bump(&mut self) -> &Token {
        let token = &self.tokens[self.cursor];
        self.cursor += 1;
        token
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use nosqo_model::Statement;

    #[test]
    fn parses_subject_blocks_into_flat_statements() {
        let statement_set = Parser::parse_str(
            r#"
            berlin {
              label "Berlin"
              speaks 'de', 'en'
            }
            "#,
        )
        .unwrap();

        assert_eq!(statement_set.as_slice().len(), 3);
        assert_eq!(
            statement_set.as_slice()[0],
            Statement::from_strings("berlin", "label", "Berlin")
        );
        assert_eq!(
            statement_set.as_slice()[1],
            Statement::from_strings("berlin", "speaks", "'de'")
        );
    }

    #[test]
    fn parses_predicate_blocks_into_flat_statements() {
        let statement_set = Parser::parse_str(
            r#"
            capitalof {
              berlin -> @germany
              paris -> @france
            }
            "#,
        )
        .unwrap();

        assert_eq!(statement_set.as_slice().len(), 2);
        assert_eq!(
            statement_set.as_slice()[0],
            Statement::from_strings("berlin", "capitalof", "@germany")
        );
    }

    #[test]
    fn parses_the_core_ontology_file() {
        let statement_set = Parser::parse_str(include_str!(
            "../../../knowledge/ontologies/meta-ontology.nosqo"
        ))
        .unwrap();

        assert!(statement_set.as_slice().len() >= 62);
        assert!(
            statement_set
                .as_slice()
                .contains(&Statement::from_strings("#Type", "label", "Type"))
        );
        assert!(statement_set.as_slice().contains(&Statement::from_strings(
            "#String",
            "~attribute",
            "~description"
        )));
        assert!(statement_set.as_slice().contains(&Statement::from_strings(
            "~label",
            "~targetType",
            "#String"
        )));
        assert!(statement_set.as_slice().contains(&Statement::from_strings(
            "#Type",
            "~attribute",
            "~label"
        )));
        assert!(statement_set.as_slice().contains(&Statement::from_strings(
            "~targetType",
            "~targetType",
            "#Type"
        )));
    }

    #[test]
    fn rejects_bare_object_identifiers() {
        let error = Parser::parse_str("berlin capitalof germany").unwrap_err();

        assert!(
            error
                .to_test_string()
                .contains("bare object identifier `germany` is not allowed")
        );
    }
}
