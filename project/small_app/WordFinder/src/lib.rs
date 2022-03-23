
 pub mod config_handle {
    use std::fs;
    use std::error::Error;
    
    
    pub struct Config {
        pub query: String,
        pub filename: String,
        pub search_word: String,
    }
    impl Config {
        fn new(args: &[String]) -> Result<Config, &str>{
            if args.len() <3 {
                return Err("Not enough arguments. Please check the requirement.");
            }
            let query = args[1].clone();
            if query == "search" || query == "SEARCH" {

            }
            if query == "" {

            }
            let filename = args[2].clone();
            let search_word = args[3].clone();
            Ok(Config {query, filename,search_word})
        }
    }
    pub fn parse_config(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        let search_word= args[3].clone();
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


    pub fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
        (s, length)
    }
    pub fn generate_txtfile(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn trim_newline(s: &mut String){
        if s.ends_with('\n'){
            s.pop();
            if s.ends_with('\r'){
                s.pop();
            }
        }
    }
    pub fn read_textfile(filename: &str) -> Result<(), Box<dyn std::error::Error>>{
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
    
    pub fn remove_textfile(filename: &str) -> Result<(), Box<dyn std::error::Error>>{
        Ok(())
    }
    
    pub fn insert_str_front(s: &mut String, input_str: String){
        s.insert_str(0, &input_str,)
    }
    pub fn check_file_exist(filename: &str) -> bool {
        let mut s = filename.to_string();
        s.pop();
        let file = std::path::Path::new(&s).exists();
        file
    }
}

