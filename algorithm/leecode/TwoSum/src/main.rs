use std::collections::HashMap;


// Bruce Force way. 
//Time complexity : O(n^2)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32]
            }
        }
    }
    vec![]
}

// using hashmap.
//The idea is to make use of a hashmap to store the indices of the elements that are already visited. 
pub fn two_sum_better(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut maps: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len(){
        let check = target - nums[i];
        if let Some(test) = maps.get(&check){
            return vec![*test as i32,i as i32];
        }else{
            maps.insert(nums[i], i as i32);
        }
    }
    vec![]
}

// hasmap but using the matching(https://levelup.gitconnected.com/leetcode-problems-two-sum-efa0b69b9505)
pub fn two_sum_best_ans(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut maps: HashMap<i32, i32> = HashMap::new();
    for (i, num) in nums.iter().enumerate(){
        match maps.get(num) {
            Some(&index) => return vec![index, i as i32],
            None => maps.insert(target - num, i as i32),
        };
    }
    vec![]

}

fn main() {
    println!("Hello, world!");
    let numcheck = vec![1,5,6,7,3,10];
    // let check = two_sum(numcheck, 15);
     let check2 = two_sum_better(numcheck, 15);
    println!("{:?}", check2);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let numcheck = vec![1,5,6,7,3,10];
        let check = two_sum(numcheck, 15);
        assert_eq!(check, vec![1,5]);
    }
    #[test]
    fn test_two_sum_better() {
        let numcheck = vec![1,5,6,7,3,10];
        let check = two_sum_better(numcheck, 15);
        assert_eq!(check, vec![1,5]);
    }
    #[test]
    fn test_two_sum_best_ans() {
        let numcheck = vec![1,5,6,7,3,10];
        let check = two_sum_best_ans(numcheck,  15);
        assert_eq!(check, vec![1,5]);
    }

}