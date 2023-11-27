pub fn number_logic(num: u32) -> bool {
    let num_digits = num.to_string().len() as u32;
    let sum: u32 = num
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d.pow(num_digits))
        .sum();

    sum == num
}
