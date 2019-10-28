// Copyright (C) 2019 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

//! Test cases derived from real-world boarding pass data.

extern crate iata_bcbp;

use std::str::FromStr;

use iata_bcbp::*;

#[test]
fn alaska_boarding_pass() {
    const PASS_STR: &str = "M1MROZ/MARTIN         EXXXXXX SJCLAXAS 3317 207U001A0006 34D>218 VV8207BAS              2502771980993865 AS AS XXXXX55200000000Z29  00010";
    let pass_data = Bcbp::from_str(PASS_STR).unwrap();
    assert_eq!(pass_data.passenger_name(), "MROZ/MARTIN         ");
    assert_eq!(pass_data.electronic_ticket_indicator(), 'E');
    assert_eq!(pass_data.legs().len(), 1);

    assert_eq!(pass_data.passenger_description(), Some(' '));
    assert_eq!(pass_data.source_of_check_in(), Some('V'));
    assert_eq!(pass_data.source_of_boarding_pass_issuance(), Some('V'));
    assert_eq!(pass_data.date_of_issue_of_boarding_pass(), Some("8207"));
    assert_eq!(pass_data.document_type(), Some('B'));
    assert_eq!(pass_data.airline_designator_of_boarding_pass_issuer(), Some("AS "));
    assert_eq!(pass_data.baggage_tag_license_plate_numbers(), Some("             "));
    assert_eq!(pass_data.first_non_consecutive_baggage_tag_license_plate_numbers(), None);
    assert_eq!(pass_data.second_non_consecutive_baggage_tag_license_plate_numbers(), None);

    {
        // Fields in leg 1 of 1.
        let first_leg = &pass_data.legs()[0];
        assert_eq!(first_leg.operating_carrier_pnr_code(), "XXXXXX ");
        assert_eq!(first_leg.from_city_airport_code(), "SJC");
        assert_eq!(first_leg.to_city_airport_code(), "LAX");
        assert_eq!(first_leg.operating_carrier_designator(), "AS ");
        assert_eq!(first_leg.flight_number(), "3317 ");
        assert_eq!(first_leg.date_of_flight(), "207");
        assert_eq!(first_leg.compartment_code(), 'U');
        assert_eq!(first_leg.seat_number(), "001A");
        assert_eq!(first_leg.check_in_sequence_number(), "0006 ");
        assert_eq!(first_leg.passenger_status(), '3');

        assert_eq!(first_leg.airline_numeric_code(), Some("027"));
        assert_eq!(first_leg.document_form_serial_number(), Some("7198099386"));
        assert_eq!(first_leg.selectee_indicator(), Some('5'));
        assert_eq!(first_leg.international_document_verification(), Some(' '));
        assert_eq!(first_leg.marketing_carrier_designator(), Some("AS "));
        assert_eq!(first_leg.frequent_flyer_airline_designator(), Some("AS "));
        assert_eq!(first_leg.frequent_flyer_number(), Some("XXXXX55200000000"));
        assert_eq!(first_leg.id_ad_indicator(), None);
        assert_eq!(first_leg.free_baggage_allowance(), None);
        assert_eq!(first_leg.fast_track(), None);
        assert_eq!(first_leg.airline_individual_use(), Some("Z29  00010"));
    }
}

#[test]
fn air_canada_boarding_pass() {
    const PASS_STR: &str = "M1Mroz/Martin         EXXXXXX YVRYOWAC 0344 211          072>20B0  8203IAC 250140000000000 0AC AC AC000000000     *20000AC 223                14080003068        0B          N";
    let pass_data = Bcbp::from_str(PASS_STR).unwrap();
    assert_eq!(pass_data.passenger_name(), "Mroz/Martin         ");
    assert_eq!(pass_data.electronic_ticket_indicator(), 'E');
    assert_eq!(pass_data.legs().len(), 1);

    assert_eq!(pass_data.passenger_description(), Some('0'));
    assert_eq!(pass_data.source_of_check_in(), Some(' '));
    assert_eq!(pass_data.source_of_boarding_pass_issuance(), Some(' '));
    assert_eq!(pass_data.date_of_issue_of_boarding_pass(), Some("8203"));
    assert_eq!(pass_data.document_type(), Some('I'));
    assert_eq!(pass_data.airline_designator_of_boarding_pass_issuer(), Some("AC "));
    assert_eq!(pass_data.baggage_tag_license_plate_numbers(), None);
    assert_eq!(pass_data.first_non_consecutive_baggage_tag_license_plate_numbers(), None);
    assert_eq!(pass_data.second_non_consecutive_baggage_tag_license_plate_numbers(), None);

    {
        // Fields in leg 1 of 1.
        let first_leg = &pass_data.legs()[0];
        assert_eq!(first_leg.operating_carrier_pnr_code(), "XXXXXX ");
        assert_eq!(first_leg.from_city_airport_code(), "YVR");
        assert_eq!(first_leg.to_city_airport_code(), "YOW");
        assert_eq!(first_leg.operating_carrier_designator(), "AC ");
        assert_eq!(first_leg.flight_number(), "0344 ");
        assert_eq!(first_leg.date_of_flight(), "211");
        assert_eq!(first_leg.compartment_code(), ' ');
        assert_eq!(first_leg.seat_number(), "    ");
        assert_eq!(first_leg.check_in_sequence_number(), "     ");
        assert_eq!(first_leg.passenger_status(), '0');

        assert_eq!(first_leg.airline_numeric_code(), Some("014"));
        assert_eq!(first_leg.document_form_serial_number(), Some("0000000000"));
        assert_eq!(first_leg.selectee_indicator(), Some(' '));
        assert_eq!(first_leg.international_document_verification(), Some('0'));
        assert_eq!(first_leg.marketing_carrier_designator(), Some("AC "));
        assert_eq!(first_leg.frequent_flyer_airline_designator(), Some("AC "));
        assert_eq!(first_leg.frequent_flyer_number(), Some("AC000000000     "));
        assert_eq!(first_leg.id_ad_indicator(), None);
        assert_eq!(first_leg.free_baggage_allowance(), None);
        assert_eq!(first_leg.fast_track(), None);
        assert_eq!(first_leg.airline_individual_use(), Some("*20000AC 223                14080003068        0B          N"));
    }
}
