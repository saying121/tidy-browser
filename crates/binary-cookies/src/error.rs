use std::{error::Error, fmt::Display};

use winnow::error::ContextError;

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum ParseError {
    #[error("{0}")]
    WinnowCtx(ContextError),
    #[error("Time broken: {0:?}")]
    Time(chrono::offset::LocalResult<chrono::DateTime<chrono::Utc>>),
    #[error(transparent)]
    Bplist(#[from] BplistErr),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum BplistErr {
    #[error(r#"Not start with b"bplist00""#)]
    Magic,
    #[error(r#"The object not dict, need update decoder"#)]
    NotDict,
    #[error(r#"The dict key not `NSHTTPCookieAcceptPolicy`, need update decoder"#)]
    BadKey,
    #[error(r#"The int not one byte, need update decoder"#)]
    OneByteInt,
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
            Self::U32(binary) => f.write_fmt(format_args!("{:#>06x}", binary)),
            Self::U64(binary) => f.write_fmt(format_args!("{:#010x}", binary)),
            Self::Magic(e) => {
                let s: String = e
                    .iter()
                    .map(|&v| v as char)
                    .collect();
                f.write_fmt(format_args!(r#"b"{s}""#))
            },
            Self::EndHeader(e) => f.write_fmt(format_args!("{e:?}")),
        }
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;
