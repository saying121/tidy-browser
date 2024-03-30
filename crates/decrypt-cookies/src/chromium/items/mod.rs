use chrono::offset::LocalResult;
use chrono::{DateTime, TimeZone, Utc};

pub mod cookie;
pub mod passwd;

trait I64ToChromiumDateTime {
    fn micros_to_chromium_utc(&self) -> LocalResult<DateTime<Utc>>;
}

// https://source.chromium.org/chromium/chromium/src/+/main:base/time/time.h;l=5;
impl I64ToChromiumDateTime for i64 {
    fn micros_to_chromium_utc(&self) -> LocalResult<DateTime<Utc>> {
        Utc.timestamp_micros(self - 11_644_473_600 * 1_000_000)
    }
}
