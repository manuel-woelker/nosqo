/// Identifies one child process output stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessOutputStream {
    /// Standard output.
    Stdout,
    /// Standard error.
    Stderr,
}
