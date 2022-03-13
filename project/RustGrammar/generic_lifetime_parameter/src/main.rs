// Generic lifetime parameters.

// A generic lifetime parameter imposes a lifetime constraint on the references and the return values of a function.
// while compiling the code, a generic lifetime is substituted for a concrete lifetime, which is equal to the smaller of the passed
// references' lifetimes. This enables rust to identify a violation of the constraint by a paremter or the variable strong the return value. 
struct OurIterator<'a> {
    num : &'a [i32],
    i: usize,
    x: i32,
}

impl<'a> OurIterator<'a> {
    fn new(num: &'a [i32], x: i32) -> Self {
        Self { num, i: 0, x}
    }
}

impl<'a> Iterator for OurIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.num.len() {
            let value = self.num[self.i];
            self.i += 1;
            Some(value * self.x)
        }else{
            None
        }
    }
}

// Not working if we don't put 'a due to the lifetime issue.
// The return type needs a generic liefetime prameter on it because Rust can't tell whether the ref being refurned
// refers to x or y. 
fn high_num_check<'a>(n1: &'a i32, n2: &'a i32) -> &'a i32 {
    if n1> n2{
        n1
    }
    else if n1 == n2 {
        n1
    }
    else {
        n2
    }
}

fn main() {
    println!("Comparing two number ");
    let n1 = 20;
    let n2 = 30;
    let result = high_num_check(&n1, &n2);
    println!("the Bigger number is : {}", result);

    let mut nums = vec![1,2,3,4,5,6,7];
    {
        let mut iter = nums.iter();
        while let Some(n) = iter.next() {
            println!("{}", n);
        }

        let mut my_iter = OurIterator::new(&nums, 2);
        let mut my_iter2 = OurIterator::new(&nums, 3);
        while let Some(n) = my_iter.next() {
            println!("{}", n);
        }
        nums.push(8);

    }



}
