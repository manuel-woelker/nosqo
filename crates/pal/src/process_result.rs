use nosqo_base::timestamp::Timestamp;

/// Describes the completed outcome of one child process execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcessResult {
    /// Timestamp recorded when the process was started.
    pub started_at: Timestamp,
    /// Timestamp recorded when the process exit was observed.
    pub finished_at: Timestamp,
    /// Process exit code when available.
    pub exit_code: Option<i32>,
}
