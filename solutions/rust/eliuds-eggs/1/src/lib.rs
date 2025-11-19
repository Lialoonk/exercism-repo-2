pub fn egg_count(mut a: u32) -> usize {
    let mut s = 0;
    while a != 0 {
        s += (a & 1) as usize;
        a >>= 1;
    }
    s
}