use std::{
    process::exit,
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
