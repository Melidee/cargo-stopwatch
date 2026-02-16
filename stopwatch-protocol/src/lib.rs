use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    /// Close the server connection
    Close,
    /// 
    Started(CommandInfo, u64),
    /// Shows a command is running
    Stopped(CommandInfo, u64),
    /// Update the timeout of the server
    Timeout(u64),
    /// Check for a response from the server without doing anything
    Ping,
    /// Server acknowledge response
    Ok,
    /// Server error response
    Error(StopwatchError),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandInfo {
    pub crate_name: String,
    pub command: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopwatchError {
    UnexpectedMessage,
    ExistingConnection,
}