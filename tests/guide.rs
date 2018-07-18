// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

//! Tests from the IATA Bar Coded Boarding Pass (BCBP)
//! Implementation Guide Book, Fifth Edition, dated June 1st, 2016.

extern crate iata_bcbp;

use iata_bcbp::*;

#[test]
fn test_ba_example_p19() {
  // This example comes from page 19 of the implementation guide.
  // Of note, the flight number is specified incorrectly as '0072' instead of
  // '0072 '. The lack of trailing space is accounted for in the parser.
  const DATA: &str = "M1LEOPOLD/EMR         EZQ7O92 GVALHRBA 00723319C002F00009100";
  let boarding_pass = str::parse::<Bcbp>(DATA).unwrap();
  print!("{:?}", &boarding_pass);
  assert!(false);
}

#[test]
fn test_b2_bcbp_printed_at_kiosk() {
  // This example comes from page 53 of the implementation guide.
  // This is a BCBP document printed at a kiosk.
  const DATA: &str = "M1ASKREN/TEST         EA272SL ORDNRTUA 0881 007F002K0303 15C>3180 K6007BUA              2901624760758980 UA UA EY975897            *30600    09  UAG    ";
  let boarding_pass = str::parse::<Bcbp>(DATA).unwrap();
  print!("{:?}", &boarding_pass);
  assert!(false);
}
