use nosqo_base::timestamp::Timestamp;

/// Reports that a child process has exited.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcessExitedEvent {
    /// Timestamp recorded when process exit was observed.
    pub timestamp: Timestamp,
    /// Process exit code when available.
    pub exit_code: Option<i32>,
}
