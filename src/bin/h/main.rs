#[allow(unused_imports)]

use clap::{ArgMatches, Command};
use hiiro::subcmd_bin;

fn main_command() -> Command {
    Command::new("h")
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
}

fn main() {
    let parsed_args = main_command().get_matches();

    match parsed_args.subcommand() {
        Some((subcmd, _matches)) => println!("subcmd_bin_name: {}", subcmd_bin("h", subcmd)),
        None => println!("should print help...")
    }
    println!("args: {:#?}", parsed_args.subcommand());
}
