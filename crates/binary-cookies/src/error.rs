use std::{error::Error, fmt::Display};

use winnow::error::{ContextError, ErrMode, ParserError};

#[derive(Clone)]
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum ParseError {
    #[error("Cookies end header broken")]
    EndHeader,
    #[error("{0}")]
    Winnow(ErrMode<ContextError>),
    // #[error("{0}")]
    // Cookie(CookieError),
}

type Result<T> = std::result::Result<T, ParseError>;
