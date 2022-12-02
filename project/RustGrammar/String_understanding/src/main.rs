fn change_string(any_string: &mut String){
    any_string.push_str(",Testing");
}
// return string slice
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn first_word_2(s: &str) -> &str {
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
    let str_hello = String::from("Hello world");
    let hello = &str_hello[0..5];
    let world = &str_hello[6..11];
    println!("{}", hello);
    println!("{}", world);
    println!("{}", str_hello);
    let hithere = "HI kevin. This is the Rust study place."; // String literal , &str
    // Fixed sizem &str is a reference to a sequence of UTF-8 bytes. -> Immutable reference.
    // its type is &'static str.  String literal is a string slice pointing to that specific point of the binary.

    // &str is a simple string. 
    // let my_var = "Hello" -> you create a &str. which is very fast.println!
    // String is a pointer, with data on the heap.println!

    let hit = &hithere[0..2];
    println!("{}", hit);
    let hit2 = hit;
    println!("{}", hit2);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word()
    {
        let mut s = String::from("hello world");
        let word = first_word(&s); 
        assert_eq!(word, "hello");
    }

    #[test]
    fn test_first_word_2()
    {
        let mut s = String::from("hello world");
        let word = first_word_2(&s);
        assert_eq!(word, "hello");
    }
    #[test]
    fn test_first_word_2_with_literal_string()
    {
        let literal_string = "hello world";
        let word = first_word_2(literal_string);
        let word2 = first_word_2(&literal_string[..]);

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