#[derive(Debug)]
pub enum TimeStorageError {
    CannotFindEnvVar(std::env::VarError),
    IOErr(std::io::Error),
    ParsingError(chrono::ParseError),
}

impl From<std::env::VarError> for TimeStorageError {
    fn from(error: std::env::VarError) -> TimeStorageError {
        TimeStorageError::CannotFindEnvVar(error)
    }
}

impl From<std::io::Error> for TimeStorageError {
    fn from(error: std::io::Error) -> TimeStorageError {
        TimeStorageError::IOErr(error)
    }
}

impl From<chrono::ParseError> for TimeStorageError {
    fn from(error: chrono::ParseError) -> TimeStorageError {
        TimeStorageError::ParsingError(error)
    }
}
