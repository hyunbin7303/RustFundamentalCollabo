struct Person {
    pub first_name: String,
    pub last_name: Option<String>,
    pub age: i32,
}


fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    for element in v2 {
        println!("{}", element);
    } 

    let mut old1 = vec!["richard", "Peter", "charles"];
    // println!("{:?}", old1);
    
    let old2 = vec!["Maria", "Helena", "Rachel"];
    let new_one = [&old1[..], &old2[..]].concat();
    println!("{:?}", old1);
    println!("{:?}", new_one);

    let mut iter = split_whitespace_indices(" Hello world");
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
}




// how they did this? 
fn addr_of(s: &str) -> usize {
    s.as_ptr() as usize
}
fn split_whitespace_indices(s: &str) -> impl Iterator<Item = (usize, &str)> {
    s.split_whitespace()
        .map(move |sub| (addr_of(sub) - addr_of(s), sub))
}
