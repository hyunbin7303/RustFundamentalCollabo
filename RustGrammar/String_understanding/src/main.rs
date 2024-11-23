use std::{collections, fmt};
fn merge_two_string(s1: &String, s2: &String) -> String {
    //Create a String from a literal string with String::from
    let s1 = String::from(s1);
    let s2 = String::from(s2);
    let s3 = s1 + &s2; // s1 + s2 is not valid.
    // Why? we can only add a &str to a String; we can’t add two String values together
    s3
}
// return string slice
fn get_firstword_usingString(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn firstword_usingSlice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// you have a string and want to change the string within a function, use &mut String for the argument
fn modify_string(s: &mut String) {
    s.push_str(", By Kevin Park");
}

fn main() {
    let mut s1 = String::with_capacity(25);
    println!("Size of the String: {}", std::mem::size_of_val(&s1));
    println!("{:p}", s1.as_ptr());
    s1.push_str("Hello");
    println!("Size in the stack: {}", std::mem::size_of_val(&s1));
    let string_ptr= &s1;
    println!("String pointer in the stak: {:p}", string_ptr);
    println!("Heap pointer to the string content: {:p}", s1.as_ptr());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_chracters_from_string_literal(){
        let hithere = "HI kevin. This is the Rust study place."; // String literal , &str
        let get_hi = &hithere[0..2];
        assert_eq!(get_hi,"HI");
    }

    #[test]
    fn get_string_from_String_object() {
        let str_hello = String::from("Hello world");
        let hello = &str_hello[0..5];
        let world = &str_hello[6..11];
        assert_eq!(hello, "Hello");
        assert_eq!(world, "world");
    }

    #[test]
    fn merge_string_return_Helloworld() {
        let s1 = String::from("Hello");
        let s2 = String::from(", world!");
        assert_eq!(merge_two_string(&s1,&s2), "Hello, world!");
    }
    #[test]
    fn test_get_firstword_usingString_success()
    {
        let mut s = String::from("hello world");
        let word = get_firstword_usingString(&s);
        assert_eq!(word, "hello");
    }

    #[test]
    fn test_get_firstword_usingSlice_success()
    {
        let mut s = String::from("hello world");
        let word = firstword_usingSlice(&s);
        assert_eq!(word, "hello");
    }
    #[test]
    fn test_firstword_usingSlice_with_literal_string()
    {
        let literal_string = "hello world";
        let word = firstword_usingSlice(literal_string);
        let word2 = firstword_usingSlice(&literal_string[..]);

        assert_eq!(word, "hello");
        assert_eq!(word, "hello");
    }

    #[test]
    fn test_modify_string_adding_bykevin_str()
    {
        let mut s = String::from("hello world");
        modify_string(&mut s);
        assert_eq!(s, "hello world, By Kevin Park");
    }
}

//https://www.reddit.com/r/rust/comments/5t5zq1/when_to_use_str_over_string_in_a_struct/
