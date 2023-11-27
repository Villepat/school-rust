pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }

    // Adds an element to ref_list
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    // Removes all references that point to the same allocation as the given element
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        self.ref_list.retain(|x| !Rc::ptr_eq(x, &element));
    }
}

// Returns the number of strong references to the Rc
pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}
