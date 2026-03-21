use crate::pal::{FileChangeCallback, FileChangeEvent, Pal, PalHandle, ReadSeek};
use crate::process_command::ProcessCommand;
use crate::process_event::ProcessEvent;
use crate::process_event_sink::ProcessEventSink;
use crate::process_exited_event::ProcessExitedEvent;
use crate::process_output_event::ProcessOutputEvent;
use crate::process_output_stream::ProcessOutputStream;
use crate::process_result::ProcessResult;
use crate::process_started_event::ProcessStartedEvent;
use crate::process_stream_closed_event::ProcessStreamClosedEvent;
use ignore::WalkBuilder;
use ignore::gitignore::GitignoreBuilder;
use ignore::overrides::OverrideBuilder;
use nosqo_base::RwLock;
use nosqo_base::bail;
use nosqo_base::file_path::FilePath;
use nosqo_base::logging::{error, info};
use nosqo_base::result::{NosqoResult, OptionExt, ResultExt};
use nosqo_base::timestamp::Timestamp;
use notify_debouncer_full::notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, Debouncer, RecommendedCache, new_debouncer};
use std::fmt::Debug;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::IsTerminal as _;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, Instant, SystemTime};
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::process::Command;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

pub struct PalReal {
    base_path: PathBuf,
    watchers: RwLock<Vec<Debouncer<RecommendedWatcher, RecommendedCache>>>,
    reference_instant: Instant,
    /* 📖 # Why keep the Tokio runtime private to `PalReal`?
    `nosqo` keeps PAL consumers synchronous so the rest of the workspace does not need to
    depend on Tokio or expose async process types through domain APIs.

    `PalReal` still needs async I/O internally to read child process pipes efficiently on Linux
    and Windows, so it owns one shared runtime and translates those async operations into
    synchronous `run_process` calls with sink-delivered events.
    */
    runtime: Runtime,
}

impl PalReal {
    pub fn new_handle() -> PalHandle {
        PalHandle::new(Self::new())
    }

    pub fn new() -> Self {
        let current_dir = std::env::current_dir().expect("Unable to access current directory");
        Self {
            base_path: current_dir,
            watchers: RwLock::new(Vec::new()),
            reference_instant: Instant::now(),
            runtime: Runtime::new().expect("Unable to create Tokio runtime"),
        }
    }

    fn resolve_path(&self, path: &FilePath) -> NosqoResult<PathBuf> {
        Ok(self.base_path.join(path.as_path()))
    }

    fn relativize_path(&self, path: &Path) -> NosqoResult<FilePath> {
        let relative_path = path.strip_prefix(&self.base_path).with_context(|| {
            format!(
                "Unable to relativize path '{}' against '{}'",
                path.display(),
                self.base_path.display()
            )
        })?;
        Ok(FilePath::new(relative_path))
    }

    fn resolve_process_path(&self, path: &FilePath) -> NosqoResult<PathBuf> {
        if path.is_absolute() {
            Ok(path.as_path().to_path_buf())
        } else {
            self.resolve_path(path)
        }
    }

    fn resolve_working_directory(&self, path: &FilePath) -> NosqoResult<PathBuf> {
        self.resolve_process_path(path)
    }

    fn timestamp_from(reference_instant: &Instant) -> Timestamp {
        Timestamp::new(reference_instant.elapsed().as_nanos())
    }

    async fn run_process_async(
        &self,
        command: &ProcessCommand,
        sink: &mut dyn ProcessEventSink,
    ) -> NosqoResult<ProcessResult> {
        let mut child_command = Command::new(command.executable.as_str());
        child_command.args(command.arguments.iter().map(|argument| argument.as_str()));
        child_command.stdout(Stdio::piped());
        child_command.stderr(Stdio::piped());

        if let Some(working_directory) = &command.working_directory {
            child_command.current_dir(self.resolve_working_directory(working_directory)?);
        }

        for variable in &command.environment {
            child_command.env(variable.name.as_str(), variable.value.as_str());
        }

        let mut child = child_command.spawn().with_context(|| {
            format!("Unable to spawn process '{}'", command.executable.as_str())
        })?;
        let reference_instant = self.reference_instant;
        let started_at = Self::timestamp_from(&reference_instant);
        sink.handle_event(ProcessEvent::Started(ProcessStartedEvent {
            timestamp: started_at,
            process_id: child.id(),
        }))?;

        let (tx, mut rx) = mpsc::unbounded_channel();
        let mut expected_stream_closes = 0usize;

        if let Some(stdout) = child.stdout.take() {
            expected_stream_closes += 1;
            tokio::spawn(read_stream(
                stdout,
                ProcessOutputStream::Stdout,
                reference_instant,
                tx.clone(),
            ));
        }

        if let Some(stderr) = child.stderr.take() {
            expected_stream_closes += 1;
            tokio::spawn(read_stream(
                stderr,
                ProcessOutputStream::Stderr,
                reference_instant,
                tx.clone(),
            ));
        }

        let mut stream_closes = 0usize;
        let mut finished_at = started_at;
        let mut exit_code = None;
        let mut wait_future = Box::pin(child.wait());
        let mut exit_observed = false;

        while !exit_observed || stream_closes < expected_stream_closes {
            tokio::select! {
                maybe_event = rx.recv(), if stream_closes < expected_stream_closes => {
                    let event = maybe_event.context("Process output channel closed unexpectedly")?;
                    if let ProcessEvent::StreamClosed(_) = &event {
                        stream_closes += 1;
                    }
                    sink.handle_event(event).with_context(|| {
                        format!(
                            "Unable to deliver process event for '{}'",
                            command.executable.as_str()
                        )
                    })?;
                }
                exit_status = &mut wait_future, if !exit_observed => {
                    let exit_status = exit_status.with_context(|| {
                        format!(
                            "Unable to wait for process '{}'",
                            command.executable.as_str()
                        )
                    })?;
                    finished_at = Self::timestamp_from(&reference_instant);
                    exit_code = exit_status.code();
                    sink.handle_event(ProcessEvent::Exited(ProcessExitedEvent {
                        timestamp: finished_at,
                        exit_code,
                    }))?;
                    exit_observed = true;
                }
            }
        }

        Ok(ProcessResult {
            started_at,
            finished_at,
            exit_code,
        })
    }
}

impl Default for PalReal {
    fn default() -> Self {
        Self::new()
    }
}

impl Pal for PalReal {
    fn file_exists(&self, path: &FilePath) -> NosqoResult<bool> {
        Ok(std::fs::exists(self.resolve_path(path)?)?)
    }

    fn read_file(&self, path: &FilePath) -> NosqoResult<Box<dyn ReadSeek + 'static>> {
        Ok(Box::new(
            File::open(self.resolve_path(path)?)
                .with_context(|| format!("Unable to open file '{}'", path))?,
        ))
    }

    fn walk_directory(
        &self,
        path: &FilePath,
        globs: &[String],
    ) -> NosqoResult<Box<dyn Iterator<Item = NosqoResult<FilePath>> + '_>> {
        let real_path = self.resolve_path(path)?;
        if !real_path.is_dir() {
            bail!("Path is not a directory: '{}'", path);
        }
        let mut walk_builder = WalkBuilder::new(&real_path);
        let mut overrides = OverrideBuilder::new(&real_path);
        for glob in globs {
            overrides.add(glob)?;
        }
        walk_builder.overrides(overrides.build()?);
        Ok(Box::new(
            walk_builder
                .build()
                .filter(|entry| match entry {
                    Ok(dir_entry) => {
                        if let Some(file_type) = &dir_entry.file_type()
                            && file_type.is_file()
                        {
                            true
                        } else {
                            false
                        }
                    }
                    Err(_) => false,
                })
                .flat_map(|entry| entry.map(|path| self.relativize_path(path.path()))),
        ))
    }

    fn watch_directory(
        &self,
        directory: &FilePath,
        globs: &[String],
        callback: FileChangeCallback,
    ) -> NosqoResult<()> {
        let mut gitignore_builder = GitignoreBuilder::new(&self.base_path);
        for glob in globs {
            gitignore_builder.add_line(None, glob)?;
        }
        let gitignore = gitignore_builder.build()?;
        let base_path = self.base_path.clone();
        let mut debouncer = new_debouncer(
            Duration::from_millis(500),
            None,
            move |result: DebounceEventResult| match result {
                Ok(events) => {
                    let mut changed_files = Vec::new();
                    for event in &events {
                        if !(event.kind.is_create()
                            || event.kind.is_modify()
                            || event.kind.is_remove())
                        {
                            continue;
                        }
                        for path in &event.paths {
                            let matches = gitignore.matched_path_or_any_parents(path, false);
                            if !matches.is_ignore()
                                && let Ok(relative_path) = path.strip_prefix(&base_path)
                            {
                                changed_files.push(FilePath::new(relative_path));
                            }
                        }
                    }
                    #[allow(clippy::collapsible_if)]
                    if !changed_files.is_empty() {
                        if let Err(error) = callback(FileChangeEvent { changed_files }) {
                            error!("Failed to call filewatcher callback for {events:?}: {error:?}");
                        }
                    }
                }
                Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
            },
        )?;
        let path = self.resolve_path(directory)?;
        info!(
            "Watching directory {}, globs: {}",
            directory,
            globs.join(", ")
        );
        debouncer.watch(path, RecursiveMode::Recursive)?;
        self.watchers.write().push(debouncer);
        Ok(())
    }

    fn create_directory_all(&self, path: &FilePath) -> NosqoResult<()> {
        std::fs::create_dir_all(self.resolve_process_path(path)?)
            .with_context(|| format!("Unable to create directory '{}'", path))?;
        Ok(())
    }

    fn write_file(&self, path: &FilePath, content: &[u8]) -> NosqoResult<()> {
        std::fs::write(self.resolve_process_path(path)?, content)
            .with_context(|| format!("Unable to write file '{}'", path))?;
        Ok(())
    }

    fn append_file(&self, path: &FilePath, content: &[u8]) -> NosqoResult<()> {
        let resolved_path = self.resolve_process_path(path)?;
        if let Some(parent) = resolved_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Unable to create parent directory for '{}'", path))?;
        }
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&resolved_path)
            .with_context(|| format!("Unable to open file '{}' for append", path))?;
        std::io::Write::write_all(&mut file, content)
            .with_context(|| format!("Unable to append file '{}'", path))?;
        Ok(())
    }

    fn is_interactive_terminal(&self) -> bool {
        std::io::stdout().is_terminal()
    }

    fn default_parallelism(&self) -> usize {
        std::thread::available_parallelism()
            .map(usize::from)
            .unwrap_or(1)
    }

    fn run_process(
        &self,
        command: &ProcessCommand,
        sink: &mut dyn ProcessEventSink,
    ) -> NosqoResult<ProcessResult> {
        self.runtime.block_on(self.run_process_async(command, sink))
    }

    fn now(&self) -> Timestamp {
        Timestamp::new(self.reference_instant.elapsed().as_nanos())
    }

    fn system_time(&self) -> SystemTime {
        SystemTime::now()
    }
}

impl Debug for PalReal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PalReal").finish()
    }
}

async fn read_stream<R>(
    mut reader: R,
    stream: ProcessOutputStream,
    reference_instant: Instant,
    tx: mpsc::UnboundedSender<ProcessEvent>,
) where
    R: AsyncRead + Unpin,
{
    let mut buffer = [0u8; 4096];

    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => {
                let _ = tx.send(ProcessEvent::StreamClosed(ProcessStreamClosedEvent {
                    timestamp: PalReal::timestamp_from(&reference_instant),
                    stream,
                }));
                return;
            }
            Ok(read) => {
                let _ = tx.send(ProcessEvent::Output(ProcessOutputEvent {
                    timestamp: PalReal::timestamp_from(&reference_instant),
                    stream,
                    bytes: buffer[..read].to_vec(),
                }));
            }
            Err(_) => {
                let _ = tx.send(ProcessEvent::StreamClosed(ProcessStreamClosedEvent {
                    timestamp: PalReal::timestamp_from(&reference_instant),
                    stream,
                }));
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PalReal;
    use crate::pal::Pal;
    use crate::process_command::ProcessCommand;
    use crate::process_event::ProcessEvent;
    use crate::process_event_sink::ProcessEventSink;
    use nosqo_base::result::NosqoResult;
    use nosqo_base::shared_string::SharedString;

    #[derive(Default)]
    struct RecordingSink {
        events: Vec<ProcessEvent>,
    }

    impl ProcessEventSink for RecordingSink {
        fn handle_event(&mut self, event: ProcessEvent) -> NosqoResult<()> {
            self.events.push(event);
            Ok(())
        }
    }

    #[test]
    fn runs_process_and_reports_events() {
        let pal = PalReal::new();
        let mut sink = RecordingSink::default();

        #[cfg(windows)]
        let command = ProcessCommand {
            executable: SharedString::from("cmd"),
            arguments: vec![
                SharedString::from("/C"),
                SharedString::from("(echo hello)&(echo warn 1>&2)"),
            ],
            working_directory: None,
            environment: Vec::new(),
        };

        #[cfg(not(windows))]
        let command = ProcessCommand {
            executable: SharedString::from("sh"),
            arguments: vec![
                SharedString::from("-c"),
                SharedString::from("printf 'hello\\n'; printf 'warn\\n' 1>&2"),
            ],
            working_directory: None,
            environment: Vec::new(),
        };

        let result = pal.run_process(&command, &mut sink).unwrap();

        assert_eq!(result.exit_code, Some(0));
        assert!(
            sink.events
                .iter()
                .any(|event| matches!(event, ProcessEvent::Started(_)))
        );
        assert!(sink.events.iter().any(|event| {
            matches!(event, ProcessEvent::Output(output) if String::from_utf8_lossy(&output.bytes).contains("hello"))
        }));
        assert!(sink.events.iter().any(|event| {
            matches!(event, ProcessEvent::Output(output) if String::from_utf8_lossy(&output.bytes).contains("warn"))
        }));
        assert!(
            sink.events
                .iter()
                .any(|event| matches!(event, ProcessEvent::Exited(_)))
        );
    }
}
