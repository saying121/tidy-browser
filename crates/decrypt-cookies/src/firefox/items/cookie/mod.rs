use chrono::{DateTime, TimeZone, Utc};

use self::entities::moz_cookies;

pub mod dao;
pub mod entities;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct MozCookies {
    pub id:                 i32,
    pub origin_attributes:  String,
    pub name:               String,
    pub value:              String,
    pub host:               String,
    pub path:               String,
    pub expiry:             Option<DateTime<Utc>>,
    pub last_accessed:      Option<DateTime<Utc>>,
    pub creation_time:      Option<DateTime<Utc>>,
    pub is_secure:          bool,
    pub is_http_only:       bool,
    pub in_browser_element: i32,
    pub same_site:          i32,
    pub raw_same_site:      i32,
    pub scheme_map:         i32,
}

// reference: https://support.moonpoint.com/network/web/browser/firefox/sqlite_cookies.php
trait I64ToMozTime {
    fn micros_to_moz_utc(&self) -> DateTime<Utc>;
    fn secs_to_moz_utc(&self) -> DateTime<Utc>;
}

impl I64ToMozTime for i64 {
    fn micros_to_moz_utc(&self) -> DateTime<Utc> {
        Utc.timestamp_micros(*self)
            .unwrap()
    }
    fn secs_to_moz_utc(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(*self, 0)
            .unwrap()
    }
}

impl From<moz_cookies::Model> for MozCookies {
    fn from(value: moz_cookies::Model) -> Self {
        Self {
            id:                 value.id,
            origin_attributes:  value.origin_attributes,
            name:               value.name.unwrap_or_default(),
            value:              value.value.unwrap_or_default(),
            host:               value.host.unwrap_or_default(),
            path:               value.path.unwrap_or_default(),
            expiry:             value
                .expiry
                .map(|v| v.secs_to_moz_utc()),
            last_accessed:      value
                .last_accessed
                .map(|v| v.micros_to_moz_utc()),
            creation_time:      value
                .creation_time
                .map(|v| v.micros_to_moz_utc()),
            is_secure:          value
                .is_secure
                .is_some_and(|v| v != 0),
            is_http_only:       value
                .is_http_only
                .is_some_and(|v| v != 0),
            in_browser_element: value
                .in_browser_element
                .unwrap_or_default(),
            same_site:          value.same_site.unwrap_or_default(),
            raw_same_site:      value
                .raw_same_site
                .unwrap_or_default(),
            scheme_map:         value
                .scheme_map
                .unwrap_or_default(),
        }
    }
}
