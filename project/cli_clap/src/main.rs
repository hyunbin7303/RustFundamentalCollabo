extern crate clap;



use std::process;
use clap::{Arg, App, ArgMatches, SubCommand };
use std::ffi::OsString;



#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;


#[derive(Debug, PartialEq)]
struct HelloArgs {
    name: String,
    // field1: Option<String>
}

impl HelloArgs {
    fn new() -> Self {

        Self::new_from(std::env::args_os().into_iter()).unwrap_or_else(|e| e.exit())
    }

    fn new_from<I, T>(args: I) -> Result<Self, clap::Error>
        where
            I: Iterator<Item = T>,
            T: Into<OsString> + Clone,
        {
            let app = App::new("hello-clap")
            .version("1.0")
            .about("Says hello")
            .author("Hyunbin Park");

            // Define the name command line option
            let name_option = Arg::with_name("name")
            .long("name") // allow --name
            .short('n') // allow -n
            .takes_value(true)
            .help("Who to say hello to")
            .required(true);

            // now add in the argument we want to parse
            let app = app.arg(name_option);
            // extract the matches
            let matches = app.get_matches_from_safe(args)?;

            let name = matches
                .value_of("name")
                .expect("This can't be None, we said it was required");
            
                Ok(HelloArgs {
                    name: name.to_string(),
                })
        }
}



trait Prints {
    fn prints_something() {
        println!("I like to print something");
    }
}
struct Person;
struct Building;

fn main() {
    // Blanket Trait Implementation? 
    let my_person = Person;
    let my_building = Building;

    let hello = HelloArgs::new();
    println!("Hello, {}!", hello.name);
}




#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_no_args(){
        HelloArgs::new_from(["exename"].iter()).unwrap_err();
    }
    #[test]
    fn test_incomplete_name(){
        HelloArgs::new_from(["exename", "--name"].iter()).unwrap_err();
    }

    #[test]
    fn test_complete_name(){
        assert_eq!(
            HelloArgs::new_from(["exename", "--name", "HelloKevin"].iter()).unwrap(),
            HelloArgs { name : "HelloKevin".to_string()}
        );
    }

    #[quickcheck]
    fn prop_never_panics(args: Vec<String>) {
        let _ignored = HelloArgs::new_from(args.iter());
    }


}