
// #[path = "../../util.rs"]
// use util::load;

// or in this way
//#[path = "../src/calculator.rs"] mod calculator;


#[cfg(test)]
mod tests {
    use traits::formula::Rectangle;


    #[test]
    fn test_is_square_success() {
        let mut r = Rectangle {
            x: 0,
            y: 0,
            width: 47,
            height: 47,
        };

        assert!(r.is_square());


    }
    #[test]
    fn test_is_square_failure() {
        let mut r = Rectangle {
            x: 0,
            y: 0,
            width: 47,
            height: 34,
        };
        assert!(!r.is_square());
    }
}