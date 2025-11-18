const KD: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
    "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry",
];

pub fn plants(d: &str, k: &str) -> Vec<&'static str> {
    let pos = KD.iter().position(|&x| x == k).unwrap();
    let idx = pos * 2;
    let mut out = Vec::new();
    for line in d.lines() {
        let a = line.chars().nth(idx).unwrap();
        let b = line.chars().nth(idx + 1).unwrap();

        out.push(t(a));
        out.push(t(b));
    }
    out
}

fn t(x: char) -> &'static str {
    match x {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        _   => "violets",
    }
}
