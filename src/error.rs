// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::error;
use std::fmt;
use std::result;

#[derive(Clone,Eq,PartialEq,Ord,PartialOrd,Hash,Debug)]
pub enum Error {
    /// The end of the input was reached prematurely.
    UnexpectedEndOfInput,
    /// The contents of a field parsed as a numeric was not a numeric value.
    ExpectedInteger,
    /// The start-of-version-number value is not valid.
    InvalidStartOfVersionNumber,
    /// The start-of-security-data value is not valid.
    InvalidStartOfSecurityData,
    /// The BCBP string does not contain exclusively ASCII characters.
    InvalidCharacters,
    /// The BCBP format is not supported.
    UnsupportedFormat,
    /// After parsing, additional characters remain.
    TrailingCharacters,
    /// An attempt was made to create a fixed-sized field using incorrectly sized data.
    FieldLengthMismatch { required: usize, found: usize },
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

pub type Result<T> = result::Result<T, Error>;
