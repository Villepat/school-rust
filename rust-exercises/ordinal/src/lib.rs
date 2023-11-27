pub fn num_to_ordinal(x: u32) -> String {
    let last_two_digits = x % 100;
    let last_digit = x % 10;

    let suffix = match last_two_digits {
        11 | 12 | 13 => "th",
        _ => match last_digit {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };

    format!("{}{}", x, suffix)
}
