// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use bcbp;
use error::{Error, Result};
use de::field;

#[derive(Clone,Eq,PartialEq,Hash,Debug)]
struct Scanner<'a> {
    input: &'a str,
}

impl<'a> Scanner<'a> {

    /// Return a new intance of the receiver over the `input`.
    pub fn new(input: &'a str) -> Self {
        Scanner { input }
    }

    /// Returns `true` if no more input is available.
    #[inline]
    pub fn is_at_end(&self) -> bool {
        self.remaining_len() == 0
    }

    /// Returns the number of bytes of input remaining to process.
    #[inline]
    pub fn remaining_len(&self) -> usize {
        self.input.len()
    }

    /// Returns a scanner over a fixed-length sub-section of the input.
    /// The entire amount is consumed immediately if space is available whether or not
    /// any fields within the sub-section are invalid.
    /// 
    /// # Panics
    /// Will panic if `len` is `0`.
    pub fn scan_section(&mut self, len: usize) -> Result<Scanner<'a>> {
        println!("[TRACE] Scan Sub-Field List Length {}", len);
        assert!(len > 0, "Attempting to scan a zero-length sub-field list is not valid.");
        if self.remaining_len() < len {
            Err(Error::UnexpectedEndOfInput)
        } else {
            let sub_fields = &self.input[ .. len ];
            self.input = &self.input[ len .. ];
            Ok(Scanner::new(sub_fields))
        }
    }

    /// Scans and returns the string underlying a field (variable or fixed-length)
    /// with a specified length value.
    /// 
    /// # Panics
    /// Will panic if `len` is `0`.
    /// Will panic if the fixed-length field intrinsic length is not equal to `len`.
    pub fn scan_str_field_len(&mut self, field: field::Field, len: usize) -> Result<&'a str> {
        println!("[TRACE] {} (Length {})", field, len);
        assert!(len > 0, "Attempting to scan zero bytes of data.");
        assert!(field.len() == 0 || field.len() == len, "Length is not compatible the intrinsic length of the field.");
        if self.remaining_len() < len {
            Err(Error::UnexpectedEndOfInput)
        } else {
            let substring = &self.input[ .. len ];
            self.input = &self.input[ len .. ];
            Ok(substring)
        }
    }

    /// Scans and returns the string underlying a fixed-length field.
    /// Uses the intrinsic length.
    /// 
    /// # Panics
    /// Will panic if `field` is variable-length.
    pub fn scan_str_field(&mut self, field: field::Field) -> Result<&'a str> {
        assert!(field.len() != 0, "Attempting to scan a variable-length field as fixed-length.");
        self.scan_str_field_len(field, field.len())
    }

    /// Scans and returns an optional string underlying a fixed-length field.
    /// If there is no more input to process, returns `Ok(None)`.
    /// Uses the intrinsic length.
    /// 
    /// # Panics
    /// Will panic if `field` is variable-length.
    pub fn scan_optional_str_field(&mut self, field: field::Field) -> Result<Option<&'a str>> {
        assert!(field.len() != 0, "Attempting to scan a variable-length field as fixed-length.");
        if self.is_at_end() {
            Ok(None)
        } else {
            self.scan_str_field(field).map(|result| Some(result))
        }
    }

    /// Scans a fixed-length numeric field yielding the numeric value interpreted
    /// with the given `radix`.
    /// 
    /// # Panics
    /// Will panic if `field` is variable-length.
    /// 
    /// # Issues
    /// Should not advance the input until the numeric value is sucessfully scanned.
    pub fn scan_unsigned_field(&mut self, field: field::Field, radix: u32) -> Result<u64> {
        self.scan_str_field(field)
            .and_then(|str_value| {
                u64::from_str_radix(str_value, radix).map_err(|_| Error::ExpectedInteger)
            })
    }

    /// Scans and returns the character value underlying a fixed-length field.
    /// 
    /// # Panics
    /// Will panic if `field` is a length other than 1.
    pub fn scan_char_field(&mut self, field: field::Field) -> Result<char> {
        assert!(field.len() == 1, "Attempting to scan a single character out of a longer field.");
        self.scan_str_field(field)
            .map(|value| value.chars().next().unwrap())
    }

    /// Scans and returns an optional character value underlying a fixed-length field.
    /// If there is no more input to process, returns `Ok(None)`.
    /// 
    /// # Panics
    /// Will panic if `field` is a length other than 1.
    pub fn scan_optional_char_field(&mut self, field: field::Field) -> Result<Option<char>> {
        assert!(field.len() == 1, "Attempting to scan a single character out of a longer field.");
        if self.is_at_end() {
            Ok(None)
        } else {
            self.scan_char_field(field).map(|c| Some(c))
        }
    }

}

pub fn from_str<'a>(input: &'a str) -> Result<bcbp::Bcbp> {
    if !input.chars().all(|c| c.is_ascii()) {
        return Err(Error::InvalidCharacters)
    }

    println!("Start of Trace.");
    let mut scanner = Scanner::new(input);

    // Check that the input string is likely an M-type BCBP string.
    if scanner.scan_str_field(field::Field::FormatCode)? != "M" {
        return Err(Error::UnsupportedFormat);
    }

    // The number of legs informs the breakdown of the various field iterators.
    let number_of_legs_encoded = scanner.scan_unsigned_field(field::Field::NumberOfLegsEncoded, 10)?;

    // Create a parser for the mandatory unique fields.
    let mut bcbp = bcbp::Bcbp::default();
    bcbp.passenger_name =
        scanner.scan_str_field(field::Field::PassengerName)?.into();
    bcbp.electronic_ticket_indicator =
        scanner.scan_char_field(field::Field::ElectronicTicketIndicator)?;

    for leg_index in 0 .. number_of_legs_encoded {
        let mut leg = bcbp::Leg::default();
        
        // Mandatory fields common to all legs.
        leg.operating_carrier_pnr_code =
            scanner.scan_str_field(field::Field::OperatingCarrierPnrCode)?.into();
        leg.from_city_airport_code =
            scanner.scan_str_field(field::Field::FromCityAirportCode)?.into();
        leg.to_city_airport_code =
            scanner.scan_str_field(field::Field::ToCityAirportCode)?.into();
        leg.operating_carrier_designator =
            scanner.scan_str_field(field::Field::OperatingCarrierDesignator)?.into();
        leg.flight_number =
            scanner.scan_str_field(field::Field::FlightNumber)?.into();
        leg.date_of_flight =
            scanner.scan_str_field(field::Field::DateOfFlight)?.into();
        leg.compartment_code =
            scanner.scan_char_field(field::Field::CompartmentCode)?;
        leg.seat_number =
            scanner.scan_str_field(field::Field::SeatNumber)?.into();
        leg.check_in_sequence_number =
            scanner.scan_str_field(field::Field::CheckInSequenceNumber)?.into();
        leg.passenger_status =
            scanner.scan_char_field(field::Field::PassengerStatus)?;

        // Field size of the variable size field that follows for the leg.
        let conditional_item_size = scanner.scan_unsigned_field(field::Field::FieldSizeOfVariableSizeField, 16)?;
        if conditional_item_size > 0 {
 
            // Scanner over the entire set of conditional fields.
            let mut conditional_item_scanner = scanner.scan_section(conditional_item_size as usize)?;

            // The first leg may contain some optional fields at the root level.
            if leg_index == 0 {

                // Validate the beginning of version number tag as a sanity check.
                if conditional_item_scanner.remaining_len() > 0 {
                    if conditional_item_scanner.scan_str_field(field::Field::BeginningOfVersionNumber)? != ">" {
                        return Err(Error::InvalidStartOfVersionNumber);
                    }
                }

                // The version number is part of the structure and must be consumed, but is not used.
                if conditional_item_scanner.remaining_len() > 0 {
                    let _ = conditional_item_scanner.scan_str_field(field::Field::VersionNumber)?;
                }

                // Conditional unique fields are embedded in their own variable-length wrapper.
                if conditional_item_scanner.remaining_len() > 0 {
                    let len = conditional_item_scanner.scan_unsigned_field(field::Field::FieldSizeOfStructuredMessageUnique, 16)?;
                    if len > 0 {
                        let mut unique_scanner = conditional_item_scanner.scan_section(len as usize)?;

                        bcbp.passenger_description =
                            unique_scanner.scan_optional_char_field(field::Field::PassengerDescription)?;
                        bcbp.source_of_check_in =
                            unique_scanner.scan_optional_char_field(field::Field::SourceOfCheckIn)?;
                        bcbp.source_of_boarding_pass_issuance =
                            unique_scanner.scan_optional_char_field(field::Field::SourceOfBoardingPassIssuance)?;
                        bcbp.date_of_issue_of_boarding_pass =
                            unique_scanner.scan_optional_str_field(field::Field::DateOfIssueOfBoardingPass)?.map(Into::into);
                        bcbp.document_type =
                            unique_scanner.scan_optional_char_field(field::Field::DocumentType)?;
                        bcbp.airline_designator_of_boarding_pass_issuer =
                            unique_scanner.scan_optional_str_field(field::Field::AirlineDesignatorOfBoardingPassIssuer)?.map(Into::into);
                        bcbp.baggage_tag_license_plate_numbers =
                            unique_scanner.scan_optional_str_field(field::Field::BaggageTagLicensePlateNumbers)?.map(Into::into);
                        bcbp.first_non_consecutive_baggage_tag_license_plate_number =
                            unique_scanner.scan_optional_str_field(field::Field::FirstNonConsecutiveBaggageTagLicensePlateNumber)?.map(Into::into);
                        bcbp.second_non_consecutive_baggage_tag_license_plate_number =
                            unique_scanner.scan_optional_str_field(field::Field::SecondNonConsecutiveBaggageTagLicensePlateNumber)?.map(Into::into);
                    }
                }
            }

            // Conditional fields common to all legs.
            if conditional_item_scanner.remaining_len() > 0 {
                let len = conditional_item_scanner.scan_unsigned_field(field::Field::FieldSizeOfStructuredMessageRepeated, 16)?;
                if len > 0 {
                    let mut repeated_scanner = conditional_item_scanner.scan_section(len as usize)?;

                    leg.airline_numeric_code =
                        repeated_scanner.scan_optional_str_field(field::Field::AirlineNumericCode)?.map(Into::into);
                    leg.document_form_serial_number =
                        repeated_scanner.scan_optional_str_field(field::Field::DocumentFormSerialNumber)?.map(Into::into);
                    leg.selectee_indicator =
                        repeated_scanner.scan_optional_char_field(field::Field::SelecteeIndicator)?;
                    leg.international_document_verification =
                        repeated_scanner.scan_optional_char_field(field::Field::InternationalDocumentVerification)?;
                    leg.marketing_carrier_designator =
                        repeated_scanner.scan_optional_str_field(field::Field::MarketingCarrierDesignator)?.map(Into::into);
                    leg.frequent_flyer_airline_designator =
                        repeated_scanner.scan_optional_str_field(field::Field::FrequentFlyerAirlineDesignator)?.map(Into::into);
                    leg.frequent_flyer_number =
                        repeated_scanner.scan_optional_str_field(field::Field::FrequentFlyerNumber)?.map(Into::into);
                    leg.id_ad_indicator =
                        repeated_scanner.scan_optional_char_field(field::Field::IdAdIndicator)?;
                    leg.free_baggage_allowance =
                        repeated_scanner.scan_optional_str_field(field::Field::FreeBaggageAllowance)?.map(Into::into);
                    leg.fast_track =
                        repeated_scanner.scan_optional_char_field(field::Field::FastTrack)?;
                }
            }

            // Any remaining text is ascribed to airline use.
            if conditional_item_scanner.remaining_len() > 0 {
                let remaining_len = conditional_item_scanner.remaining_len();
                let body = conditional_item_scanner.scan_str_field_len(field::Field::AirlineIndividualUse, remaining_len)?;
                leg.airline_individual_use = Some(body.into());
            }
        }

        bcbp.legs.push(leg);
    }

    // Remaining input is ascribed to Security Data.
    if scanner.remaining_len() > 0 {
        if scanner.scan_str_field(field::Field::BeginningOfSecurityData)? != "^" {
            return Err(Error::InvalidStartOfSecurityData);
        }

        let mut security_data = bcbp::SecurityData::default();

        // The security data type captured as a separate field set as the next field, data length, is discarded.
        security_data.type_of_security_data =
            scanner.scan_optional_char_field(field::Field::TypeOfSecurityData)?;

        // Scan the length of the security data.
        if scanner.remaining_len() > 0 {
            let len = scanner.scan_unsigned_field(field::Field::LengthOfSecurityData, 16)?;
            if len > 0 {
                let body = scanner.scan_str_field_len(field::Field::SecurityData, len as usize)?;
                security_data.security_data = Some(body.into());
            }
        }

        bcbp.security_data = Some(security_data);
    }

    if !scanner.is_at_end() {
        Err(Error::TrailingCharacters)
    } else {
        Ok(bcbp)
    }
}
