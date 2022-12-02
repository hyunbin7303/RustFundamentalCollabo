fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 + s2 is not valid. 
    // Why? we can only add a &str to a String; we can’t add two String values together
    println!("{}", s3);
}
