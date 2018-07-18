// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

//! Scanner processes an input string in response to expected fields.
//! The scanner operates by returning substrings of the input without
//! performing allocations.
//!
//! # Notes
//! 1. This module does support Unicode.

use std::error;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CharacterSet {
  /// All ASCII Characters.
  All,
  /// IATA Resolution 729 Appendix A format specifier 'f'.
  /// Any valid ASCII character.
  IataAlphaNumerical,
  /// IATA Resolution 729 Appendix A format specifier 'N'.
  /// In the ASCII range `'0' ... '9'`.
  IataNumerical,
  /// A subset of IATA Resolution 729 Appendix A format specifier 'f'.
  /// In the ASCII range `'0' ... '9'` and `'A' ... 'F'`.
  IataNumericalHexadecimal,
  /// IATA Resolution 729 Appendix A format specifier 'a'.
  /// In the ASCII range `'A' ... 'Z'`.
  IataAlphabetical,
}

impl CharacterSet {
  fn contains(&self, character: char) -> bool {
    match self {
      &CharacterSet::All =>
        character.is_ascii(),
      &CharacterSet::IataAlphaNumerical =>
        character.is_ascii(),
      &CharacterSet::IataNumerical =>
        character.is_ascii_digit(),
      &CharacterSet::IataNumericalHexadecimal =>
        character.is_ascii_hexdigit() && (character.is_ascii_uppercase() || character.is_ascii_digit()),
      &CharacterSet::IataAlphabetical =>
        character.is_ascii_uppercase(),
    }
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ScannerError {
  /// The remaining input is not long enough to extract the desired field.
  FieldLongerThanRemainingInput,
  /// A character is not in the required set for the specified item.
  InvalidCharacter { value: char, set: CharacterSet },
  /// An valid numeric literal was encountered with `value` out of range.
  NumericLiteralOutOfRange,
}

impl error::Error for ScannerError {
  /// Returns a string slice with a general description of a scanner error.
  /// No specific information is contained. To obtain a printable representation,
  /// use the `fmt::Display` attribute.
  fn description(&self) -> &str {
    "scanner error"
  }
}

impl fmt::Display for ScannerError {
  /// Formats the receiver for display purposes into formatter `f`. Names are lower-case.
  /// Returns a result representing the formatted receiver or a failure to write into `f`.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ScannerError::FieldLongerThanRemainingInput =>
        write!(f, "field length exceeds the length of the input remaining"),
      ScannerError::NumericLiteralOutOfRange =>
        write!(f, "numeric literal out of range"),
      ScannerError::InvalidCharacter { .. } =>
        write!(f, "encountered character is not in the specified character set"),
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

  /// Returns `true` if the scanner has reached the end of the input.
  #[inline]
  pub fn is_at_end(&self) -> bool {
    self.offset >= self.input.len()
  }

  /// Returns a substring representing the unprocessed part of the input.
  #[inline]
  fn remaining(&self) -> &'a str {
    &self.input[self.offset .. ]
  }

  /// Advances over the specified number of bytes of input returning the
  /// character range to which it corresponds.
  ///
  /// # Panics
  /// Will panic if attempting to advance over the end of the string.
  #[inline]
  fn advance_by(&mut self, bytes: usize) {
    assert!(self.remaining().len() >= bytes, "Attempting to advance past the end of the string.");
    self.offset += bytes
  }

  /// Obtains a substring of exactly `characters` or an error.
  /// Does not advance the scan offset.
  fn peek(&self, characters: usize) -> Option<&'a str> {
    let substring_bytes = self.remaining()
      .chars()
      .take(characters)
      .map(|c| c.len_utf8())
      .sum();
    let substring = &self.remaining()[ .. substring_bytes];
    if substring.chars().count() == characters {
      Some(substring)
    } else {
      None
    }
  }

  /// Scans a specific character, returns `true` if consumed.
  pub fn scan_character(&mut self, character: char) -> bool {
    if let Some(character) = self.remaining().chars().next() {
      self.advance_by(character.len_utf8());
      true
    } else {
      false
    }
  }

  /// Scans a single character in the IATA 'f' character set.
  pub fn scan_character_from_set(&mut self, set: CharacterSet) -> Result<char, ScannerError> {
    let next_char = self.remaining()
      .chars()
      .next()
      .ok_or(ScannerError::FieldLongerThanRemainingInput)?;

    if !set.contains(next_char) {
      return Err(ScannerError::InvalidCharacter { value: next_char, set: set });
    }

    self.advance_by(next_char.len_utf8());

    Ok(next_char)
  }

  /// Scans an arbitrary input string of exactly `characters` in length.
  /// If found, a reference to the substring is returned.
  pub fn scan_characters_from_set(&mut self, characters: usize, set: CharacterSet) -> Result<&str, ScannerError> {
    let substring = self.peek(characters).ok_or(ScannerError::FieldLongerThanRemainingInput)?;

    // Validate that all characters in the string are in the set.
    let first_invalid = substring
      .chars()
      .filter(|&c| !set.contains(c))
      .next();
    if let Some(invalid_character) = first_invalid {
      return Err(ScannerError::InvalidCharacter { value: invalid_character, set: set });
    }

    self.advance_by(substring.len());

    Ok(substring)
  }

  /// Scans a positive numeric input string of exactly `characters` in length,
  /// in the given character set, with the specified radix.
  /// The string may be zero-padded.
  /// If found, the parsed value is returned.
  fn scan_numeric(&mut self, characters: usize, set: CharacterSet, radix: u32) -> Result<u64, ScannerError> {
    let slice = self.peek(characters).ok_or(ScannerError::FieldLongerThanRemainingInput)?;

    // Validate that all characters in the string are IATA type 'N'.
    let first_invalid = slice
      .chars()
      .filter(|&c| !set.contains(c))
      .next();
    if let Some(invalid_character) = first_invalid {
      return Err(ScannerError::InvalidCharacter {
        value: invalid_character,
        set: CharacterSet::IataNumerical,
      });
    }

    // Parse the number.
    let decimal = u64::from_str_radix(slice, radix).map_err(|_| {
      ScannerError::NumericLiteralOutOfRange
    })?;
    
    self.advance_by(slice.len());

    Ok(decimal)
  }

  /// Scans a positive numeric input string of exactly `characters` in length.
  /// The string may be zero-padded.
  /// If found, the parsed value is returned.
  pub fn scan_decimal(&mut self, characters: usize) -> Result<u64, ScannerError> {
    self.scan_numeric(characters, CharacterSet::IataNumerical, 10)
  }

  /// Scans a positive hexadecimal numeric input string of exactly `characters` in length.
  /// The string may be zero-padded.
  /// If found, the parsed value is returned.
  pub fn scan_hexadecimal(&mut self, characters: usize) -> Result<u64, ScannerError> {
    self.scan_numeric(characters, CharacterSet::IataNumericalHexadecimal, 16)
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
