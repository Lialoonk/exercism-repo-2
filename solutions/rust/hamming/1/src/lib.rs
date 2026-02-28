pub fn hamming_distance(a: &str, b: &str) -> Option<usize>{
    if a.len() != b.len(){
        return None;
    }
    if a == b{
        return Some(0);
    }
    let mut c = 0;
    for (x, y) in a.chars().zip(b.chars()){
        if x != y{
            c += 1;
        }
    }
    Some(c)
}
