use crate::process_environment_variable::ProcessEnvironmentVariable;
use nosqo_base::file_path::FilePath;
use nosqo_base::shared_string::SharedString;

/// Describes a child process invocation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcessCommand {
    /// Executable path or program name.
    pub executable: SharedString,
    /// Positional process arguments.
    pub arguments: Vec<SharedString>,
    /// Working directory for the child process, relative to the PAL base path when not absolute.
    pub working_directory: Option<FilePath>,
    /// Environment variable overrides for the child process.
    pub environment: Vec<ProcessEnvironmentVariable>,
}
