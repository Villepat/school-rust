// Declare the function score that takes a string slice and returns a u64.
pub fn score(s: &str) -> u64 {
    // Initialize sum to hold the total score.
    let mut sum: u64 = 0;

    // Iterate over each character in the string.
    for c in s.chars() {
        // Convert the character to uppercase to make it case-insensitive.
        let char = c.to_ascii_uppercase();

        // Find the value of each character using a match expression.
        let value = match char {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0, // Any other characters (non-letters, special characters) have a score of 0.
        };

        // Add the value to the sum.
        sum += value;
    }

    // Return the total sum.
    sum
}
