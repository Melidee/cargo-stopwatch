use anyhow::{Ok, Result};
use std::{io::Read, net::TcpListener, thread, time::{Duration, Instant}};

mod args;
mod server;

fn main() {
    let _ = args::command().get_matches();
}
