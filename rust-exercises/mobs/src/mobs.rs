pub mod boss; // The actual file will be in src/mobs/boss.rs
pub mod boss_helper; // The actual file will be in src/mobs/boss_helper.rs
pub mod member; // The actual file will be in src/mobs/member.rs
pub mod member_helper; // The actual file will be in src/mobs/member_helper.rs
pub use boss::Boss; // Import Boss struct for use in this file
pub use member::{Member, Role}; // Import Member struct and Role enum for use in this file

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,              // The name of the Mob
    pub boss: Boss,                // The Boss leading the Mob
    pub members: Vec<Member>,      // A vector of Members in the Mob
    pub cities: Vec<(String, u8)>, // A vector of tuples containing city names and a u8 value
    pub wealth: u32,               // The wealth of the Mob
}

impl Mob {
    // Adds a new Member to the members vector.
    pub fn recruit(&mut self, name: &str, age: u8) {
        // Convert the &str to a String
        let name_string = name.to_string();

        // Add a new member with the role of Associate
        let new_member = Member::new(&name_string, Role::Associate, age);

        // Add the new member to the members vector
        self.members.push(new_member);
    }

    // // Performs an attack on another Mob.
    // pub fn attack(&mut self, target: &mut Mob) {
    //     // Calculate the power combat score for self
    //     let self_score: u32 = self.members.iter().map(|member| member.role_score()).sum();

    //     // Calculate the power combat score for target
    //     let target_score: u32 = target
    //         .members
    //         .iter()
    //         .map(|member| member.role_score())
    //         .sum();

    //     // Determine the outcome
    //     let loser = match self_score.cmp(&target_score) {
    //         std::cmp::Ordering::Greater => {
    //             target.members.pop();
    //             "target"
    //         }
    //         std::cmp::Ordering::Less => {
    //             self.members.pop();
    //             "self"
    //         }
    //         std::cmp::Ordering::Equal => {
    //             self.members.pop(); // In case of a draw, the attacker loses
    //             "self"
    //         }
    //     };

    //     // Transfer cities and wealth if a mob is defeated (has zero members)
    //     if loser == "self" && self.members.is_empty() {
    //         target.cities.append(&mut self.cities);
    //         target.wealth += self.wealth;
    //     } else if loser == "target" && target.members.is_empty() {
    //         self.cities.append(&mut target.cities);
    //         self.wealth += target.wealth;
    //     }
    // }
    pub fn attack(&mut self, enemy: &mut Mob) {
        let attacker_power = Mob::calculate_power(self);
        let enemy_power = Mob::calculate_power(enemy);

        if attacker_power < enemy_power || attacker_power == enemy_power {
            self.members.pop();
        } else {
            enemy.members.pop();
        }

        if enemy.members.is_empty() {
            self.wealth += enemy.wealth;
            enemy.wealth = 0;
            self.cities.append(&mut enemy.cities);
        } else if self.members.is_empty() {
            enemy.wealth += self.wealth;
            self.wealth = 0;
            enemy.cities.append(&mut self.cities);
        }
    }
    fn calculate_power(mob: &Mob) -> usize {
        let mut result_power: usize = 0;
        for member in &mob.members {
            match member.role {
                member::Role::Underboss => result_power += 4,
                member::Role::Caporegime => result_power += 3,
                member::Role::Soldier => result_power += 2,
                member::Role::Associate => result_power += 1,
            }
        }

        return result_power;
    }

    // Steals a specified amount of wealth from another Mob.
    pub fn steal(&mut self, target: &mut Mob, amount: u32) {
        // Calculate the actual amount that can be stolen
        let actual_amount = std::cmp::min(amount, target.wealth);

        // Deduct the amount from the target's wealth
        target.wealth -= actual_amount;

        // Add the stolen amount to self's wealth
        self.wealth += actual_amount;
    }

    // Adds a city to its list of cities if no other Mob owns it.
    // Adds a city to its list of cities if no other Mob owns it.
    pub fn conquer_city(&mut self, all_mobs: Vec<Mob>, city_name: String, value: u8) {
        // Check if any other mob already controls the city
        let city_already_controlled = all_mobs
            .iter()
            .any(|mob| mob.cities.iter().any(|(name, _)| name == &city_name));

        // If no other mob controls the city, add it to the list of cities
        if !city_already_controlled {
            self.cities.push((city_name, value));
        }
    }
}
