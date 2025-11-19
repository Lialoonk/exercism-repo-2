#[derive(Debug,PartialEq)]
pub enum Error{
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(vs:&[u32])->Vec<u8>{
    let mut o=Vec::new();
    for mut v in vs.iter().cloned(){
        let mut b=Vec::new();
        if v==0{
            o.push(0);
            continue;
        }
        let mut f=true;
        while v>0{
            let x=(v%128)as u8;
            if f{
                b.push(x);
                f=false;
            }else{
                b.push(x|0x80);
            }
            v>>=7;
        }
        while let Some(x)=b.pop(){
            o.push(x);
        }
    }
    o
}

pub fn from_bytes(bs:&[u8])->Result<Vec<u32>,Error>{
    if bs.is_empty()||bs.last().unwrap()&0x80!=0{
        return Err(Error::IncompleteNumber);
    }
    let mut o=Vec::new();
    let mut n=0u32;
    for &b in bs{
        n=(n<<7)+(b&0x7f)as u32;
        if b&0x80==0{
            o.push(n);
            n=0;
        }else if n>=(1<<25){
            return Err(Error::Overflow);
        }
    }
    Ok(o)
}
