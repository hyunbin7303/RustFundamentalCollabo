use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Rectangle<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T
}

// Trait bounds on generic structs
impl<T: PartialEq> Rectangle<T> {
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}
impl <T: Display> Rectangle<T> {

}


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}