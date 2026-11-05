use tokio::sync::mpsc::{Receiver, Sender, channel};

use crate::log::Log;

pub struct Stream {
    tx: Sender<Log>,
    rx: Receiver<Log>,
}

impl Stream {
    pub fn new(buffer: usize) -> Self {
        let (tx, rx) = channel(buffer);

        Self { tx, rx }
    }

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
