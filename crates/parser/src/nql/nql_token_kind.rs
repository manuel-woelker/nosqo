/// The lexical token kinds used by the NQL parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NqlTokenKind {
    /// An unquoted word token.
    Word(String),
    /// A query variable such as `?city`.
    Variable(String),
    /// A double-quoted string literal.
    DoubleQuotedString(String),
    /// `*`
    Star,
    /// A newline separator.
    Newline,
    /// End of input.
    Eof,
}
