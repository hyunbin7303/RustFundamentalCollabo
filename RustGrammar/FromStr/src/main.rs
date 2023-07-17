use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;


pub struct TypeV2 {
    my_str : String,
}
impl TypeV2 {
    
}


#[derive(Debug,PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')')
            .split(",")
            .collect();

        let x_fromStr = coords[0].parse::<i32>()?;
        let y_fromStr = coords[1].parse::<i32>()?;
     
        Ok(Point{x: x_fromStr, y: y_fromStr})
    }
}



fn main() {

    let p = Point::from_str("(10,20)");
    println!("{:?}", p);

}


// example from https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html
