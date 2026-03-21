use crate::token_kind::TokenKind;

/// A token produced by the nosqo text lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The token kind.
    pub kind: TokenKind,
    /// The byte offset where the token starts.
    pub offset: usize,
}

impl Token {
    /// Creates a new token.
    pub fn new(kind: TokenKind, offset: usize) -> Self {
        Self { kind, offset }
    }
}
