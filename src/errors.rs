use crate::{error, Debug, Display, Formatter};

#[derive(Debug)]
pub(crate) struct ParseArgsError {
    pub(crate) err: &'static str,
}

impl Display for ParseArgsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl error::Error for ParseArgsError {}

impl ParseArgsError {
    pub(crate) fn new(err: &'static str) -> ParseArgsError {
        ParseArgsError { err }
    }
}

#[derive(Debug)]
pub(crate) struct GetLockError;

impl Display for GetLockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "get thread lock error")
    }
}

impl error::Error for GetLockError {}
