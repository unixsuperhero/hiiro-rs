use clap::{ArgMatches, Command};

pub fn config() -> Command {
    Command::new("note")
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("new").allow_external_subcommands(true))
}

pub fn exec(args: &ArgMatches) {
    println!("{:#?}", args);
}

