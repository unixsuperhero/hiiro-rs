#[allow(unused_imports)]

use clap::{ArgMatches, Command};
use hiiro::*;

fn main_command() -> Command {
    Command::new("h")
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
}

fn main() {
    let parsed_args = main_command().get_matches();

    if let Some((subcmd, _matches)) = parsed_args.subcommand() {
        //Some((subcmd, _matches)) => {
            // TODO: look for internally defined subcmd

            // find external subcmd
            let bin_name = find_external_subcmd(subcmd);

            if let Some(extern_bin) = bin_name {
                println!("external binary: {}", extern_bin.display());
            };
        //},
        //None =>
        //    println!("should print help...")
    }
}
