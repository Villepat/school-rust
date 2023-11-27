#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(&self, first: u8, second: u8) -> Color {
        let mut new_color = self.clone(); // Create a new Color object based on the existing one

        // The swapping logic remains unchanged
        match (first, second) {
            (a, b) if a == new_color.r && b == new_color.g => {
                std::mem::swap(&mut new_color.r, &mut new_color.g)
            }
            (a, b) if a == new_color.r && b == new_color.b => {
                std::mem::swap(&mut new_color.r, &mut new_color.b)
            }
            (a, b) if a == new_color.r && b == new_color.a => {
                std::mem::swap(&mut new_color.r, &mut new_color.a)
            }
            (a, b) if a == new_color.g && b == new_color.r => {
                std::mem::swap(&mut new_color.g, &mut new_color.r)
            }
            (a, b) if a == new_color.g && b == new_color.b => {
                std::mem::swap(&mut new_color.g, &mut new_color.b)
            }
            (a, b) if a == new_color.g && b == new_color.a => {
                std::mem::swap(&mut new_color.g, &mut new_color.a)
            }
            (a, b) if a == new_color.b && b == new_color.r => {
                std::mem::swap(&mut new_color.b, &mut new_color.r)
            }
            (a, b) if a == new_color.b && b == new_color.g => {
                std::mem::swap(&mut new_color.b, &mut new_color.g)
            }
            (a, b) if a == new_color.b && b == new_color.a => {
                std::mem::swap(&mut new_color.b, &mut new_color.a)
            }
            (a, b) if a == new_color.a && b == new_color.r => {
                std::mem::swap(&mut new_color.a, &mut new_color.r)
            }
            (a, b) if a == new_color.a && b == new_color.g => {
                std::mem::swap(&mut new_color.a, &mut new_color.g)
            }
            (a, b) if a == new_color.a && b == new_color.b => {
                std::mem::swap(&mut new_color.a, &mut new_color.b)
            }
            _ => {}
        }
        new_color
    }
}
