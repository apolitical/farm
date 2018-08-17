use mysql;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "MySqlError: {:#?}", _0)]
    MySqlError(mysql::Error)
}

impl From<mysql::Error> for Error {
    fn from(err: mysql::Error) -> Error {
        Error::MySqlError(err)
    }
}
