// extern crate File;

use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct ErrorA;

#[derive(Debug)]
struct ErrorB;
fn read_file_string(path: &str) -> Result<String,Box<dyn std::error::Error>>{
    let data = fs::read_to_string(path)?;
    Ok(data)
}

fn read_file_string_from_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file_line_by_line(path: &str) -> Result<(), Box<dyn std::error::Error>>{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines(){
        println!("{}", line?);
    }
    Ok(())
}
fn read_file_vec(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>>{
    let data = fs::read(path)?;
    Ok(data)
}

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let f = File::open(path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// fn read_file_buffer(path: &str) -> Result<(), Box<dyn std::error::Error>>{
//     const BUF_LEN: usize =512;
//     let mut buf = [0u8; BUF_LEN];
//     let mut file = File::open(path)?;

//     loop {
//         let read_count = file.read(&mut buf)?;
//         do_something(&buf[..read_count]);
//         if read_count != BUF_LEN {
//             break;
//         }
//     }
//     Ok(())
// }


// the Box<dyn Error> type os called a trait object
// allows for values of different types.println!

fn main() {

    let s1 = read_file_string("test.txt");
    println!("{}", s1.unwrap());

    let s2 = read_file_string_from_file("test.txt");
    println!("{}", s2.unwrap());

    println!("Read file line by line testing.-------");
    if let Err(e) = read_file_line_by_line("test.txt") {
        println!("{}", e); // "There is an error: Oops"
    }

    let path_to_read = Path::new("new.txt");
    let stdout_handle = Handle::stdout()?;
    let handle = Handle::from_path(path_to_read)?;
    // TODO : https://docs.rs/memmap/0.7.0/memmap/struct.Mmap.html#method.map

}


#[cfg(test)]
mod tests {
    use super::*;
    // TODO : Make some test cases. 
    
}