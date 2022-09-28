use std::cmp::min;

pub(crate) fn calculate_elo_range(time: i64) -> u64 {
    let elo_range: u64 = 50;
    let time: u32 = time.try_into().unwrap();
    let elo_range: (u64, bool) = elo_range.overflowing_pow(time / 10);
    match elo_range.1 {
        true => 500,
        false => min(elo_range.0, 500),
    }
}
