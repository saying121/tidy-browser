use chrono::{offset::LocalResult, DateTime, TimeZone, Utc};

pub mod cookie;
pub mod passwd;

pub(super) trait I64ToChromiumDateTime {
    fn micros_to_chromium_utc(&self) -> Option<DateTime<Utc>>;
}

// https://source.chromium.org/chromium/chromium/src/+/main:base/time/time.h;l=5;
impl I64ToChromiumDateTime for i64 {
    fn micros_to_chromium_utc(&self) -> Option<DateTime<Utc>> {
        let unix_timestamp = self - 11_644_473_600 * 1_000_000;

        match Utc.timestamp_micros(unix_timestamp) {
            LocalResult::Single(time) => Some(time),
            LocalResult::Ambiguous(..) | LocalResult::None => None,
        }
    }
}
