pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    } else if n == 1_000_000 {
        return "one million".to_string();
    }

    let mut result = String::new();

    let thousands = n / 1000;
    if thousands > 0 {
        if thousands == 1 {
            result += "one thousand";
        } else {
            result += &digit_spell(thousands as u8);
            result += " thousand";
        }
        if n % 1000 > 0 {
            result += " ";
        }
    }

    let hundreds = n % 1000 / 100;
    if hundreds > 0 {
        result += &digit_spell(hundreds as u8);
        result += " hundred";
        if n % 100 != 0 {
            result += " ";
        }
    }

    let tens = n % 100 / 10;
    let ones = n % 10;
    if tens == 1 {
        result += &digit_spell((tens * 10 + ones) as u8);
    } else {
        if tens > 0 {
            result += &digit_spell(tens as u8 * 10);
            if ones > 0 {
                result += "-";
            }
        }
        if ones > 0 && tens != 1 {
            result += &digit_spell(ones as u8);
        }
    }

    result
}

fn digit_spell(dig: u8) -> String {
    match dig {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        _ => panic!("Invalid digit: {}", dig),
    }
}
