// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::str::FromStr;

mod field;
mod parser;

use bcbp;
use error::{Error, Result};

pub use self::parser::from_str_strict;

impl FromStr for bcbp::Bcbp {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        from_str_strict(input, true)
    }
}
