use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::fmt::Display;
pub fn parse_csv_document(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
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

fn main() -> io::Result<()> {
    let f = File::open("foo.txt")?;
    let f = BufReader::new(f);

    // for line in f.lines() {
    //     println!("{}", line.unwrap());
    // }


    let checking = parse_csv_document(f);
    for lines in checking {
        println!("{:?}", lines);

        // for line in lines {
        //     println!("{:?}", line);
        // }
    }
    Ok(())
}