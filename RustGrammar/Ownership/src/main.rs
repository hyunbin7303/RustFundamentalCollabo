use model::{Hands};
mod model {
    use std::fmt::Display;

    pub trait Displayable {
        fn display(&self) -> String;
    }
    enum Champ {
        Garen,
        Nami,
        Darius
    }
    impl Display for Champ {
        fn fmt(&self, f: &mut std::fmt::Formatter)-> std::fmt::Result {
            match self {
                Champ::Garen => f.write_str("A Garen"),
                Champ::Nami => f.write_str("A nami"),
                Champ::Darius => f.write_str("Darius")
            }
        }

    }

    pub struct Hands {
        left: Option<Champ>,
        right : Option<Champ>
    }
    impl Hands {
        pub fn new() -> Self {
            Self {
                left: Option::Some(Champ::Garen),
                right : Option::Some(Champ::Darius)
            }
        }
        pub fn juggle(mut self)-> Self{
            println!("Let's juggle!");
            let air = self.left;
            self.left = self.right;
            self.right = air;
            self
        }
        pub fn report(&self){
            report_item(&self.left,"left");
            report_item(&self.right, "right");
        }
    }
    pub fn report_item<T: Display>(item: &Option<T>, which: &str) {
        match item {
            Some(what)=>{
                println!("{} hand is holding {}", which,what);
            }
            _ => {
                println!("{} hand is not holding.", which);
            }
        }
    }
}

fn gives_ownership() -> String {             // gives_ownership will move its
    let some_string = String::from("Giving ownership"); // some_string comes into scope
    some_string                              // some_string is returned and
}
fn print_main_owner(mainstr: String) {
    println!("{}", mainstr);
}
fn print_main_owner_using_ref(mainstr: &String) {
    println!("{}", mainstr);
}
fn print_main_owner_using_ref_with_mut(mainstr: &mut String){
    mainstr.push_str("-Testing");
}
fn add_up_string(mut mainstr: String){
    mainstr.push_str("-NewString");
    println!("{}", mainstr);
}
fn get_byte_length(strliteral : &str) -> usize {
    strliteral.bytes().len()
}
// fn get_String_length(str: String) -> usize {
//     str.bytes().len()
// }
#[derive(Debug)]
struct DropMe;
impl Drop for DropMe {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

struct Owned {
    bla: String,
}
struct Borrowed<'a> {
    bla: &'a str,
}
fn create_borrowed_obj(strpass: &str) -> Borrowed {
    let bor = Borrowed { bla :strpass };
    bor
}

fn vec_pass_by_reference(vec: &Vec<i32>) -> bool {
    vec.is_empty()
}


fn main() {

    let first_owner = String::from("FirstOwner String");
    let ref_first_owner = &first_owner;
    let ref_second_owner = &first_owner;
    println!("{} {} {}", first_owner, ref_first_owner, ref_second_owner);

    let second_owner = first_owner; // Moving ownership
    // println!("x={}", firstOwner); -- This gives an error because the String data type in Rust does not implement the Copy trait,
    // its ownership gets transferred to SecondOwner.
    println!("second Owner={}", second_owner);

    let mut s = String::from("Testing Kevin"); // S owns the Testing Kevin.
    println!("First time s:{}", s);
    let s = gives_ownership();
    println!("After giving ownership s:{}", s); //Rust guarantees memory safety with a feature called ownership.  ** Rust borrow checker

    let s = String::from("Main string");
    print_main_owner(s); // pass ownership to print_main_owner and ownership is over in the method.
    // println!("{}", s);  --> return error since s doesn't valid anymore.

    // Fix above issues by passing the ref.
    let s = String::from("Main string - 2");
    print_main_owner_using_ref(&s);
    let mut s = String::from("Mutable string");
    print_main_owner_using_ref_with_mut(&mut s);

    let test = String::from("Modify string");
    // modify_string(mainstr);
    add_up_string(test);

    // Use the owned string : String
    // since both &String and &str are borrowed types.

    println!("Objecting handling testing ----");
    let mut hands = Hands::new();
    hands.report();
    hands = hands.juggle();
    hands.report();

    println!("Testing ----");
    let ownTest = Owned{
        bla : String::from("Kevin"),
    };
    create_borrowed_obj(&ownTest.bla);// the borrowed value is still in scope.
    println!("Own Test Bla data : {}", ownTest.bla);

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_give_ownership(){
        assert_eq!(gives_ownership(), "Giving ownership");
    }

    #[test]
    fn test_print_main_owner_using_ref_with_mut()
    {
        let mut s = String::from("Mutable string");
        print_main_owner_using_ref_with_mut(&mut s);
        assert_eq!(s, "Mutable string-Testing");
    }

    #[test]
    fn test_vec_pass_by_reference_success()
    {
        let vec = vec![];
        let result = vec_pass_by_reference(&vec);
        assert_eq!(result, true);
    }

    #[test]
    fn test_get_byte_length()
    {
        let check = "KevinPark";
        let num = get_byte_length(check);
        println!("{}", num);
        assert_eq!(num, 9);
    }

    #[test]
    fn test_get_byte_length_String() {
        let mut s = String::from("Mutable string");

    }
}