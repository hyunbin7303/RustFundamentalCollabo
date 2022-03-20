

//https://depth-first.com/articles/2020/06/22/returning-rust-iterators/

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

    // fn values2(&self) -> impl

}

trait ContainerAnnotation<'a> {
    type ItemIterator: Iterator<Item=&'a u8>;
    fn items(&'a self) -> Self::ItemIterator; 
}


fn main() {
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
}
