use derive_more::From;

pub type Result<T, Error> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- User
    UserCreateCouldNotCreateUser,

    // -- Externals
    #[from]
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
