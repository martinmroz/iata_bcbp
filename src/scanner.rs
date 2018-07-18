// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::error;
use std::fmt;

use field;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ScannerError {
  /// The requested field is longer than the un-processed input.
  FieldTooLong,
  /// Unable to validate the field.
  ValidationFailed,
}

impl error::Error for ScannerError {}

impl fmt::Display for ScannerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ScannerError::FieldTooLong =>
        write!(f, "field length exceeds the length of the un-processed input"),
      ScannerError::ValidationFailed =>
        write!(f, "unable to validate the data in the field"),
    }
  }
}

/// An iterator over the tokens in an input stream.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Scanner<'a> {
  /// Input string to process.
  input: &'a str,
  /// Offset from the start of input, in bytes, not in code points.
  offset: usize,
}

impl<'a> Scanner<'a> {

  /// Creates a new tokenizer over the `input` string.
  fn new(input: &'a str) -> Self {
    Scanner {
      input: input,
      offset: 0,
    }
  }

  /// Returns a substring representing the unprocessed part of the input.
  #[inline]
  fn remaining(&self) -> &'a str {
    &self.input[self.offset .. ]
  }

  /// Returns the number of characters left to process.
  #[inline]
  pub fn remaining_len(&self) -> usize {
    self.input.len().saturating_sub(self.offset)
  }

  /// Returns `true` if the scanner has reached the end of the input.
  #[inline]
  pub fn is_at_end(&self) -> bool {
    self.remaining_len() == 0
  }

  /// Attempts to scan a field with an explicit length.
  /// 
  /// # Panics
  /// Will panic if attempting to scan the wrong number of characters for a fixed-length field.
  /// Will panic if attempting to scan zero bytes.
  pub fn scan_field_len(&mut self, field: field::Field, length: usize, strict: bool) -> Result<&'a str, ScannerError> {
    assert!(length > 0 && (field.len() == 0 || field.len() == length));

    if self.remaining_len() < length {
      return Err(ScannerError::FieldTooLong);
    }

    // Extract the requested substring and advance the scanner.
    let substring = &self.remaining()[ .. length];
    self.offset += length;

    // If Strict mode is requsted, validate the field.
    if strict {
      let validated = match field.data_format() {

        // Arbitrary or alphanumerical fields can contain any valid ASCII characters.
        field::DataFormat::Arbitrary =>
          substring.is_ascii(),
        field::DataFormat::IataAlphaNumerical =>
          substring.is_ascii(),
        
        // Numerical fields must either be all spaces or zero padded.
        field::DataFormat::IataNumerical =>
          substring.chars().all(|c| c == ' ') || substring.chars().all(|c| c.is_ascii_digit()),
        field::DataFormat::IataNumericalHexadecimal =>
          substring.chars().all(|c| c == ' ') || substring.chars().all(|c| c.is_ascii_hexdigit() && c.is_uppercase()),

        // Alphabetical fields can contain a mix of uppercase ASCII characters and spaces.
        field::DataFormat::IataAlphabetical =>
          substring.chars().all(|c| c == ' ' || c.is_ascii_uppercase()),
        
        // A literal must have a single character matching the specified value.
        field::DataFormat::Literal(literal) =>
          (substring.len() == 1) && substring.chars().all(|c| c == literal),

        // A flight number can be all spaces or match the format 'NNNN[a]'.
        field::DataFormat::FlightNumber => {
          if substring.len() != 5 {
            false
          } else {
            // The field may be all spaces.
            if substring.chars().all(|c| c == ' ') {
              true
            } else {
              let numeric_portion_valid = substring[ .. 4].chars().all(|c| c.is_ascii_digit());
              let optional_alphabetic_portion_valid = substring[4 .. 5].chars().all(|c| c == ' ' || c.is_ascii_uppercase());
              numeric_portion_valid && optional_alphabetic_portion_valid
            }
          }
        }

        // A seat number can be all spaces or match the format 'NNNa'.
        field::DataFormat::SeatNumber => {
          if substring.len() != 4 {
            false
          } else {
            // The field may be all spaces.
            if substring.chars().all(|c| c == ' ') {
              true
            } else {
              let numeric_portion_valid = substring[ .. 3].chars().all(|c| c.is_ascii_digit());
              let alphabetic_portion_valid = substring[3 .. 4].chars().all(|c| c.is_ascii_uppercase());
              numeric_portion_valid && alphabetic_portion_valid
            }
          }
        }

        // A check-in sequence number can be all spaces or match the format 'NNNN[f]'.
        field::DataFormat::CheckInSequenceNumber => {
          if substring.len() != 5 {
            false
          } else {
            // The field may be all spaces.
            if substring.chars().all(|c| c == ' ') {
              true
            } else {
              let numeric_portion_valid = substring[ .. 4].chars().all(|c| c.is_ascii_digit());
              let optional_alphanumeric_portion_valid = substring[4 .. 5].chars().all(|c| c.is_ascii());
              numeric_portion_valid && optional_alphanumeric_portion_valid
            }
          }
        }

      };

      if !validated {
        return Err(ScannerError::ValidationFailed);
      }
    }

    Ok(substring)
  }

  /// Attempts to scan a field using the default length.
  /// 
  /// # Panics
  /// Will panic if attempting to scan a variable-length field.
  pub fn scan_field(&mut self, field: field::Field, strict: bool) -> Result<&'a str, ScannerError> {
    self.scan_field_len(field, field.len(), strict)
  }

}

/// Trait to return a scanner over the input.
pub trait Scannable<'a> {
  fn scanner(&'a self) -> Scanner<'a>;
}

/// Allows scanning of anything representable as a `str`.
impl<'a, T> Scannable<'a> for T
where
  T: AsRef<str> + 'a {
  fn scanner(&'a self) -> Scanner<'a> {
    Scanner::new(self.as_ref())
  }
}
