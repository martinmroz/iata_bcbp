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
  /// The conditional item list does not match the expected field list.
  InvalidConditionalItemList,
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
      &BcbpParserError::InvalidConditionalItemList =>
        write!(f, "conditional item list invalid"),
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

/// Parses all of the given sequential fields.
/// An error occurs if a field fails to scan properly and completely.
fn collect_required_fields(
  scanner: &mut Scanner,
  strict: bool,
  fields: &[Field],
) -> Result<HashMap<Field, String>, BcbpParserError> {
  let mut scanned_fields = HashMap::new();

  for &field in fields {
    let value = scanner
      .scan_field(field, strict)
      .map(String::from)
      .map_err(|e| BcbpParserError::UnableToReadField { field: field, reason: e } )?;
    scanned_fields.insert(field, value);
  }

  Ok(scanned_fields)
}

/// Parses as many of the given sequential fields as possible until the scanner runs out of input.
/// An error occurs if: 
/// * A field fails to scan properly OR
/// * The scanner runs out of input before the list of fields is exhausted AND
///   * the number of bytes remaining is more than zero AND
///   * the number of bytes remaining is less than the number of bytes required for the next field.
fn collect_optional_fields(
  scanner: &mut Scanner,
  strict: bool,
  fields: &[Field],
) -> Result<HashMap<Field, String>, BcbpParserError> {
  let mut scanned_fields = HashMap::new();

  // Parse conditional fields until either the scanner runs out of input or the list
  // of fields is exhausted, either of which may happen as either the BCBP
  // does not include all optional fields or future versions of the standard
  // add additional fields.
  for &field in fields {
    if scanner.remaining_len() == 0 {
      return Ok(scanned_fields);
    } else if scanner.remaining_len() < field.len() {
      return Err(BcbpParserError::InvalidConditionalItemList);
    } else {
      let value = scanner
        .scan_field(field, strict)
        .map(String::from)
        .map_err(|e| BcbpParserError::UnableToReadField { field: field, reason: e } )?;
      scanned_fields.insert(field, value);
    }
  }

  Ok(scanned_fields)
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
    self.target.unique_fields = collect_required_fields(&mut self.scanner, self.strict, &[
      Field::PassengerName,
      Field::ElectronicTicketIndicator,
    ])?;

    // Scan each encoded leg.
    for leg_index in 0 .. legs_encoded {
      let mut leg = Leg { fields: HashMap::new() };

      // Scan the mandatory elements of a leg.
      leg.fields = collect_required_fields(&mut self.scanner, self.strict, &[
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
      ])?;

      // A conditional section follows.
      let conditional_fields_len = {
        let field = Field::FieldSizeOfVariableSizeField;
        self.scanner
          .scan_field(field, true)
          .map(|n| usize::from_str_radix(n, 16).unwrap_or(0) )
          .map_err(|e| BcbpParserError::UnableToReadField { field: field, reason: e } )?
      };

      // Conditional fields section.
      if conditional_fields_len > 0 {
        let mut conditional_fields_scanner = self.scanner
          .scan_field_list(conditional_fields_len)
          .map_err(|_| BcbpParserError::VariableLengthFieldTooLong )?;

        // Collect unique conditional fields.
        if leg_index == 0 {
          let marker_field = Field::BeginningOfVersionNumber;
          let marker = conditional_fields_scanner
            .scan_field(marker_field, true)
            .map_err(|e| BcbpParserError::UnableToReadField { field: marker_field, reason: e } )?;
          if marker != ">" {
            return Err(BcbpParserError::InvalidStartOfVersionNumber);
          }

          // The version number is not particularly relevant at this point.
          let _ = conditional_fields_scanner.scan_field(Field::VersionNumber, true);

          // A nested structure of unique conditional fields may follow.
          let unique_conditional_fields_len = {
            let field = Field::FieldSizeOfStructuredMessageUnique;
            conditional_fields_scanner
              .scan_field(field, true)
              .map(|n| usize::from_str_radix(n, 16).unwrap_or(0) )
              .map_err(|e| BcbpParserError::UnableToReadField { field: field, reason: e } )?
          };

          // Collect all the provided unique conditional fields.
          if unique_conditional_fields_len > 0 {
            let mut unique_conditional_fields_scanner = conditional_fields_scanner
              .scan_field_list(unique_conditional_fields_len)
              .map_err(|_| BcbpParserError::VariableLengthFieldTooLong )?;
            let fields = collect_optional_fields(&mut unique_conditional_fields_scanner, self.strict, &[
              Field::PassengerDescription,
              Field::SourceOfCheckIn,
              Field::SourceOfBoardingPassIssuance,
              Field::DateOfIssueOfBoardingPass,
              Field::DocumentType,
              Field::AirlineDesignatorOfBoardingPassIssuer,
              Field::BaggageTagLicensePlateNumbers,
              Field::FirstNonConsecutiveBaggageTagLicensePlateNumber,
              Field::SecondNonConsecutiveBaggageTagLicensePlateNumber,
            ])?;
            self.target.unique_fields.extend(fields);
          }
        }

        // A nested structure of repeated conditional fields may follow.
        let repeated_conditional_fields_len = {
          let field = Field::FieldSizeOfStructuredMessageRepeated;
          conditional_fields_scanner
            .scan_field(field, true)
            .map(|n| usize::from_str_radix(n, 16).unwrap_or(0) )
            .map_err(|e| BcbpParserError::UnableToReadField { field: field, reason: e } )?
        };

        // Collect repeated conditional fields.
        if repeated_conditional_fields_len > 0 {
          let mut repeated_conditional_fields_scanner = conditional_fields_scanner
            .scan_field_list(repeated_conditional_fields_len)
            .map_err(|_| BcbpParserError::VariableLengthFieldTooLong )?;
          let fields = collect_optional_fields(&mut repeated_conditional_fields_scanner, self.strict, &[
            Field::AirlineNumericCode,
            Field::DocumentFormSerialNumber,
            Field::SelecteeIndicator,
            Field::InternationalDocumentVerification,
            Field::MarketingCarrierDesignator,
            Field::FrequentFlyerAirlineDesignator,
            Field::FrequentFlyerNumber,
            Field::IdAdIndicator,
            Field::FreeBaggageAllowance,
            Field::FastTrack,
          ])?;
          leg.fields.extend(fields);
        }

        // If there are any bytes left, they are for airline use.
        if conditional_fields_scanner.remaining_len() > 0 {
          let field = Field::AirlineIndividualUse;
          let airline_use_len = conditional_fields_scanner.remaining_len();
          let airline_use = conditional_fields_scanner
            .scan_field_len(field, airline_use_len, self.strict)
            .map(String::from)
            .map_err(|e| BcbpParserError::UnableToReadField { field: field, reason: e } )?;
          leg.fields.insert(field, airline_use);
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
