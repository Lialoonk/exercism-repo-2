pub fn factors(mut x: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut c = 2..;

    while x > 1 {
        let k = c.next().unwrap();

        while x % k == 0 {
            x /= k;
            v.push(k);
        }
    }
    v
}
