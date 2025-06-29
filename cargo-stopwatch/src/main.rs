use std::{io::Write, net::TcpStream, process::{exit, Command, Stdio}, thread::{self, Thread}, time::{SystemTime, UNIX_EPOCH}};

use stopwatch_protocol::{Message, Run};

use crate::args::get_stopwatch_config;

mod args;

fn main() {
    let config = get_stopwatch_config();
    if let Some(server_args) = config.server {
        Command::new("cargo-stopwatch-server")
            .args(server_args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to execute server command");
    }
}