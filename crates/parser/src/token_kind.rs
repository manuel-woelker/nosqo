/// The lexical token kinds used by the nosqo text parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// An unquoted word token.
    Word(String),
    /// A double-quoted string literal.
    DoubleQuotedString(String),
    /// A single-quoted string literal.
    SingleQuotedString(String),
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `,`
    Comma,
    /// `->`
    Arrow,
    /// End of input.
    Eof,
}
