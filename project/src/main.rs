use std::env;
use std::fs;
use std::process;
use std::error::Error;
use std::io;

fn calculate_length_using_addr(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn insert_str_front(s: &mut String, input_str: String){
    s.insert_str(0, &input_str,)
}

fn gives_ownership() -> String {             // gives_ownership will move its
let some_string = String::from("yours"); // some_string comes into scope
some_string                              // some_string is returned and
}

fn main() {

    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str)
            .expect("Failed to read line");
    let (s2, len) = calculate_length(input_str);
    println!("String :{} Length: {}", s2, len);

    let s = gives_ownership();
    println!("s:{}", s);
    //Rust guarantees memory safety with a feature called ownership.  ** Rust borrow checker
    let mut s1 = String::from("abc");
    println!("S1:{}", s1);
    let str_hi= String::from("Hi");
    insert_str_front(&mut s1, str_hi);
    println!("After Insertion S1:{}", s1);


    println!("Testing App");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments : {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In  FIle {}", config.filename);
    if let Err(e) = run(config){
        println!("Application Error : {}", e);
        process::exit(1);
    }


    
}

fn run(config: Config) -> Result<(), Box<dyn Error>>
{ 
    let contents = fs::read_to_string(config.filename)?;
    println!("With text: \n{}", contents);
    Ok(())
}


struct Config{
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() <3  {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{query, filename})
    }

}





//https://images.cafonline.com/image/upload/caf-dev/wlwor80q15xpomqeqxjm.pdf
//https://www.youtube.com/watch?v=OkmZc_H0NNo&list=PL7r-PXl6ZPcB4jn1_VR3D8tSK9DxOaiQE&index=3
//https://www.youtube.com/watch?v=alzJsaOuUF8

// fn another_function(){
//     println!("Hello from another function");
// }
// fn fun_with_args(x : i32, y: f64){
//     println!("fun with x : {}, y : {}", x,y);
// }
// fn plus_one (x: i32) -> i32 {
//     let value = x + 20;
//     value
// }


// Rust Ownership
// Stack and Heap.
pub fn testing_ownership(){
    
}
// Smart Pointer.

// Reference and Borrowing.

// Expore the memory layout in GDB?
//Explore the Ownership and Borrowing in GDB


//Enums
// Structs

// TCP Conenctions.

// Logging and incoming Requests.

// Match Expressions.
//Advanced Error handling

//Iterating over strings.
