// https://github.com/clap-rs/clap/blob/master/examples/20_subcommands.rs
use clap::{App, AppSettings, Arg, ArgMatches};
mod structs;

fn main() {
    let matches = define_clap_definition();
    handle_matches(matches)
}

fn define_clap_definition() -> ArgMatches {
    return App::new("Rust Playground CLI")
        .version("1.0.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new(MainCommands::CloneCmd.as_str())
                .about("clones repos")
                .arg(Arg::new("repo").about("The repo to clone").required(true)),
        )
        .subcommand(
            App::new(MainCommands::Add.as_str())
                .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
                .arg(
                    Arg::new("stuff")
                        .long("stuff")
                        .about("Stuff to add")
                        .takes_value(true)
                        .multiple(true),
                ),
        )
        .get_matches();
}

fn handle_matches(matches: ArgMatches) {
    // The most common way to handle subcommands is via a combined approach using
    // `ArgMatches::subcommand` which returns a tuple of both the name and matches
    let (cmd, matches_opt) = matches.subcommand();
    let subcmd_matches = (MainCommands::from_str(cmd).unwrap(), matches_opt);

    match subcmd_matches {
        (MainCommands::CloneCmd, Some(clone_matches)) => {
            structs::methods::show_struct();
            // Now we have a reference to clone's matches
            println!("Cloning {}", clone_matches.value_of("repo").unwrap());
        }
        (MainCommands::Add, Some(add_matches)) => {
            // Now we have a reference to add's matches
            println!(
                "Adding {}",
                add_matches
                    .values_of("stuff")
                    .unwrap()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        // If all subcommands are defined above, anything else is unreachabe!()
        (_, None) => unreachable!(),
    }
}

enum MainCommands {
    CloneCmd,
    Add,
}

impl MainCommands {
    pub fn from_str(s: &str) -> Option<MainCommands> {
        match s {
            "clone" => Some(MainCommands::CloneCmd),
            "add" => Some(MainCommands::Add),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MainCommands::CloneCmd => "clone",
            MainCommands::Add => "add",
        }
    }
}
