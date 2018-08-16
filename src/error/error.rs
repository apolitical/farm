use diesel;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Connection error: {}", _0)]
    ConnectionError(diesel::ConnectionError),
    #[fail(display = "Query error: {}", _0)]
    ResultError(diesel::result::Error),
}

impl From<diesel::ConnectionError> for Error {
    fn from(err: diesel::ConnectionError) -> Error {
        Error::ConnectionError(err)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Error {
        Error::ResultError(err)
    }
}
