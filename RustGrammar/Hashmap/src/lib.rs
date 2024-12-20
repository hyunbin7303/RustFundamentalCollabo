use std::collections::HashMap;
use std::env;


pub fn convert_hashmap_to_vec(hashmap: HashMap<String, String>){
    let vec1 = Vec::from_iter(hashmap.iter());
    println!("{:?}", vec1);
}
#[cfg(test)]
mod tests {
    use super::*;
    fn base_seed_hashmap() -> HashMap<String,String>{
        let mut hashmap_str=HashMap::new();
        hashmap_str.insert("Data Structures".to_string(),"Required course".to_string());
        hashmap_str.insert("Algorithms".to_string(),"ncessary course".to_string());
        hashmap_str.insert("Interview Preparations".to_string(),"Need more work".to_string());
        hashmap_str
    }
    #[test]
    fn hashmap_lengh_successfully_count(){
        let mut hashmap_str=base_seed_hashmap();
        assert_eq!(hashmap_str.len(), 3);
    }

    #[test]
    fn hashmap_get_numvalue_by_existing_key(){
        let mut hashmap_num = HashMap::new();
        hashmap_num.insert("TestingA", 10);
        hashmap_num.insert("TestingB", 20);
        hashmap_num.insert("TestingC", 30);

        let score = hashmap_num.get("TestingA");
        let score2 = hashmap_num.get("TestingB").expect("20");
        assert_eq!(score.unwrap(), &10);
        assert_eq!(score2, &20);
    }

    #[test]
    fn hashmap_for_in_valid(){
        let mut map = HashMap::new();
        map.insert("Kevin", 90);
        map.insert("Macy", 100);
        map.insert("Julio", 95);
        map.insert("Adam", 120);

        for(&k, &v) in &map {
            match k {
                "Kevin" => assert_eq!(v, 90),
                "Macy" => assert_eq!(v, 100),
                "Julio" => assert_eq!(v, 95),
                "Adam" => assert_eq!(v, 120),
                _ => unreachable!(),
            }
        }
        assert_eq!((&map).into_iter().count(), 4);
    }
    #[test]
    fn empty_hashmap() {
        let mut map = HashMap::<&str, &str>::new();
        assert_eq!(map.contains_key("key"), false);
        assert_eq!(map.get("key"), None);
        assert_eq!(map.remove("key"), None);
    }
}