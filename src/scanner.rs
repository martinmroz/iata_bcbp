// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::cmp;
use std::error;
use std::fmt;

use field::{self, DataFormat};

pub trait DataFormatValidation {
  /// Returns `true` if the receiver matches the given `format`.
  fn conforms_to(&self, format: DataFormat) -> bool;
}

impl DataFormatValidation for str {
  fn conforms_to(&self, format: DataFormat) -> bool {
    match format {

      // Simple field types.
      DataFormat::Arbitrary =>
        self.chars().all(|c| c.is_ascii()),
      DataFormat::IataAlphaNumerical =>
        self.chars().all(|c| c.is_ascii()),
      DataFormat::IataNumerical =>
        self.chars().all(|c| c.is_ascii_digit()),
      DataFormat::IataAlphabetical =>
        self.chars().all(|c| c.is_ascii_uppercase()),

      // A flight number matches the format 'NNNN[a]'.
      DataFormat::FlightNumber => {
        if self.len() != 5 {
          false
        } else {
          let numeric_valid = self[ .. 4].chars().all(|c| c.is_ascii_digit());
          let optional_alphabetic_valid = self[4 .. 5].chars().all(|c| c == ' ' || c.is_ascii_uppercase());
          numeric_valid && optional_alphabetic_valid
        }
      }

      // A seat number matches the format 'NNNa'.
      DataFormat::SeatNumber => {
        if self.len() != 4 {
          false
        } else {
          let numeric_valid = self[ .. 3].chars().all(|c| c.is_ascii_digit());
          let alphabetic_valid = self[3 .. 4].chars().all(|c| c.is_ascii_uppercase());
          numeric_valid && alphabetic_valid
        }
      }

      // A check-in sequence matches the format 'NNNN[f]'.
      DataFormat::CheckInSequenceNumber => {
        if self.len() != 5 {
          false
        } else {
          let numeric_valid = self[ .. 4].chars().all(|c| c.is_ascii_digit());
          let ascii_valid = self[4 .. 5].chars().all(|c| c.is_ascii());
          numeric_valid && ascii_valid
        }
      }

    }
  }
}

#[derive(Copy,Clone,Eq,PartialEq,Debug,Hash)]
pub enum ScannerError {
  /// The requested field is longer than the un-processed input.
  FieldTooLong,
  /// The requested field list is longer than the un-processed input.
  FieldListTooLong,
  /// Unable to validate the field.
  ValidationFailed,
}

impl error::Error for ScannerError {}

impl fmt::Display for ScannerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ScannerError::FieldTooLong =>
        write!(f, "field length exceeds the length of the un-processed input"),
      ScannerError::FieldListTooLong =>
        write!(f, "field list length exceeds the length of the un-processed input"),
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
  pub fn new(input: &'a str) -> Self {
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

  /// Skips over `bytes` of the remaining string.
  #[inline]
  pub fn skip_over(&mut self, bytes: usize) {
    self.offset += cmp::min(self.remaining_len(), bytes);
  }

  /// Create a scanner over a block of the remaining input
  /// and consumes the entire portion from the perspective of the receiver.
  /// The full length is consumed whether or not the new scanner consumes any input.
  pub fn scan_field_list(&mut self, length: usize) -> Result<Scanner<'a>, ScannerError> {
    if length > self.remaining_len() {
      return Err(ScannerError::FieldListTooLong);
    }

    let variable_length_field_scanner = Scanner {
      input: &self.remaining()[ .. length],
      offset: 0,
    };

    self.skip_over(length);
    Ok(variable_length_field_scanner)
  }

  /// Attempts to scan a field with an explicit length.
  /// 
  /// # Panics
  /// Will panic if attempting to scan the wrong number of characters for a fixed-length field.
  /// Will panic if attempting to scan zero bytes.
  pub fn scan_field_len(&mut self, field: field::Field, length: usize, strict: bool) -> Result<&'a str, ScannerError> {
    assert!(length > 0 && (field.len() == 0 || field.len() == length));

    print!("[TRACE] {} ", field);

    if self.remaining_len() < length {
      println!("<FAILED: Field Too Long> remaining={} requested={}", self.remaining_len(), length);
      return Err(ScannerError::FieldTooLong);
    }

    // Extract the requested substring and advance the scanner.
    let substring = &self.remaining()[ .. length];
    self.skip_over(length);

    // If Strict mode is requsted, validate the field.
    if strict {
      if !substring.chars().all(|c| c == ' ') && 
         !substring.conforms_to(field.data_format()) {
        println!("<FAILED: Validation of Strict Field Failed>");
        return Err(ScannerError::ValidationFailed);
      }
    }

    println!("is '{}'", substring);
    Ok(substring)
  }

  /// Attempts to scan a field using the default length.
  /// Leading and trailing whitespace will be trimmed off.
  /// 
  /// # Panics
  /// Will panic if attempting to scan a variable-length field.
  pub fn scan_field(&mut self, field: field::Field, strict: bool) -> Result<&'a str, ScannerError> {
    self.scan_field_len(field, field.len(), strict)
  }

}
