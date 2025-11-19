pub fn is_valid(s: &str)->bool{
    s.chars()
        .rev()
        .filter(|z|!z.is_whitespace())
        .try_fold((0u32, 0u32),|(a,b),z|{
            z.to_digit(10).and_then(|d0|{
                let d1 = if b % 2 == 0{ d0 }else{ d0 * 2 };
                let d2 = if d1 > 9{ d1 - 9 }else{ d1 };
                Some((a + d2, b + 1))
            })
        })
        .map_or(false,|(a, b)|b >1 && a % 10 == 0)
}
