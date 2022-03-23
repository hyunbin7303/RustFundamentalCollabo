mod word;

use std::io::prelude::*;
use std::fmt::Display;
use std::env::args;
use WordFinder::config_handle::{ parse_config };
use WordFinder::file_util::*;
//TODO
//1. Command line : Search for the specific word in the text file/json file.
// Word counts, How many words in the txt file?
// Based on the input file from the config file? // How to work on using the txt file.????
//2. How to use trait in this Rust file?????
// Use Closure at least once????????????
// Use Fn : Itcannot modify the objects it captures
// Use FnMut : It can modify the objects it captures
// Use FnOnce : The most restricted. Can only ba called once because when it is called it consumes
// itself and its captures.


// These are just an idea.
//1. Sentence finder. Find the all sentense that uses the specific word.
//2. Read json file of the dictionary.
//3. COnfigure with my ENglish sentence..... The sentence i use, it should be stored in the text file.

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

fn print_vec<T:Display>(input: &Vec<T>){
    for item in input{
        println!("{}", item);
    }
    println!();
}
#[derive(Debug)]
struct Sentence {
    words: Vec<word::Word>,
}

fn main() -> std::io::Result<()> {
    let args_vec : Vec<String> = args().collect();
    let config = parse_config(&args_vec); 
    if config.query == "--help" || config.query == "-h" || config.query == "-H" {
        println!("Command list. ");
        println!("Type command for the file handling.");
    }

    println!("Action : {}", config.query);

    if config.query == "dictionary" {
        //TODO get the word from the json file? 

        let word_apple = word::Word::new("APPLE", "the round fruit of a tree of the rose family, which typically has thin red or green skin and crisp flesh. Many varieties have been developed as dessert or cooking fruit or for making cider.", "");
        let word_upgrade = word::Word::new("UPGRADE","raise (something) to a higher standard, in particular improve (equipment or machinery) by adding or replacing components.", "BOOST" );
        let words = vec![word_apple, word_upgrade];
        let sentences = Sentence::from(words);
        sentences.print_words();
    
    }
    else if config.query == "search" || config.query == "-s" || config.query == "-S" {
        println!("Searching for the specific word?");
    }
    else if config.query == "create" || config.query == "-c" || config.query == "-C" {

    }
    else if config.query == "delete" || config.query == "-d" || config.query == "-D" {
        let is_exist = check_file_exist(&config.filename);
        if is_exist {
            remove_textfile(&config.filename);
        }else{
            println!("File doesn't exist.");
        }
    }
    else{
        println!("In file : {}", config.filename);
        println!("Search word or stirng. {}", config.search_word);
    
        let mut txt_name = String::new();
        println!("Type your text file name.");
        let bl = std::io::stdin().read_line(&mut txt_name).unwrap();
    
        // Checking if file exists in the directory.
        let is_exist = check_file_exist(&txt_name);
        if is_exist {
            println!("File Exists!");
            read_textfile(&txt_name); // put test.txt file in here for testing.
        }else{
            println!("File doesn't exist. Create file : {}", txt_name);
            generate_txtfile(&txt_name); 
        }
    }
    Ok(())
}
//https://superuser.com/questions/886132/where-is-the-zshrc-file-on-mac