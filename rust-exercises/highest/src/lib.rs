#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    // Creates a new instance of Numbers
    pub fn new(numbers: &'a [u32]) -> Self {
        Self { numbers }
    }

    // Returns the list of numbers
    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    // Returns the latest (last) number in the list as an Option
    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }

    // Returns the highest number in the list as an Option
    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    // Returns a Vec<u32> with the three highest numbers
    pub fn highest_three(&self) -> Vec<u32> {
        let mut sorted = self.numbers.to_vec();
        sorted.sort();
        sorted.iter().rev().take(3).copied().collect()
    }
}

// Usage example
fn main() {
    let expected = [30, 500, 20, 70];
    let n = Numbers::new(&expected);
    println!("{:?}", n.list());
    println!("{:?}", n.highest());
    println!("{:?}", n.latest());
    println!("{:?}", n.highest_three());
}
