pub fn answer(c:&str)->Option<i32>{
    let mut it=c.trim_start_matches("What is ").trim_end_matches('?').split_whitespace().peekable();
    let mut v: i32=it.next().and_then(|x|x.parse().ok())?;

    while let Some(t)=it.next(){
        v=match t{
            "plus"=>{
                let r: i32=it.next().and_then(|x|x.parse().ok())?;
                v+r
            }
            "minus"=>{
                let r: i32=it.next().and_then(|x|x.parse().ok())?;
                v-r
            }
            "multiplied"=>{
                it.next_if_eq(&"by")?;
                let r: i32=it.next().and_then(|x|x.parse().ok())?;
                v*r
            }
            "divided"=>{
                it.next_if_eq(&"by")?;
                let r: i32=it.next().and_then(|x|x.parse().ok())?;
                v/r
            }
            "raised"=>{
                it.next_if_eq(&"to")?;
                it.next_if_eq(&"the")?;
                let r=it.next()?.trim_end_matches(char::is_alphabetic).parse().ok()?;
                it.next_if_eq(&"power")?;
                v.pow(r)
            }
            _=>return None
        }
    }
    Some(v)
}
