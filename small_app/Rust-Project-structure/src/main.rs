mod garden;
mod school;

use garden::vegetables::Asparagus;
use school::people::Student::Student;
use school::people::Teacher::Teacher;
fn main() {
    println!("Hello, world!");
    let plant = Asparagus {
        name: String::from("Asparagus"),
        stalks: 5,
    };

    let student = Student {
        id: 1,
        name: String::from("hyunbin park"),
    };

    let teacher = Teacher {
        id: 1,
        name: String::from("Macy Horvath"),
    };


}


// checkout this project - https://github.com/superjose/rust-include-files-example/tree/master

