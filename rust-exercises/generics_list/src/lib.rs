// Create a linked list of generic values with the following methods.

// new: returns a new empty list.
// push: adds an element to the beginning of the list.
// pop: deletes an element from the list based on LIFO.
// len: returns the size of the list.

#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
    pub len: usize,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None, len: 0 }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.head.take().map(Box::new),
        };
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop(&mut self) {
        self.head = self
            .head
            .take()
            .and_then(|node| node.next.map(|boxed| *boxed));
        self.len -= 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
