#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}

impl GameSession {
    // Ownership: Takes ownership of id, p1_name, p2_name, and nb_games as arguments
    // Returns a Box<GameSession>, transferring ownership to the caller
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        // Transfers ownership of all variables into the new Box
        Box::new(GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        })
    }

    // Ownership: Takes an immutable reference to self, so no ownership change
    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 == self.p2.1 {
            ("Same score! tied".to_string(), self.p1.1)
        } else if self.p1.1 > self.p2.1 {
            (self.p1.0.clone(), self.p1.1) // Clones the string, transferring ownership of the clone
        } else {
            (self.p2.0.clone(), self.p2.1) // Clones the string, transferring ownership of the clone
        }
    }

    // Ownership: Takes a mutable reference to self and ownership of user_name
    pub fn update_score(&mut self, user_name: String) {
        // No ownership change for self as it's a mutable reference
        // user_name is consumed within the function
        if self.p1.1 + self.p2.1 >= self.nb_games
            || self.p1.1 * 2 > self.nb_games
            || self.p2.1 * 2 > self.nb_games
        {
            return; // Game is finished or one player has won
        }
        if self.p1.0 == user_name {
            self.p1.1 += 1; // Mutably borrows self
        } else if self.p2.0 == user_name {
            self.p2.1 += 1; // Mutably borrows self
        }
    }

    // Ownership: Takes ownership of self, consuming the Box<GameSession>
    pub fn delete(self) -> String {
        // Consumes self and returns a new String, transferring its ownership to the caller
        format!("game deleted: id -> {}", self.id)
    }
}
