use std::collections::HashSet;
pub fn check(s: &str) -> bool{
    let mut h = HashSet::new();

    s.to_lowercase()
        .chars()
        .filter(|x| x.is_alphabetic())
        .all(|x| h.insert(x))
}
