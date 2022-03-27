mod word;

use std::env::args;
use std::process;
use WordFinder::config_handle::{ parse_config };
use WordFinder::file_util::*;
use std::collections::HashMap;
extern crate serde_json;

use serde_json::{Value, Map};
// Use Fn : Itcannot modify the objects it captures
// Use FnMut : It can modify the objects it captures
// Use FnOnce : The most restricted. Can only ba called once because when it is called it consumes


// Use the argument parsing as well. 
//https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html

// Use Function/ FnONce, 
//https://stackoverflow.com/questions/36390665/how-do-you-pass-a-rust-function-as-a-parameter

impl Sentence {
    fn print_words(&self) {
        for word in &self.words {
            println!("{:?} means {:?}", word.letter, word.meaning);
        }
    }
}
impl Default for word::Word {
    fn default () -> word::Word {
        word::Word{letter: "".to_string(), meaning: "".to_string(), synonym: "".to_string()}
    }
}

impl From<Vec<word::Word>> for Sentence {
    fn from(words: Vec<word::Word>) -> Self {
        Self { words }
    }
}

pub fn read_jsonfile() {

    let data = std::fs::read_to_string("./simple_english_dictionary.json").expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&data).expect("JSON does not have correct format.");
    // let map: Map<String, String> = serde_json::from_value(json).unwrap();
    // json[0]

    let mut scores = HashMap::new();
    scores.insert(String::from("Word"), String::from("example"));
    println!("{}", json);
}
#[derive(Debug)]
struct Sentence {
    words: Vec<word::Word>,
}

fn main() -> std::io::Result<()> {

    //Testing 
    //read_jsonfile();

    let args_vec : Vec<String> = args().collect();
    let config = parse_config(&args_vec); 
    if config.query == "--help" || config.query == "-h" || config.query == "-H" {
        println!("Command list. ");
        println!("Type command for the file handling.");
    }
    println!("Action : {}", config.query);

    if config.query == "dictionary" {
        // TODO get the word from the json file? 
        // TODO ask for the specific word to the user.
        // TODO Display to the user.
        let word_apple = word::Word::new("APPLE", "the round fruit of a tree of the rose family, which typically has thin red or green skin and crisp flesh. Many varieties have been developed as dessert or cooking fruit or for making cider.", "");
        let word_upgrade = word::Word::new("UPGRADE","raise (something) to a higher standard, in particular improve (equipment or machinery) by adding or replacing components.", "BOOST" );
        let words = vec![word_apple, word_upgrade];
        let sentences = Sentence::from(words);
        sentences.print_words();
    
    }
    else if config.query == "search" || config.query == "-s" || config.query == "-S" {
        println!("Searching for the specific word?");
        //TODO Searching the specific word from the file? 
        //TODO Command line : Search for the specific word in the text file/json file.
        // Word counts, How many words in the txt file?
        // Based on the input file from the config file? // How to work on using the txt file.????
    }
    else if config.query == "search-sentence" {
        //TODO Sentence finder. Find the all sentense that uses the specific word.
        // TODO Get sentence from the user.
    }
    else if config.query == "create" || config.query == "-c" || config.query == "-C" {
        let is_exist = check_file_exist(&config.filename);
        if is_exist {
            println!("File Already Exists.");
        }else{
            if let Err(e) = generate_txtfile(&config.filename) {
                println!("{}", e); // "There is an error: Oops"
                process::exit(1);
            }
            
        }
    }
    else if config.query == "read" || config.query == "-r" || config.query == "-R" {
        let is_exist = check_file_exist(&config.filename);
        if is_exist {
            if let Err(e) = read_textfile(&config.filename) {
                println!("{}", e); // "There is an error: Oops"
                process::exit(1);
            }
        } else {
            println!("File doesn't exist.");
        }
    }
    else if config.query == "delete" || config.query == "-d" || config.query == "-D" {
        let is_exist = check_file_exist(&config.filename);
        if is_exist {
            if let Err(e) = remove_textfile(&config.filename) {
                println!("{}", e); 
                process::exit(1);
            }
        }else{
            println!("File doesn't exist.");
        }
    }
    else{
        println!("Please check your input.");
    }
    Ok(())
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works(){
        let txt = "hello world kevin";
        // assert_eq!(txt, result);

    }
}