use std::collections::HashMap;

fn main() {
    let mut eng2span = HashMap::new();
    eng2span.insert("one", "uno");
    let my_vec = vec![("two", "dos"), ("three", "tres"), ("four", "cuatro")];
    let temp: HashMap<_, _> = my_vec.into_iter().collect();
    eng2span.extend(temp);
    let hash_to_vec = eng2span.iter().collect::<Vec<_>>();
    println!("{:?}", hash_to_vec);

    let english = vec!["one", "two", "three"];
    let spanish = vec!["uno", "dos", "tres"];
    let english2spanish: HashMap<_, _> = english.iter().zip(spanish.iter()).collect();
    println!("{:?}", english2spanish);
}



