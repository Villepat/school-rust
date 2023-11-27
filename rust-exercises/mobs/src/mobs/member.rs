// Import helper functions from member_helper
//use crate::mobs::member_helper;

// Define the Role enum
#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

// Define the Member struct
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String, // The name of the Member
    pub role: Role,   // The role of the Member
    pub age: u8,      // The age of the Member
}
impl Member {
    // Associated function to create a new Member
    pub fn new(name: &str, role: Role, age: u8) -> Self {
        Self {
            name: name.to_string(),
            role,
            age,
        }
    }

    // Associated function to promote a Member
    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Associate => Role::Soldier,
            Role::Soldier => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            Role::Underboss => Role::Underboss, // No promotion beyond Underboss
        };
    }
    pub fn role_score(&self) -> u32 {
        match self.role {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
        }
    }
}
