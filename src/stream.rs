use tokio::sync::mpsc::{Receiver, Sender, channel};

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
