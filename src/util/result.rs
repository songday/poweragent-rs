use std::convert::From;

use serde::Serialize;

pub(crate) type Result<D> = core::result::Result<D, Error>;

#[derive(Debug, Serialize)]
pub(crate) enum Error {
    WithMessage(String),
}

impl From<redb::Error> for Error {
    fn from(err: redb::Error) -> Self {
        Error::WithMessage(err.to_string())
    }
}

impl From<redb::TransactionError> for Error {
    fn from(err: redb::TransactionError) -> Self {
        Error::WithMessage(err.to_string())
    }
}

impl From<redb::DatabaseError> for Error {
    fn from(err: redb::DatabaseError) -> Self {
        Error::WithMessage(err.to_string())
    }
}

impl From<redb::StorageError> for Error {
    fn from(err: redb::StorageError) -> Self {
        Error::WithMessage(err.to_string())
    }
}

impl From<redb::TableError> for Error {
    fn from(err: redb::TableError) -> Self {
        Error::WithMessage(err.to_string())
    }
}

impl From<redb::CommitError> for Error {
    fn from(err: redb::CommitError) -> Self {
        Error::WithMessage(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::WithMessage(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::WithMessage(format!("{err:?}"))
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        Error::WithMessage(format!("{err:?}"))
    }
}

impl From<candle::Error> for Error {
    fn from(err: candle::Error) -> Self {
        Error::WithMessage(format!("{err:?}"))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::WithMessage(format!("{err:?}"))
    }
}