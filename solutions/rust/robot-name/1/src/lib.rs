use std::cell::RefCell;

thread_local! {
    static N:RefCell<Vec<String>>={
        let mut s=Vec::with_capacity(26*26*1000);
        for a in 'A'..='Z'{
            for b in 'A'..='Z'{
                for x in '0'..='9'{
                    for y in '0'..='9'{
                        for z in '0'..='9'{
                            s.push(format!("{}{}{}{}{}",a,b,x,y,z));
                        }
                    }
                }
            }
        }
        RefCell::new(s)
    };
}

#[derive(Debug)]
pub struct Robot{
    r:String,
}
impl Robot{
    pub fn new()->Self{
        Self{r:g()}
    }
    pub fn name(&self)->&str{
        &self.r
    }
    pub fn reset_name(&mut self){
        self.r=g();
    }
}

fn g()->String{
    N.with(|s|s.borrow_mut().pop()).expect("no names left")
}
impl Drop for Robot{
    fn drop(&mut self){
        N.with(|s|s.borrow_mut().push(self.r.clone()))
    }
}
