use anyhow::{Ok, Result};
use std::{io::Read, net::TcpListener, thread, time::{Duration, Instant}};

use crate::args::StartConfig;

#[derive(Debug)]
pub(crate) struct Server {
    listener: TcpListener,
    config: StartConfig,
}

impl Server {
    pub(crate) fn new(config: StartConfig) -> Result<Server> {
        let listener = TcpListener::bind("127.0.0.1:44355")?;
        let server = Self {listener, config};
        Ok(server)
    }

    pub fn handle_incoming(&mut self) -> Result<()> {
        let start = Instant::now();
        loop {
            if start.elapsed().as_secs() / 60 > self.config.timeout {
                break;
            }
            let (mut sock, _addr) = self.listener.accept()?;
            let mut recv = "".to_string();
            sock.read_to_string(&mut recv)?;
            println!("{}", recv);
        }
        Ok(())
    }
}
