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


fn main() {
    let firstOwner = String::from("FirstOwner String"); // String is the Heap Allocation
    let secondOwner = firstOwner; // Moving ownership
    // println!("x={}", firstOwner); -- This gives an error because the String data type in Rust does not implement the Copy trait,
    // its ownership gets transferred to SecondOwner.
    println!("second Owner={}", secondOwner);

    let mut s = String::from("Testing Kevin"); // S owns the Testing Kevin.
    println!("First time s:{}", s);
    let s = gives_ownership();
    println!("After giving ownership s:{}", s); //Rust guarantees memory safety with a feature called ownership.  ** Rust borrow checker


    // Use the owned string : String
    // since both &String and &str are borrowed types.


    let mut hands = Hands::new();
    hands.report();
    hands = hands.juggle(); 
    hands.report();

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_give_ownership(){
        assert_eq!(gives_ownership(), "Giving ownership");
    }

}