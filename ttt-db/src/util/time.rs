use chrono::{NaiveDateTime, Utc};

pub(crate) fn get_time_in_queue(time_joined: i64) -> i64 {
    let time = Utc::now().naive_utc();
    let time_joined = NaiveDateTime::from_timestamp(time_joined, 0);
    let time = time - time_joined;
    let time = time.num_seconds();
    time
}
