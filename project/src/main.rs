use std::env;
use std::fs;
use std::process;
use std::error::Error;
use std::io;


//https://images.cafonline.com/image/upload/caf-dev/wlwor80q15xpomqeqxjm.pdf
//https://www.youtube.com/watch?v=OkmZc_H0NNo&list=PL7r-PXl6ZPcB4jn1_VR3D8tSK9DxOaiQE&index=3
//https://www.youtube.com/watch?v=8uoPNVPUrpQ&ab_channel=DodgyCoding


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


fn main() {
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



