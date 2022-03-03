
use std::path::Path;
use std::fs::File;
use std::fs;
use std::io;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter, yaml::Array};

use serde_yaml;



//https://stackoverflow.com/questions/53243795/how-do-you-read-a-yaml-file-in-rust

#[derive(Debug)]
pub struct RTM<'a> {
    pub config : &'a Array
}

impl<'a> RTM<'a> {
    pub fn run(&self) {
        let gus_config = &self.config[0];
        println!("{:?}", gus_config[0]);
    }
}


fn testing_serde_yaml() -> Result<(), Box<dyn std::error::Error>>{
    let f = std::fs::File::open("something.yaml")?;
    let d: String = serde_yaml::from_reader(f)?;
    println!("Read Yaml string : {}", d);
    Ok(())
}


fn main() {
    println!("Hello, world!");
    let s = 
    "
    foo:
        - list1
        - list2
    bar:
        - 1
        - 2.0
    ";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];
    println!("{:?}", doc);

    // println!("{}", doc["foo"][0].as_str().unwrap()); 

    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
    }
    println!("{}", out_str);

    static CONFIG_STRING: &'static str = "test.yaml";
   // let cfg = get_yaml_config(&CONFIG_STRING);


    let path_to_file = Path::new("test.yaml");
    let display = path_to_file.display();
    let mut fd  = File::open(&path_to_file);
    let mut content = String::new();
    fd.read_to_string("test.yaml");


}



// pub fn get_yaml_config(config_file: &str) -> Array {
//     let path_to_file = Path::new(&config_file);
//     let display = path_to_file.display();
//     let mut fd = match File::open(&path_to_file) {
//         Err(why) => panic!("couldn't open {}:", display),
//         Ok(file) => file
//     };

//     let mut content = String::new();

//     // fd.read_to_string()
//     match fd.read_to_string("test.yaml") {
//         Err(why) => panic!("couldn't read {}:", display),
//         Ok(_) => println!(""),
//     };
//     // return Array
//     // yaml_loader::load_from_str(&content).unwrap()
// }




