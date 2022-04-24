mod word;

use std::env::args;
use std::process;
use WordFinder::config_handle::{ parse_config };
use WordFinder::file_util::*;
use std::collections::HashMap;
extern crate serde_json;
use serde_json::{Value, Map};

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
        word::Word{letter: "".to_string(), meaning: "".to_string(), synonyms: Vec::new() }
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
        let words = {
            let text = std::fs::read_to_string("simple_english_dictionary.json").unwrap();
            let dic = serde_json::from_str::<Map<String, Value>>(&text).unwrap();
            dic
        };
        match config.search_word {
            Some(x) => println!("{}", words[&x]),
            None => println!("Nothing for the search word")
        };
        
    }
    else if config.query == "search" || config.query == "-s" || config.query == "-S" {
        if config.search_word == None {
            println!("Please type search word.");
            process::exit(1);
        }
        println!("Search file name : {}", &config.filename);
        let is_exist = check_file_exist(&config.filename);
        if is_exist {
            let contents = read_textfile(&config.filename);
            println!("{}", contents);
            let search_word = &config.search_word.unwrap();
            let index:Option<usize> = contents.find(search_word).map(|i| i+1);
            println!("first Index number : {}", index.unwrap());
            let index_vec: Vec<_> = contents.match_indices(search_word).map(|(i, _)| i+1).collect();
            println!("All index numbers {:?}", index_vec);

            // TODO : Find the specific lines of the string. 
            // Display the line numbers to the screen. 
            
        }else{
            println!("File Doesn't exist. Please check the file name.");
        }
    }
    else if config.query == "csv-handler" {
      parse_csv_document(&config.filename);
      let is_exist = check_file_exist(&config.filename);
      if is_exist {

      }else {
          println!("File doesn't exist. Please check the file name.");
          process::exit(1);
      }
    }
    else if config.query == "print-line"{
        println!("Testing. Just printout the first line.");
        print_line_at(&config.filename, 1);
    }
    else if config.query == "search-sentence" {
        //TODO 
        //Read the specific file you mentioned.

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
            if let Err(e) = read_textfile_print(&config.filename) {
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
        read_jsonfile();
    }
}