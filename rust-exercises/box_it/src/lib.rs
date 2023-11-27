pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    // Split the string by spaces and map each substring
    let vec: Vec<u32> = s
        .split_whitespace()
        .map(|num_str| {
            // Check if the number has a 'k' suffix
            if num_str.ends_with('k') {
                // Remove the 'k' and parse the remaining part as f32
                let num: f32 = num_str.trim_end_matches('k').parse().unwrap();
                // Multiply by 1000 and cast to u32
                (num * 1000.0) as u32
            } else {
                // Parse the number as u32
                num_str.parse().unwrap()
            }
        })
        .collect();

    // Box the vector and return
    Box::new(vec)
}
// Think of the box as a "container" on the heap. 
// Using * is like opening the container and taking out what's inside.
//  Once you do, the container (Box) is discarded, and you are responsible for the contents (Vec<u32>).
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    // Unbox and return the value
    *a
}
