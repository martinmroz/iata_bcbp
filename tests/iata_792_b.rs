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

#[test]
fn example_2_m2_multiple_legs() {
    const EXAMPLE_2: &str = "M2DESMARAIS/LUC       EABC123 YULFRAAC 0834 226F001A0025 14D>6181WW6225BAC 00141234560032A0141234567890 1AC AC 1234567890123    20KYLX58ZDEF456 FRAGVALH 3664 227C012C0002 12E2A0140987654321 1AC AC 1234567890123    2PCNWQ^164GIWVC5EH7JNT684FVNJ91W2QA4DVN5J8K4F0L0GEQ3DF5TGBN8709HKT5D3DW3GBHFCVHMY7J5T6HFR41W2QA4DVN5J8K4F0L0GE";
    let pass_data = Bcbp::from_str(EXAMPLE_2).unwrap();
    assert_eq!(pass_data.passenger_name(), "DESMARAIS/LUC       ");
    assert_eq!(pass_data.electronic_ticket_indicator(), 'E');
    assert_eq!(pass_data.legs().len(), 2);

    assert_eq!(pass_data.passenger_description(), Some('1'));
    assert_eq!(pass_data.source_of_check_in(), Some('W'));
    assert_eq!(pass_data.source_of_boarding_pass_issuance(), Some('W'));
    assert_eq!(pass_data.date_of_issue_of_boarding_pass(), Some("6225"));
    assert_eq!(pass_data.document_type(), Some('B'));
    assert_eq!(pass_data.airline_designator_of_boarding_pass_issuer(), Some("AC "));
    assert_eq!(pass_data.baggage_tag_license_plate_numbers(), Some("0014123456003"));
    assert_eq!(pass_data.first_non_consecutive_baggage_tag_license_plate_numbers(), None);
    assert_eq!(pass_data.second_non_consecutive_baggage_tag_license_plate_numbers(), None);

    { // Fields in leg 1 of 2.
        let first_leg = &pass_data.legs()[0];
        assert_eq!(first_leg.operating_carrier_pnr_code(), "ABC123 ");
        assert_eq!(first_leg.from_city_airport_code(), "YUL");
        assert_eq!(first_leg.to_city_airport_code(), "FRA");
        assert_eq!(first_leg.operating_carrier_designator(), "AC ");
        assert_eq!(first_leg.flight_number(), "0834 ");
        assert_eq!(first_leg.date_of_flight(), "226");
        assert_eq!(first_leg.compartment_code(), 'F');
        assert_eq!(first_leg.seat_number(), "001A");
        assert_eq!(first_leg.check_in_sequence_number(), "0025 ");
        assert_eq!(first_leg.passenger_status(), '1');

        assert_eq!(first_leg.airline_numeric_code(), Some("014"));
        assert_eq!(first_leg.document_form_serial_number(), Some("1234567890"));
        assert_eq!(first_leg.selectee_indicator(), Some(' '));
        assert_eq!(first_leg.international_document_verification(), Some('1'));
        assert_eq!(first_leg.marketing_carrier_designator(), Some("AC "));
        assert_eq!(first_leg.frequent_flyer_airline_designator(), Some("AC "));
        assert_eq!(first_leg.frequent_flyer_number(), Some("1234567890123   "));
        assert_eq!(first_leg.id_ad_indicator(), Some(' '));
        assert_eq!(first_leg.free_baggage_allowance(), Some("20K"));
        assert_eq!(first_leg.fast_track(), Some('Y'));
        assert_eq!(first_leg.airline_individual_use(), Some("LX58Z"));
    }

    { // Fields in leg 2 of 2.
        let second_leg = &pass_data.legs()[1];
        assert_eq!(second_leg.operating_carrier_pnr_code(), "DEF456 ");
        assert_eq!(second_leg.from_city_airport_code(), "FRA");
        assert_eq!(second_leg.to_city_airport_code(), "GVA");
        assert_eq!(second_leg.operating_carrier_designator(), "LH ");
        assert_eq!(second_leg.flight_number(), "3664 ");
        assert_eq!(second_leg.date_of_flight(), "227");
        assert_eq!(second_leg.compartment_code(), 'C');
        assert_eq!(second_leg.seat_number(), "012C");
        assert_eq!(second_leg.check_in_sequence_number(), "0002 ");
        assert_eq!(second_leg.passenger_status(), '1');

        assert_eq!(second_leg.airline_numeric_code(), Some("014"));
        assert_eq!(second_leg.document_form_serial_number(), Some("0987654321"));
        assert_eq!(second_leg.selectee_indicator(), Some(' '));
        assert_eq!(second_leg.international_document_verification(), Some('1'));
        assert_eq!(second_leg.marketing_carrier_designator(), Some("AC "));
        assert_eq!(second_leg.frequent_flyer_airline_designator(), Some("AC "));
        assert_eq!(second_leg.frequent_flyer_number(), Some("1234567890123   "));
        assert_eq!(second_leg.id_ad_indicator(), Some(' '));
        assert_eq!(second_leg.free_baggage_allowance(), Some("2PC"));
        assert_eq!(second_leg.fast_track(), Some('N'));
        assert_eq!(second_leg.airline_individual_use(), Some("WQ"));
    }

    assert_eq!(pass_data.security_data().type_of_security_data(), Some('1'));
    assert_eq!(pass_data.security_data().security_data(), Some("GIWVC5EH7JNT684FVNJ91W2QA4DVN5J8K4F0L0GEQ3DF5TGBN8709HKT5D3DW3GBHFCVHMY7J5T6HFR41W2QA4DVN5J8K4F0L0GE"));
}
