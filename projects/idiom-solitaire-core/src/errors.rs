#[derive(Debug, Clone)]
pub enum Error {
    UnknownError,
    DictionaryNotFound,
    IOError(String),
    ParseError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Self {
        use csv::ErrorKind::*;
        match e.kind() {
            Io(_) => Self::IOError(format!("{}", e)),
            _ => Self::ParseError(format!("{}", e)),
        }
    }
}
