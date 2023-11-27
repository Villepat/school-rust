extern crate case; // Import the case crate

pub use case::CaseExt;

// Your provided edit_distance function
pub fn edit_distance(source: &str, target: &str) -> usize {
    let source = source.chars().collect::<Vec<_>>();
    let target = target.chars().collect::<Vec<_>>();
    let m = source.len();
    let n = target.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            if source[i - 1] == target[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] =
                    1 + std::cmp::min(dp[i - 1][j - 1], std::cmp::min(dp[i - 1][j], dp[i][j - 1]));
            }
        }
    }
    dp[m][n]
}
pub fn expected_variable(c: &str, exp: &str) -> Option<String> {
    let comp = lowercase_first_char(c);

    if !comp.to_snake().contains("_") {
        return None;
    }

    let ed = edit_distance(&comp.to_lowercase(), &exp.to_lowercase());
    if ed == 0 {
        return Some("100%".to_string());
    }

    let perc = (exp.len() as f64 - ed as f64) / exp.len() as f64;

    if perc < 0.5 {
        return None;
    }

    Some(format!("{}%", (perc * 100.0).round()))
}
fn lowercase_first_char(s: &str) -> String {
    let mut chars = s.chars();
    if let Some(c) = chars.next() {
        let first_char_lower = c.to_lowercase().next().unwrap();
        let rest = chars.as_str();
        let result = format!("{}{}", first_char_lower, rest);
        return result;
    }
    s.to_string()
}
