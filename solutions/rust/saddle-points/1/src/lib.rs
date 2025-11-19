pub fn find_saddle_points(a:&[Vec<u64>])->Vec<(usize,usize)>{
    let mut r=Vec::new();
    let h=a.len();
    for (i,row) in a.iter().enumerate(){
        for (j,v) in row.iter().enumerate(){
            if row.iter().all(|x| x <= v)
                && (0..h).all(|x| a[x][j] >= *v)
            {
                r.push((i,j));
            }
        }
    }
    r
}
