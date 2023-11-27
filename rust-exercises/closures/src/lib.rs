pub fn first_fifty_even_square() -> Vec<i32> {
    // Create an iterator starting from 2, and increment by 2 to produce even numbers
    let even_numbers = (2..).step_by(2);

    // Use the map method with a closure to square each even number
    let even_numbers_squared = even_numbers.map(|x| x * x);

    // Collect the first 50 squared even numbers into a Vec<i32>
    let result: Vec<i32> = even_numbers_squared.take(50).collect();

    result
}

// Test function
fn main() {
    println!("Hello, world!");
    let v1 = first_fifty_even_square();
    println!("All elements in {:?}, len = {}", v1, v1.len());
}
