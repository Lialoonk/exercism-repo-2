pub fn encode(s:&str)->String{
    let (mut r,c,n)=s.chars().fold(
        (String::new(),' ',0usize),
        |(mut r,c,n),x|{
            if x==c{
                (r,c,n+1)
            }else{
                if n>1{r+=&n.to_string();}
                if n>0{r.push(c);}
                (r,x,1)
            }
        }
    );
    if n>1{r+=&n.to_string();}
    if n>0{r.push(c);}

    r
}

pub fn decode(s:&str)->String{
    s.chars().fold(
        (String::new(),0usize),
        |(mut r,n),x|{
            if let Some(d)=x.to_digit(10){
                (r,n*10+d as usize)
            }else{
                let k=if n==0{1}else{n};
                r.push_str(&x.to_string().repeat(k));
                (r,0)
            }
        }
    ).0
}
