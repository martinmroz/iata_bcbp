// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::collections::HashMap;
use std::error;
use std::fmt;
use std::mem;

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
  /// Scanner over the input string.
  scanner: Scanner<'a>,
  /// The boarding pass under construction.
  target: Bcbp,
}

impl<'a> Parser<'a> {

  /// Creates a new instance of the parser and boarding pass under construction.
  pub fn new(input: &'a str, strict: bool) -> Self {
    let mut parser = Parser {
      strict: strict,
      scanner: Scanner::new(input),
      target: Bcbp {
        unique_fields: HashMap::new(),
        legs: Vec::new(),
      }
    };
    parser
  }
  
  /// Parses the fields of the BCBP starting from the root, consuming the receiver.
  pub fn parse(mut self) -> Result<Bcbp, BcbpParserError> {
    Ok(self.target)
  }

}

/// Parses the `input` with optional conformance verification to yield a BCBP object or parse error.
pub(crate) fn parse<I>(input: I, strict: bool) -> Result<Bcbp, BcbpParserError>
where
  I: AsRef<str>
{
  let input_string = input.as_ref();

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

  Parser::new(input_string, strict).parse()
}
