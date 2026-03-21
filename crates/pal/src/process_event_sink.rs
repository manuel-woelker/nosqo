use crate::process_event::ProcessEvent;
use nosqo_base::result::NosqoResult;

/// Receives child process lifecycle events during execution.
pub trait ProcessEventSink {
    /// Handles one process event.
    fn handle_event(&mut self, event: ProcessEvent) -> NosqoResult<()>;
}
