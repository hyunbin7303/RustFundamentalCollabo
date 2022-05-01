


// Bubble sort 
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    // for i in 0..arr.len() {
    //     for j in 0..arr.len() - 1 - i {
    //         if arr[j] > arr[j + 1] {
    //             arr.swap(j, j + 1);
    //         }
    //     }
    // }
}

fn main() {
    println!("Hello, world!");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    bubble_sort(&mut numbers);
}
