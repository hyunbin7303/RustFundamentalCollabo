use std::mem;


struct OurIterator<'a>{
    nums: &'a [i32],
    i: usize,
    x: i32,
}
impl<'a> OurIterator<'a>{
    fn new(nums: &'a [i32], x: i32) -> Self {
        Self { nums, i:0, x }
    }
}
impl <'a> Iterator for OurIterator<'a>{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item>{
        if self.i < self.nums.len() {
            let value = self.nums[self.i];
            self.i += 1;
            Some(value * self.x)
        }else {
            None
        }
    }
}

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32){
    println!("x is {} and y is {}", x,y);
}
fn just_print(x: i32, y: i32){
    println!("x is {} and y is {}", x,y);
}

struct Person {
    first_name : String,
    last_name: String,
    age : u8,
    is_alive : bool,
}
struct Config {
    conn_string : String,
    env : String, 
    default_user : String,
}
struct App<'a> {
    config : &'a Config
}
struct Platform<'a> {
    app: App<'a>
}
struct Point<'a> {
    x: &'a i32,
    y: &'a i32
}

fn test1() {
    let nums = vec![1,2,3];
    let mut iterator1 = OurIterator::new(&nums, 2);
    let mut iterator2 = OurIterator::new(&nums, 3);
    while let Some(n)= iterator1.next(){ 
        println!("{}", n);
    }
    while let Some(n)= iterator2.next(){ 
        println!("{}", n);
    }
}



fn to_words<'a>(text: &'a str) -> impl Iterator<Item = &'a str> {
    text.split(' ')
}

fn main() {
    let text = "word1 word2 word3";
    println!("{}", to_words(text).take(2).count());

    
    test1();

    let (mut a,b) = (10,20);
    just_print(a, b);  

    let p = Person {
        first_name : "Kevin".to_string(),
        last_name : "Park".to_string(),
        age: 29,
        is_alive:true,
    };
    let check= &p;
    println!("{},{}", check.first_name, check.last_name);
    println!("{},{}", (*check).first_name, (*check).last_name);
    


    let mut i:i32 = 1;
    let ref_i = &mut i;
    test (ref_i);
    *ref_i = 2;
    *ref_i= 40;
    test(ref_i);
    let numbers = vec![1,2,3,4,5,6,7,8];
    // let top = &numbers[...3];
    // smallest_number(n: &[i32])
    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];
    let getNum = smallest_number(&ys[1 .. 4]);
    println!("Get the smallest number : {}", getNum);

}
// the borrow checker uses explicit lifetime annotations to determine how long references
// should be valid.
// In cases where lifetimes are not elided, 
fn analyze_slice_PersonObj(slice: &[Person]) {
    println!("First element of the Person: {}", slice[0].first_name);
}
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
fn smallest_number(n: &[i32]) -> &i32 {
    let mut s = &n[0];
    for r in &n[1..] {
        if r < s {
            s = r;
        }
    }
    s
}



fn test(i:&mut i32){
    println!("number : {}", i);
}
