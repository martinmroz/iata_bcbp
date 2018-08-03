// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::fmt;
use std::ops::RangeInclusive;
use std::str::FromStr;

use error::{Error, Result};

pub trait Field: FromStr<Err = Error> + fmt::Debug + Default {
    /// The raw string representation of the field, regardless of validity of contents.
    fn raw(&self) -> &str;
    /// Returns `true` if the receiver is valid.
    /// The default implementation checks to see if all the characters in the raw string are ASCII values.
    fn is_valid(&self) -> bool { self.raw().is_ascii() }
    /// Returns `true` if the receiver is empty.
    /// The default implementation checks to see if all the characters in the raw string are spaces.
    fn is_empty(&self) -> bool { self.raw().chars().all(|c| c == ' ' ) }
}
