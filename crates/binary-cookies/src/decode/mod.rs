macro_rules! mode_err {
    ($self:ident, $e:ident, $buffer:expr) => {
        use winnow::error::ErrMode;
        use winnow::error::Needed;
        use $crate::decode::DecodeResult;

        match $e {
            ErrMode::Backtrack(e) | ErrMode::Cut(e) => Err(crate::error::WinnowCtxSnafu { render: e }.build()),
            ErrMode::Incomplete(Needed::Unknown) => {
                // The branch is unreachable?
                let new_cap = $buffer.capacity() * 2;
                $buffer.grow(new_cap);
                Ok(DecodeResult::Continue($self))
            },
            ErrMode::Incomplete(Needed::Size(size)) => {
                let need_size = size.get();
                $buffer
                    .grow($buffer.available_data() + need_size);
                if $buffer.available_space() < need_size {
                    $buffer.shift();
                }
                Ok(DecodeResult::Continue($self))
            },
        }
    };
}

pub mod binary_cookies;
pub mod cookies;
pub mod meta;
pub mod pages;
#[cfg(any(feature = "sync", feature = "tokio"))]
pub mod stream;

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
