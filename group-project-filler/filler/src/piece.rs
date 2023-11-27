use std::fmt;

pub struct Piece {
    pub shape: Vec<Vec<String>>,
}

impl Piece {
    pub fn from_string(shape: &str) -> Piece {
        let mut parsed_shape = Vec::new();
        for row in shape.lines() {
            let mut parsed_row = Vec::new();
            for ch in row.chars() {
                parsed_row.push(match ch {
                    '.' => ".".to_string(),
                    '#' => "#".to_string(),
                    _ => panic!("Unknown piece cell type"),
                });
            }
            parsed_shape.push(parsed_row);
        }
        Piece {
            shape: parsed_shape,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.shape {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
