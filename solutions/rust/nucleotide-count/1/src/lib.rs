use std::collections::HashMap;

pub fn count(x: char, s: &str)->Result<usize, char>{
    let mut h = nucleotide_counts(s)?;
    match h.remove(&x){
        Some(v) => Ok(v),
        None => Err(x),
    }
}

pub fn nucleotide_counts(s: &str)->Result<HashMap<char,usize>,char>{
    let mut h: HashMap<char, usize>=
        ['A','C','G','T'].iter().map(|c| (*c, 0)).collect();

    for c in s.chars(){
        match h.get_mut(&c){
            Some(v)=>*v += 1,
            None=>return Err(c),
        }
    }
    Ok(h)
}
