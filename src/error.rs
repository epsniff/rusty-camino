use std::fmt::{self, Debug, Display, Formatter};
use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    #[serde(rename(serialize = "error_message"))]
    msg: String,
}

impl Error {
    pub fn new<M: Into<String>>(msg: M) -> Self {
        Error { msg: msg.into() }
    }
}

impl From<tantivy::query::QueryParserError> for Error {
    fn from(err: tantivy::query::QueryParserError) -> Error {
        Error::new(format!("Error: {}", err))
    }
}
impl From<tantivy::TantivyError> for Error {
    fn from(err: tantivy::TantivyError) -> Error {
        Error::new(format!("Error: {}", err))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "rusty_camino::Error: {}", self.msg)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "rusty_camino::Error: {}", self.msg)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.msg.as_str()
    }
}

pub trait ErrorExt {
    fn wrap(self) -> Error;
    fn context<C: Display + Send + Sync + 'static>(self, ctx: C) -> Error;
}

impl<E: std::error::Error + Send + Sync + 'static> ErrorExt for E {
    fn wrap(self) -> Error {
        Error { msg: self.to_string() }
    }

    fn context<C: Display + Send + Sync + 'static>(self, ctx: C) -> Error {
        let msg = format!("{}: {}", ctx, self.to_string());
        Error { msg }
    }
}

pub trait ResultExt<T> {
    fn wrap(self) -> Result<T, Error>;
    fn context<C: Display + Send + Sync + 'static>(self, ctx: C) -> Result<T, Error>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> ResultExt<T> for Result<T, E> {
    fn wrap(self) -> Result<T, Error> {
        self.map_err(|e| e.wrap())
    }

    fn context<C: Display + Send + Sync + 'static>(self, ctx: C) -> Result<T, Error> {
        self.map_err(|e| e.context(ctx))
    }
}


