use std::{
    process::exit,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use crate::args::StartConfig;
use anyhow::anyhow;
use atomic_time::AtomicInstant;
use discord_presence::{Client, models::ActivityTimestamps};
use stopwatch_protocol::{CommandInfo, Message, StopwatchError};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

#[tokio::main]
pub async fn start_server(config: StartConfig) -> anyhow::Result<()> {
    let addr = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(addr).await?;
    let discord = Discord::new();
    let timeout = Timeout::new(config.timeout);
    while timeout.remaining() > 0 {
        let (stream, _addr) = listener.accept().await?;
        let discord = discord.clone();
        let _handle = tokio::spawn(handle_connection(stream, discord));
    }
    Ok(())
}

async fn handle_connection(mut stream: TcpStream, mut discord: Discord) -> anyhow::Result<()> {
    loop {
        let response = match receive_message(&mut stream).await? {
            Message::Close => exit(0),
            Message::Started(command, start_time) => {
                discord.start(&command, start_time).await?;
                Message::Ok
            }
            Message::Stopped(command, length) => {
                discord.stop(&command, length).await.unwrap();
                Message::Ok
            }
            Message::Timeout(_timeout) => todo!(),
            Message::Ping => Message::Ok,
            Message::Ok => Message::Error(StopwatchError::UnexpectedMessage),
            Message::Error(_) => Message::Error(StopwatchError::UnexpectedMessage),
        };
        send_message(&mut stream, response).await?;
    }
}

async fn receive_message(stream: &mut TcpStream) -> anyhow::Result<Message> {
    let mut buf = [b' '; 256];
    stream.read(&mut buf).await?;
    let message: Message = serde_json::from_slice(&buf)?;
    Ok(message)
}

async fn send_message(stream: &mut TcpStream, message: Message) -> anyhow::Result<()> {
    let message = serde_json::to_vec(&message)?;
    stream.write(&message).await?;
    Ok(())
}

struct Timeout {
    last: AtomicInstant,
    timeout: AtomicU64,
}

impl Timeout {
    fn new(timeout: u64) -> Self {
        Self {
            last: AtomicInstant::now(),
            timeout: AtomicU64::new(timeout),
        }
    }

    fn remaining(&self) -> i64 {
        self.timeout.load(Ordering::Acquire) as i64 * 60
            - self.last.load(Ordering::Acquire).elapsed().as_secs() as i64
    }
}

#[derive(Clone)]
struct Discord {
    client: Arc<Mutex<Client>>,
}

impl Discord {
    const DISCORD_APPLICATION_ID: u64 = 1365792898851012658;

    fn new() -> Self {
        let mut client = Client::new(Self::DISCORD_APPLICATION_ID);
        client.start();
        Self {
            client: Arc::new(Mutex::new(client)),
        }
    }

    async fn start(&mut self, command: &CommandInfo, start_time: u64) -> anyhow::Result<()> {
        self.clear().await?;
        self.client
            .lock()
            .await
            .set_activity(|act| {
                act.details(format!("Working on {}", command.crate_name))
                    .state(format!("Running `{}` for", command.command))
                    .timestamps(|_| ActivityTimestamps::new().start(start_time))
            })
            .map(|_| ())
            .map_err(|e| anyhow!(e.to_string()))
    }

    async fn stop(&mut self, command: &CommandInfo, time: u64) -> anyhow::Result<()> {
        self.clear().await?;
        self.client
            .lock()
            .await
            .set_activity(|act| {
                act.details(format!("Working on {}", command.crate_name))
                    .state(format!("Last ran `{}` for", command.command))
                    .timestamps(|timestamps| timestamps.start(1).end(time+1))
            })
            .map(|_| ())
            .map_err(|e| anyhow!(e.to_string()))
    }

    async fn clear(&mut self) -> anyhow::Result<()> {
        self.client
            .lock()
            .await
            .clear_activity()
            .map(|_| ())
            .map_err(|e| anyhow!(e.to_string()))
    }
}
