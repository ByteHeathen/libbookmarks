use diesel::result::Error as DieselError;
use diesel::result::ConnectionError;
use diesel_migrations::RunMigrationsError;

use std::env::VarError;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    InvalidDatabasePath,
    Io(IoError),
    Environment(VarError),
    Connection(ConnectionError),
    Diesel(DieselError),
    Migration(RunMigrationsError)
}

#[cfg(feature = "pyo3")]
use pyo3::exceptions;

#[cfg(feature = "pyo3")]
use pyo3::PyErr;

#[cfg(feature = "pyo3")]
impl From<Error> for pyo3::PyErr {
    fn from(err: Error) -> PyErr {
        match err {
            Error::InvalidDatabasePath => exceptions::FileNotFoundError::py_err(format!("{:?}", err)),
            Error::Io(err) => exceptions::IOError::py_err(format!("{:?}", err)),
            Error::Environment(err) => exceptions::EnvironmentError::py_err(format!("{:?}", err)),
            Error::Connection(err) => exceptions::ConnectionError::py_err(format!("{:?}", err)),
            Error::Diesel(err) => exceptions::IOError::py_err(format!("{:?}", err)),
            Error::Migration(err) => exceptions::IOError::py_err(format!("{:?}", err))
        }
    }
}

macro_rules! impl_simple_from {

    ($err:tt, $variant:tt) => {
        impl From<$err> for Error {
            fn from(err: $err) -> Error {
                Error::$variant(err)
            }
        }
    }
}

impl_simple_from!(DieselError, Diesel);
impl_simple_from!(ConnectionError, Connection);
impl_simple_from!(RunMigrationsError, Migration);
impl_simple_from!(VarError, Environment);
impl_simple_from!(IoError, Io);
