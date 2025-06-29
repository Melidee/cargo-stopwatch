use std::{
    error::Error,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    process::exit,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicI64, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient,
    activity::{Activity, Timestamps},
};
use stopwatch_protocol::{Message, Run};

use crate::args::StartConfig;

struct Discord {
    client: DiscordIpcClient,
    working_on: String,
    command: String,
    start_time: Option<i64>,
}

impl Discord {
    const DISCORD_APPLICATION_ID: &str = "1365792898851012658";

    fn new() -> Result<Self, Box<dyn Error>> {
        let mut client = DiscordIpcClient::new(Self::DISCORD_APPLICATION_ID)?;
        client.connect()?;
        client.clear_activity()?;
        let discord = Discord {
            client,
            working_on: String::new(),
            command: String::new(),
            start_time: None,
        };
        Ok(discord)
    }

    fn set_run(&mut self, run: Run) {
        self.working_on = run.crate_name;
        self.command = run.command;
        self.start_time = Some(run.time);
    }

    fn clear_run(&mut self) -> Result<(), Box<dyn Error>> {
        self.working_on.clear();
        self.command.clear();
        self.start_time = None;
        self.client.clear_activity()?;
        Ok(())
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let working_on = format!("Working on {}", &self.working_on);
        let command = format!("Running `{}` for", &self.command);
        let mut activity = Activity::new().details(&working_on).state(&command);
        if let Some(time) = self.start_time {
            activity = activity.timestamps(Timestamps::new().start(time));
        }
        self.client.set_activity(activity)?;
        Ok(())
    }
}

pub(crate) fn start_server(cfg: StartConfig) -> Result<(), Box<dyn std::error::Error>> {
    // opaque error because the discord ipc client uses them
    let config = Arc::new(Mutex::new(cfg));
    let discord = Arc::new(Mutex::new(Discord::new()?));
    let listener = TcpListener::bind("127.0.0.1:44355")?;
    let mut last = Instant::now();
    listener.set_nonblocking(true)?;
    for stream in listener.incoming() {
        if last.elapsed().as_secs() > config.lock().unwrap().timeout * 60 {
            break;
        }
        match stream {
            Ok(stream) => {
                last = Instant::now();
                let config = config.clone();
                let discord = discord.clone();
                thread::spawn(move || {
                    handle_incoming(stream, config.clone(), discord.clone()).unwrap();
                });
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(500));
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
    Ok(())
}

fn handle_incoming(
    mut stream: TcpStream,
    config: Arc<Mutex<StartConfig>>,
    discord: Arc<Mutex<Discord>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut recv = [32u8; 1024];
    stream.read(&mut recv)?;
    let msg = serde_json::from_slice::<Message>(&recv.trim_ascii_end())?;
    match msg {
        Message::Close => {
            write_message(&mut stream, Message::Ok)?;
            exit(0)
        }
        Message::Started(run) => {
            let mut discord = discord.lock().unwrap();
            discord.set_run(run);
            discord.update()?;
            write_message(&mut stream, Message::Ok)?;
        }
        Message::Stopped(time) => {
            discord.lock().unwrap().clear_run()?;
            write_message(&mut stream, Message::Ok)?;
        }
        Message::Timeout(timeout) => {
            config.lock().unwrap().timeout = timeout;
            write_message(&mut stream, Message::Ok)?;
        }
        Message::Ping => {
            write_message(&mut stream, Message::Ok)?;
        }
        Message::Ok | Message::Error(_) => {
            write_message(
                &mut stream,
                Message::Error("Unexpected message kind.".into()),
            )?;
        }
    }
    Ok(())
}

fn write_message(stream: &mut TcpStream, msg: Message) -> Result<(), Box<dyn std::error::Error>> {
    stream.write(&serde_json::to_vec(&msg)?)?;
    Ok(())
}
