// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::error;
use std::fmt;
use std::result;

use de::field;

#[derive(Clone,Eq,PartialEq,Ord,PartialOrd,Hash,Debug)]
pub enum Error {
    /// The end of the input was reached prematurely.
    UnexpectedEndOfInput(field::Field),
    /// The length of the subsection encoded exceeds the remaining length of the input.
    SubsectionTooLong,
    /// The contents of a field parsed as a numeric was not a numeric value.
    ExpectedInteger(field::Field),
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
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::UnexpectedEndOfInput(field) =>
                write!(f, "unexpected end-of-input before {}", field),
            &Error::SubsectionTooLong =>
                write!(f, "subsection too long"),
            &Error::ExpectedInteger(field) =>
                write!(f, "the {} field is non-numeric", field),
            &Error::InvalidStartOfVersionNumber =>
                write!(f, "the version number field does not begin with the '>' marker"),
            &Error::InvalidStartOfSecurityData =>
                write!(f, "the security data section does not begin with the '^' marker"),
            &Error::InvalidCharacters =>
                write!(f, "non-ASCII characters"),
            &Error::UnsupportedFormat =>
                write!(f, "not an IATA BCBP Type M boarding pass"),
            &Error::TrailingCharacters =>
                write!(f, "input includes data after a valid boarding pass"),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
