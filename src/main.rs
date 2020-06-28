use clap::{App, Arg};

fn main() {
    let matches = App::new("Rust Playground CLI")
        .subcommand(
            App::new("test").arg(
                Arg::new("config")
                    .short('c')
                    .long("config")
                    .value_name("FILE")
                    .about("Sets a custom config file")
                    .takes_value(true),
            ),
        )
        .get_matches();

    println!("Value for config: {:?}", matches);

    if let Some(ref matches) = matches.subcommand_matches("test") {
        if matches.is_present("config") {
            println!("Test with config");
        } else {
            println!("Normal test");
        }
    }
}
