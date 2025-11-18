pub fn a(n: u32) -> u64 {
if !(1..=64).contains(&n) {
        panic!("pu-pu-pu errorrrrr");
    }
    let mut g = 1u64;
    for _ in 0..(n - 1) {
        g *= 2;
    }
    g
}

pub fn b() -> u64 {
    let mut t = 0u64;
    for i in 1..=64 {
        t += a(i);
    }
    t
}

pub fn square(n: u32) -> u64 {
    a(n)
}
pub fn total() -> u64 {
    b()
}
