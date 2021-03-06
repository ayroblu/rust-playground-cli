use crate::types::MainCommands;
use clap::{App, AppSettings, Arg, ArgMatches, Shell};
use std::str::FromStr;
use std::{io, process};

pub fn build_cli_app() -> App<'static, 'static> {
    App::new("rust-playground-cli")
        .version("1.0.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new(MainCommands::FormatPrint.as_str()).about("demos print formatting"))
        .subcommand(App::new(MainCommands::Primitives.as_str()).about("demos primitives"))
        .subcommand(App::new(MainCommands::CustomTypes.as_str()).about("demos custom types"))
        .subcommand(App::new(MainCommands::Testing.as_str()).about("demos testing"))
        .subcommand(App::new(MainCommands::BorrowChecker.as_str()).about("demos borrow checker"))
        .subcommand(
            App::new(MainCommands::ConversionTraits.as_str()).about("demos conversion traits"),
        )
        .subcommand(App::new(MainCommands::ControlFlow.as_str()).about("demos control flow"))
        .subcommand(App::new(MainCommands::Functions.as_str()).about("demos functions"))
        .subcommand(
            App::new(MainCommands::GenCompletions.as_str())
                .about("get completions")
                .arg(
                    Arg::with_name("shell")
                        .help("shell to generate for")
                        .required(true),
                ),
        )
}
pub fn gen_completions(arg_matches: &ArgMatches) {
    let shell_str = arg_matches.value_of("shell").unwrap();
    match Shell::from_str(shell_str) {
        Ok(shell) => {
            build_cli_app().gen_completions_to("rust-playground-cli", shell, &mut io::stdout())
        }
        Err(e) => {
            eprintln!("Sorry, shell \"{}\" not recognised {}", shell_str, e);
            process::exit(1);
        }
    }
}
