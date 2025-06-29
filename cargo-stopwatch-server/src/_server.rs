use anyhow::anyhow;
use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient,
    activity::{Activity, ActivityType, Button, Timestamps},
};
use std::{
    io::{Read, Write}, process::exit, sync::Arc, time::Instant
};
use stopwatch_protocol::Message;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream}, sync::Mutex,
};

use crate::args::StartConfig;

pub async fn start_server(cfg: StartConfig) -> anyhow::Result<()> {
    let addr = format!("127.0.0.1:{}", cfg.port);
    let listener = TcpListener::bind(addr).await?;
    let config = Arc::new(Mutex::new(cfg));
    let discord = Arc::new(Mutex::new(new_presence()));
    let start = Instant::now();
    loop {
        if start.elapsed().as_secs() > config.clone().lock().await.timeout * 60 {
            break;
        }
        println!("loop!");
        let (sock, _addr) = listener.accept().await?;
        let config = config.clone();
        let discord = discord.clone();
        let handle = tokio::spawn(async move {
            handle_connection(sock, config, discord).await
        });
        if let Err(_) = handle.await {
            continue;
        }
    }
    Ok(())
}

async fn handle_connection(
    mut sock: TcpStream,
    config: Arc<Mutex<StartConfig>>,
    discord: Arc<Mutex<DiscordIpcClient>>,
) -> anyhow::Result<()> {
    let mut recv = "".to_string();
    sock.read_to_string(&mut recv).await?;
    let res = handle_message(&recv, config, discord).await?;
    let json = serde_json::to_string(&res)?;
    sock.write_all(json.as_bytes()).await?;
    sock.flush().await?;
    Ok(())
}

async fn handle_message(
    msg_json: &str,
    config: Arc<Mutex<StartConfig>>,
    discord: Arc<Mutex<DiscordIpcClient>>,
) -> anyhow::Result<Message> {
    println!("Received message: {}", msg_json);
    let msg = serde_json::from_str::<Message>(&msg_json)?;
    println!("{:?}", msg);
    let res = match msg {
        Message::Close => {
            exit(0)
        }
        Message::Running(run) => {
            let working_on = format!("Working on {}", run.crate_name);
            let command = format!("`{}`ing for", run.command);
            let result = discord.lock().await
                .set_activity(Activity::new().state(&command).details(&working_on))
                .map_err(|err| anyhow!(err.to_string()));
            match result {
                Ok(_) => Message::Ok,
                Err(_) => Message::Error("Failed to set activity".to_string()),
            }
        }
        Message::Started(run) => {
            let working_on = format!("Working on {}", run.crate_name);
            let command = format!("`{}`ing for", run.command);
            let result = discord.lock().await
                .set_activity(
                    Activity::new()
                        .state(&command)
                        .details(&working_on)
                        .timestamps(Timestamps::new().start(run.time)),
                )
                .map_err(|err| anyhow!(err.to_string()));
            match result {
                Ok(_) => Message::Ok,
                Err(_) => Message::Error("Failed to set activity".to_string()),
            }
        }
        Message::Stopped(time) => {
            let result = discord.lock().await
                .set_activity(Activity::new().timestamps(Timestamps::new().end(time)))
                .map_err(|err| anyhow!(err.to_string()));
            match result {
                Ok(_) => Message::Ok,
                Err(_) => Message::Error("Failed to set activity".to_string()),
            }
        }
        Message::Timeout(timeout) => {
            config.lock().await.timeout = timeout;
            Message::Ok
        }
        Message::Ping => Message::Ok,
        Message::Ok => Message::Error("Unexpected message".to_string()),
        Message::Error(_) => Message::Error("Unexpected message".to_string()),
    };
    Ok(res)
}

const DISCORD_APPLICATION_ID: &str = "1365792898851012658";

pub(crate) fn new_presence() -> DiscordIpcClient {
    let working_on = &format!("Working on {}", env!("CARGO_PKG_NAME"));
    let mut discord = DiscordIpcClient::new(DISCORD_APPLICATION_ID).unwrap();
    discord.connect().unwrap();
    discord
        .set_activity(
            Activity::new()
                .state("This is the state")
                .details(&working_on),
        )
        .unwrap();
    discord
}
