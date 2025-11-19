use std::collections::BTreeMap;

pub fn transform(m: &BTreeMap<i32,Vec<char>>)->BTreeMap<char,i32>{
    m.iter()
        .flat_map(|(&a, b)| b.iter().map(move|x|(x.to_ascii_lowercase(),a)))
        .collect()
}
