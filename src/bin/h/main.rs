#[allow(unused_imports)]

use std::path::PathBuf;
use std::ffi::{OsStr, OsString};
use clap::{ArgMatches, Command};
use hiiro::*;

mod commands;

#[derive(Debug)]
enum Subcommand {
    Internal(commands::Exec),
    External(PathBuf),
    Unknown
}

/*
* there are 2 ways that i can think to handle executing
* the different types of subcommands:
*
* - keep them as an enum, and use a match inside some
*   impl call(...) method
* - undo the enum and make different structs
*   - each with their own call(...) method, specific
*     to the type
* - i _could_ also use the internal/external fns that
*   find the subcmd, to run them too
*   - it would then return a Result instead of Option
*     and i think i could still use .or() for that
*/

fn main() {
    let parsed_args = main_config().get_matches();

    if let Some((subcmd, matches)) = parsed_args.subcommand() {
        let subcommand = internal_subcmd(&subcmd)
            .or(external_subcmd(&subcmd))
            .unwrap_or(Subcommand::Unknown);

        println!("{:#?}", subcommand);
        println!("");

        let remaining_args: Vec<_> = matches.get_many::<OsString>("")
            .unwrap_or_default()
            .map(OsString::as_os_str)
            .collect();

        println!("{:#?}", remaining_args);
    }
}

fn main_config() -> Command {
    Command::new("h")
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
        .subcommands(commands::configs())
}

fn internal_subcmd(name: &str) -> Option<Subcommand> {
    if let Some(exec) = commands::exec(name) {
        let subcmd = Subcommand::Internal(exec);

        return Some(subcmd);
    }

    None
}

fn external_subcmd(name: &str) -> Option<Subcommand> {
    let bin_name = find_external_subcmd(name);

    if let Some(path) = bin_name {
        let subcmd = Subcommand::External(path);

        return Some(subcmd);
    };

    None
}
