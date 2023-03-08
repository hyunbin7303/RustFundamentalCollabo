struct Kevin;
// struct Single(Kevin);
struct Kev<T,U>{
    a: T,
    b: U
}


fn main() {
    println!("Hello, world!");
    let check = Kev { a: "Hyunbin", b: "Park"};
    println!("First Name : {}", check.a);
    println!("Last Name : {}", check.b);
    
}
