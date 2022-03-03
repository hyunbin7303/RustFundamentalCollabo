
extern crate clap;
use clap::{Arg, Command};


#[derive(Debug)]
struct MyAppArgs {

    help: bool,
    number: u32,
    opt_num: Option<u32>,
    width: u32,
    input: Vec<std::path::PathBuf>,
}

fn is_width(s: &str) -> Result<(), String> {
    let w: u32 = s.parse().map_err(|_| "Not a number")?;
    if w != 0 {
        Ok(())
    } else {
        Err("Width must be positive".to_string())
    }
}

fn main() {


    let enum_checks = ["Kevin", "Adam", "Julio"];


    let matches = Command::new("MyApp")
                        .version("1.0")
                        .author("Kevin Park. <hyunbin7303@gmail.com>")
                        .about("Does awesome things")
                        .arg(
                            Arg::new("number")
                            .long("number")
                            .required(true)
                            .help("Sets a number")
                            .takes_value(true),
                        )
                        .arg(Arg::new("input")
                            .help("Sets the input file to use")
                            .short('i')
                            .long("input")
                            // .required(true)
                            .takes_value(true))
                            // .index(1))
                        .arg(Arg::new("output")
                            .short('o')
                            .long("output")
                            // .required(true)
                            .takes_value(true))
                        .arg(Arg::new("config")
                            .short('c')
                            .long("config")
                            .takes_value(true))
                        .arg(Arg::new("verbose")
                            .help("Sets the level of verbosity")
                            .short('v')
                            .long("verbose")
                            .takes_value(true))

                        .arg(Arg::new("friends")
                            .help("Call all friends")
                            .long("friends")
                            // .requires("config")
                            // .conflicts_with("output")
                            .multiple(true)
                            .takes_value(true))
                        .subcommand(Command::new("test")
                                                .about("does testing things")
                                                .version("1.0")
                                                .author("Kevin Park. <hyunbin7303@gmail.com")
                                                .arg(Arg::new("debug")
                                                    .short('d')
                                                    .help("print debug info verbosely")))
                        .get_matches();

    // You can check the value provided by positional arguments, or option arguments


    if matches.is_present("debug") {
        println!("Debugging is turned on");
    }

    if let Some(i) = matches.value_of("input"){
        println!("Value for --i: {}", i);
        println!("Value for --input: {}", matches.value_of("input").unwrap());
    }
    if let Some(o) = matches.value_of("output"){
        println!("Value for --i: {}", o);      
        println!("Value for --output: {}", matches.value_of("output").unwrap());
   
        if matches.occurrences_of("debug") >2 {
            println!("Debugging mode ON. BEING MORE SPECIFIC");
        }
    }
    if let Some(ref file) = matches.value_of("config"){
        println!("Using config file: {}", file);
    }

    // // You can see how many times a particular flag or argument occurred
    // // Note, only flags can have multiple occurrences
    match matches.occurrences_of("debug") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
     }

     match matches.occurrences_of("friends"){
         0 => println!("Nothing is friends."),
         1 => println!("Friends. more than 1. "),
         2 => println!("Friends are more than 2."),
         3 | _ => println!("Checking all friends?"),
     }




    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(ref matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.is_present("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }
}
