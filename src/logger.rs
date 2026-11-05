use tokio::sync::mpsc::Sender;

use crate::{
    LogConfig,
    file::{FileConfig, FileService},
    log::Log,
    stream::Stream,
};

/// External port for logger
///
/// When register, it spawns a tokio task and listens indefinitely
/// in the background for incoming stream.
/// Exposes a stream so external callers can send logs through.
pub struct Logger {
    file_service: FileService,
    stream: Stream,
}

impl Logger {
    pub fn new(config: &LogConfig) -> Self {
        let file_service = FileService::new(FileConfig {
            path: config.path.to_string(),
            file_name: config.file_name.to_string(),
            max_file_size: config.max_file_size,
        })
        .expect("couldn't init file service");

        let stream = Stream::new(config.max_buffer_size);

        Self {
            stream,
            file_service,
        }
    }

    /// Spawn a listening task
    pub fn register(mut self) {
        self.stream.register(Box::new(move |logs| {
            self.file_service
                .write_batch(&logs)
                .expect("couldn't write log");
        }))
    }

    /// Expose the stream receiver
    pub fn stream(&self) -> Sender<Log> {
        self.stream.tx()
    }
}
