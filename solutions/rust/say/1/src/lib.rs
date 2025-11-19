use std::iter::successors;

const A:&[&str]=&[
    "zero","one","two","three","four","five","six","seven","eight","nine",
    "ten","eleven","twelve","thirteen","fourteen","fifteen","sixteen",
    "seventeen","eighteen","nineteen"
];

const B:&[&str]=&[
    "zero","ten","twenty","thirty","forty","fifty","sixty","seventy","eighty","ninety"
];

const C:&[&str]=&[
    "zero","thousand","million","billion","trillion","quadrillion","quintillion"
];
pub fn encode(n:u64)->String{
    match n{
        0..= 19 =>A[n as usize].into(),
        20..= 99 =>{
            let u = (n/10) as usize;
            let r =n%10;
            if r==0{
                B[u].into()
            }else{
                format!("{}-{}",B[u],encode(r))
            }
        }
        100..= 999 =>h(n,100,"hundred"),
        _=>{
            let(d,o)=successors(Some(1u64),|x|x.checked_mul(1000))
                .zip(C.iter())
                .find(|(x,_)|*x>n/1000)
                .unwrap();
            h(n,d,o)
        }
    }
}

fn h(n:u64,d:u64,o:&str) -> String{
    let u = n/d;
    let r = n%d;
    if r==0{
        format!("{} {}",encode(u),o)
    }else{
        format!("{} {} {}",encode(u),o,encode(r))
    }
}
