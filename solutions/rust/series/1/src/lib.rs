pub fn series(t: &str, k: usize) -> Vec<String> {
    let c: Vec<char> = t.chars().collect();
    if k == 0 || k > c.len() {
        return Vec::new();
    }
    let mut u: Vec<String> = Vec::new();
    for p in c.windows(k) {
        let s: String = p.iter().collect();
        u.push(s);
    }
    u
}
