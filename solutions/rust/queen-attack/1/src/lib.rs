#[derive(Debug)]
pub struct ChessPosition{
    rank:i32,
    file:i32,
}
#[derive(Debug)]
pub struct Queen{
    position:ChessPosition,
}
impl ChessPosition{
    pub fn new(r:i32,f:i32)->Option<Self>{
        match(r,f){
            (0..=7,0..=7)=>Some(Self{rank:r,file:f}),
            _=>None,
        }
    }
}

impl Queen{
    pub fn new(p:ChessPosition)->Self{
        Self{position:p}
    }
    pub fn can_attack(&self,o:&Queen)->bool{
        let a=&self.position;
        let b=&o.position;

        a.rank==b.rank
        || a.file==b.file
        || (a.rank-b.rank).abs()==(a.file-b.file).abs()
    }
}
