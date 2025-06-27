use anyhow::{Ok, Result};
use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient,
    activity::{Activity, ActivityType},
};
use std::{
    io::Read,
    net::TcpListener,
    thread,
    time::{Duration, Instant},
};

use crate::args::StartConfig;

const DISCORD_APPLICATION_ID: &str = "1365792898851012658";

pub fn start_server(config: StartConfig) -> Result<()> {
    let addr = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(addr)?;
    let discord = new_presence();
    let start = Instant::now();
    loop {
        if start.elapsed().as_secs() / 60 > config.timeout {
            break;
        }
        let (mut sock, _addr) = listener.accept()?;
        let mut recv = "".to_string();
        sock.read_to_string(&mut recv)?;
        println!("{}", recv);
    }
    Ok(())
}

fn new_presence() -> DiscordIpcClient {
    let mut discord = DiscordIpcClient::new(DISCORD_APPLICATION_ID).unwrap();
    discord.connect().unwrap();
    discord
        .set_activity(Activity::new().state("FOOBAR"))
        .unwrap();
    discord
}