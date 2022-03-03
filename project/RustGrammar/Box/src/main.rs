

// box - smart pointer that has its data on the heap.
// box - 8 bytes.



// What is Boxing? 
// You can put a type on the heap instead of the stack...
// But people use the Box for error handling?
fn get_input_var<T>(item: T) {
}
struct List {
    item: Option<Box<List>>,
}
impl List {
}

fn main() {
    println!("Hello, world!");
    let num = 20;


    let box1 = Box::new(1);
    println!("{}", box1);

}
