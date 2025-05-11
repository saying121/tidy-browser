use chrono::{offset::LocalResult, DateTime, TimeZone, Utc};

// pub mod passwd;
pub mod cookie;

// reference: https://support.moonpoint.com/network/web/browser/firefox/sqlite_cookies.php
pub(super) trait I64ToMozTime {
    fn micros_to_moz_utc(self) -> Option<DateTime<Utc>>;
    fn secs_to_moz_utc(self) -> Option<DateTime<Utc>>;
}

impl I64ToMozTime for i64 {
    fn micros_to_moz_utc(self) -> Option<DateTime<Utc>> {
        match Utc.timestamp_micros(self) {
            LocalResult::Single(time) => Some(time),
            LocalResult::Ambiguous(..) | LocalResult::None => None,
        }
    }
    fn secs_to_moz_utc(self) -> Option<DateTime<Utc>> {
        match Utc.timestamp_opt(self, 0) {
            LocalResult::Single(time) => Some(time),
            LocalResult::Ambiguous(..) | LocalResult::None => None,
        }
    }
}
