use std::iter;
use std::vec::IntoIter;
use std::fmt;

struct Person {
    pub first_name: String,
    pub last_name: Option<String>,
    pub age: i32,
}
#[derive(Debug)]

struct Album {
    pub title: String,
    pub artist: String,
}

fn main() {
    let v = vec!["bat", "man"];
    let s: String = v.concat();
    println!("{}", s);

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    for element in v2 {
        println!("{}", element);
    } 

    let old1 = vec!["richard", "Peter", "charles"];
    let old2 = vec!["Maria", "Helena", "Rachel"];
    let new_one = [&old1[..2], &old2[..]].concat();
    println!("old1 {:?}", old1);
    println!("new_one : {:?}", new_one);


    std::vector<int> A{ 1, 2, 3, 4, 5};
    std::vector<int> B{ 10, 20, 30 };

    VecProxy<int> AB(A, B);  // ----> O(1). No copies performed.


    let mut albums_t = vec![
        Album { title: "you can do it".into(),artist: "Kevin Park".into(),},
        Album { title: "No matter how hard it is".into(), artist: "hyunbin park".into(),},
    ];
    let mut albums_t2 = vec![
        Album { title: "How to be better at Rust?".into(),artist: "Hyunbin Park".into(),},
        Album { title: "hahaha artist!".into(), artist: "Hojun Park".into(),},
    ];
    albums_t.extend(albums_t2);
    println!("{:?}", albums_t);



    let iter = split_whitespace_indices(" Hello world");
    for element in iter {
        println!("{} - {}", element.0, &element.1);
    }
    //assert_eq!(Some((1, "Hello")), iter.next());
    //assert_eq!(Some((7, "world")), iter.next());
    let items = vec!["kevin", "check", "Vector"];
    for (i, x) in items.iter().enumerate() {
        println!("Item {} = {}", i, x);
    }
    let mut persons: Vec<Person> = Vec::new();
    persons.push(Person {
        first_name: "Asnim".to_string(),
        last_name: None,
        age: 1,
    });
    persons.push(Person {
        first_name: "Fahim".to_string(),
        last_name: Some("Ansari".to_string()),
        age: 2,
    });
    persons.push(Person {
        first_name: "Shahul".to_string(),
        last_name: None,
        age: 6,
    });
    persons.push(Person {
        first_name: "Mujeeb".to_string(),
        last_name: Some("Rahuman".to_string()),
        age: 6,
    });

    let ages_of_people_with_second_name_using_seperate_filter_map: Vec<i32> = persons
        .iter()
        .filter(|p| p.last_name.is_some())
        .map(|p| p.age)
        .collect();

    println!("{:?}", ages_of_people_with_second_name_using_seperate_filter_map);

    testing();

    let albums = vec![
        Album {
            title: "Sgt. Pepper's Lonely Hearts Club Band".into(),
            artist: "The Beatles".into(),
        },
        Album {
            title: "Dark Side of the Moon".into(),
            artist: "Pink Floyd".into(),
        },
    ];

    println!("Displaying testing using the fm:: Display - {:?}", albums);
}

impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.artist)
    }
}


fn testing(){
    let my_string = "Some small words, they're this.\nTogether";
    let stripped_lines = ["Some small words, they\'re this.", "Together"];

    let substring = my_string
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match (33..41).contains(&i) {
            true => Some(c),
            false => None,
        })
        .collect::<String>();
        println!("{}", substring);
}

fn combine_cycle_testing(){
    let v1= vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut result = combine_two_vectors_cycle(v1, v2);
    for element in result {
        println!("{}", element);
    }
}

fn combine_two_vectors_cycle(v1: Vec<i32>, v2: Vec<i32>) -> impl Iterator<Item=i32> {
    v1.into_iter().chain(v2.into_iter()).cycle()
}


// how they did this? 
fn addr_of(s: &str) -> usize {
    s.as_ptr() as usize
}
fn split_whitespace_indices(s: &str) -> impl Iterator<Item = (usize, &str)> {
    s.split_whitespace()
        .map(move |sub| (addr_of(sub) - addr_of(s), sub))
}
