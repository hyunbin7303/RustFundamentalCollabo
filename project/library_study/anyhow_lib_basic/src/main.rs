
// Rust doesn't have exceptions, but it has the type Result<T,E> for recoverable errors
// and the panic! macro that stops execution when the program encounters an unrecoverable error.
// panic! - causes the error msg contained in the last



// reocoverable Errors with Result
// enum Result<T,E>
// T - Represents the type of the value that will be returned in a success case within the OK variant.


#[derive(Debug)]
struct ErrorA;

#[derive(Debug)]
struct ErrorB;

fn returns_error_A() -> Result<bool, ErrorA> {
    Err(ErrorA)
}


fn main() {
    println!("Hello, world!");


}
