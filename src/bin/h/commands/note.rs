use clap::{ArgMatches, Command, arg};

pub fn config() -> Command {
    Command::new("note")
        .arg_required_else_help(true)
        .subcommand(new_config())
}

pub fn new_config() -> Command {
    Command::new("new")
        .arg(arg!([files] ... "files to create"))
}

pub fn exec(args: &ArgMatches) {
    println!("{:#?}", args);

    if let Some((subcmd, matches)) = args.subcommand() {
        println!("in Note, matching subcmd: {:?}", &subcmd);
        println!("in Note, matching matches: {:#?}", &matches);
        let files: &Vec<_> = &matches.get_many::<String>("files").unwrap_or_default().collect();
        println!("in Note, files: {:#?}", &files);
    } else {
        println!("in Note, no matching subcmd");
    }
}

