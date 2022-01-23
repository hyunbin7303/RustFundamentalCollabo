

fn main() {
    println!("Hello, world!");
    // Tuple testing
    
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let x: i32 = 42;
    let y = -20;
    let xy = x as i64 * y;

}

fn multiply(x:i32, y:i32) -> i32{
    x * y
}

#[cfg(test)]
mod tests {
    use super::multiply;

    #[test]
    fn test_multiply(){
        let answer = multiply(7, 5);
        assert_eq!(35,answer);
    }
}