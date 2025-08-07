use derive_more::From;

pub type Result<T, Error> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- User
    UserCreateCouldNotCreateUser,

    // -- Repository
    ConnectionLockError,

    #[from]
    #[warn(dead_code)]
    RusqliteError(rusqlite::Error),

    // -- Externals
    #[from]
    #[warn(dead_code)]
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(format, "{self:?}")
    }
}

impl std::error::Error for Error {}
