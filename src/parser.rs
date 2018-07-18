// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::collections::HashMap;
use std::error;
use std::fmt;

use field::Field;
use scanner::Scanner;

use { Bcbp, Leg };

#[derive(Copy,Clone,Eq,PartialEq,Debug,Hash)]
pub enum BcbpParserError {
  /// The BCBP string contains invalid characters.
  InvalidCharacters,
  /// The BCBP string is too short to encode all the required fields of a boarding pass.
  TooShort,
  /// The first character of the BCBP string specifies the data format.
  MissingFormatSpecifier,
  /// The parser supports Type 'M' BCBP strings.
  UnsupportedFormat(char),
  /// The BCBP string does not contain any flight legs.
  NoLegs,
  /// Unable to validate a field in strict mode.
  ValidationFailed(Field),
}

impl error::Error for BcbpParserError {}

impl fmt::Display for BcbpParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &BcbpParserError::InvalidCharacters =>
        write!(f, "data contains invalid characters"),
      &BcbpParserError::TooShort =>
        write!(f, "data not long enough to encode all required fields"),
      &BcbpParserError::MissingFormatSpecifier =>
        write!(f, "missing format specifier"),
      &BcbpParserError::UnsupportedFormat(found) =>
        write!(f, "unsupported data format '{}'", found),
      &BcbpParserError::NoLegs =>
        write!(f, "no flight legs"),
      &BcbpParserError::ValidationFailed(field) =>
        write!(f, "validation failed for field {}", field),
    }
  }
}

#[derive(Clone,Eq,PartialEq,Debug)]
struct Parser<'a> {
  /// If true, all fields are validated while parsing.
  strict: bool,
  /// The boarding pass under construction.
  target: Bcbp<'a>,
}

impl<'a> Parser<'a> {

  /// Creates a new instance of the parser and boarding pass under construction.
  fn new(input: String, strict: bool) -> Self {
    Parser {
      strict: strict,
      target: Bcbp {
        pass_data: input,
        unique_fields: HashMap::new(),
        legs: Vec::with_capacity(1),
      }
    }
  }

}

/// Parses the `input` with optional conformance verification to yield a BCBP object or parse error.
pub(crate) fn parse<'a, I>(input: I, strict: bool) -> Result<Bcbp<'a>, BcbpParserError>
where
  I: Into<String> 
{
  let input_string = input.into();

  // BCBP strings are required to be 7-bit ASCII.
  if !input_string.is_ascii() {
    return Err(BcbpParserError::InvalidCharacters);
  }

  lazy_static! {
    // The minimum acceptable BCBP string is comprised of the 'header' fields.
    static ref MINIMUM_LENGTH: usize = {
      let mut len: usize = 0;
      len += Field::FormatCode.len();
      len += Field::NumberOfLegsEncoded.len();
      len += Field::PassengerName.len();
      len += Field::ElectronicTicketIndicator.len();
      len
    };
  }

  if input_string.len() < *MINIMUM_LENGTH {
    return Err(BcbpParserError::TooShort);
  }



  Err(BcbpParserError::NoLegs)
}
