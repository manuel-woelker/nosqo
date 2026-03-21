use crate::process_output_stream::ProcessOutputStream;
use nosqo_base::timestamp::Timestamp;

/// Reports one raw output chunk emitted by a child process.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcessOutputEvent {
    /// Timestamp recorded when the output chunk was observed.
    pub timestamp: Timestamp,
    /// Stream that produced this output.
    pub stream: ProcessOutputStream,
    /// Raw output bytes.
    pub bytes: Vec<u8>,
}
