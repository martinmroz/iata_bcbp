// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use std::str::FromStr;

mod field;
pub use field::*;
mod parser;
pub use parser::BcbpParserError;
mod scanner;

#[derive(Clone,Eq,PartialEq,Debug)]
pub struct Leg {
  /// Backing store for the string slices pertaining to each repeated field.
  fields: HashMap<Field, String>,
}

#[derive(Clone,Eq,PartialEq,Debug)]
pub struct Bcbp {
  /// Backing store for the string slices pertaining to each unique field.
  unique_fields: HashMap<Field, String>,
  /// Backing stores for string slices in repeated fields.
  legs: Vec<Leg>,
}

impl Bcbp {

  /// Constructs a new instance of the receiver with `input` data.
  /// If `strict` mode is `true`, the format of each encountered field is validated.
  /// Otherwise, only fields required for parsing the BCBP string are validated.
  pub fn from_str_strict<T>(input: T, strict: bool) -> Result<Self, BcbpParserError>
  where
    T: AsRef<str>
  {
    parser::parse(input, strict)
  }

}

/// The `FromStr` parse operation utilizes strict mode by default.
impl FromStr for Bcbp {
  type Err = BcbpParserError;
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    Bcbp::from_str_strict(input, true)
  }
}
