#[allow(unused_imports)]

use clap::{ArgMatches, Command};
use hiiro::*;

mod commands;

fn main_command() -> Command {
    Command::new("h")
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
        .subcommands(commands::configs())
}

fn main() {
    let parsed_args = main_command().get_matches();

    if let Some((subcmd, _matches)) = parsed_args.subcommand() {


        /*
        * this could be written a few ways:
        *
        * - write a fn to return an enum of types like
        *   - Internal(runner)
        *   - External(bin_path)
        * - write the external fn that takes an option
        *   - return the arg if Some(_)
        *   - look for external subcmd if None
        * - use the Option methods like .value_or(find_external...)
        *   to do the chaining for me
        */



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
