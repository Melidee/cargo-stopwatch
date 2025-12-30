use std::{
    io::{Read, Write},
    net::TcpStream,
    process::{Command, Stdio},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use fork::{Fork, daemon};
use stopwatch_protocol::{CommandInfo, Message};

use crate::args::{StopwatchConfig, get_stopwatch_config};

mod args;

fn main() {
    let config = get_stopwatch_config();
    if let Some(ref server_args) = config.server {
        Command::new("cargo-stopwatch-server")
            .args(server_args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to execute server command");
    }
    start_server(&config.clone()).expect("Failed to start server");
    let command = config
        .commands
        .get(0)
        .expect("no cargo command provided")
        .to_owned();
    let start = Instant::now();
    send_message(
        &Message::Started(
            CommandInfo {
                crate_name: env!("CARGO_PKG_NAME").into(),
                command: command.clone(),
            },
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        ),
        config.port,
    )
    .expect("Failed to send message");
    Command::new("cargo")
        .args(config.commands)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute cargo command")
        .wait()
        .expect("Failed to execute cargo command");
    send_message(
        &Message::Stopped(
            CommandInfo {
                crate_name: env!("CARGO_PKG_NAME").into(),
                command: command.clone(),
            },
            start.elapsed().as_secs(),
        ),
        config.port,
    )
    .expect("Failed to send message");
    println!("stopped");
}

fn send_message(message: &Message, port: u16) -> Result<(), anyhow::Error> {
    let msg = serde_json::to_vec(message)?;
    let addr = format!("127.0.0.1:{}", port);
    let mut stream = TcpStream::connect(addr)?;
    stream.write(&msg)?;
    Ok(())
}

fn start_server(config: &StopwatchConfig) -> Result<(), anyhow::Error> {
    if server_is_alive(config.port) {
        println!("already running");
        return Ok(());
    }
    Command::new("../target/debug/cargo-stopwatchd")
        .args([
            "start",
            "--timeout",
            &config.timeout.to_string(),
            "--port",
            &config.port.to_string(),
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    println!("alive? : {}", server_is_alive(config.port));
    Ok(())
}

fn server_is_alive(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    if let Ok(mut stream) = TcpStream::connect(addr) {
        let msg = serde_json::to_vec(&Message::Ping).unwrap();
        if stream.write(&msg).is_err() {
            return false;
        }
        let mut buf = [32u8; 1024];
        if let Ok(_) = stream.read(&mut buf)
            && let Ok(msg) = serde_json::from_slice::<Message>(&buf)
            && msg == Message::Ok
        {
            return true;
        }
    }
    return false;
}
