// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::error;
use std::fmt;
use std::result;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Error {
    /// The BCBP string does not contain exclusively ASCII characters.
    InvalidCharacters,
    /// The BCBP format is not supported.
    UnsupportedFormat,
    /// The end of otherwise-valid IATA BCBP data was reached prematurely.
    UnexpectedEndOfInput,
    /// Parsing the encoded data failed.
    ParseFailed(String),
    /// After successfully parsing a BCBP object, additional characters remain.
    TrailingCharacters,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::InvalidCharacters =>
                write!(f, "non-ASCII characters"),
            &Error::UnsupportedFormat =>
                write!(f, "not an IATA BCBP Type M boarding pass"),
            &Error::UnexpectedEndOfInput =>
                write!(f, "unexpected end-of-input"),
            &Error::ParseFailed(ref reason) =>
                write!(f, "parse failed: {}", reason),
            &Error::TrailingCharacters =>
                write!(f, "input includes data after a valid boarding pass"),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
