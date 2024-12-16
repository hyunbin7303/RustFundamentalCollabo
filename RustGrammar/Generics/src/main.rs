use std::fmt::Debug;//print marker: `{:?}`.


struct A;
struct Testing(A);

// struct Single(Kevin);
struct Kev<T,U>{
    a: T,
    b: U
}
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self{
        Self {x,y}
    }
}
impl<T: Display + PartialOrd> Pair<T> {

}

// Generic with bound
fn printer<T: Display>(t: T, name: String) {
    println!("{}", t);
}

fn main() {
    println!("Hello, world!");
    let check = Kev { a: "Hyunbin", b: "Park"};
    println!("First Name : {}", check.a);
    println!("Last Name : {}", check.b);
}
