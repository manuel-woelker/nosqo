use crate::process_output_stream::ProcessOutputStream;
use nosqo_base::timestamp::Timestamp;

/// Reports that one child process output stream has closed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcessStreamClosedEvent {
    /// Timestamp recorded when the stream close was observed.
    pub timestamp: Timestamp,
    /// Stream that closed.
    pub stream: ProcessOutputStream,
}
