use std::collections::HashSet;

pub fn is_pangram(a:&str)->bool{
    a.to_lowercase()
        .chars()
        .filter(|z| z.is_alphabetic())
        .collect::<HashSet<_>>()
        .len()==26
}
