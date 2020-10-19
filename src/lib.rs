//! # Rust Playground CLI
//!
//! `rust-playground-cli` is an exploration of the rust language as a learning exercise. This crate
//! will follow a similar structure to "rust by example", so you should be able to translate
//! between the two fairly easily
//!
//! The CLI part of this app is built with the clap crate, with code examples taken from [their
//! examples](https://github.com/clap-rs/clap/blob/master/examples/20_subcommands.rs)
//!
//! Note that this style of comment explains the enclosing scope, i.e. the crate

use clap::ArgMatches;

pub mod borrow_checker;
mod cli;
pub mod custom_types;
pub mod format_print;
pub mod primitives;
pub mod testing;
mod types;

use types::MainCommands;

// Comments
/*
 * This is another type of comment, a block comment. Useful for commenting out
 * chunks of code. /* Block comments can be /* nested, */ */
 */

/// This is for playing around roughly in line with rust by example
/// Common docs include Examples, Panics, Errors, Safety

pub fn run_cli() {
    let matches = cli::build_cli_app().get_matches();
    handle_matches(matches)
}

fn handle_matches(matches: ArgMatches) {
    let (cmd, matches_opt) = matches.subcommand();
    let subcmd_matches = (MainCommands::from_str(cmd).unwrap(), matches_opt);

    match subcmd_matches {
        (MainCommands::FormatPrint, _) => format_print::methods::show_print(),
        (MainCommands::Primitives, _) => primitives::methods::show_primitives(),
        (MainCommands::CustomTypes, _) => custom_types::methods::show_custom_types(),
        (MainCommands::Testing, _) => testing::methods::show_testing(),
        (MainCommands::BorrowChecker, _) => borrow_checker::methods::show_borrow_checker(),
        (MainCommands::GenCompletions, Some(sub_matches)) => cli::gen_completions(sub_matches),
        // If all subcommands are defined above, anything else is unreachabe!()
        (_, None) => unreachable!(),
    }
}
