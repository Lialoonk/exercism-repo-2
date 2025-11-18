pub fn raindrops(x: u32) -> String {
    let mut s = String::new();

    if x % 3 == 0 {
        s.push_str("Pling");
    }

    if x % 5 == 0 {
        s.push_str("Plang");
    }

    if x % 7 == 0 {
        s.push_str("Plong");
    }

    if s == "" {
        return x.to_string();
    }
    s
}
