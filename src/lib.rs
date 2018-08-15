// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

//! `iata_bcbp is` a Rust library for parsing IATA BCBP Type M
//! objects conforming to versions 2 through 6 of the standard inclusively. 
//! This format is used by airlines to encode boarding pass information into 
//! electronic ticket itinerary document barcodes in addition to paper and 
//! mobile boarding passes.
//!
//! # Example
//! ```rust
//! extern crate iata_bcbp;
//!
//! use std::str::FromStr;
//! 
//! use iata_bcbp::Bcbp;
//!
//! fn main() {
//!     const PASS_STR: &str = "M1DESMARAIS/LUC       EABC123 YULFRAAC 0834 326J001A0025 100";
//!     let pass_data = Bcbp::from_str(PASS_STR).unwrap();
//!
//!     println!("Passenger: {}", pass_data.passenger_name());
//!  
//!    for leg in pass_data.legs().iter() {
//!        println!();
//!        println!("      PNR: {}"  , leg.operating_carrier_pnr_code());
//!        println!("     From: {}"  , leg.from_city_airport_code());
//!        println!("       To: {}"  , leg.to_city_airport_code());
//!        println!("   Flight: {}{}", leg.operating_carrier_designator(), leg.flight_number());
//!        println!("     Seat: {}"  , leg.seat_number());
//!        println!(" Sequence: {}"  , leg.check_in_sequence_number());
//!    }
//! }
//! ```

#[macro_use]
extern crate log;

mod bcbp;
mod de;
mod error;

pub use bcbp::{Bcbp, Leg, SecurityData};
pub use de::{from_str, field::Field};
pub use error::{Error, Result};
