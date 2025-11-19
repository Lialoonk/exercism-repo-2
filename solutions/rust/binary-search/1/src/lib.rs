use core::cmp::Ordering;
pub fn find<R, T>(a: R, k: T)->Option<usize>
where
    R: AsRef<[T]>,
    T: Ord,
{
    g(a.as_ref(),&k)
}
fn g<T>(s: &[T], k: &T) -> Option<usize>
where
    T: Ord,
{
    let m = s.len()/2;
    match k.cmp(s.get(m)?) {
        Ordering::Equal =>Some(m),
        Ordering::Less =>g(&s[..m], k),
        Ordering::Greater => g(&s[m + 1..],k).map(|x|x + m + 1),
    }
}
