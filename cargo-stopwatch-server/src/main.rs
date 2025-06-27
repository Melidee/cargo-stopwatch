use anyhow::{Ok, Result};
use std::{io::Read, net::TcpListener, process::exit, thread, time::{Duration, Instant}};

use crate::args::{ServerConfig, Subcommand};

mod args;
mod server;

fn main() {
    let ServerConfig {
        command,
        start_config,
    } = args::stopwatch_server_config();
    match command {
        Subcommand::Alive => alive(),
        _ => unimplemented!()
    }
}

fn alive() {
    let is_alive = true;
    if is_alive {
        exit(0)
    } else {
        exit(1)
    }
}
