use chrono::{DateTime, TimeZone, Utc};

use crate::browser::info::BrowserTime;

pub mod cookie;
pub mod passwd;

pub(super) trait I64ToChromiumDateTime {
    fn micros_to_chromium_utc(&self) -> DateTime<Utc>;
}

// https://source.chromium.org/chromium/chromium/src/+/main:base/time/time.h;l=5;
impl I64ToChromiumDateTime for i64 {
    fn micros_to_chromium_utc(&self) -> DateTime<Utc> {
        let unix_timestamp = self - 11_644_473_600 * 1_000_000;

        Utc.timestamp_micros(unix_timestamp.clamp(
            Self::MIN_TIME.timestamp_micros(),
            Self::MAX_TIME.timestamp_micros(),
        ))
        .unwrap()
    }
}
