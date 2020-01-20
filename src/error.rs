use diesel::result::Error as DieselError;
use diesel::result::ConnectionError;
use diesel_migrations::RunMigrationsError;

#[derive(Debug)]
pub enum Error {
    Connection(ConnectionError),
    Diesel(DieselError),
    Migration(RunMigrationsError)
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
