use crate::pal::{FileChangeCallback, Pal, ReadSeek};
use crate::process_command::ProcessCommand;
use crate::process_event::ProcessEvent;
use crate::process_event_sink::ProcessEventSink;
use crate::process_result::ProcessResult;
use expect_test::Expect;
use nosqo_base::RwLock;
use nosqo_base::file_path::FilePath;
use nosqo_base::result::{NosqoResult, OptionExt};
use nosqo_base::timestamp::Timestamp;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Cursor;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

#[derive(Clone)]
pub struct PalMock {
    inner: Arc<RwLock<PalMockInner>>,
}

struct PalMockInner {
    effects_string: String,
    file_map: HashMap<FilePath, Vec<u8>>,
    process_executions: HashMap<ProcessCommand, (Vec<ProcessEvent>, ProcessResult, Duration)>,
    interactive_terminal: bool,
    default_parallelism: usize,
    current_timestamp: Timestamp,
    current_system_time: SystemTime,
}

impl PalMock {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(PalMockInner {
                effects_string: String::new(),
                file_map: HashMap::new(),
                process_executions: HashMap::new(),
                interactive_terminal: false,
                default_parallelism: 1,
                current_timestamp: Timestamp::new(0),
                current_system_time: SystemTime::UNIX_EPOCH,
            })),
        }
    }

    pub fn log_effect(&self, effect: impl AsRef<str>) {
        let mut inner = self.inner.write();
        inner.effects_string.push_str(effect.as_ref());
        inner.effects_string.push('\n');
    }

    pub fn verify_effects(&self, expected: Expect) {
        expected.assert_eq(&self.inner.read().effects_string);
        self.inner.write().effects_string.clear();
    }

    #[allow(dead_code)]
    pub fn get_effects(&self) -> String {
        self.inner.read().effects_string.clone()
    }

    pub fn clear_effects(&self) {
        self.inner.write().effects_string.clear();
    }

    pub fn set_file(&self, file_path: &str, content: impl Into<Vec<u8>>) {
        self.inner
            .write()
            .file_map
            .insert(FilePath::from(file_path), content.into());
    }

    pub fn set_process_execution(
        &self,
        command: ProcessCommand,
        events: Vec<ProcessEvent>,
        result: ProcessResult,
    ) {
        self.set_process_execution_with_delay(command, events, result, Duration::ZERO);
    }

    pub fn set_process_execution_with_delay(
        &self,
        command: ProcessCommand,
        events: Vec<ProcessEvent>,
        result: ProcessResult,
        delay: Duration,
    ) {
        self.inner
            .write()
            .process_executions
            .insert(command, (events, result, delay));
    }

    pub fn set_current_timestamp(&self, timestamp: Timestamp) {
        self.inner.write().current_timestamp = timestamp;
    }

    pub fn set_interactive_terminal(&self, interactive_terminal: bool) {
        self.inner.write().interactive_terminal = interactive_terminal;
    }

    pub fn set_default_parallelism(&self, default_parallelism: usize) {
        self.inner.write().default_parallelism = default_parallelism;
    }

    pub fn set_current_system_time(&self, system_time: SystemTime) {
        self.inner.write().current_system_time = system_time;
    }

    pub fn read_file_bytes(&self, path: &str) -> Option<Vec<u8>> {
        self.inner
            .read()
            .file_map
            .get(&FilePath::from(path))
            .cloned()
    }

    pub fn read_file_string(&self, path: &str) -> Option<String> {
        self.read_file_bytes(path)
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
    }
}

impl Default for PalMock {
    fn default() -> Self {
        Self::new()
    }
}

impl Pal for PalMock {
    fn file_exists(&self, path: &FilePath) -> NosqoResult<bool> {
        Ok(self.inner.read().file_map.contains_key(path))
    }

    fn read_file(&self, path: &FilePath) -> NosqoResult<Box<dyn ReadSeek + 'static>> {
        self.log_effect(format!("READ FILE: {path}"));
        Ok(Box::new(Cursor::new(
            self.inner
                .read()
                .file_map
                .get(path)
                .with_context(|| format!("File '{path}' does not exist"))?
                .clone(),
        )))
    }

    fn walk_directory(
        &self,
        path: &FilePath,
        _globs: &[String],
    ) -> NosqoResult<Box<dyn Iterator<Item = NosqoResult<FilePath>> + '_>> {
        let mut result = vec![];
        for file_path in self.inner.read().file_map.keys() {
            if file_path.as_path().starts_with(path.as_path()) {
                result.push(Ok(file_path.clone()))
            }
        }
        Ok(Box::new(result.into_iter()))
    }

    fn watch_directory(
        &self,
        _directory: &FilePath,
        _globs: &[String],
        _callback: FileChangeCallback,
    ) -> NosqoResult<()> {
        Ok(())
    }

    fn create_directory_all(&self, path: &FilePath) -> NosqoResult<()> {
        self.log_effect(format!("CREATE DIRECTORY: {path}"));
        Ok(())
    }

    fn write_file(&self, path: &FilePath, content: &[u8]) -> NosqoResult<()> {
        self.log_effect(format!(
            "WRITE FILE: {} -> {}",
            path,
            String::from_utf8_lossy(content)
        ));
        self.inner
            .write()
            .file_map
            .insert(path.clone(), content.to_vec());
        Ok(())
    }

    fn append_file(&self, path: &FilePath, content: &[u8]) -> NosqoResult<()> {
        self.log_effect(format!(
            "APPEND FILE: {} -> {}",
            path,
            String::from_utf8_lossy(content)
        ));
        self.inner
            .write()
            .file_map
            .entry(path.clone())
            .and_modify(|existing| existing.extend_from_slice(content))
            .or_insert_with(|| content.to_vec());
        Ok(())
    }

    fn is_interactive_terminal(&self) -> bool {
        self.inner.read().interactive_terminal
    }

    fn default_parallelism(&self) -> usize {
        self.inner.read().default_parallelism
    }

    fn run_process(
        &self,
        command: &ProcessCommand,
        sink: &mut dyn ProcessEventSink,
    ) -> NosqoResult<ProcessResult> {
        self.log_effect(format!(
            "RUN PROCESS: {} {}",
            command.executable,
            command
                .arguments
                .iter()
                .map(|argument| argument.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        ));
        let (events, result, delay) = self
            .inner
            .read()
            .process_executions
            .get(command)
            .cloned()
            .with_context(|| {
                format!(
                    "No process execution registered for '{}'",
                    command.executable
                )
            })?;

        if delay > Duration::ZERO {
            thread::sleep(delay);
        }

        for event in events {
            sink.handle_event(event)?;
        }

        Ok(result)
    }

    fn now(&self) -> Timestamp {
        self.inner.read().current_timestamp
    }

    fn system_time(&self) -> SystemTime {
        self.inner.read().current_system_time
    }
}

impl Debug for PalMock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PalMock").finish()
    }
}
