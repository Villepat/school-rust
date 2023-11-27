pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();

    for c in input.chars() {
        let rotated_char = if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() {
                'a' as i8
            } else {
                'A' as i8
            };
            let rotated = ((c as i8 - base + key + 26) % 26) + base;
            rotated as u8 as char
        } else {
            c
        };

        result.push(rotated_char);
    }

    result
}
