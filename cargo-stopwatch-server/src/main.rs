use anyhow::{Ok, Result};
use discord_rich_presence::{activity::{Activity, Button, Timestamps}, DiscordIpc, DiscordIpcClient};
use std::{
    io::Read,
    net::TcpListener,
    process::exit,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use crate::{
    args::ServerConfig, server::start_server,
};

mod args;
mod server;

fn main() {
    let config = args::stopwatch_server_config();
    match config {
        ServerConfig::Alive => {
            if alive() {
                exit(0)
            } else {
                exit(1)
            }
        }
        ServerConfig::Start(start_config) => {
            start_server(start_config).expect("failed to start stopwatch server");
        }
    }
}

fn alive() -> bool {
    true
}
