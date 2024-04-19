use chrono::{DateTime, TimeZone, Utc};

use crate::browser::info::BrowserTime;

// pub mod passwd;
pub mod cookie;

// reference: https://support.moonpoint.com/network/web/browser/firefox/sqlite_cookies.php
pub(super) trait I64ToMozTime: BrowserTime {
    fn micros_to_moz_utc(&self) -> DateTime<Utc>;
    fn secs_to_moz_utc(&self) -> DateTime<Utc>;
}

impl I64ToMozTime for i64 {
    fn micros_to_moz_utc(&self) -> DateTime<Utc> {
        Utc.timestamp_micros(*self.clamp(
            &Self::MIN_TIME.timestamp_micros(),
            &Self::MAX_TIME.timestamp_micros(),
        ))
        .unwrap()
    }
    fn secs_to_moz_utc(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(
            *self.clamp(&Self::MIN_TIME.timestamp(), &Self::MAX_TIME.timestamp()),
            0,
        )
        .unwrap()
    }
}
