use std::cell::{Cell, RefCell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    // Initialize a new Workers object with default values
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    // Create a new thread and return its pid along with the thread object
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let pid = self.track_worker();
        let thread = Thread::new_thread(pid, c, self);
        (pid, thread)
    }

    // Get the index for the next new thread
    pub fn track_worker(&self) -> usize {
        let mut states = self.states.borrow_mut();
        states.push(false);
        states.len() - 1
    }

    // Check if a thread is dropped based on its pid
    pub fn is_dropped(&self, id: usize) -> bool {
        let states = self.states.borrow();
        states[id]
    }

    // Update the state and increment the drops when a thread is dropped
    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if states[id] {
            panic!("{} is already dropped", id);
        }
        states[id] = true;
        let new_drops = self.drops.get() + 1;
        self.drops.set(new_drops);
    }
}

use std::ops::Drop;

#[derive(Debug, Clone, Eq, PartialEq)]
//'a in the definition of Thread is a lifetime specifier
//as we place 'a in the parent field, we are saying that the Thread struct
//cannot outlive the Workers struct
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a Workers,
}
//here we are implementing a lifetime for the Thread struct and its methods
impl<'a> Thread<'a> {
    // Initialize a new Thread object
    pub fn new_thread(pid: usize, cmd: String, parent: &'a Workers) -> Thread<'a> {
        Thread { pid, cmd, parent }
    }

    // Drop the thread, which will invoke the Drop trait logic
    pub fn skill(self) {
        // This function is intentionally left blank.
        // The Drop trait will handle the resource cleanup.
    }
}

impl<'a> Drop for Thread<'a> {
    // Implement the Drop trait to update the Workers struct
    fn drop(&mut self) {
        self.parent.add_drop(self.pid);
    }
}
