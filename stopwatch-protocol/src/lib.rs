use std::time::Instant;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    /// Close the server connection
    Close,
    /// 
    Started(StartInfo),
    /// Shows a command is running
    Stopped(String, u64),
    /// Update the timeout of the server
    Timeout(u64),
    /// Check for a response from the server without doing anything
    Ping,
    /// Server acknowledge response
    Ok,
    /// Server error response
    Error(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StartInfo {
    pub crate_name: String,
    pub command: String,
    pub time: u64,
}

pub enum StopwatchError {
    UnexpectedMessage,
    ExistingConnection,
}