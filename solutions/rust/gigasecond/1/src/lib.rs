use time::{Duration, PrimitiveDateTime};

const GIGASECOND: i64 = 1_000_000_000;

pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    let dur = Duration::seconds(GIGASECOND);
    match start.checked_add(dur) {
        Some(result) => result,
        None => {
            start
        }
    }
}