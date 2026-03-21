use nosqo_base::shared_string::SharedString;

/// Describes one environment variable to be passed to a child process.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcessEnvironmentVariable {
    /// Environment variable name.
    pub name: SharedString,
    /// Environment variable value.
    pub value: SharedString,
}
