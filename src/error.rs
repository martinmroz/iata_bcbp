
use std::error;
use std::fmt;
use std::result;

#[derive(Clone,Eq,PartialEq,Ord,PartialOrd,Hash,Debug)]
pub enum Error {
    /// The end of the input was reached prematurely.
    UnexpectedEndOfInput,
    /// The format of a field could not be validated in strict mode.
    InvalidFieldFormat,
    /// The contents of a field parsed as a numeric was not a numeric value.
    ExpectedInteger,
    /// The start-of-version-number value is not valid.
    InvalidStartOfVersionNumber,
    /// The start-of-security-data value is not valid.
    InvalidStartOfSecurityData,
    /// The BCBP encoding is not supported.
    UnsupportedEncoding,
    /// After parsing, additional characters remain.
    TrailingCharacters,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

pub type Result<T> = result::Result<T, Error>;
