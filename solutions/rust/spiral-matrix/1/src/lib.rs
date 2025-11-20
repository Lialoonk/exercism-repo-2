pub fn spiral_matrix(n:u32)->Vec<Vec<u32>>{
    let n=n as usize;
    let mut m=vec![vec![0;n];n];
    if n==0{return m;}

    let(mut i,mut j)=(0usize,0usize);
    m[0][0]=1;

    let mut v=2u32;
    let max=(n*n)as u32;

    while v<=max{
        while j+1<n && m[i][j+1]==0{
            j+=1;
            m[i][j]=v;
            v+=1;
        }

        while i+1<n && m[i+1][j]==0{
            i+=1;
            m[i][j]=v;
            v+=1;
        }
        while j>0 && m[i][j-1]==0{
            j-=1;
            m[i][j]=v;
            v+=1;
        }

        while i>0 && m[i-1][j]==0{
            i-=1;
            m[i][j]=v;
            v+=1;
        }
    }
    m
}
