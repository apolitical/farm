use mysql;
use r2d2;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "MySqlError: {:#?}", _0)]
    MySqlError(mysql::Error),
    #[fail(display = "R2D2Error: {:#?}", _0)]
    R2D2Error(r2d2::Error),
}

impl From<mysql::Error> for Error {
    fn from(err: mysql::Error) -> Error {
        Error::MySqlError(err)
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Error {
        Error::R2D2Error(err)
    }
}
