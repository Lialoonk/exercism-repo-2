use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, candidates: &'a [&str]) -> HashSet<&'a str> {
    let mut found = HashSet::new();
    let key = make_key(word);
    let lowered_word = word.to_lowercase();

    for &c in candidates {
        if c.to_lowercase() == lowered_word {
            continue;
        }
        if make_key(c) == key {
            found.insert(c);
        }
    }
    found
}
fn make_key(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars
}
