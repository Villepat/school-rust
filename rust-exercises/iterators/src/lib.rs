#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        // Return None if the value reaches 1
        if self.v == 1 {
            return None;
        }

        // Store the current value to return
        let current_value = self.v;

        // Update the value for the next iteration
        self.v = if self.v % 2 == 0 {
            self.v / 2
        } else {
            3 * self.v + 1
        };

        Some(Collatz { v: current_value })
    }
}

impl Collatz {
    // Constructor for Collatz struct
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

// Function to calculate the number of steps to reach 1
pub fn collatz(n: u64) -> usize {
    if n == 0 {
        return 0;
    }
    let mut count = 0;
    let mut collatz_iterator = Collatz::new(n);
    while let Some(_) = collatz_iterator.next() {
        count += 1;
    }
    count
}
