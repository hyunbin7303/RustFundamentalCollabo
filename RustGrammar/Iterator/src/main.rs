//https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator-or-any-other-trait
struct CustomIter {
    v: Vec<String>,
    counter: usize,
}
impl CustomIter {
    fn new() -> Self {
        CustomIter {
            v: Vec::new(),
            counter: 0,
        }
    }

    fn add(&mut self, val: String) {
        self.v.push(val)
    }
}

impl Iterator for CustomIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.v.get(self.counter) {
            Some(s) => {
                self.counter += 1;
                Some(s.to_owned())
            }
            None => None,
        }
    }
}

struct Student {
    courses: Vec<String>,
    counter: usize,
}
impl Student {
    fn new() -> Self {
        Student {
            courses: Vec::new(),
            counter: 0,
        }
    }
    fn add(&mut self, course: String) {
        // self.counter += 1; // TODO : if you do this one, it's not working. Why is that? 
        self.courses.push(course)
    }
}
impl Iterator for Student {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.courses.get(self.counter) {
            Some(s) => {
                self.counter += 1;
                Some(s.to_owned())
            }
            None => None,
        }
    }
}

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.count {
            0 => {
                self.count = self.count + 1;
                Option::Some(1)
            }
            1 => {
                self.count = self.count + 1;
                Option::Some(10) 
            }
            // self.count >= 5 {
            //     self.count = self.count +1;
            //     Some(self.count)
            // }
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
pub struct Fibonacci {
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

fn to_words<'a>(text: &'a str) -> impl Iterator<Item = &'a str> {
    text.split(' ')
}
fn to_words_dynamic_dispatching<'a>(text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    Box::new(text.split(' '))
}

struct SliceIter<'a, T> {
    slice: &'a [T],
    index: usize,
}
impl<'a, T> Iterator for SliceIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.slice.get(self.index)?;
        self.index += 1;

        Some(item)
    }
}
fn iterator_over_slices() {

}
//Use the into_iter() function when you want to move, instead of borrow
fn get_first_para_string(v : Vec<(String, usize)>) -> Vec<String> {
    v.into_iter().map(|(account,score)| account).collect()
}


fn main() {
    let mut cis = CustomIter::new();

    cis.add("Amirreza".to_string());
    cis.add("Parsa".to_string());
    cis.add("Yas".to_string());

    for fruit in cis {
        println!("{}", fruit);
    }

    let mut students = Student::new();
// println!("Application start");
    students.add("Hyunbin".to_string());
    students.add("Macy".to_string());
    students.add("Julio".to_string());

    for student in students {
        println!("studnet info {}", student);
    }
    // let v = Vec::from_iter(i);                        
    // for value in v {
    //     println!("value: {}", value);
    // }

    let text = "word1 word2 word3";
    let mut iter_text = to_words(text); // println!("{}", iter_text.next()); // println!("{}", iter_text[0]);
    println!("To Word: {}", to_words(text).take(2).count());
    println!("{}", to_words_dynamic_dispatching(text).take(2).count());
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iterator_for_vec_num()
    {
        let v1 = vec![10, 20, 30];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&10));
        assert_eq!(v1_iter.next(), Some(&20));
        assert_eq!(v1_iter.next(), Some(&30));
        assert_eq!(v1_iter.next(), None);
    }
    #[test]
    fn iterator_for_slice()
    {
        let slice = &[10, 20, 30];
        let mut iter = slice.iter();
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&30));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iterator_for_enumerate()
    {
        let v = vec![1; 10];
        for (pos, e) in v.iter().enumerate() {
            println!("Element at position {}: {:?}", pos, e);
        }
        // let items = vec!["kevin", "check", "Vector"];
        // for (i, x) in items.iter().enumerate() {
        //     println!("Item {} = {}", i, x);
        // }
    }

    #[test]
    fn iterator_for_String() // Iter iterates over &T
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
        // let mut check = iter.next().value;
        // assert_eq!(iter.next(), Some(1));
        // for element in wrapping_container.items {
        //     println!("{}", element.value);
        // }
    }
    #[test]
    fn fibonacchi_iter()
    {
        let check = fibonacci_numbers();
        let mut iter = check.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
    }
    // #[test]
    // fn calling_next_directly() {
    //     let mut counter = Counter::new();

    //     assert_eq!(counter.next(), Some(1));
    //     assert_eq!(counter.next(), Some(2));
    //     assert_eq!(counter.next(), Some(3));
    //     assert_eq!(counter.next(), Some(4));
    //     assert_eq!(counter.next(), Some(5));
    //     assert_eq!(counter.next(), None);
    // }

    #[test]
    fn test_custom_iterator() {
        let mut students = Student::new();

        students.add("Hyunbin".to_string());
        students.add("Macy".to_string());
        students.add("Julio".to_string());

        for student in students {
            println!("{}", student);
        }
    }
    #[test]
    fn test_custom_iterator() {
        let mut cis = CustomIter::new();

        cis.add("Amirreza".to_string());
        cis.add("Parsa".to_string());
        cis.add("Yas".to_string());

        for fruit in cis {
            println!("{}", fruit);
        }
    }
}
