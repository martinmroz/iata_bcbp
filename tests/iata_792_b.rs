// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

//! Test cases derived from the examples listed in IATA Resolution 792
//! Attachment B.

extern crate iata_bcbp;

use std::str::FromStr;

use iata_bcbp::*;

#[test]
fn example_1_m1_using_mandatory_elements_and_security_fields() {
  const EXAMPLE_1: &str = "M1DESMARAIS/LUC       EABC123 YULFRAAC 0834 326J001A0025 100^164GIWVC5EH7JNT684FVNJ91W2QA4DVN5J8K4F0L0GEQ3DF5TGBN8709HKT5D3DW3GBHFCVHMY7J5T6HFR41W2QA4DVN5J8K4F0L0GE";
  let pass_data = Bcbp::from_str(EXAMPLE_1).unwrap();
  println!("{:?}", pass_data);
  assert!(false);
}
