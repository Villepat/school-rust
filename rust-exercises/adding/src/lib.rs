// Define the add_curry function that takes an i32 as input
// and returns a closure of type Box<dyn Fn(i32) -> i32>.
pub fn add_curry(x: i32) -> Box<dyn Fn(i32) -> i32> {
    // Create a Box containing a closure that captures the value of x
    // and takes another i32 as input, returning their sum.
    Box::new(move |y| x + y)
}

// The main function to test the add_curry function.
fn main() {
    // Create closures that add -10, 2066, and 300000 respectively.
    let add10 = add_curry(-10);
    let add20 = add_curry(2066);
    let add30 = add_curry(300000);

    // Test the closures.
    println!("{}", add10(5));
    println!("{}", add20(195));
    println!("{}", add30(5696));
}
