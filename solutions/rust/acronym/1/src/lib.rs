pub fn abbreviate(p: &str) -> String {
    p.split(|c: char| c.is_whitespace() || c == '-')
        .filter(|t| !t.is_empty())
        .flat_map(|t| {
            let u: String = t.chars().filter(|c| c.is_alphabetic()).collect();
            if u.is_empty() {
                return Vec::new();
            }
            let mut a = u.chars();
            let f = a.next().unwrap();
            if u.chars().all(|x| x.is_uppercase()) {
                return vec![f];
            }
            let r = a
                .skip_while(|c| c.is_lowercase())
                .filter(|c| c.is_uppercase());
            std::iter::once(f).chain(r).collect::<Vec<_>>()
        })
        .map(|c| c.to_ascii_uppercase())
        .collect()
}
