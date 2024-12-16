#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node { value: val, next: None }
    }
}