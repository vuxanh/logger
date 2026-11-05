use std::env::{self, VarError};

/// Configs for logger
///
/// It contains the path and filename of the log to be created.
/// A max file size to know when it should rotate logs
/// and a max buffer size for its mpsc channel.
pub struct LogConfig {
    pub(crate) path: String,
    pub(crate) file_name: String,
    pub(crate) max_file_size: usize,
    pub(crate) max_buffer_size: usize,
}

impl LogConfig {
    pub fn new() -> Self {
        Self {
            path: "./log".to_string(),
            file_name: "app.log".to_string(),
            max_file_size: 10 * (1 << 20),
            max_buffer_size: 6,
        }
    }

    pub fn default() -> Result<Self, VarError> {
        let path = env::var("LOG_PATH")?;
        let file_name = env::var("FILE_NAME")?;

        Ok(Self {
            path,
            file_name,
            max_file_size: 10 * (1 << 20),
            max_buffer_size: 6,
        })
    }

    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    pub fn file_name(mut self, file_name: &str) -> Self {
        self.file_name = file_name.to_string();
        self
    }

    pub fn max_file_size(mut self, max_file_size: usize) -> Self {
        self.max_file_size = max_file_size;
        self
    }

    pub fn max_buffer_size(mut self, max_buffer_size: usize) -> Self {
        self.max_buffer_size = max_buffer_size;
        self
    }
}
