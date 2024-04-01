use chrono::{DateTime, TimeZone, Utc};

use crate::browser::info::BrowserTime;

// pub mod passwd;
pub mod cookie;

// reference: https://support.moonpoint.com/network/web/browser/firefox/sqlite_cookies.php
trait I64ToMozTime: BrowserTime {
    fn micros_to_moz_utc(&self) -> DateTime<Utc>;
    fn secs_to_moz_utc(&self) -> DateTime<Utc>;
}

impl I64ToMozTime for i64 {
    fn micros_to_moz_utc(&self) -> DateTime<Utc> {
        if *self < Self::MIN_TIME.timestamp_micros() || *self > Self::MAX_TIME.timestamp_micros() {
            return Self::MIN_TIME;
        }
        Utc.timestamp_micros(*self)
            .unwrap()
    }
    fn secs_to_moz_utc(&self) -> DateTime<Utc> {
        if *self < Self::MIN_TIME.timestamp_micros() || *self > Self::MAX_TIME.timestamp_micros() {
            return Self::MIN_TIME;
        }
        Utc.timestamp_opt(*self, 0)
            .unwrap()
    }
}
