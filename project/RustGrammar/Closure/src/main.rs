
fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
    f(x) + f(x)
}
fn triple<F: Fn(i8) -> i8>(x:i8, f:F) -> i8 {
    f(x)
}
fn compose<F, G>(x: i32, f: F, g: G) -> i32
    where F: Fn(i32) -> i32, G: Fn(i32) -> i32 {
    g(f(x))
}

fn number_return() -> i32 {
    return 40;
}

fn main() {

 //   let num: i32 = number_return();
 //   println!("number checking : {}", num);

    compose(5,
        |n: i32| { n + 42 },
        |n: i32| { n * 2 }); // evaluates to 94


    println!("Hello, world! Closure Testing");
    let square = |x: i32| { x * x };
    twice(5, |x: i32| { x * x }); // evaluates to 50
    //twice(5, square); // evaluates to 50
    println!("the square value will be {}.", square);

    let add_one = |x| { 1 + x };
    println!("The sum of 5 plus 1 is {}.", add_one(5));

    // Increment via closures and functions.
    fn function(i: i32) -> i32 { i + 1 }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}",  closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());



    // Move Closure. 
    
}
