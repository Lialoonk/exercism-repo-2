use std::collections::HashMap;

const H:&str="Team                           | MP |  W |  D |  L |  P";

#[derive(Default,Eq,PartialEq)]
struct T{
    n:String,
    m:u8,
    w:u8,
    d:u8,
    l:u8,
    p:u16,
}
impl T{
    fn new(n:String)->Self{
        Self{n,..Default::default()}
    }
    fn w(&mut self){ self.w+=1; self.m+=1; self.p+=3; }
    fn l(&mut self){ self.l+=1; self.m+=1; }
    fn d(&mut self){ self.d+=1; self.m+=1; self.p+=1; }
    fn add(&mut self,r:&R){
        match r{
            R::W=>self.w(),
            R::L=>self.l(),
            R::D=>self.d(),
        }
    }
}
impl From<&T> for String{
    fn from(t:&T)->String{
        format!("{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
                t.n,t.m,t.w,t.d,t.l,t.p)
    }
}

enum R{W,L,D}
impl From<&str> for R{
    fn from(s:&str)->Self{
        match s{
            "win"=>R::W,
            "loss"=>R::L,
            "draw"=>R::D,
            _=>panic!()
        }
    }
}
impl R{
    fn rev(&self)->Self{
        match self{
            R::W=>R::L,
            R::L=>R::W,
            R::D=>R::D
        }
    }
}

pub fn tally(s:&str)->String{
    let m:HashMap<String,T>=s.lines().fold(
        HashMap::new(),
        |mut h,l|{
            let f:Vec<&str>=l.split(';').collect();
            let a=f[0];
            let b=f[1];
            let r:R=f[2].into();
            h.entry(a.into())
                .or_insert(T::new(a.into()))
                .add(&r);
            h.entry(b.into())
                .or_insert(T::new(b.into()))
                .add(&r.rev());
            h
        }
    );

    let mut v:Vec<&T>=m.values().collect();
    v.sort_by(|x,y|y.p.cmp(&x.p).then_with(||x.n.cmp(&y.n)));
    std::iter::once(H.to_string())
        .chain(v.into_iter().map(|t|t.into()))
        .collect::<Vec<_>>()
        .join("\n")
}
