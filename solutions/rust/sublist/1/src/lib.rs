#[derive(Debug, PartialEq, Eq)]
pub enum Comparison{
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
pub fn sublist<T:PartialEq>(a: &[T], b: &[T])->Comparison{
    let (x, y)=(a.len(),b.len());
    match (x, y){
        (0, 0)=>Comparison::Equal,
        (0, _)=>Comparison::Sublist,
        (_, 0)=>Comparison::Superlist,
        _ if x== y =>{
            if a == b{Comparison::Equal }else{ Comparison::Unequal}
        }
        _ if x<y=>{
            if b.windows(x).any(|w|w == a){
                Comparison::Sublist
            }else{
                Comparison::Unequal
            }
        }
        _=>{
            if a.windows(y).any(|w|w == b){
                Comparison::Superlist
            }else{
                Comparison::Unequal
            }
        }
    }
}
