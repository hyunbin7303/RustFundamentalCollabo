

#[warn(non_snake_case)]
fn main() {
    println!("A trait object is an opaque value of another type that implements a set of traits.");
}

// Two trait object

// The purpose of trait objects is to permit late binding of methods.
//Like all DSTs, trait objects are used behind some type of pointer;....

trait Printable {
    fn stringify(&self) -> String;
}
impl Printable for i32 {
    fn stringify(&self) -> String {self.to_string()}
}