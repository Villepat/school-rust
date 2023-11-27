// Function takes a Vec<usize> as an argument and returns a Vec<usize>
pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    // Calculate the total product of all elements
    let total_product: usize = arr.iter().product();
    //print total_product
    //if len is 0 or 1 return
    if arr.len() == 0 || arr.len() == 1 {
        return [].to_vec();
    }

    // Create a new vector to store the result
    let mut result: Vec<usize> = Vec::new();

    // Loop through the original array
    for &num in arr.iter() {
        // For each element, the product of all elements except itself
        // is the total product divided by the current element.
        if num != 0 {
            result.push(total_product / num);
        } else {
            // Handle the special case where the current element is 0
            result.push(0);
        }
    }

    result
}
