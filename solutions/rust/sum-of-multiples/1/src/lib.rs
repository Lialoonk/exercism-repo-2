pub fn sum_of_multiples(l: u32, v: &[u32]) -> u32 {
    (1..l)
        .filter(|&x| {
            for &d in v {
                if d != 0 && x % d == 0 {
                    return true;
                }
            }
            false
        })
        .sum()
}
