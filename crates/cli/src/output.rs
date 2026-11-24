//! Stream the output to stdout, to a file, or to both at once.
//!
//! `core` writes through [`std::fmt::Write`] in many small pieces;
//! [`Sink`] gathers them in one buffer and hands every full block to
//! each target, so the CLI never holds the whole output in memory and
//! the small writes stay as cheap as appending to a `String`.
//! `fmt::Error` carries no detail, so each target keeps the
//! `io::Error` that stopped it.

use std::fmt;
use std::fs::File;
use std::io::{self, Write};

/// Size of the block handed to the targets.
const BLOCK: usize = 256 * 1024;

/// One output target and the first error it returned.
struct Target<W: Write> {
    writer: W,
    error: Option<io::Error>,
}

impl<W: Write> Target<W> {
    fn new(writer: W) -> Self {
        Self { writer, error: None }
    }

    /// Write `bytes` unless an earlier write failed; `false` on failure.
    fn write(&mut self, bytes: &[u8]) -> bool {
        if self.error.is_some() {
            return false;
        }
        match self.writer.write_all(bytes) {
            Ok(()) => true,
            Err(error) => {
                self.error = Some(error);
                false
            }
        }
    }

    fn finish(mut self) -> io::Result<()> {
        match self.error.take() {
            Some(error) => Err(error),
            None => self.writer.flush(),
        }
    }

    fn is_closed_pipe(&self) -> bool {
        self.error.as_ref().is_some_and(|error| error.kind() == io::ErrorKind::BrokenPipe)
    }
}

/// Where the output goes: stdout, a file, or both.
pub struct Sink {
    buffer: Vec<u8>,
    stdout: Option<Target<io::StdoutLock<'static>>>,
    file: Option<Target<File>>,
}

/// Why writing failed.
#[derive(Debug)]
pub enum SinkError {
    /// The file could not be written.
    File(io::Error),
    /// Stdout could not be written (a closed pipe is not an error).
    Stdout(io::Error),
}

impl Sink {
    /// A sink to stdout, to `file`, or to both.
    pub fn new(stdout: bool, file: Option<File>) -> Self {
        Self {
            buffer: Vec::with_capacity(BLOCK),
            stdout: stdout.then(|| Target::new(io::stdout().lock())),
            file: file.map(Target::new),
        }
    }

    /// Write `text` to stdout only (for example the final newline
    /// that the text format prints but does not store).
    pub fn stdout_only(&mut self, text: &str) {
        if self.flush_block().is_ok()
            && let Some(stdout) = &mut self.stdout
        {
            stdout.write(text.as_bytes());
        }
    }

    /// Write what is left and report the first real error.
    ///
    /// A closed stdout pipe (for example `long-multiplication 99 99 |
    /// head`) is a normal way to stop reading: it is not reported, and
    /// the file, if any, is still written completely.
    pub fn finish(mut self) -> Result<(), SinkError> {
        // An error here is stored in its target and reported below.
        let _ = self.flush_block();
        if let Some(file) = self.file.take() {
            file.finish().map_err(SinkError::File)?;
        }
        if let Some(stdout) = self.stdout.take() {
            match stdout.finish() {
                Err(error) if error.kind() == io::ErrorKind::BrokenPipe => {}
                result => result.map_err(SinkError::Stdout)?,
            }
        }
        Ok(())
    }

    /// Hand the buffered block to every target.
    ///
    /// Any write error stops the output, except a closed stdout pipe
    /// while a file is still being written: the file is completed.
    fn flush_block(&mut self) -> fmt::Result {
        let block = &self.buffer[..];
        let mut result = Ok(());
        if let Some(file) = &mut self.file
            && !file.write(block)
        {
            result = Err(fmt::Error);
        }
        if let Some(stdout) = &mut self.stdout
            && !stdout.write(block)
            && (!stdout.is_closed_pipe() || self.file.is_none())
        {
            result = Err(fmt::Error);
        }
        self.buffer.clear();
        result
    }
}

impl fmt::Write for Sink {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        if self.buffer.len() + text.len() > BLOCK {
            self.flush_block()?;
        }
        self.buffer.extend_from_slice(text.as_bytes());
        Ok(())
    }
}
