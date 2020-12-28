use backtrace::Backtrace;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub backtrace: Backtrace,
}

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error(transparent)]
    DotEnv(#[from] dotenv::Error),
    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
    #[error(transparent)]
    R2d2(#[from] r2d2::Error),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    #[error("{0}")]
    Unknown(String),
}

// Print out the error and backtrace, including source errors
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}\nBacktrace: \n{:?}", self.kind, self.backtrace)?;

        // Go down the chain of errors
        let mut error: &dyn StdError = &self.kind;
        while let Some(source) = error.source() {
            write!(f, "\n\nCaused by: {}", source)?;
            error = source;
        }

        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.kind.source()
    }
}

// Forward From impls to Error from ErrorKind. Because From is reflexive,
// this impl also takes care of From<ErrorKind>.
impl<T> From<T> for Error
where
    ErrorKind: From<T>,
{
    fn from(item: T) -> Self {
        Error {
            kind: ErrorKind::from(item),
            backtrace: Backtrace::new(),
        }
    }
}
