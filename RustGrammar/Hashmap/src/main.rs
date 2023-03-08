use std::collections::HashMap;
use std::env;
mod person;
pub fn main(){
    let mut scores = HashMap::new();
    scores.insert("Kevin", 98);
    scores.insert("Macy", 95);
    scores.insert("Julio", 79);
    // Get returns an Option<&V>
    let score = scores.get("Kevin");
    println!("Kevin Score : {:?}", score.unwrap());
    let person_name = String::from("Macy");
    let new_person: &i32 = if let Some(score) = scores.get(&person_name) {
        score
    }
    else {
        &0i32
    }

    let mut hashmap_num = HashMap::new();
    hashmap_num.insert("TestingA", 10);
    hashmap_num.insert("TestingB", 20);
    hashmap_num.insert("TestingC", 30);

    let checkNum= hashmap_num.get("TestingA").unwrap();
    println!("{}", checkNum);


    let user = person::User {
        userid: 1,
        username: "hyunbin7303",
    };

    let myname = "Hyunbin".to_string();
    let myemail = "hyunbin7303@gmail.com".to_string();
    let person1 = person::Person {
        username: String::from("Hyunbin7303"),
        email : String::from("Hyunbin7303@gmail.com"),
        active : true,
        sign_in_count : 0 
    };


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
pub fn get_userhashmap_search_by_active()
{

}


/*
Today Todo List
//https://practice.rs/collections/hashmap.html
1. hash map. Searching for the specific items? 
2. Vector, Searching for the specific items?
3. Algorithm Study. 
 */