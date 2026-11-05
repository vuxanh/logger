use std::{
    fs::{self, File, OpenOptions, rename},
    io::{BufWriter, Error, Write},
    path::PathBuf,
    sync::Mutex,
};

use crate::{log::Log, utils::LogDateTime};

pub struct FileConfig {
    pub path: String,
    pub file_name: String,
    pub max_file_size: usize,
}

/// Service for managing log write and rotate
///
/// Remembers the number of bytes that have been written
/// so it can rotate if it exceeds max_file_size.
pub struct FileService {
    file: Mutex<BufWriter<File>>,
    bytes_written: usize,
    config: FileConfig,
}

impl FileService {
    pub fn new(config: FileConfig) -> Result<Self, Error> {
        let file = create_new_file(&config)?;
        Ok(Self {
            file: Mutex::new(BufWriter::new(file)),
            bytes_written: 0,
            config,
        })
    }

    /// Function to write logs in batch to a file.
    ///
    /// Self is put inside scope to make sure lock is released and
    /// it can be mutable for rotate later.
    pub fn write_batch(&mut self, logs: &Vec<Log>) -> Result<(), Error> {
        {
            let mut file = self.file.lock().map_err(|e| Error::other(e.to_string()))?;

            for log in logs {
                writeln!(file, "{log}")?;
                self.bytes_written += log.to_string().as_bytes().len();
            }

            file.flush()?;
        }

        if self.bytes_written >= self.config.max_file_size {
            self.rotate()?;
            self.bytes_written = 0;
        }

        Ok(())
    }

    /// Function to rotate file when it has exceeded max size
    ///
    /// Flush write buffer to write the remaining messages to file
    /// if any. The old file will be renamed with the current date
    /// i.e. file_1970-01-01_00_00_00.log
    /// Then it creates a new file to write in
    pub fn rotate(&mut self) -> Result<(), Error> {
        {
            self.file
                .lock()
                .map_err(|e| Error::other(e.to_string()))?
                .flush()?;
        }

        let timestamp = LogDateTime::now().format("%Y-%m-%d_%H_%M_%S");

        let mut new_file_path = PathBuf::from(&self.config.path);
        new_file_path.push(
            &self
                .config
                .file_name
                .replace(".", &format!("_{}.", timestamp).to_string()),
        );

        let mut file_path = PathBuf::from(&self.config.path);
        file_path.push(&self.config.file_name);

        rename(file_path, new_file_path)?;

        let file = create_new_file(&self.config)?;

        self.file = Mutex::new(BufWriter::new(file));

        Ok(())
    }
}

/// Helper function to create a new file in the path provided
fn create_new_file(config: &FileConfig) -> Result<File, Error> {
    let mut write_path = PathBuf::from(&config.path);

    fs::create_dir_all(&write_path)?;
    write_path.push(&config.file_name);

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&write_path)?;

    Ok(file)
}
