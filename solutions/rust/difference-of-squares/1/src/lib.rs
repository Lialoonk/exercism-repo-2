pub fn a(n: u32) -> u32 {
    let mut s = 0;
    for i in 1..=n {
        s += i;
    }
    s * s
}

pub fn b(n: u32) -> u32 {
    let mut s = 0;
    for i in 1..=n {
        s += i * i;
    }
    s
}

pub fn square_of_sum(n: u32) -> u32 {
    a(n)
}

pub fn sum_of_squares(n: u32) -> u32 {
    b(n)
}

pub fn difference(n: u32) -> u32 {
    a(n) - b(n)
}
