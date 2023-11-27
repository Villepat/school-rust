pub fn talking(text: &str) -> &'static str {
    let trimmed_text = text.trim();

    if trimmed_text.is_empty() {
        return "Just say something!";
    }

    let has_alphabetic = text.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_alphabetic
        && text
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase());
    let is_question = text.trim_end().ends_with('?');

    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        (false, false) => "Interesting",
    }
}
