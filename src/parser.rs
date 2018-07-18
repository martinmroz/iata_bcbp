// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::collections::HashMap;
use std::error;
use std::fmt;

use field::Field;
use scanner::{Scanner, ScannerError};

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
  /// Unable to read a field.
  UnableToReadField { field: Field, reason: ScannerError },
  /// Invalid Start of Version Number; this is used as a sanity check.
  InvalidStartOfVersionNumber,
  /// The variable size field is longer than the remaining BCBP data.
  VariableLengthFieldTooLong,
}

impl error::Error for BcbpParserError {
  fn cause(&self) -> Option<&error::Error> {
    if let &BcbpParserError::UnableToReadField { ref reason, .. } = self {
      Some(reason)
    } else {
      None
    }
  }
}

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
      &BcbpParserError::UnableToReadField { field, .. } =>
        write!(f, "validation failed for field {}", field),
      &BcbpParserError::InvalidStartOfVersionNumber =>
        write!(f, "invalid start of version number marker"),
      &BcbpParserError::VariableLengthFieldTooLong =>
        write!(f, "length of variable field exceeds length of remaining input"),
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

macro_rules! parse_fields {
  ($scanner:expr, $target_map:expr, $strict:expr, $fields:expr) => {
    for &field in $fields.iter() {
      let value = $scanner
        .scan_field(field, $strict)
        .map(String::from)
        .map_err(|e| BcbpParserError::UnableToReadField { field: field, reason: e } )?;
      $target_map.insert(field, value);
    }
  };
}

impl<'a> Parser<'a> {

  /// Creates a new instance of the parser and boarding pass under construction.
  pub fn new(input: &'a str, strict: bool) -> Self {
    Parser {
      strict: strict,
      scanner: Scanner::new(input),
      target: Bcbp {
        unique_fields: HashMap::new(),
        legs: Vec::new(),
      }
    }
  }
  
  /// Parses the fields of the BCBP starting from the root, consuming the receiver.
  pub fn parse(mut self) -> Result<Bcbp, BcbpParserError> {
    println!("Trace Begins");

    let format_code = {
      self.scanner.scan_field(Field::FormatCode, true).map_err(|_|
        BcbpParserError::MissingFormatSpecifier
      )?
    };

    // Only M-type BCBP data is supported.
    if format_code != "M" {
      return Err(BcbpParserError::UnsupportedFormat(format_code.chars().next().unwrap()));
    }

    // Read the number of embedded flight legs.
    let legs_encoded = {
      let field = Field::NumberOfLegsEncoded;
      self.scanner
        .scan_field(field, true)
        .map(|n| u64::from_str_radix(n, 10).unwrap_or(0) )
        .map_err(|e| BcbpParserError::UnableToReadField { field: field, reason: e } )?
    };

    // At least one leg is required.
    if legs_encoded == 0 {
      return Err(BcbpParserError::NoLegs);
    }
    
    // Scan the root-level mandatory items (they are permitted to be empty).
    parse_fields![self.scanner, self.target.unique_fields, self.strict, &[
      Field::PassengerName,
      Field::ElectronicTicketIndicator,
    ]];

    // Scan each encoded leg.
    for leg_index in 0 .. legs_encoded {
      let mut leg = Leg { fields: HashMap::new() };

      // Scan the mandatory elements of a leg.
      parse_fields![self.scanner, leg.fields, self.strict, &[
        Field::OperatingCarrierPnrCode,
        Field::FromCityAirportCode,
        Field::ToCityAirportCode,
        Field::OperatingCarrierDesignator,
        Field::FlightNumber,
        Field::DateOfFlight,
        Field::CompartmentCode,
        Field::SeatNumber,
        Field::CheckInSequenceNumber,
        Field::PassengerStatus,
      ]];

      // A conditional section follows.
      let variable_size_field_length = {
        let field = Field::FieldSizeOfVariableSizeField;
        self.scanner
          .scan_field(field, true)
          .map(|n| usize::from_str_radix(n, 16).unwrap_or(0) )
          .map_err(|e| BcbpParserError::UnableToReadField { field: field, reason: e } )?
      };

      // Newer versions of the BCBP format will add additional fields beyond the
      // ones this parser knows about at the end of the variable size section.
      // This tracks how much of the variable field is parsed successfully
      // so the rest can be advanced over.
      let mut variable_size_field_bytes_left = self.scanner.remaining_len();
      if variable_size_field_bytes_left < variable_size_field_length {
        return Err(BcbpParserError::VariableLengthFieldTooLong);
      }

      // If this is the first leg, some additional unique data follows.
      if leg_index == 0 {
        let marker_field = Field::BeginningOfVersionNumber;
        let marker = self.scanner
          .scan_field(marker_field, true)
          .map_err(|e| BcbpParserError::UnableToReadField { field: marker_field, reason: e } )?;
        if marker != ">" {
          return Err(BcbpParserError::InvalidStartOfVersionNumber);
        }

        // The version number is not particularly relevant at this point.
        let _ = self.scanner.scan_field(Field::VersionNumber, true);

        // A nested structure follows.
        let variable_size_unique_length = {
          let field = Field::FieldSizeOfStructuredMessageUnique;
          self.scanner
            .scan_field(field, true)
            .map(|n| usize::from_str_radix(n, 16).unwrap_or(0) )
            .map_err(|e| BcbpParserError::UnableToReadField { field: field, reason: e } )?
        };

        let mut variable_size_unique_bytes_left = self.scanner.remaining_len();
        if variable_size_unique_bytes_left < variable_size_unique_length {
          return Err(BcbpParserError::VariableLengthFieldTooLong);
        }
        
      }

      self.target.legs.push(leg);
    }

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
