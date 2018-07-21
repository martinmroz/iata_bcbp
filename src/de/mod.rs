
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
