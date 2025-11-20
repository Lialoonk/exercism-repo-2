use Direction::*;

#[derive(PartialEq,Debug)]
pub enum Direction{North,East,South,West}

#[derive(Debug)]
pub struct Robot{
    a:isize,
    b:isize,
    d:Direction
}
impl Robot{
    pub fn new(a:isize,b:isize,d:Direction)->Self{
        Self{a,b,d}
    }
    pub fn turn_right(self)->Self{
        match self.d{
            North=>Self{d:East,..self},
            East=>Self{d:South,..self},
            South=>Self{d:West,..self},
            West=>Self{d:North,..self}
        }
    }

    pub fn turn_left(self)->Self{
        match self.d{
            North=>Self{d:West,..self},
            West=>Self{d:South,..self},
            South=>Self{d:East,..self},
            East=>Self{d:North,..self}
        }
    }

    pub fn advance(self)->Self{
        match self.d{
            North=>Self{b:self.b+1,..self},
            South=>Self{b:self.b-1,..self},
            East=>Self{a:self.a+1,..self},
            West=>Self{a:self.a-1,..self}
        }
    }

    pub fn instructions(self,s:&str)->Self{
        s.chars().fold(self,|r,c|match c{
            'L'=>r.turn_left(),
            'R'=>r.turn_right(),
            'A'=>r.advance(),
            _=>r
        })
    }
    pub fn position(&self)->(isize,isize){
        (self.a,self.b)
    }
    pub fn direction(&self)->&Direction{
        &self.d
    }
}
