#[cfg(test)]
mod test {
    use std::{fs, path::PathBuf, str::FromStr};

    use crate::{Log, LogConfig, Logger, after_test};

    const LOG_PATH: &str = "./log_test";

    #[tokio::test]
    async fn log() {
        const LOG_FILE_NAME: &str = "log.log";

        after_test!({
            let mut after_test_path = PathBuf::from_str(LOG_PATH).unwrap();
            after_test_path.push(LOG_FILE_NAME);

            fs::remove_file(&after_test_path)
        });

        let config = LogConfig::new()
            .path(LOG_PATH)
            .file_name(LOG_FILE_NAME)
            .max_buffer_size(4)
            .max_file_size(1000000);
        let service = Logger::new(&config);
        let tx = service.stream();

        service.register();

        for _ in 0..5 {
            tx.send(Log::from_error(std::io::Error::other("error")))
                .await
                .unwrap();
        }

        let mut path = PathBuf::from(config.path);
        path.push(config.file_name);

        assert!(fs::exists(&path).unwrap());

        let metadata = fs::metadata(path).unwrap();

        assert_ne!(metadata.len(), 0);
    }

    #[tokio::test]
    async fn log_info() {
        const LOG_FILE_NAME: &str = "info.log";

        after_test!({
            let mut after_test_path = PathBuf::from_str(LOG_PATH).unwrap();
            println!("removing");
            after_test_path.push(LOG_FILE_NAME);

            fs::remove_file(&after_test_path)
        });

        let config = LogConfig::new()
            .path(LOG_PATH)
            .file_name(LOG_FILE_NAME)
            .max_buffer_size(4)
            .max_file_size(1000000);
        let service = Logger::new(&config);
        let tx = service.stream();

        service.register();

        for _ in 0..5 {
            tx.send(Log::from_str("message")).await.unwrap();
        }

        let mut path = PathBuf::from(LOG_PATH);
        path.push(LOG_FILE_NAME);

        assert!(fs::exists(&path).unwrap());

        let metadata = fs::metadata(path).unwrap();

        assert_ne!(metadata.len(), 0);
    }

    #[tokio::test]
    async fn rotate() {
        const LOG_FILE_NAME: &str = "rotate.log";

        after_test!({
            let mut after_test_path = PathBuf::from_str(LOG_PATH).unwrap();
            after_test_path.push(LOG_FILE_NAME);

            let mut after_test_new_path = PathBuf::from(LOG_PATH);
            after_test_new_path.push(LOG_FILE_NAME.replace(".", "_1970-01-01_00_00_00."));

            fs::remove_file(&after_test_path)?;
            fs::remove_file(&after_test_new_path)
        });

        let config = LogConfig::new()
            .path(LOG_PATH)
            .file_name(LOG_FILE_NAME)
            .max_buffer_size(4)
            .max_file_size(100);
        let service = Logger::new(&config);
        let tx = service.stream();

        service.register();

        for _ in 0..10 {
            tx.send(Log::from_error(std::io::Error::other("error")))
                .await
                .unwrap();
        }

        let mut path = PathBuf::from(LOG_PATH);
        path.push(LOG_FILE_NAME.replace(".", "_1970-01-01_00_00_00."));

        assert!(fs::exists(&path).unwrap());

        let metadata = fs::metadata(path).unwrap();

        assert_ne!(metadata.len(), 0);
    }
}
