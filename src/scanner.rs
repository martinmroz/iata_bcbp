// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::error;
use std::fmt;

use field;
use field::DataFormatValidation;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ScannerError {
  /// The requested field is longer than the un-processed input.
  FieldTooLong,
  /// Required Field Empty.
  RequiredFieldEmpty,
  /// Unable to validate the field.
  ValidationFailed,
}

impl error::Error for ScannerError {}

impl fmt::Display for ScannerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ScannerError::FieldTooLong =>
        write!(f, "field length exceeds the length of the un-processed input"),
      ScannerError::RequiredFieldEmpty =>
        write!(f, "the field was marked as required however it contains no data"),
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

  /// Attempts to scan a field with an explicit length.
  /// Leading and trailing whitespace will be trimmed off.
  /// 
  /// # Panics
  /// Will panic if attempting to scan the wrong number of characters for a fixed-length field.
  /// Will panic if attempting to scan zero bytes.
  pub fn scan_field_len(&mut self, field: field::Field, length: usize, required: bool, strict: bool) -> Result<&'a str, ScannerError> {
    assert!(length > 0 && (field.len() == 0 || field.len() == length));

    print!("[TRACE] {} ", field);

    if self.remaining_len() < length {
      println!("<FAILED: Field Too Long>");
      return Err(ScannerError::FieldTooLong);
    }

    // Extract the requested substring and advance the scanner.
    let substring = &self.remaining()[ .. length];
    self.offset += length;

    // If the field is required, it cannot be empty.
    if required {
      if substring.chars().all(|c| c == ' ') {
        println!("<FAILED: Required Field Empty>");
        return Err(ScannerError::RequiredFieldEmpty);
      }
    }

    // If Strict mode is requsted, validate the field.
    if strict {
      if !substring.conforms_to(field.data_format()) {
        println!("<FAILED: Validation of Strict Field Failed>");
        return Err(ScannerError::ValidationFailed);
      }
    }

    println!("is '{}'", substring.trim());
    Ok(substring.trim())
  }

  /// Attempts to scan a field using the default length.
  /// Leading and trailing whitespace will be trimmed off.
  /// 
  /// # Panics
  /// Will panic if attempting to scan a variable-length field.
  pub fn scan_field(&mut self, field: field::Field, required: bool, strict: bool) -> Result<&'a str, ScannerError> {
    self.scan_field_len(field, field.len(), required, strict)
  }

}
