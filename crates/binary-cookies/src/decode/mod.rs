pub mod binary_cookies;
pub mod cookies;
pub mod meta;
pub mod pages;

use chrono::{offset::LocalResult, DateTime, TimeZone as _, Utc};
use winnow::Partial;

pub(crate) type StreamIn<'i> = Partial<&'i [u8]>;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum DecodeResult<C, R> {
    Continue(C),
    Done(R),
}

/// A item offset in binarycookies and it's size.
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct OffsetSize {
    /// The item relative binarycookies start offset.
    pub(crate) offset: u64,
    /// The item size.
    pub(crate) size: u32,
}

pub(crate) trait F64ToSafariTime {
    fn to_utc(&self) -> Option<DateTime<Utc>>;
}
impl F64ToSafariTime for f64 {
    #[expect(clippy::cast_sign_loss, reason = "Don't worry")]
    fn to_utc(&self) -> Option<DateTime<Utc>> {
        let seconds = self.trunc() as i64 + 978_307_200;
        let nanos = ((self.fract()) * 1_000_000_000_f64) as u32;

        match Utc.timestamp_opt(seconds, nanos) {
            LocalResult::Single(time) => Some(time),
            LocalResult::Ambiguous(..) | LocalResult::None => None,
        }
    }
}
