use chrono::{DateTime, TimeZone, Utc};

use crate::browser::info::BrowserTime;

pub mod cookie;
pub mod passwd;

trait I64ToChromiumDateTime {
    fn micros_to_chromium_utc(&self) -> DateTime<Utc>;
}

// https://source.chromium.org/chromium/chromium/src/+/main:base/time/time.h;l=5;
impl I64ToChromiumDateTime for i64 {
    fn micros_to_chromium_utc(&self) -> DateTime<Utc> {
        let unix_timestamp = self - 11_644_473_600 * 1_000_000;
        if unix_timestamp < Self::MIN_TIME.timestamp_micros()
            || unix_timestamp > Self::MAX_TIME.timestamp_micros()
        {
            return Self::MIN_TIME;
        }

        Utc.timestamp_micros(unix_timestamp)
            .unwrap()
    }
}
