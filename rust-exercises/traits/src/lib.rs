// Traits and Structures
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub strength: f64,
    pub score: i32,
    pub money: i32,
    pub weapons: Vec<String>,
}

pub struct Fruit {
    pub weight_in_kg: f64,
}

pub struct Meat {
    pub weight_in_kg: f64,
    pub fat_content: f64,
}

// Food Trait
pub trait Food {
    fn gives(&self) -> f64;
}

// Implement Food for Fruit
impl Food for Fruit {
    fn gives(&self) -> f64 {
        self.weight_in_kg * 4.0
    }
}

// Implement Food for Meat
impl Food for Meat {
    fn gives(&self) -> f64 {
        let fat = self.weight_in_kg * self.fat_content;
        let protein = self.weight_in_kg - fat;
        (protein * 4.0) + (fat * 9.0)
    }
}

// Implement Display for Player
use std::fmt;

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)?;

        // Check if strength is an integer and format it accordingly
        let strength_str = if self.strength.fract() == 0.0 {
            format!("{:.0}", self.strength)
        } else {
            format!("{:.1}", self.strength)
        };

        writeln!(
            f,
            "Strength: {}, Score: {}, Money: {}",
            strength_str, self.score, self.money
        )?;
        write!(f, "Weapons: {:?}", self.weapons)
    }
}

// Player eat method
impl Player {
    pub fn eat<T: Food>(&mut self, food: T) {
        self.strength += food.gives();
    }
}
