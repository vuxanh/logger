const MAX_FILE_SIZE: usize = 10000000;
const MAX_BUFFER_SIZE: usize = 6;
pub struct LogConfig {
    pub(crate) path: String,
    pub(crate) file_name: String,
    pub(crate) max_file_size: usize,
    pub(crate) max_buffer_size: usize,
}

impl LogConfig {
    pub fn new() -> Self {
        Self {
            path: Default::default(),
            file_name: Default::default(),
            max_file_size: MAX_FILE_SIZE,
            max_buffer_size: MAX_BUFFER_SIZE,
        }
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
