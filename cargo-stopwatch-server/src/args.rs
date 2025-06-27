use std::str::FromStr;

use clap::{Arg, ArgAction, ArgMatches, Command};

#[derive(Debug, Clone)]
pub(crate) struct ServerConfig {
    pub(crate) command: Subcommand,
    pub(crate) start_config: StartConfig,
}

pub(crate) fn stopwatch_server_config() -> ServerConfig {
    let matches = command().get_matches();
    ServerConfig {
        command: get_subcommand(&matches),
        start_config: start_config(&matches),
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Subcommand {
    Start,
    Alive,
    None,
}

fn get_subcommand(matches: &ArgMatches) -> Subcommand {
    match matches.subcommand_name() {
        Some("start") => Subcommand::Start,
        Some("alive") => Subcommand::Alive,
        _ => Subcommand::None,
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StartConfig {
    pub(crate) port: u16,
    pub(crate) timeout: u64,
    pub(crate) quiet: bool,
}

fn start_config(root_matches: &ArgMatches) -> StartConfig {
    if let Some(("start", matches)) = root_matches.subcommand() {
        StartConfig {
            port: parse_arg(&matches, "port")
                .expect("Port must be a number in the range (0, 65535)."),
            timeout: parse_arg(&matches, "timeout").expect("Timeout must be a positive integer"),
            quiet: matches.get_flag("quiet"),
        }
    } else {
        StartConfig {
            port: 0,
            timeout: 0,
            quiet: false,
        }
    }
}

fn parse_arg<F: FromStr>(matches: &ArgMatches, id: &str) -> Option<F> {
    return matches
        .get_one::<String>(id)
        .map(|arg| arg.parse::<F>().ok())
        .flatten();
}

fn command() -> Command {
    Command::new("cargo stopwatch-server")
        .about("Server daemon for the cargo stopwatch cli tool")
        .version("0.1.0")
        .author("Amelia Rossi")
        .subcommand(Command::new("start")
            .about("Starts the server")
            .long_about("Starts the stopwatch server and starts broadcasting rich presence to discord.")
            .arg(Arg::new("port")
                .help("Port for the server to run on.")
                .short('p')
                .long("port")
                .action(ArgAction::Set)
                .value_name("PORT")
                .default_value("44355"))
            .arg(Arg::new("timeout")
                .help("Server timeout in minutes.")
                .short('t').long("timeout")
                .action(ArgAction::Set)
                .value_name("MINUTES")
                .default_value("10"))
            .arg(Arg::new("quiet")
                .help("Don't display package name or command on discord")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)))
        .subcommand(Command::new("alive").about("Check if the server is running").long_about("Check if the server is running, exits with 0 if the server is running or 1 if the server is not"))
}
