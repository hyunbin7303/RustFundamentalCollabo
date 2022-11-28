




pub mod config_handle {
    use std::fs;
    use std::error::Error;
    
    
    pub struct Config {
        pub query: String,
        pub filename: String,
        pub search_word: Option<String>,
    }
    impl Config {
        fn new(args: &[String]) -> Result<Config, &str>{
            if args.len() < 2 {
                return Err("Not enough arguments. Please check the requirement.");
            }
            let query = args[1].clone();
            if query == "search" || query == "SEARCH" {

            }
            if query == "" {

            }
            let filename = args[2].clone();
            let mut search_word: Option<String> = Option::None;
            if args.len() >= 3 {
                search_word = Some(args[3].clone());
            }
            Ok(Config {query, filename,search_word})
        }
    }
    pub fn parse_config(args: &[String]) -> Config {
        
        let query = args[1].clone();
        let filename = args[2].clone();
        let mut search_word: Option<String> = Option::None;
        if args.len() >3 {
            search_word = Some(args[3].clone());
        }
        Config { query, filename,search_word }
    }
    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;
        println!("With text : \n{}", contents);
        Ok(())
    }
}

pub mod file_util {
    use std::fs::File;
    use same_file::Handle;
    use std::path::Path;
    use std::io::{Write,BufReader, BufRead, ErrorKind};
    use std::fmt::Display;

    pub fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
        (s, length)
    }
    pub fn generate_txtfile(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let s = filename.to_string();
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
    fn print_vec<T:Display>(input: &Vec<T>){
        for item in input{
            println!("{}", item);
        }
        println!();
    }
    pub fn trim_newline(s: &mut String){
        if s.ends_with('\n'){
            s.pop();
            if s.ends_with('\r'){
                s.pop();
            }
        }
    }
    pub fn read_textfile_print(filename: &str) -> Result<(), Box<dyn std::error::Error>>{
        //trim_newline(&filename);
        let mut s = filename.to_string();
        s.pop();
    
        let path_to_read = Path::new(&s);
        let stdout_handle = Handle::stdout()?;
        let handle = Handle::from_path(path_to_read)?;
    
        if handle == stdout_handle {
            // return Err("You are reading and writing to the same file.");
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
    pub fn read_textfile(filename: &str) -> String {
        let data = std::fs::read_to_string(filename).expect("Unable to read file");
        data
    }
    pub fn remove_textfile(filename: &str) -> Result<(), Box<dyn std::error::Error>>{
        
        
        Ok(())
    }
    pub fn insert_str_front(s: &mut String, input_str: String){
        s.insert_str(0, &input_str,)
    }
    pub fn check_file_exist(filename: &str) -> bool {
        let s = filename.to_string();
        // s.pop();
        let file = std::path::Path::new(&s).exists();
        file
    }

    // source from https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html
    pub fn parse_csv_document(filename: &str) -> std::io::Result<Vec<Vec<String>>> {
        let f = File::open(&filename)?;
        let f = BufReader::new(f);
        f.lines()
            .map(|line| {
                // For each line in the source
                line.map(|line| {
                    // If the line was read successfully, process it, if not, return the error
                    line.split(',') // Split the line separated by commas
                        .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                        .collect() // Collect all strings in a row into a Vec<String>
                })
            })
            .collect() // Collect all lines into a Vec<Vec<String>>
    }
    
    pub fn print_line_at(filename: &str, line_num: usize) {
        if line_num < 1 {
            panic!("Line number has to be > 0");
        }
        let line_num = line_num - 1;
        let file = File::open(filename).expect("File not found or cannot be opened");
        let content = BufReader::new(&file);
        let mut lines = content.lines();
        let line = lines.nth(line_num).expect("No line found at given position");
        println!("{}", line.expect("None line"));
    }
}

