pub fn is_valid_isbn(s: &str) -> bool {
    let mut d = Vec::new();
    for c in s.chars() {
        match c {
            x if x.is_digit(10) => d.push(x.to_digit(10).unwrap()),
            'X' if d.len() == 9 => d.push(10),
            '-' => {}
            _ => return false,
        }
    }
    if d.len() != 10{
        return false;
    }
    let mut t = 0;
    for (a, z) in d.iter().enumerate(){
        t += z * (10 - a as u32);
    }
    t % 11 == 0
}
