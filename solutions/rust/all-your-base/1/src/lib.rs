#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(n: &[u32], a: u32, b: u32) -> Result<Vec<u32>, Error>{
    if a < 2{
        return Err(Error::InvalidInputBase);
    }
    if b < 2{
        return Err(Error::InvalidOutputBase);
    }
    if let Some(&d)=n.iter().find(|&x|*x >= a){
        return Err(Error::InvalidDigit(d));
    }
    let mut v = n.iter().fold(0,|s,&x|s * a + x);
    let mut r = Vec::new();
    if v == 0{
        r.push(0);
        return Ok(r);
    }
    while v > 0{
        r.push(v % b);
        v /= b;
    }
    r.reverse();
    Ok(r)
}
