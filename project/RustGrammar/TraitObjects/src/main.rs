
// Two trait object
// The purpose of trait objects is to permit late binding of methods.
//Like all DSTs, trait objects are used behind some type of pointer;....

//Trait objects implement the base trait, its auto traits, and any supertraits of the base trait.
use TraitObjects::{Screen, Button, Draw};

struct SelectBox {
    width : u32,
    height : u32,
    options : Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("{}, {}, {}", self.width, self.height, self.options[0]);
    }
}
//Trait objects, like &Foo or Box<Foo>, are normal values that store a value of any type that implements the given trait, where the precise type can only be known at runtime. T
#[warn(non_snake_case)]
fn main() {
    println!("A trait object is an opaque value of another type that implements a set of traits.");
    let screen = Screen { 
        components : vec![ 
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![String::from("Testing"), String::from("Testing2"), String::from("Testing3")]
            }),
            Box::new(SelectBox {
                width: 50,
                height: 200,
                options: vec![String::from("Checking")]
            }),
        ],
    };
    screen.run();
}

