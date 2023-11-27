// Import helper functions from boss_helper
//use crate::mobs::boss_helper;

// Define the Boss struct
#[derive(Debug, Clone, PartialEq)]
pub struct Boss {
    pub name: String, // The name of the Boss
    pub age: u8,      // The age of the Boss
}
impl Boss {
    // Associated function to create a new Boss
    pub fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}
