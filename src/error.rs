use core::fmt;
use std::string::FromUtf8Error;

use lalrpop_util::ParseError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Failed to parse the contract. {0}")]
    ParseError(String),

    #[error("Failed to visit AST {0}")]
    AstVisitError(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
}

impl<L, T, E> From<ParseError<L, T, E>> for Error
where
    L: fmt::Debug,
    T: fmt::Debug,
    E: fmt::Debug,
{
    fn from(value: ParseError<L, T, E>) -> Self {
        Self::ParseError(format!("{:?}", value))
    }
}
