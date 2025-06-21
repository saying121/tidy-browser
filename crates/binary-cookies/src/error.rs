use winnow::error::{ContextError, ErrMode};

#[derive(Clone)]
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum ParseError {
    #[error("{0}")]
    WinnowCtx(ContextError),
    #[error("Time broken: {0:?}")]
    Time(chrono::offset::LocalResult<chrono::DateTime<chrono::Utc>>),
    #[error(transparent)]
    Bplist(#[from] BplistErr),
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

pub type Result<T> = std::result::Result<T, ParseError>;
