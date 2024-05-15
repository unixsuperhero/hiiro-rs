#[allow(unused_imports)]

use std::path::PathBuf;
use std::ffi::OsString;
use clap::{ArgMatches, Command};
use std::process::Command as Shell;
use hiiro::*;

mod commands;

#[derive(Debug)]
enum Subcommand {
    Internal(commands::Exec),
    External(PathBuf),
    Unknown
}

impl Subcommand {
    pub fn run(&self, args: &ArgMatches) {
        match self {
            Self::Internal(runner) => runner(args),
            Self::External(ref path) => self.run_external(path, args),
            Self::Unknown => (),
        }
    }

    pub fn run_external(&self, path: &PathBuf, args: &ArgMatches) {
        let remaining_args: Vec<_> = args.get_many::<OsString>("")
            .unwrap_or_default()
            .map(OsString::as_os_str)
            .collect();

        let mut cmd = Shell::new(path);
        cmd.args(remaining_args);

        println!("cmd status: {:?}", cmd.spawn());
    }
}

fn main() {
    let parsed_args = main_config().get_matches();
    
    /*
    *  From a high-level:
    *
    *    detect subcmd
    *    then
    *      execute subcmd
    *    else
    *      print help
    */

    if let Some((subcmd, matches)) = parsed_args.subcommand() {
        let subcommand = internal_subcmd(&subcmd)
            .or(external_subcmd(&subcmd))
            .unwrap_or(Subcommand::Unknown);

        println!("returned subcommand: {:#?}", subcmd);
        println!("returned inner subcommand: {:#?}", &matches.subcommand());
        let (inner_subcmd, inner_matches) = &matches.subcommand().unwrap();
        println!("returned inner_subcmd: {:#?}", &inner_subcmd);
        println!("returned inner_matches: {:#?}", &inner_matches);
        let inner_inner_opt = &inner_matches.subcommand();
        println!("returned inner_inner_opt: {:#?}", &inner_inner_opt);
        println!("returned matches: {:#?}", &matches);

        subcommand.run(matches);
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
