use winnow::error::{ContextError, ErrMode};

#[derive(Clone)]
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum ParseError {
    #[error("Cookies end header broken")]
    EndHeader,
    #[error("{0}")]
    Winnow(ErrMode<ContextError>),
}

pub type Result<T> = std::result::Result<T, ParseError>;
