#[derive(Debug,PartialEq)]
pub struct Dna{n:String}

#[derive(Debug,PartialEq)]
pub struct Rna{n:String}

const R:[char;4]=['C','G','A','U'];
const D:[char;4]=['G','C','T','A'];
fn v(s:&str,a:[char;4])->Result<String,usize>{
    match s.chars().position(|c|!a.contains(&c)){
        Some(i)=>Err(i),
        None=>Ok(s.to_string())
    }
}

fn t(x:char)->char{
    R[D.iter().position(|&c|c==x).unwrap()]
}
impl Dna{
    pub fn new(s:&str)->Result<Dna,usize>{
        v(s,D).map(|n|Dna{n})
    }
    pub fn into_rna(self)->Rna{
        let r=self.n.chars().fold(String::new(),|mut a,c|{
            a.push(t(c));
            a
        });
        Rna{n:r}
    }
}
impl Rna{
    pub fn new(s:&str)->Result<Rna,usize>{
        v(s,R).map(|n|Rna{n})
    }
}
