// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

//! iata_bcbp is a Rust library for parsing IATA Bar Coded Boarding Pass
//! (or BCBP) data. The library support reading 'M' type pass data and
//! supports all the fields defined in version 6 of the standard as
//! defined in IATA Resolution 792.
//!
//! # Example
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! iata_bcbp = "0.1"
//! ```
//!
//! And to the top of your `main.rs`:
//!
//! ```rs
//! extern crate iata_bcbp;
//! ```

#[macro_use]
extern crate log;

mod bcbp;
mod de;
mod error;

pub use bcbp::{Bcbp, Leg, SecurityData};
pub use de::{from_str, field::Field};
pub use error::{Error, Result};
