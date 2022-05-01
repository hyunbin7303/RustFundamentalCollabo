use std::collections::HashMap;

fn main() {


 //   hashmap_to_vec();
    vec_to_hashmap();

}

//converts HashMap to vec
pub fn hashmap_to_vec(){
    let mut map = HashMap::new();
    map.insert("cat", 800);
    map.insert("frog", 200);
    // Version 1: get vector of pairs.
    let vec1 = Vec::from_iter(map.iter());
    println!("{:?}", vec1);
}
pub fn vec_to_hashmap(){
    let mut eng2span = HashMap::new();
    //eng2span.insert("one", "uno");
    let my_vec = vec![("two", "dos"), ("three", "tres"), ("four", "cuatro")];
    let temp: HashMap<_, _> = my_vec.into_iter().collect();
    eng2span.extend(temp);
    // let hash_to_vec = eng2span.iter().collect::<Vec<_>>();
    // println!("{:?}", hash_to_vec);
}

pub fn vec_to_hashmap2(){
    let english = vec!["one", "two", "three"];
    let spanish = vec!["uno", "dos", "tres"];
    let english2spanish: HashMap<_, _> = english.iter().zip(spanish.iter()).collect();
    println!("{:?}", english2spanish);
}


