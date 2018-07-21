
#[derive(Clone,Eq,PartialEq,Hash,Debug,Default)]
pub struct Leg {
    pub(crate) operating_carrier_pnr_code: String,
    pub(crate) from_city_airport_code: String,
    pub(crate) to_city_airport_code: String,
    pub(crate) operating_carrier_designator: String,
    pub(crate) flight_number: String,
    pub(crate) date_of_flight: String,
    pub(crate) compartment_code: char,
    pub(crate) seat_number: String,
    pub(crate) check_in_sequence_number: String,
    pub(crate) passenger_status: char,
    pub(crate) airline_numeric_code: Option<String>,
    pub(crate) document_form_serial_number: Option<String>,
    pub(crate) selectee_indicator: Option<char>,
    pub(crate) international_document_verification: Option<char>,
    pub(crate) marketing_carrier_designator: Option<String>,
    pub(crate) frequent_flyer_airline_designator: Option<String>,
    pub(crate) frequent_flyer_number: Option<String>,
    pub(crate) id_ad_indicator: Option<char>,
    pub(crate) free_baggage_allowance: Option<String>,
    pub(crate) fast_track: Option<char>,
    pub(crate) airline_individual_use: Option<String>,
}

#[derive(Clone,Eq,PartialEq,Hash,Debug,Default)]
pub struct SecurityData {
    pub(crate) type_of_security_data: Option<char>,
    pub(crate) security_data: Option<String>,
}

#[derive(Clone,Eq,PartialEq,Hash,Debug,Default)]
pub struct Bcbp {
    pub(crate) passenger_name: String,
    pub(crate) electronic_ticket_indicator: char,
    pub(crate) passenger_description: Option<char>,
    pub(crate) source_of_check_in: Option<char>,
    pub(crate) source_of_boarding_pass_issuance: Option<char>,
    pub(crate) date_of_issue_of_boarding_pass: Option<String>,
    pub(crate) document_type: Option<char>,
    pub(crate) airline_designator_of_boarding_pass_issuer: Option<String>,
    pub(crate) baggage_tag_license_plate_numbers: Option<String>,
    pub(crate) first_non_consecutive_baggage_tag_license_plate_number: Option<String>,
    pub(crate) second_non_consecutive_baggage_tag_license_plate_number: Option<String>,
    pub(crate) legs: Vec<Leg>,
    pub(crate) security_data: Option<SecurityData>,
}
