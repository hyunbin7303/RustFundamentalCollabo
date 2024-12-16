 // Source code from : https://dev.to/felixfaisal/implementing-linked-list-in-rust-3and



#[derive(Clone)]
pub enum address {
    address(Box<LList>),
    Nil,
}

#[derive(Clone)]
pub struct LList {
    pub value: i32,
    pub next: address,
}


impl LList {
    pub fn add_node_to_the_end(&mut self, value: i32) {
        match self.next {
            address::address(ref mut next_addr) => {
                next_addr.add_node_to_the_end(value);
            }
            address::Nil => {
                let node = LList {
                    value: value,
                    next: address::Nil
                };
                self.next = address::address(Box::new(node))
            }
        }
    }
    pub fn print_out(&self) {
        if self.value == 0 {
            println!("The list is empty.");
        } else {
            println!("{}", self.value);
            match self.next {
                address::address(ref next_addr) => next_addr.print_out(),
                address::Nil => {}
            }
        }
    }

}