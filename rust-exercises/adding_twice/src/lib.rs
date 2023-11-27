// Reusing the add_curry function
pub fn add_curry(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

// Define the twice function.
// It takes a Box containing a closure of type Fn(i32) -> i32 as an argument,
// and returns another Box containing a closure of the same type.
pub fn twice(F: Box<dyn Fn(i32) -> i32>) -> Box<dyn Fn(i32) -> i32> {
    // Return a Box containing a closure that applies F twice
    Box::new(move |x| F(F(x)))
}
