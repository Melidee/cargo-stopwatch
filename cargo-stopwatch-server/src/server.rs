use std::any;

use crate::args::StartConfig;
use anyhow::{Error, anyhow};
use discord_rpc_client::{Client, models::Activity};
use stopwatch_protocol::StartInfo;
use tokio::net::TcpListener;

async fn start_server(config: StartConfig) -> anyhow::Result<()> {
    let addr = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind("127.0.0.1:44355").await?;
    Ok(())
}

struct Discord {
    client: Client,
}

impl Discord {
    const DISCORD_APPLICATION_ID: u64 = 1365792898851012658;

    fn new() -> Self {
        let mut client = Client::new(Self::DISCORD_APPLICATION_ID);
        client.start();
        Self { client }
    }

    fn start(&mut self, start_info: StartInfo) -> anyhow::Result<()> {
        self.client
            .set_activity(|act| {
                act.details(format!("Working on {}", start_info.crate_name))
                    .state(format!("Running `{}` for", start_info.command))
                    .timestamps(|timestamps| timestamps.start(start_info.time))
            })
            .map(|_| ())
            .map_err(|e| anyhow!(e.to_string()))
    }

    fn stop(&mut self, command: &str, time: u64) -> anyhow::Result<()> {
        self.client
            .set_activity(|act| {
                act.state(format!("Last ran `{}` for", command))
                    .timestamps(|timestamps| timestamps.start(0).end(time))
            })
            .map(|_| ())
            .map_err(|e| anyhow!(e.to_string()))
    }
}
