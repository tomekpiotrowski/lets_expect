pub fn prepend(lines: &[String], prefix: &str) -> Vec<String> {
    lines
        .iter()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<String>>()
}
