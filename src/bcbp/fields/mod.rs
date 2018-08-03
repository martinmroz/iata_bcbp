// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

mod baggage_tags;

use std::fmt;
use std::str::FromStr;

#[derive(Copy,Clone,Eq,PartialEq,Ord,PartialOrd,Hash,Debug)]
pub enum DataKind {
    /// The field contains invalid data.
    Invalid,
    /// The field is empty.
    Empty,
    /// The field contains valid accessible data.
    Valid,
}

pub trait Field: fmt::Debug + fmt::Display + Default + FromStr {
    /// Returns the raw string value of the field unconditionally.
    fn raw_value(&self) -> &str;
    /// Returns the kind of the data contained within the field.
    fn data_kind(&self) -> DataKind;
}
