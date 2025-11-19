pub fn annotate(g:&[&str])->Vec<String>{
    (0..g.len())
        .map(|i|(0..g[i].len()).map(|j|cell(g,i as i32,j as i32)).collect())
        .collect()
}

fn cell(g:&[&str],y:i32,x:i32)->char{
    if g[y as usize].as_bytes()[x as usize]==b'*'{return '*'}
    let h=g.len() as i32-1;
    let w=g[y as usize].len() as i32-1;

    let c=((y-1).max(0)..=(y+1).min(h))
        .flat_map(|u|((x-1).max(0)..=(x+1).min(w)).map(move|v|(u,v)))
        .filter(|&(u,v)|g[u as usize].as_bytes()[v as usize]==b'*')
        .count();

    if c==0{' '}else{(b'0'+c as u8)as char}
}
