use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver, Sender, channel, error::SendError};

use crate::log::Log;

pub type LogSender = Sender<Log>;

/// Helper to manage stream for Logger
pub struct Stream {
    tx: Sender<Log>,
    rx: Receiver<Log>,
}

impl Stream {
    pub fn new(buffer: usize) -> Self {
        let (tx, rx) = channel(buffer);

        Self { tx, rx }
    }

    /// Function to spawn a stream listener in the background
    ///
    /// This should run indefinitely.
    pub fn register(mut self, mut callback: Box<dyn FnMut(Vec<Log>) + Send + Sync>) {
        tokio::spawn(async move {
            loop {
                let mut logs: Vec<Log> = vec![];
                self.rx.recv_many(&mut logs, 6).await;

                callback(logs)
            }
        });
    }

    pub fn tx(&self) -> Sender<Log> {
        self.tx.clone()
    }
}

#[async_trait]
pub trait StreamSending: Send + Sync {
    async fn send_error(
        &self,
        err: Box<dyn std::error::Error + Send + Sync + 'static>,
    ) -> Result<(), SendError<Log>>;
    async fn send_info(&self, info: &str) -> Result<(), SendError<Log>>;
}

#[async_trait]
impl StreamSending for Sender<Log> {
    async fn send_error(
        &self,
        err: Box<dyn std::error::Error + Send + Sync + 'static>,
    ) -> Result<(), SendError<Log>> {
        let log = Log::from_error(err);
        self.send(log).await
    }

    async fn send_info(&self, info: &str) -> Result<(), SendError<Log>> {
        let log = Log::from_str(info);
        self.send(log).await
    }
}
