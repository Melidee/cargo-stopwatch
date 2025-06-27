use clap::{parser::ValuesRef, value_parser, Arg, ArgAction, Command};

#[derive(Debug)]
pub(crate) struct StopwatchConfig {
    pub server: Option<Vec<String>>,
    pub port: u16,
    pub timeout: u64,
    pub quiet: bool,
    pub commands: Vec<String>,
}

pub(crate) fn get_stopwatch_config() -> StopwatchConfig {
    let matches = command().get_matches();
    StopwatchConfig {
        server: matches.subcommand_matches("server").map(|submatches| {
            submatches
                .get_many::<String>("commands")
                .unwrap_or(ValuesRef::default())
                .cloned()
                .collect()
        }),
        port: *matches
            .get_one("port")
            .expect("Port must be a number in the range (0, 65535)."),
        timeout: *matches
            .get_one("timeout")
            .expect("Timeout must be a positive integer."),
        quiet: matches.get_flag("quiet"),
        commands: matches
            .get_many::<String>("commands")
            .unwrap_or(ValuesRef::default())
            .cloned()
            .collect(),
    }
}

fn command() -> Command {
    Command::new("cargo stopwatch-server")
        .about("Time cargo ")
        .version("0.1.0")
        .author("Amelia Rossi")
        .arg_required_else_help(true)
        .subcommand(Command::new("server")
            .about("Run stopwatch server commands")
            .arg(Arg::new("commands")
                .trailing_var_arg(true)
                .allow_hyphen_values(true)
                .num_args(1..)))
        .arg(Arg::new("port")
            .help("Port to communicate with server.")
            .short('p')
            .long("port")
            .action(ArgAction::Set)
            .value_name("PORT")
            .default_value("44355")
            .value_parser(value_parser!(u16).range(0..=65535)))
        .arg(Arg::new("timeout")
            .help("Server timeout in minutes.")
            .short('t').long("timeout")
            .action(ArgAction::Set)
            .value_name("MINUTES")
            .default_value("10")
            .value_parser(value_parser!(u64)))
        .arg(Arg::new("quiet")
            .help("Don't display package name or command on discord")
            .short('q')
            .long("quiet")
            .action(ArgAction::SetTrue))
        .arg(Arg::new("commands")
            .trailing_var_arg(true)
            .allow_hyphen_values(true)
            .num_args(1..))
        .subcommand(Command::new("alive")
            .about("Check if the server is running")
            .long_about("Check if the server is running, exits with 0 if the server is running or 1 if the server is not"))
}
