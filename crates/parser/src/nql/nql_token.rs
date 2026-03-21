use super::nql_token_kind::NqlTokenKind;

/// A token produced by the NQL lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NqlToken {
    /// The token kind.
    pub kind: NqlTokenKind,
    /// The byte offset where the token starts.
    pub offset: usize,
}

impl NqlToken {
    /// Creates a new token.
    pub fn new(kind: NqlTokenKind, offset: usize) -> Self {
        Self { kind, offset }
    }
}
