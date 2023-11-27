// Declare the function `stars` that takes a u32 argument `n` and returns a String
pub fn stars(n: u32) -> String {
    // Calculate 2^n using bitwise left shift, which is efficient
    let num_stars = 1 << n;
    let result = "*".repeat(num_stars as usize);
    result
}
