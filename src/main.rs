use chrono::{Datelike, NaiveDate, NaiveDateTime};
use std::time::{SystemTime, UNIX_EPOCH};

fn now_epoch() -> u64 {
    let now = SystemTime::now();
    let elapsed = now
        .duration_since(UNIX_EPOCH)
        .expect("unable to obtain system time");
    elapsed.as_secs()
}

fn strip_hms_from_epoch(epoch: u64) -> i64 {
    let tm = NaiveDateTime::from_timestamp(epoch.try_into().unwrap(), 0);
    NaiveDate::from_ymd(tm.year(), tm.month(), tm.day())
        .and_hms(0, 0, 0)
        .timestamp()
}

fn utc_date_epoch() -> i64 {
    let now = now_epoch();
    strip_hms_from_epoch(now)
}

fn main() {
    print!("{}", utc_date_epoch())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_gets_now() {
        let now = now_epoch();
        assert_eq!(now > 0, true)
    }

    #[test]
    fn it_strips_hms() {
        let now = now_epoch();
        let epoch = strip_hms_from_epoch(now);
        assert_eq!(epoch < now.try_into().unwrap(), true)
    }
}
