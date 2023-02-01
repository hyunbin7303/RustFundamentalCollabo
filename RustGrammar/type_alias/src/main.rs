// source from https://www.youtube.com/watch?v=maFEjLQQm40&ab_channel=mithradates
// https://github.com/Dhghomon




// TOO LONG!!!!
fn skip_five_take_5(input : Vec<char>) -> std::iter::Take<std::iter::Skip<std::vec::IntoIter<char>>>{
    input.into_iter().skip(5).take(5)
}

//What if we use the type alias?
//Clean return type!
type SkipAndTake = std::iter::Take<std::iter::Skip<std::vec::IntoIter<char>>>;
fn skip_five_take_5_better(input: Vec<char>) -> SkipAndTake {
    input.into_iter().skip(5).take(5)
}

// todo!() - Will do it later.SkipAndTake
struct TestType{
    name: String,
    age : u8
}

fn test_func(input: TestType) -> Vec<TestType>{
    todo!();
}

fn main() {
    println!("Hello, world!");
    // skip_five_take_5(input: vec<char>)
}
