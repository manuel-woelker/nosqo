use nosqo_base::timestamp::Timestamp;

/// Reports that a child process has started.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcessStartedEvent {
    /// Timestamp recorded when the process was observed as started.
    pub timestamp: Timestamp,
    /// Operating-system process identifier when available.
    pub process_id: Option<u32>,
}
