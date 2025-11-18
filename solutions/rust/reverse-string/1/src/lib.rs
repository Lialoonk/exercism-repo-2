pub fn reverse(s: &str) -> String {
    let mut result = String::new();

    let chars: Vec<char> = s.chars().collect();
    let mut i = chars.len();
    while i > 0 {
        i -= 1;
        result.push(chars[i]);
    }

    result
}
