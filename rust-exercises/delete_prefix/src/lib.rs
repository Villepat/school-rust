pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    // Check if the given string starts with the prefix
    if s.starts_with(prefix) {
        // Calculate the length of the prefix
        let prefix_len = prefix.len();

        // Return the remaining part of the string wrapped in Some
        Some(&s[prefix_len..])
    } else {
        // If the string doesn't start with the prefix, return None
        None
    }
}
