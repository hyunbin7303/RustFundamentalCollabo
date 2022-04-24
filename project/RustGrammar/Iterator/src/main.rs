



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

    let i = Foo { count: 0 };
    let v = Vec::from_iter(i);                        
    for value in v {
        println!("value: {}", value);
    }


    let v = vec![1, 2, 3];
    let mut iter = v.into_iter();
    while let Some(number) = iter.next() {
        println!("{}", number);
    }
    // First, we declare a type which has `iter` method to get the `Iter` struct (`&[usize]` here):
    let slice = &[10, 20, 30];
    for element in slice {
        println!("{}", element);
    }
    // Then, we iterate over it:
    for element in slice.iter() {
        println!("{}", element);
    }

    let names = vec!["Kevin", "Adam", "Julio", "Tudor"];
    let mut iterator = (names).into_iter();
    while let Some(name) = iterator.next() {
        println!("{}", name);
    }


    let wrap_test = vec![
        Wrapper { value : 1},
        Wrapper { value : 2},
        Wrapper { value : 3},
        Wrapper { value : 4},
    ];
    let wrapping_container = ContainerWithWrapper { items : wrap_test };
    for element in wrapping_container.items {
        println!("{}", element.value);
    }
    let text = "word1 word2 word3";
    println!("{}", to_words(text).take(2).count());
    println!("{}", to_words_dynamic_dispatching(text).take(2).count());

    let num_1 = num_impl_iterator(10);
    for element in num_1 {
        println!("{}", element);
    }
}
