// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl<T> Point<T> {
//     fn swap(&mut self) {
//         std::mem::swap(&mut self.x, &mut self.y);
//     }
// }

// fn main() {
//     println!("Hello, world!");
//     let int_origin = Point { x: 0, y: 0 };
//     let float_origin = Point { x: 0.0, y: 0.0 };
// }


enum Option<T> {
    Some(T),
    None,
}
enum Result<T,E>{
    Ok(T),
    Err(E),
}
// The Result enum is generic over two types, T and E, and has two variants: Ok,
// which holds a value of type T, and Err, which holds a value of type E.


fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}


