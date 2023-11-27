// use crate::mobs::member::{Member, Role}; // Import Member and Role for use in this file

// impl Member {
//     // Associated function to create a new Member
//     pub fn new(name: String, role: Role, age: u8) -> Self {
//         Self { name, role, age }
//     }

//     // Associated function to promote a Member
//     pub fn get_promotion(&mut self) {
//         self.role = match self.role {
//             Role::Associate => Role::Soldier,
//             Role::Soldier => Role::Caporegime,
//             Role::Caporegime => Role::Underboss,
//             Role::Underboss => Role::Underboss, // No promotion beyond Underboss
//         };
//     }
//     pub fn role_score(&self) -> u32 {
//         match self.role {
//             Role::Underboss => 4,
//             Role::Caporegime => 3,
//             Role::Soldier => 2,
//             Role::Associate => 1,
//         }
//     }
// }
