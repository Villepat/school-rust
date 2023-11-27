pub fn scytale_cipher(message: String, i: u32) -> String {
    //if length of message is not divisible by i, add spaces to the end
    let mut message = message;
    let mut message_length = message.len();
    let mut remainder = message_length % i as usize;
    if remainder != 0 {
        for _ in 0..(i - remainder as u32) {
            message.push(' ');
        }
    }
    let chars: Vec<char> = message.chars().collect();
    let n = chars.len();
    let mut result = vec![' '; n];

    // Calculate the number of rows, rounding up if necessary
    let rows = (n as f64 / i as f64).ceil() as usize;

    for j in 0..n {
        // Calculate the row and column for the current character
        let row = j % i as usize;
        let col = j / i as usize;

        // Calculate the index where this character will appear in the encrypted string
        let index = col + row * rows;

        // Place the character
        if index < n {
            result[index] = chars[j];
        }
    }

    // Convert the vector of characters back into a string
    let x: String = result.into_iter().collect();
    //remove trailing spaces using trim
    x.trim().to_string()
}
