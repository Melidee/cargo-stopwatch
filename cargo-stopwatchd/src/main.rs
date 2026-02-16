use std::{
    process::exit,
};

mod args;
mod server;

use crate::{
    args::ServerConfig, server::start_server,
};



fn main() {
    let config = args::stopwatch_server_config();
    match config {
        ServerConfig::Alive => {
            if alive() {
                println!("Stopwatch server running");
                exit(0)
            } else {
                println!("Stopwatch server not running");
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
