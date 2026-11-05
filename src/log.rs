use core::fmt;
use std::backtrace::Backtrace;

use chrono::{DateTime, Utc};

use crate::utils::LogDateTime;

/// The model for a log
///
/// It contains the date (UTC) and the inner details
/// including a string message, trace and kind of the
/// error. To enable trace make sure RUST_BACKTRACE=1
pub struct Log {
    inner: Inner,
    created_at: DateTime<Utc>,
}

/// Name of the enum dictates its level
pub enum Inner {
    Info(String),
    Error {
        message: String,
        trace: String,
        kind: String,
    },
}

impl Log {
    pub fn from_str(s: &str) -> Self {
        Self {
            inner: Inner::Info(s.to_string()),
            created_at: LogDateTime::now(),
        }
    }

    pub fn from_error<E: core::error::Error + Send + Sync + 'static>(e: E) -> Self {
        Self {
            inner: Inner::Error {
                kind: std::any::type_name_of_val(&e).to_string(),
                message: e.to_string(),
                trace: Backtrace::capture().to_string(),
            },
            created_at: LogDateTime::now(),
        }
    }

    pub(self) fn level(&self) -> String {
        match self.inner {
            Inner::Info(_) => "Info".to_string(),
            Inner::Error { .. } => "Error".to_string(),
        }
    }

    pub(self) fn created_at(&self) -> String {
        self.created_at.to_string()
    }

    pub(self) fn inner(&self) -> String {
        match &self.inner {
            Inner::Info(text) => text.to_string(),
            Inner::Error {
                message,
                trace,
                kind,
            } => {
                format!("{}: {}\nTrace:\n{}\n", kind, message, trace)
            }
        }
    }
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}][{}]: {}",
            self.created_at(),
            self.level(),
            self.inner()
        )
    }
}
