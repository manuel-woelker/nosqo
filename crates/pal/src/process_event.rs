use crate::process_exited_event::ProcessExitedEvent;
use crate::process_output_event::ProcessOutputEvent;
use crate::process_started_event::ProcessStartedEvent;
use crate::process_stream_closed_event::ProcessStreamClosedEvent;

/// Reports one lifecycle event from a running child process.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProcessEvent {
    /// Child process started.
    Started(ProcessStartedEvent),
    /// Child process emitted one raw output chunk.
    Output(ProcessOutputEvent),
    /// Child process stream closed.
    StreamClosed(ProcessStreamClosedEvent),
    /// Child process exited.
    Exited(ProcessExitedEvent),
}
