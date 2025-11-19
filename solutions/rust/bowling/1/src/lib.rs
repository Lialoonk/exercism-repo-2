#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}
#[derive(Debug, Default)]
pub struct BowlingGame {
    t: Vec<u16>,
    s: bool, 
}
impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn roll(&mut self, p: u16) -> Result<(), Error> {
        if p > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.s {
            if let Some(&l) = self.t.last() {
                if l + p > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
        }
        if self.score().is_some() {
            return Err(Error::GameComplete);
        }
        self.t.push(p);
        self.s = if p == 10 { false } else { !self.s };
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut r = 0u16;      
        let mut f = 0usize;   
        let v = &self.t;    

        for _ in 0..10 {
            let a = *v.get(f)?;     
            let b = *v.get(f + 1)?;
            r += a + b;
            if a == 10 || a + b == 10 {
                r += *v.get(f + 2)?;
            }
            f += if a == 10 { 1 } else { 2 };
        }
        Some(r)
    }
}
