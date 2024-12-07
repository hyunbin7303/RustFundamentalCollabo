// Definitely need to use the Box to keep the data in Heap.
// Check out this source code. https://gist.github.com/hardvain/32fca033bb61a5e3bf8bbeeb32fbbd5e
//https://www.youtube.com/watch?v=k0cL6K28SL0
#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node { value: val, next: None }
    }
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}
impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn insert_node_at_end(&mut self, data: T) {
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

// type List = Option<Box<Node<T>>>;
// fn vec_to_list(vector: Vec<i32>) -> List {
//     let mut curr = None;
//     for &value in vector.iter().rev() {
//         let mut new_node = Node::new(value);
//         new_node.next = curr;
//         curr = Some(Box::new(new_node));

//     }
//     curr
// }

fn main() {
    // let node = Node { value: 0, next: Some(Box::new(Node::new(1))) };
    // let vector = vec![0,1,2,3,4,5];
    // println!("{:?}", vec_to_list(vector));

    println!("Linked List Testing.");
    let mut linked_list: LinkedList<i32> = LinkedList::new();
    linked_list.insert_node_at_end(1);
    linked_list.insert_node_at_end(2);
    linked_list.insert_node_at_end(3);
    linked_list.insert_node_at_end(4);
    linked_list.insert_node_at_end(5);

    println!("{:?}", linked_list.head);
}

