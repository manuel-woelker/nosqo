use nosqo_base::{err, result::NosqoResult};

use crate::{token::Token, token_kind::TokenKind};

/// A basic lexer for the nosqo text format.
pub struct Lexer<'a> {
    input: &'a str,
    offset: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer for the given input.
    pub fn new(input: &'a str) -> Self {
        Self { input, offset: 0 }
    }

    /// Lexes the full input into a flat token stream.
    pub fn lex(mut self) -> NosqoResult<Vec<Token>> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.bump_char();
                continue;
            }

            if self.starts_with("//") {
                self.consume_line_comment();
                continue;
            }

            if self.starts_with("/*") {
                self.consume_block_comment()?;
                continue;
            }

            let token_offset = self.offset;
            match ch {
                '{' => {
                    self.bump_char();
                    tokens.push(Token::new(TokenKind::LeftBrace, token_offset));
                }
                '}' => {
                    self.bump_char();
                    tokens.push(Token::new(TokenKind::RightBrace, token_offset));
                }
                ',' => {
                    self.bump_char();
                    tokens.push(Token::new(TokenKind::Comma, token_offset));
                }
                '-' if self.starts_with("->") => {
                    self.bump_char();
                    self.bump_char();
                    tokens.push(Token::new(TokenKind::Arrow, token_offset));
                }
                '"' => {
                    let text = self.lex_quoted_string('"')?;
                    tokens.push(Token::new(
                        TokenKind::DoubleQuotedString(text),
                        token_offset,
                    ));
                }
                '\'' => {
                    let text = self.lex_quoted_string('\'')?;
                    tokens.push(Token::new(
                        TokenKind::SingleQuotedString(text),
                        token_offset,
                    ));
                }
                _ => {
                    let word = self.lex_word();
                    tokens.push(Token::new(TokenKind::Word(word), token_offset));
                }
            }
        }

        tokens.push(Token::new(TokenKind::Eof, self.offset));
        Ok(tokens)
    }

    fn lex_quoted_string(&mut self, quote: char) -> NosqoResult<String> {
        self.bump_char();
        let mut value = String::new();

        while let Some(ch) = self.peek_char() {
            if ch == quote {
                self.bump_char();
                return Ok(value);
            }

            if ch == '\\' {
                self.bump_char();
                let escaped = self.peek_char().ok_or_else(|| {
                    err!(
                        "unterminated escape sequence starting at byte offset {}",
                        self.offset
                    )
                })?;
                self.bump_char();
                value.push(match escaped {
                    '\\' => '\\',
                    '"' => '"',
                    '\'' => '\'',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    other => other,
                });
                continue;
            }

            self.bump_char();
            value.push(ch);
        }

        Err(err!(
            "unterminated string literal starting at byte offset {}",
            self.offset
        ))
    }

    fn lex_word(&mut self) -> String {
        let mut value = String::new();

        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace()
                || matches!(ch, '{' | '}' | ',' | '"' | '\'')
                || self.starts_with("//")
                || self.starts_with("/*")
                || self.starts_with("->")
            {
                break;
            }

            self.bump_char();
            value.push(ch);
        }

        value
    }

    fn consume_line_comment(&mut self) {
        while let Some(ch) = self.peek_char() {
            self.bump_char();
            if ch == '\n' {
                break;
            }
        }
    }

    fn consume_block_comment(&mut self) -> NosqoResult<()> {
        self.bump_char();
        self.bump_char();

        while self.peek_char().is_some() {
            if self.starts_with("*/") {
                self.bump_char();
                self.bump_char();
                return Ok(());
            }

            self.bump_char();
        }

        Err(err!(
            "unterminated block comment starting before byte offset {}",
            self.offset
        ))
    }

    fn starts_with(&self, pattern: &str) -> bool {
        self.input[self.offset..].starts_with(pattern)
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.offset..].chars().next()
    }

    fn bump_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.offset += ch.len_utf8();
        Some(ch)
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token_kind::TokenKind;

    #[test]
    fn skips_comments_and_lexes_punctuation() {
        let tokens = Lexer::new(
            r#"
            // line
            city { label "Berlin", 'berlin' }
            /* block */
            capitalof { berlin -> @germany }
            "#,
        )
        .lex()
        .unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Word("city".into()));
        assert_eq!(tokens[1].kind, TokenKind::LeftBrace);
        assert_eq!(tokens[2].kind, TokenKind::Word("label".into()));
        assert_eq!(
            tokens[3].kind,
            TokenKind::DoubleQuotedString("Berlin".into())
        );
        assert_eq!(tokens[4].kind, TokenKind::Comma);
        assert_eq!(
            tokens[5].kind,
            TokenKind::SingleQuotedString("berlin".into())
        );
        assert_eq!(tokens[6].kind, TokenKind::RightBrace);
        assert!(tokens.iter().any(|token| token.kind == TokenKind::Arrow));
    }
}
