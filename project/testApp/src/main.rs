fn main() {
    println!("Hello, world!");
    let mut x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x));

    buz(bar::<u32>);
    buz(bar::<i32>);
    quox(bar::<i32>);

    let add_one = |x| { 1 + x }; // Create a closure using the |...| {...} syntax.
    println!("The sum of 10 + 1 is {}", add_one(10));

    let kevin = "kevin";
    let print_without_arg = || {println!("Print without any argument:{}", kevin)};
    print_without_arg();

    // Moving Closures
    // Moing closures are indicated using the move keyword. The difference is that a moving closure
    // always takes ownership of all variables that it uses.
    // Ordinary closures, in contrast, just create a reference into the enclosing stack frame.

    //let intro = S
    let immut_val = String::from("immut");
    let fn_closure = || {
        println!("Len : {}", immut_val.len());
    };
    // Variable immut_val of type String.

    let square =|x: i32| { x * x };
    twice(5, square);


    compose(5, 
            |n: i32| {n+10},
            |n: i32| {n * 2});

    use std::mem;
    let color = String::from("green");
    let display_color = || println!("Color : {}", color);
    display_color();
    let _reborrow = &color;
    display_color();
    
    let _color_moved = color;
    let display_color = || println!("Color moved : {}", _color_moved);
    display_color();
}
// Function Items.

// Function Pointer.

pub fn bar<T>() {

} 
pub fn buz(f: fn()){
    println!("{}", std::mem::size_of_val(&f));
}

pub fn quox<F>(f: F) 
         where F : Fn(),
{

}


// Closure in Rust.
// Anonymouse functions = Closures.

//Closure is an anonymous function that can capture its environment.
// Contrary to a function, it can capture its environment(capturing the environment means that
// in a closure you can use the variables defined outside the closure body but accessible in its scope).


// You can combine then with functions that take closures as argument.


fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
    f(x) + f(x)
}


// Accepts two closures.
fn compose<F,G>(x: i32, f: F, g: G) -> i32 
    where F: Fn(i32) -> i32, G: Fn(i32) -> i32 {
        g(f(x))
}

    