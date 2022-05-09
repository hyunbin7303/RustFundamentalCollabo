use std::collections::HashMap;

fn main() {


    hashmap_to_vec();
    println!("asdf");
    // Vector to hashmap 
    let mut eng2span: HashMap<&str, &str> = HashMap::new();
    let my_vec = vec![("two", "dos"), ("three", "tres"), ("four", "cuatro")];
    let temp: HashMap<_, _> = my_vec.into_iter().collect();
    eng2span.extend(temp);
    println!("{:?}", eng2span);


    // let find_str = String::from("two");
    // let test = eng2span.get(&find_str);
    // println!("{:?}",test);

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

}

//converts HashMap to vec
pub fn hashmap_to_vec(){
    let mut map = HashMap::new();
    map.insert("cat", 800);
    map.insert("frog", 200);
    let vec1 = Vec::from_iter(map.iter());
    println!("{:?}", vec1);
}

pub fn vec_to_hashmap2(){
    let english = vec!["one", "two", "three"];
    let spanish = vec!["uno", "dos", "tres"];
    let english2spanish: HashMap<_, _> = english.iter().zip(spanish.iter()).collect();
    println!("{:?}", english2spanish);
}


