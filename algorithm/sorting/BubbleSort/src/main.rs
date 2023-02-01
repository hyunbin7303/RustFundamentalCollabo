


// Bubble sort 
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    bubble_sort(&mut numbers);
}

// Dictionary<int, int> dic = new Dictionary<int, int>();
// for(int i = 0; i< nums.Length; i++)
// {
//     if(dic.ContainsKey(target -nums[i]))
//     {
//         // Answer. return the vlaue
//         int[] returnNums = new int[2];
//         returnNums[0] = dic[target - nums[i]];
//         returnNums[1] = i;
//         return returnNums;
//     }
//     else if(!dic.ContainsKey(nums[i]))
//     {
//         dic.Add(nums[i], i);
//     }
// }
// return Array.Empty<int>();