use std::{error::Error, fmt::Display};

use chrono::offset::LocalResult;
use snafu::{Location, Snafu};
use winnow::error::ContextError;

#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
pub enum ParseError {
    #[snafu(display("{render}\n@:{location}"))]
    WinnowCtx {
        render: ContextError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Time broken: {local_result:?}\n@:{location}"))]
    Time {
        local_result: LocalResult<chrono::DateTime<chrono::Utc>>,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Bplist {
        source: BplistErr,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Read: {source}\n@:{location}",))]
    Read {
        source: std::io::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("End of binarycookies, can't decode any more data\n@:{location}"))]
    ParsingCompleted {
        #[snafu(implicit)]
        location: Location,
    },
}

impl ParseError {
    pub const fn is_completed(&self) -> bool {
        matches!(self, Self::ParsingCompleted { .. })
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
pub enum BplistErr {
    #[snafu(display(
        r#"Not start with b"bplist00"
@:{location}"#
    ))]
    Magic {
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("The object not dict, need update decoder\n@:{location}"))]
    NotDict {
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display(
        "The dict key not `NSHTTPCookieAcceptPolicy`, need update decoder\n@:{location}"
    ))]
    BadKey {
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("The int not one byte, need update decoder\n@:{location}"))]
    OneByteInt {
        #[snafu(implicit)]
        location: Location,
    },
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ExpectErr {
    U32(u32),
    U64(u64),
    Magic([u8; 4]),
    EndHeader([u8; 4]),
}

impl std::fmt::Debug for ExpectErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl Error for ExpectErr {}

impl Display for ExpectErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U32(binary) => f.write_fmt(format_args!("got: {:#>06x}", binary)),
            Self::U64(binary) => f.write_fmt(format_args!("got: {:#010x}", binary)),
            Self::Magic(e) => {
                let s: String = e
                    .iter()
                    .map(|&v| v as char)
                    .collect();
                f.write_fmt(format_args!(r#"got: b"{s}""#))
            },
            Self::EndHeader(e) => f.write_fmt(format_args!("got: {e:?}")),
        }
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;
