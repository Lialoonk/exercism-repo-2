pub struct PascalsTriangle {
    n:u32,
}

impl PascalsTriangle {
    pub fn new(n:u32)->Self{
        Self{n}
    }
    pub fn rows(&self)->Vec<Vec<u32>>{
        (0..self.n).map(|i|Self::c(i)).collect()
    }
    fn c(r:u32)->Vec<u32>{
        let mut v=vec![1];
        for k in 1..=r{
            if let Some(&x)=v.last(){
                v.push(x*(r+1-k)/k);
            }
        }
        v
    }
}
