#[derive(Debug,PartialEq,Eq)]
pub enum Classification{
    Abundant,
    Perfect,
    Deficient
}

pub fn classify(n:u64)->Option<Classification>{
    if n==0{return None;}
    let mut s=0;
    for x in 1..n{
        if n%x==0{s+=x;}
    }
    if s<n{
        Some(Classification::Deficient)
    }else if s==n{
        Some(Classification::Perfect)
    }else{
        Some(Classification::Abundant)
    }
}
