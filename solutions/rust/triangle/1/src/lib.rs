use std::ops::Add;

pub struct Triangle<T>{
    a:[T;3],
}
impl<T> Triangle<T>
where
    T:Copy+PartialOrd+Add<Output=T>+From<i32>
{
    pub fn build(mut s:[T;3])->Option<Self>{
        if s.iter().any(|x| *x <= T::from(0)) {
            return None;
        }
        s.sort_by(|x,y| x.partial_cmp(y).unwrap());
        let (x,y,z) = (s[0], s[1], s[2]);
        if x + y > z {
            Some(Self{a:s})
        } else {
            None
        }
    }

    pub fn is_equilateral(&self)->bool{
        let s=&self.a;
        s[0]==s[1] && s[1]==s[2]
    }
    pub fn is_scalene(&self)->bool{
        let s=&self.a;
        s[0]!=s[1] && s[1]!=s[2]
    }
    pub fn is_isosceles(&self)->bool{
        let s=&self.a;
        s[0]==s[1] || s[1]==s[2]
    }
}
