use crate::garden::vegetables::Asparagus;
use crate::garden::people::Student;

pub mod garden;

fn main() {
    println!("Hello, world!");
    let plant = Asparagus {
        name: String::from("Asparagus"),
        stalks: 5,
    };
}


// checkout this project - https://github.com/superjose/rust-include-files-example/tree/master

