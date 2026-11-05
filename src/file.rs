use std::{
    fs::{self, File, OpenOptions, rename},
    io::{BufWriter, Write},
    path::PathBuf,
    sync::Mutex,
};

use crate::{log::Log, utils::LogDateTime};

pub struct FileConfig {
    pub path: String,
    pub file_name: String,
    pub max_file_size: usize,
}

pub struct FileService {
    file: Mutex<BufWriter<File>>,
    bytes_written: usize,
    config: FileConfig,
}

impl FileService {
    pub fn new(config: FileConfig) -> Result<Self, std::io::Error> {
        let file = create_new_file(&config)?;
        Ok(Self {
            file: Mutex::new(BufWriter::new(file)),
            bytes_written: 0,
            config,
        })
    }

    pub fn write_batch(&mut self, logs: &Vec<Log>) -> Result<(), std::io::Error> {
        {
            let mut file = self
                .file
                .lock()
                .map_err(|e| std::io::Error::other(e.to_string()))?;

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

    pub fn rotate(&mut self) -> Result<(), std::io::Error> {
        {
            self.file
                .lock()
                .map_err(|e| std::io::Error::other(e.to_string()))?
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

fn create_new_file(config: &FileConfig) -> Result<File, std::io::Error> {
    let mut write_path = PathBuf::from(&config.path);

    fs::create_dir_all(&write_path)?;
    write_path.push(&config.file_name);

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&write_path)?;

    Ok(file)
}
