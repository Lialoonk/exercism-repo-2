#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}
pub fn solve(a:u8,b:u8,g:u8,s:&Bucket)->Option<BucketStats>{
    let (c,mut x,o,mut m)=match s{
        Bucket::One=>([a,b],[a,0],[Bucket::One,Bucket::Two],1),
        Bucket::Two=>([b,a],[b,0],[Bucket::Two,Bucket::One],1),
    };

    while x.iter().all(|v|*v!=g){
        match x{
            [_,_] if c[1] == g =>x[1]=g,
            [0,_] => x[0]=c[0],
            [u,v] if u<c[0]&&v == c[1]=>x[1]=0,
            [u,v] if u<=c[0]&&v < c[1] => {
                let d=c[1]-v;
                let t=u.min(d);
                x[0]=u - t;
                x[1]=v + t;
            }
            _=>return None,
        }
        m+=1;
    }
    let (gb,ob)=match x{
        [_,v] if v==g=>(o[1],x[0]),
        _=>(o[0],x[1]),
    };
    Some(BucketStats{moves:m,goal_bucket:gb,other_bucket:ob})
}
