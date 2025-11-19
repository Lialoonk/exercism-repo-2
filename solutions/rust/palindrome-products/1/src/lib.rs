use std::collections::HashSet;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Palindrome{
    v:u64,
    a:u64,
    b:u64,
}
impl Palindrome{
    pub fn value(&self)->u64{self.v}
    pub fn into_factors(self)->HashSet<(u64,u64)>{
        let mut h=HashSet::new();
        let x=self.v;
        for i in self.a..=((x as f64).sqrt()as u64).min(self.b){
            if x%i==0{
                let j=x/i;
                if j>=self.a&&j<=self.b{
                    h.insert((i,j));
                }
            }
        }
        h
    }
}

fn p(n:u64)->bool{
    let mut x=n;
    let mut r=0;
    while x>0{
        r=r*10+x%10;
        x/=10;
    }
    r==n
}

pub fn palindrome_products(a:u64,b:u64)->Option<(Palindrome,Palindrome)>{
    if a>b{return None;}
    let(mut mn,mut mx)=(None,None);
    for i in a..=b{
        for j in i..=b{
            let t=i*j;
            if p(t){
                if mn.map_or(true,|u|t<u){mn=Some(t);}
                if mx.map_or(true,|u|t>u){mx=Some(t);}
            }
        }
    }
    match(mn,mx){
        (Some(x),Some(y))=>Some((
            Palindrome{v:x,a,b},
            Palindrome{v:y,a,b}
        )),
        _=>None
    }
}
