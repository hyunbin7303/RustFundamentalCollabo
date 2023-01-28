use std::iter;
use std::vec::IntoIter;


// impl trait
//1. Argument type
//2. return type
fn combine_vec_explit(v: Vec<i32>, u : Vec<i32>) 
            -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>>
{
    v.into_iter().chain(u.into_iter()).cycle()
}
//Same with above but it's more simple by using impl trait.
fn combine_vec(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item=i32>{
    v.into_iter().chain(u.into_iter()).cycle()
}

trait TraitTesting {
    const Id: i32;
}
struct Struct;
impl TraitTesting for Struct {
    const Id: i32 = 1;
}


//impl Trait in argument position
fn foo(arg: impl TraitTesting){

}


fn main() {
    let v1 = vec![10, 20, 30];
    let v2 = vec![4, 5];
    let mut v3 = combine_vec_explit(v1, v2);
    println!("{:?}", v3)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn StructId_Compare() {
        assert_eq!(Struct::Id, 1);
    }

}


