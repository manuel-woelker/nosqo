use crate::process_command::ProcessCommand;
use crate::process_event_sink::ProcessEventSink;
use crate::process_result::ProcessResult;
use nosqo_base::file_path::FilePath;
use nosqo_base::result::NosqoResult;
use nosqo_base::shared_string::SharedString;
use nosqo_base::timestamp::Timestamp;
use std::fmt::Debug;
use std::io::{Read, Seek};
use std::sync::Arc;
use std::time::SystemTime;

// Define a new trait combining Read + Seek
pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {} // blanket impl

// Platform abstraction layer used to decouple logic from the underlying platform
pub trait Pal: Debug + Sync + Send + 'static {
    /// Does the file exist?
    fn file_exists(&self, path: &FilePath) -> NosqoResult<bool>;

    /// Read a file, the path is relative to the base directory
    fn read_file(&self, path: &FilePath) -> NosqoResult<Box<dyn ReadSeek + 'static>>;

    /// Read a file to a string, the path is relative to the base directory
    fn read_file_to_string(&self, path: &FilePath) -> NosqoResult<SharedString> {
        let buffer = self.read_file_to_end(path)?;
        SharedString::from_utf8(&buffer)
    }

    fn read_file_to_end(&self, path: &FilePath) -> NosqoResult<Vec<u8>> {
        let mut buffer = Vec::new();
        self.read_file(path)?.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    /// walk directory using the supplied globs
    fn walk_directory(
        &self,
        path: &FilePath,
        globs: &[String],
    ) -> NosqoResult<Box<dyn Iterator<Item = NosqoResult<FilePath>> + '_>>;

    /// Register a callback to be called when a file changes
    fn watch_directory(
        &self,
        directory: &FilePath,
        globs: &[String],
        callback: FileChangeCallback,
    ) -> NosqoResult<()>;

    /// Create a directory and all missing parent directories.
    fn create_directory_all(&self, path: &FilePath) -> NosqoResult<()>;

    /// Write a full file, replacing any previous contents.
    fn write_file(&self, path: &FilePath, content: &[u8]) -> NosqoResult<()>;

    /// Append bytes to a file, creating it if it does not exist.
    fn append_file(&self, path: &FilePath, content: &[u8]) -> NosqoResult<()>;

    /// Returns whether normal process output targets an interactive terminal.
    fn is_interactive_terminal(&self) -> bool;

    /// Returns the default task parallelism for this platform.
    fn default_parallelism(&self) -> usize;

    /// Execute a child process and synchronously forward process events to the sink.
    fn run_process(
        &self,
        command: &ProcessCommand,
        sink: &mut dyn ProcessEventSink,
    ) -> NosqoResult<ProcessResult>;

    fn now(&self) -> Timestamp;

    /// Returns the current wall clock time.
    fn system_time(&self) -> SystemTime;
}

#[derive(Debug, Clone)]
pub struct PalHandle(Arc<dyn Pal>);

impl PalHandle {
    pub fn new(pal: impl Pal + 'static) -> Self {
        Self(Arc::new(pal))
    }
}

// Implement Deref for convenience
impl std::ops::Deref for PalHandle {
    type Target = dyn Pal;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

pub struct FileChangeEvent {
    pub changed_files: Vec<FilePath>,
}

pub type FileChangeCallback = Box<dyn Fn(FileChangeEvent) -> NosqoResult<()> + Send + Sync>;
