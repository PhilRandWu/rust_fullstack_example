use mobc_postgres::tokio_postgres;

pub enum Error {
    DBPoolError(mobc::Error<tokio_postgres::Error>),
    DBQueryError(#[from] tokio_postgres::Error),
    DBInitError(tokio_postgres::Error),
    ReadFileError(#[from] std::io::Error),
}