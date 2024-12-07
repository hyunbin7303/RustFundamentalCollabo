use std::fmt::Display;

pub struct Rectangle<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T
}

impl<T: PartialEq> Rectangle<T> {
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
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