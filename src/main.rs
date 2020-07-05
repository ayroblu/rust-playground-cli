// https://github.com/clap-rs/clap/blob/master/examples/20_subcommands.rs
use clap::ArgMatches;

mod cli;
mod format_print;
mod structs;
mod testing;
mod types;

use types::MainCommands;

// Comments
/*
 * This is another type of comment, a block comment. Useful for commenting out
 * chunks of code. /* Block comments can be /* nested, */ */
 */

/// This is for playing around roughly in line with rust by example
/// Common docs include Examples, Panics, Errors, Safety
fn main() {
    let matches = cli::build_cli_app().get_matches();
    handle_matches(matches)
}

fn handle_matches(matches: ArgMatches) {
    // The most common way to handle subcommands is via a combined approach using
    // `ArgMatches::subcommand` which returns a tuple of both the name and matches
    let (cmd, matches_opt) = matches.subcommand();
    let subcmd_matches = (MainCommands::from_str(cmd).unwrap(), matches_opt);

    match subcmd_matches {
        (MainCommands::FormatPrint, _) => format_print::methods::show_print(),
        (MainCommands::Testing, _) => testing::methods::show_testing(),
        (MainCommands::Structs, _) => structs::methods::show_struct(),
        (MainCommands::GenCompletions, Some(sub_matches)) => cli::gen_completions(sub_matches),
        // If all subcommands are defined above, anything else is unreachabe!()
        (_, None) => unreachable!(),
    }
}
