use std::fs::File;
use std::io::prelude::*;
use std::io::{Write,BufReader, BufRead, Error, ErrorKind};
use same_file::Handle;
use std::path::Path;
use std::fmt::Display;
use std::env::args;
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


impl Default for Word {
    fn default () -> Word {
        Word{letter: "".to_string(), meaning: "".to_string(), synonym: "".to_string()}
    }
}

impl From<Vec<Word>> for Sentence {
    fn from(words: Vec<Word>) -> Self {
        Self { words }
    }
}
//path: std::path::PathBuf,
struct Config {
    query: String,
    filename: String,
    search_word: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() <3 {
            return Err("Not enough arguments. Please check the requirement.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let search_word = args[3].clone();
        Ok(Config {query, filename,search_word})
    }
}
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    let search_word= args[3].clone();
    Config { query, filename,search_word }
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
fn generate_txtfile(filename: &str) -> Result<(), Error> {
    let mut s = filename.to_string();
    s.pop();
    let mut file = File::create(&s)?;

    // Get user input by the user.
    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input_str)
            .expect("Failed to read line");

    insert_str_front(&mut input_str,"KEVIN PARK. Future Rust Software Developer.\n".to_string());

    let (s2, len) = calculate_length(input_str);
    println!("The length of the whole string : {}", len);
    file.write_all(s2.as_bytes())?;
    Ok(())
}
fn trim_newline(s: &mut String){
    if s.ends_with('\n'){
        s.pop();
        if s.ends_with('\r'){
            s.pop();
        }
    }
}
fn read_textfile(filename: &str) -> Result<(), Error> {
    //trim_newline(&filename);
    let mut s = filename.to_string();
    s.pop();

    let path_to_read = Path::new(&s);
    let stdout_handle = Handle::stdout()?;
    let handle = Handle::from_path(path_to_read)?;

    if handle == stdout_handle {
        return Err(Error::new(ErrorKind::Other, "You are reading and writing to the same file."));
    }
    else{
        let file = File::open(&path_to_read);
        let file = BufReader::new(file?);
        for(num , line) in file.lines().enumerate(){
            println!("{} : {}", num, line?.to_uppercase());
        }
    }
    Ok(())
}
fn insert_str_front(s: &mut String, input_str: String){
    s.insert_str(0, &input_str,)
}
fn check_file_exist(filename: &str) -> bool {
    let mut s = filename.to_string();
    s.pop();
    let file = std::path::Path::new(&s).exists();
    file
}
// count words 
fn count_words(word: &str) -> i32 {
    let mut total = 0;
    let mut previous = char::MAX;
    0
}
fn print_vec<T:Display>(input: &Vec<T>){
    for item in input{
        println!("{}", item);
    }
    println!();
}
#[derive(Debug)]
struct Sentence {
    words: Vec<Word>,
}
#[derive(Debug)]
struct Word {
    letter : String,
    meaning: String,
    synonym: String,
}
impl Word {
    fn new(letter: &str, meaning: &str, synonym: &str)-> Self{
        Self {
            letter : letter.to_string(),
            meaning : meaning.to_string(),
            synonym: synonym.to_string()
        }
    }
}

fn main() -> std::io::Result<()> {
    let w1 = Word::default();
    let x = Some("air").unwrap();


    let pattern = args().nth(1).expect("No Pattern given");
    let args_vec : Vec<String> = args().collect();
    let config = parse_config(&args_vec); 
    println!("Action : {}", config.query);

    if config.query == "dictionary"{

        //get the word from the json file? 

        let word_apple = Word::new("APPLE", "the round fruit of a tree of the rose family, which typically has thin red or green skin and crisp flesh. Many varieties have been developed as dessert or cooking fruit or for making cider.", "");
        let word_upgrade = Word::new("UPGRADE","raise (something) to a higher standard, in particular improve (equipment or machinery) by adding or replacing components.", "BOOST" );
        let words = vec![word_apple, word_upgrade];
        let sentences = Sentence::from(words);
        sentences.print_words();
    
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
    // use insert_str_front 
    // And store again
    Ok(())
}
