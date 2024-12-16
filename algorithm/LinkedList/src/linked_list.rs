use crate::node::Node;


pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn insert_node_at_end(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));
        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        //Update to the latest node.
        // Change the current node to the latest.
        let mut curr = self.head.as_mut().unwrap();
        while let Some(ref mut next_node) = curr.next {
            curr = next_node;
        }
        curr.next = Some(new_node);
    }
}
