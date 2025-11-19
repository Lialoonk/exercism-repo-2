pub fn collatz(z: u64) -> Option<u64> {
    match z {
        0 => None,
        1 => Some(0),
        _ => {
            let m = if z % 2 == 0 { z / 2 } else { z * 3 + 1 };
            collatz(m).map(|t| t + 1)
        }
    }
}
