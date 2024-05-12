use clap::{ArgMatches, Command};

pub mod note;

pub type Exec = fn(&ArgMatches);

pub fn configs() -> Vec<Command> {
    vec![
        note::config(),
    ]
}

pub fn exec(subcmd: &str) -> Option<Exec> {
    let runner = match subcmd {
        "note" => note::exec,
        _ => return None,
    };

    Some(runner)
}
