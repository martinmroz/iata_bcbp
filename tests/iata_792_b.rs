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
  assert_eq!(pass_data.passenger_name(), "DESMARAIS/LUC       ");
  assert_eq!(pass_data.electronic_ticket_indicator(), 'E');
  assert_eq!(pass_data.legs().len(), 1);

  let first_leg = &pass_data.legs()[0];
  assert_eq!(first_leg.operating_carrier_pnr_code(), "ABC123 ");
  assert_eq!(first_leg.from_city_airport_code(), "YUL");
  assert_eq!(first_leg.to_city_airport_code(), "FRA");
  assert_eq!(first_leg.operating_carrier_designator(), "AC ");
  assert_eq!(first_leg.flight_number(), "0834 ");
  assert_eq!(first_leg.date_of_flight(), "326");
  assert_eq!(first_leg.compartment_code(), 'J');
  assert_eq!(first_leg.seat_number(), "001A");
  assert_eq!(first_leg.check_in_sequence_number(), "0025 ");
  assert_eq!(first_leg.passenger_status(), '1');

  assert_eq!(pass_data.security_data().type_of_security_data(), Some('1'));
  assert_eq!(pass_data.security_data().security_data(), Some("GIWVC5EH7JNT684FVNJ91W2QA4DVN5J8K4F0L0GEQ3DF5TGBN8709HKT5D3DW3GBHFCVHMY7J5T6HFR41W2QA4DVN5J8K4F0L0GE"));

}
