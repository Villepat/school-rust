// The function takes a slice of i32 integers and a key of type i32.
pub fn search(array: &[i32], key: i32) -> Option<usize> {
    // Iterate over the slice, checking each item and its index.
    for (index, &item) in array.iter().enumerate() {
        // If the item matches the key, return its index.
        if item == key {
            return Some(index);
        }
    }
    // If the loop completes without finding the key, return None.
    None
}
