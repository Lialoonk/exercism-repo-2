pub fn is_armstrong_number(n: u32) -> bool {
    let mut v = Vec::new();
    let mut t = n;

    if t == 0 {
        v.push(0);
    } else {
        while t > 0 {
            v.push(t % 10);
            t /= 10;
        }
    }
    let c = v.len() as u32;
    let mut s = 0u32;

    for d in &v {
        s += d.pow(c);
    }

    s == n
}
