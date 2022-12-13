use std::collections::HashMap;
use std::env;

pub fn main(){
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);

    // Get returns an Option<&V>
    let score = scores.get("Sunface");
    println!("{:?}", score);

    let person1 = new Person("hyunbin", "asdf@gmail.com");


    // let mut eng2span: HashMap<&str, &str> = HashMap::new();
    // let my_vec = vec![("two", "dos"), ("three", "tres"), ("four", "cuatro")];
    // let temp: HashMap<_, _> = my_vec.into_iter().collect();
    // eng2span.extend(temp);
    // println!("{:?}", eng2span);

}


pub fn vec_to_hashmap2(){
    let english = vec!["one", "two", "three"];
    let spanish = vec!["uno", "dos", "tres"];
    let english2spanish: HashMap<_, _> = english.iter().zip(spanish.iter()).collect();
    println!("{:?}", english2spanish);
}
