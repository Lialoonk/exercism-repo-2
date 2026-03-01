pub struct Duration(f64);

impl From<u64>for Duration {
    fn from(x:u64)->Self{
        Duration(x as f64/31_557_600.0)
    }
}
pub trait Planet{
    fn period()->f64;
    fn years_during(d: &Duration)->f64{
        d.0/Self::period()
    }
}
macro_rules! m{
    ($n:ident, $v:expr)=>{
        pub struct $n;
        impl Planet for $n{
            fn period()->f64{$v}
        }
    }
}

m!(Earth,   1.0);
m!(Mercury, 0.2408467);
m!(Venus,   0.61519726);
m!(Mars,    1.8808158);
m!(Jupiter, 11.862615);
m!(Saturn,  29.447498);
m!(Uranus,  84.016846);
m!(Neptune, 164.79132);
