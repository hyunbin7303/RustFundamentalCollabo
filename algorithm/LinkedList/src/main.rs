// Definitely need to use the Box to keep the data in Heap.
// Check out this source code. https://gist.github.com/hardvain/32fca033bb61a5e3bf8bbeeb32fbbd5e
//https://www.youtube.com/watch?v=k0cL6K28SL0


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


mod node;
mod linked_list;

use linked_list::LinkedList;
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

