use clap::{ArgMatches, Command};

pub fn config() -> Command {
    Command::new("note")
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
}

