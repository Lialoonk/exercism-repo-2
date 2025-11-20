pub fn recite(s:u32,t:u32)->String{
    let mut r=String::new();
    for z in 0..t{
        let n=s-z;
        let a=format!("{} green {} hanging on the wall,\n",w(n),b(n));
        let c=a.clone();
        let d=match n-1{
            0=>"There'll be no green bottles hanging on the wall.".to_string(),
            1=>"There'll be one green bottle hanging on the wall.".to_string(),
            x=>format!("There'll be {} green {} hanging on the wall.",w(x).to_lowercase(),b(x)),
        };
        r.push_str(&a);
        r.push_str(&c);
        r.push_str("And if one green bottle should accidentally fall,\n");
        r.push_str(&d);
        r.push_str("\n\n");
    }
    r
}

fn w(n:u32)->String{
    match n{
        10=>"Ten",
        9=>"Nine",
        8=>"Eight",
        7=>"Seven",
        6=>"Six",
        5=>"Five",
        4=>"Four",
        3=>"Three",
        2=>"Two",
        1=>"One",
        0=>"No",
        _=>"Err"
    }.to_string()
}

fn b(n:u32)->String{
    match n{
        1=>"bottle",
        _=>"bottles"
    }.to_string()
}
