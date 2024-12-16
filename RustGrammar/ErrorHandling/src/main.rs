// source code from : https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html

use std::error::Error;
use std::fmt;
use std::error;

#[derive(Debug)]
struct StringError(String);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl Error for StringError {}

pub fn run() -> std::result::Result<(), Box<dyn Error>> {
    let condition = true;
    if condition {
        return Err(Box::new(StringError("HEHE".into())));
    }
    Ok(())
}

// -----------------------------------------------------------------------
type ResultDouble<T> = std::result::Result<T, DoubleError>;
#[derive(Debug, Clone)]
struct DoubleError;
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}
fn print(result: ResultDouble<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
fn double_first(vec: Vec<&str>) -> ResultDouble<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i)
        })
}
// ----------------------------------------------------------------------
type BoxResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug,Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}
impl error::Error for EmptyVec {}
fn double_first_for_box_return(vec: Vec<&str>) -> BoxResult<i32> {
    vec.first().ok_or_else(|| EmptyVec.into())
        .and_then(|s| {
            s.parse::<i32>().map_err(|e| e.into())
                            .map(|i| 2 * i)
        })

}
fn print_for_boxresult(result: BoxResult<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
//----------------------------------------------------------



fn main() {
    let numbers = vec!["42", "93", "18"];
    print(double_first(numbers));
    print(double_first(vec![]));
    print(double_first(vec!["test", "93", "18"]));

    let numbers = vec!["42", "93", "18"];
    print_for_boxresult(double_first_for_box_return(numbers));


    if let Err(e) = run() {
        println!("{}", e);
    }

}
