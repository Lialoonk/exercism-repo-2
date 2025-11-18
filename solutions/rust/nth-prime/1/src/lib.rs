pub fn p(x: u32) -> bool {
    if x < 2 {
        return false;
    }
    for i in 2..x {
        if x % i == 0 {
            return false;
        }
    }
    true
}
pub fn nth(n: u32) -> u32 {
    let mut c = 0;
    let mut k = 2;

    while c <= n {
        if p(k) {
            if c == n {
                return k;
            }
            c += 1;
        }
        k += 1;
    }
    0
}
