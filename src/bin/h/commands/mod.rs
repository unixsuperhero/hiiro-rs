use clap::Command;

pub mod note;

pub fn configs() -> Vec<Command> {
    vec![
        note::config(),
    ]
}
