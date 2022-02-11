use std::fs::File;
use std::io::prelude::*;
use std::io::{Write,BufReader, BufRead, Error, ErrorKind};
use same_file::Handle;
use std::path::Path;
use std::env::args;
//TODO
//1. Read file and parse it by / or | 
//2. Find the specific word from the array
//3. Generate the random number and store it into the text file.
//4. Use Different types of Collection 
//5. Send the data through the nextwork? 
// -------------------------------------------

// Use the argument parsing as well. 
//https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html

// merge testApp to this one. 
// how to use closure in this application? 

// Use Fn : Itcannot modify the objects it captures
// Use FnMut : It can modify the objects it captures
// Use FnOnce : The most restricted. Can only ba called once because when it is called it consumes
// itself and its captures.



struct Config {
    query: String,
    filename: String,
}
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename}
}



fn whitespace_test(c: char) -> bool {
    return c == ' ' || c == '\n';
}
fn find_inputword(input: &str) -> &str {
    ""
}

fn generate_txtfile() -> Result<(), Error> {
    let mut file = File::create("test.txt")?;
    file.write_all(b"Hello, World! \nhow are you? my name is Kevin.")?;
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


fn main() -> std::io::Result<()> {

    let input = args();
    input.skip(1).for_each(|item| {
        println!("You wrote {}, which in capital letters in {}", item, item.to_uppercase());
    });

    // let opt: Option<String> = Some("Some value".to_owned());
    // let value = opt.as_deref().unwrap_or("default string");

    let mut txt_name = String::new();
    println!("Type your text file name.");
    let bl = std::io::stdin().read_line(&mut txt_name).unwrap();
  //  generate_txtfile();
    read_textfile(&txt_name); // put test.txt file in here for testing.
    
    // use insert_str_front 
    // And store again
    Ok(())
}
