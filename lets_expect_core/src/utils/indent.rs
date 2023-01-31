const INDENT: &str = "    ";

pub fn indent(lines: &[String], levels: u8) -> Vec<String> {
    let prefix = INDENT.repeat(levels as usize);

    lines
        .iter()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<String>>()
}
