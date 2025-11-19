use std::io::{Read, Write, Result};

pub type ReadStats<T> = Io<T>;
pub type WriteStats<T> = Io<T>;
pub struct Io<T>{
    w:T,
    b:usize,
    wr:usize,
    rd:usize,
}

impl<T> Io<T>{
    pub fn new(x:T)->Self{
        Self{w:x,b:0,wr:0,rd:0}
    }
    pub fn get_ref(&self)->&T{
        &self.w
    }
    pub fn bytes_through(&self)->usize{
        self.b
    }
}

impl<T:Read> Io<T>{
    pub fn reads(&self)->usize{
        self.rd
    }
}
impl<T:Write> Io<T>{
    pub fn writes(&self)->usize{
        self.wr
    }
}
impl<T:Read> Read for Io<T>{
    fn read(&mut self,buf:&mut [u8])->Result<usize>{
        self.rd+=1;
        self.w.read(buf).map(|n|{
            self.b+=n;
            n
        })
    }
}

impl<T:Write> Write for Io<T>{
    fn write(&mut self,buf:&[u8])->Result<usize>{
        self.wr+=1;
        self.w.write(buf).map(|n|{
            self.b+=n;
            n
        })
    }

    fn flush(&mut self)->Result<()>{
        self.w.flush()
    }
}
