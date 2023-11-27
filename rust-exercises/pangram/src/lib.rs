use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut set = HashSet::new();

    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            set.insert(c.to_ascii_lowercase());
        }
    }

    set.len() == 26
}
