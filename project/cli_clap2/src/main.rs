#![allow(unused)]
extern crate clap;

#[macro_use]
extern crate slog;

use std::process;
use clap::{Arg, App, load_yaml, ArgMatches, Command, Parser };
use std::ffi::OsString;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
    input: String,
}

// fn run(matches: ArgMatches) -> Result<(), String> {
//     let min_log_level = match matches.occurrences_of("verbose"){

//     }
// }

fn main() {

    let yaml = loal_yaml!("config.yaml");


    let matches = Command::new("KevinApp")
        .version("1.0")
        .author("Kevin Park. <hyunbin7303@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::new("CONFIG")
            .short('c')
            .long("config")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::new("INPUT")
            .short('i')
            .long("INPUT")
            .help("Sets the input file to use")
            .takes_value(true))
            // .index(1))
        .subcommand(Command::new("test")
            .about("controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg(Arg::new("algorithm")
                .short('a')
                .help("Algorithm to use")
                .required(true)
                .value_name("ALGORITHM")))
        .subcommand(Command::new("kevin")
            .about("Kevin testing subcommand")
            .version("1.0")
            .author("Kevin Park. hyunbin7303@gmail.com")
            .arg(Arg::new("KEVIN")
                .short('k')
                .help("Kevin study clap!")
                .required(false)
                .value_name("KEVIN PARK")))

        .subcommand(Command::new("run")
            .about("Runs the Foo server")
            .arg(Arg::new("debug")
                .short('D')
                .help("Sends debug foos instead of normal foos.")))
        .subcommands( vec![
            Command::new("config").about("Controls configuration functionality").arg(Arg::with_name("config_file").index(1)),
            Command::new("debug").about("Controls debug functionality")
        ])
    .get_matches();


    println!("Using input file: {}", matches.value_of("INPUT").unwrap());
    let config_file = matches.value_of("CONFIG").unwrap_or("config.json");
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("verbose") {
            println!("Printing verbosely...");
        }else {
            println!("Printing normally...");
        }
    }
    if let Some(matches) = matches.subcommand_matches("kevin"){
        if matches.is_present("check"){
            println!("Printing check ...");
        }else {
            println!("Not printing check...");
        }
    }


}