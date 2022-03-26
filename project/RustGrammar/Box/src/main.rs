use std::boxed::Box;
// box - smart pointer that has its data on the heap. box - 8 bytes.
// Box<T> - This smart pointer points to the data allocated on the heap of type T.
// It's used to store data on the heap, rather than on the stack.

fn get_input_var<T>(item: T) {
}
struct List {
    item: Option<Box<List>>,
}
impl List {
}

struct Point {
    x: i32,
    y: i32,
}


#[derive(Debug)]
struct Champ<'a> {
    id : i32,
    name: &'a str,
    champ_type: &'a str,
}
enum ArithExp {
    Sum {
        lhs: Box<ArithExp>,
        rhs: Box<ArithExp>,
    },
    Mul {
        lhs: Box<ArithExp>,
        rhs: Box<ArithExp>,
    },
    Num {
        value: f64,
    },
}

fn num(value: f64) -> std::boxed::Box<ArithExp> {
    Box::new(ArithExp::Num { value })
}
fn main() {


    // Allocate structs with Box on heap
    let mut champs: Vec<Box<Champ>> = Vec::new();
    for _i in 0..3 {
        champs.push(Box::new(Champ {id:1, name: "Garen", champ_type: "tank"}));
    }

    let mut champs2: Vec<Champ> = Vec::new();
    for _i in 0..3 {
        champs2.push(Champ {id: 1, name: "Nami", champ_type : "support"});
    }
     // Print results.
     println!("HEAP: {:#?}", champs);
     println!("STACK: {:#?}", champs2);

    let boxed_point = Box::new(Point{x: 10, y:20});
    match *boxed_point {
        Point {x,y} => println!("Point is at {}. {}", x,y),
    }


    let mut number = num(1.0);
    match *number {
        ArithExp::Num { value } => println!("Value = {}", value),
        _ => (),
    }

}


