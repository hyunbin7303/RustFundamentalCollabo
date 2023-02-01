use itertools::Itertools;




//https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator-or-any-other-trait
//https://depth-first.com/articles/2020/06/22/returning-rust-iterators/

pub struct Foo {
    count: u8,
}
impl Iterator for Foo {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item>{
        match self.count {
            0 => {
                self.count = self.count + 1;
                Option::Some(1)
            }
            1 => {
                self.count = self.count + 1;
                Option::Some(10) 
            }
            _ => None 
        }
    } 
}

pub struct Repeater <'a> {
    iter: std::slice::Iter<'a, u8>,
}
impl<'a> Iterator for Repeater<'a> {
    type Item = &'a u8;
    fn next(&mut self) -> Option::<Self::Item>{
        self.iter.next()
    }
}

pub struct Fibonacci
{
    a: u32,
    b: u32,
}
impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let next_term = self.a.checked_add(self.b)?;

        self.a = self.b;
        self.b = next_term;

        Some(next_term)
    }
}
pub fn fibonacci_numbers() -> Fibonacci {
    Fibonacci { a: 1, b: 0 }
}


#[derive(Debug)]

pub struct Wrapper {
    value: u8,
}
pub struct ContainerWithWrapper {
    items: Vec<Wrapper>
}
impl ContainerWithWrapper {
    // TODO: add a method that iterates over Wrapper.value
    //use impl Trait to return an iterator that uses map or filter closures
    fn values(&self) -> impl Iterator<Item=u8> + '_ {
        self.items.iter().map(|wrapper| wrapper.value)
    }
}
trait ContainerAnnotation<'a> {
    type ItemIterator: Iterator<Item=&'a u8>;
    fn items(&'a self) -> Self::ItemIterator; 
}

fn to_words<'a>(text: &'a str) -> impl Iterator<Item = &'a str> {
    text.split(' ')
}
fn to_words_dynamic_dispatching<'a>(text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    Box::new(text.split(' '))
}
fn num_impl_iterator(n: i32) -> impl Iterator<Item= i32> {
    (0..n).map(|x| x * 10)
}

fn main() {
    let it = (1..3).interleave(vec![-1, -2]);
    itertools::assert_equal(it, vec![1, -1, 2, -2]);

    let i = Foo { count: 0 };
    let v = Vec::from_iter(i);                        
    for value in v {
        println!("value: {}", value);
    }




    let text = "word1 word2 word3";
    let mut iter_text = to_words(text);
    // println!("{}", iter_text.next());
    // println!("{}", iter_text[0]);
    println!("To Word: {}", to_words(text).take(2).count());
    println!("{}", to_words_dynamic_dispatching(text).take(2).count());

    let num_1 = num_impl_iterator(10);
    for element in num_1 {
        println!("{}", element);
    }

    println!("Fibonacchi testing ----");
    for number in fibonacci_numbers() {
        println!("{}", number);
    }
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn iterator_next_unwrap()
    {
        let v1 = vec![10, 20, 30];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&10));
        assert_eq!(v1_iter.next(), Some(&20));
        assert_eq!(v1_iter.next(), Some(&30));
        assert_eq!(v1_iter.next(), None);
    }
    #[test]
    fn iterator_demo()
    {
        let slice = &[10, 20, 30];
        let mut iter = slice.iter();
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&30));
    }
    #[test]
    fn iterator_For_String() // Iter iterates over &T
    {                        // Iteration over immutable references (&T).
        let names = vec!["Kevin", "Adam", "Julio", "Tudor"];
        let mut iter = names.iter(); 
        assert_eq!(iter.next(), Some(&"Kevin"));
        assert_eq!(iter.next(), Some(&"Adam"));
        assert_eq!(iter.next(), Some(&"Julio"));
        assert_eq!(iter.next(), Some(&"Tudor"));
    }
    #[test]
    fn iterator_into_iter() // Into_iter iterates over T.
    {   
        let names = vec!["Kevin", "Macy"];
        let mut iter = names.into_iter();
        assert_eq!(iter.next(), Some("Kevin"));
        assert_eq!(iter.next(), Some("Macy"));
    }
    #[test]
    fn iterator_filter_get_even_number()
    {
        let even = (0..10).filter(|n| n % 2 == 0);
        let mut iter = even.into_iter();
        
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(6));
    }
    #[test]
    fn test_to_name_return()
    {
        let text = "word1 word2 word3";
        println!("{}", to_words(text).take(2).count());
        // assert_eq!();
    }
    #[test]
    fn test_containers_with_wrapper()
    {
        let wrap_test = vec![
            Wrapper { value : 1},
            Wrapper { value : 2},
            Wrapper { value : 3},
            Wrapper { value : 4},
        ];
        let wrapping_container = ContainerWithWrapper { items : wrap_test };
        let mut iter = wrapping_container.items.into_iter();
        // assert_eq!(iter.next(), Some(1));
        // for element in wrapping_container.items {
        //     println!("{}", element.value);
        // }
    }
}
