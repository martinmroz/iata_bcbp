// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::ffi;
use std::ptr;
use std::str::FromStr;

use libc::{c_char, c_int};

use super::Bcbp;

/// Construct a new `Bcbp` by parsing the provided input string.
/// 
/// # Note
/// 
/// If the string passed in isn't a valid BCBP Type 'M' data string
/// this will return a null pointer.
///
/// # Safety
///
/// Make sure you destroy the object with [`bcbp_destroy()`] once you are
/// done with it.
///
/// [`BcbpDestroy()`]: fn.BcbpDestroy.html
/// 
/// # Issues
/// _: Return an error if parse fails instead of `null`.
#[no_mangle]
pub unsafe extern "C" fn BcbpCreateWithCString(input: *const c_char) -> *mut Bcbp {
    if input.is_null() {
        return ptr::null_mut();
    }

    let input_str = {
        if let Ok(value) = ffi::CStr::from_ptr(input).to_str() {
            value
        } else {
            return ptr::null_mut();
        }
    };

    if let Ok(bcbp) = Bcbp::from_str(input_str) {
        Box::into_raw(Box::new(bcbp))
    } else {
        ptr::null_mut()
    }
}

/// Destroy a `Bcbp` once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn BcbpDestroy(bcbp_ptr: *mut Bcbp) {
    if !bcbp_ptr.is_null() {
        drop(Box::from_raw(bcbp_ptr));
    }
}

/// Destroy a string copied out of a `Bcbp` instance once copied.
#[no_mangle]
pub unsafe extern "C" fn BcbpDestroyString(string: *mut c_char) {
    if !string.is_null() {
        drop(ffi::CString::from_raw(string));
    }
}

/// Returns the number of legs encoded within a boarding pass.
/// 
/// # Note
///
/// If the `Bcbp` object provided is null, this will return 0.
#[no_mangle]
pub unsafe extern "C" fn BcbpGetNumberOfLegs(bcbp_ptr: *mut Bcbp) -> c_int {
    if bcbp_ptr.is_null() {
        0
    } else {
        (&*bcbp_ptr).legs().len() as c_int
    }
}

/// Identifies a field at the root level of a boarding pass.
pub type BcbpFieldId = c_int;

/// Always `null`.
#[allow(non_upper_case_globals)]
pub const kBcbpFieldIdUnknown: c_int = 0;
/// The name of the passenger, required, 20 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFieldIdPassengerName: c_int = 1;
/// Electronic ticket indicator, required, 1 byte.
#[allow(non_upper_case_globals)]
pub const kBcbpFieldIdElectronicTicketIndicator: c_int = 2;
/// Passenger description, optional, 1 byte.
#[allow(non_upper_case_globals)]
pub const kBcbpFieldIdPassengerDescription: c_int = 3;
/// Source of check-in, optional, 1 byte.
#[allow(non_upper_case_globals)]
pub const kBcbpFieldIdSourceOfCheckIn: c_int = 4;
/// Source of boarding pass issuance, optional, 1 byte.
#[allow(non_upper_case_globals)]
pub const kBcbpFieldIdSourceOfBoardingPassIssuance: c_int = 5;
/// Date of issue of boarding pass, optional, 4 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFieldIdDateOfIssueOfBoardingPass: c_int = 6;
/// Document type, optional, 1 byte.
#[allow(non_upper_case_globals)]
pub const kBcbpFieldIdDocumentType: c_int = 7;
/// Airline designator of boarding pass issuer, optional, 3 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFieldIdAirlineDesignatorOfBoardingPassIssuer: c_int = 8;
/// Baggage tag license plate numbers, optional, 13 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFieldIdBaggageTagLicensePlateNumbers: c_int = 9;
/// First non-consecutive baggage tag license plate numbers, optional, 13 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFieldIdFirstNonConsecutiveBaggageTagLicensePlateNumbers: c_int = 10;
/// Second non-consecutive baggage tag license plate numbers, optional, 13 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFieldIdSecondNonConsecutiveBaggageTagLicensePlateNumbers: c_int = 11;

/// Returns a copy of the specified field.
///
/// # Note
///
/// If the `Bcbp` object provided is null, this will return a null pointer.
/// If an optional field is not specified, this will return a null pointer.
/// Even if a field is specified within a boarding pass, it may be empty (all space)
/// or contain invalid data.
///
/// # Safety
///
/// Make sure you destroy the result with [`BcbpDestroyString()`] once you are
/// done with it.
///
/// [`BcbpDestroyString()`]: fn.BcbpDestroyString.html
/// 
/// # Issues
/// _: Handle failure to coerce an &str into a CString differently, consider a panic.
/// _: Return a specific error in the event an invalid field is provided.
#[no_mangle]
pub unsafe extern "C" fn BcbpCopyField(bcbp_ptr: *mut Bcbp, field_id: BcbpFieldId) -> *mut c_char {
    if bcbp_ptr.is_null() {
        return ptr::null_mut();
    }

    let bcbp = &*bcbp_ptr;

    // Extract the specified field from the boarding pass root.
    let field_value: Option<ffi::CString> = {
        if field_id == kBcbpFieldIdPassengerName {
            ffi::CString::new(bcbp.passenger_name()).ok()
        } else if field_id == kBcbpFieldIdElectronicTicketIndicator {
            ffi::CString::new(vec![bcbp.electronic_ticket_indicator() as u8]).ok()
        } else if field_id == kBcbpFieldIdPassengerDescription {
            bcbp.passenger_description().and_then(|c| ffi::CString::new(vec![c as u8]).ok())
        } else if field_id == kBcbpFieldIdSourceOfCheckIn {
            bcbp.source_of_check_in().and_then(|c| ffi::CString::new(vec![c as u8]).ok())
        } else if field_id == kBcbpFieldIdSourceOfBoardingPassIssuance {
            bcbp.source_of_boarding_pass_issuance().and_then(|c| ffi::CString::new(vec![c as u8]).ok())
        } else if field_id == kBcbpFieldIdDateOfIssueOfBoardingPass {
            bcbp.date_of_issue_of_boarding_pass().and_then(|s| ffi::CString::new(s).ok())
        } else if field_id == kBcbpFieldIdDocumentType {
            bcbp.document_type().and_then(|c| ffi::CString::new(vec![c as u8]).ok())
        } else if field_id == kBcbpFieldIdAirlineDesignatorOfBoardingPassIssuer {
            bcbp.airline_designator_of_boarding_pass_issuer().and_then(|s| ffi::CString::new(s).ok())
        } else if field_id == kBcbpFieldIdBaggageTagLicensePlateNumbers {
            bcbp.baggage_tag_license_plate_numbers().and_then(|s| ffi::CString::new(s).ok())
        } else if field_id == kBcbpFieldIdFirstNonConsecutiveBaggageTagLicensePlateNumbers {
            bcbp.first_non_consecutive_baggage_tag_license_plate_numbers().and_then(|s| ffi::CString::new(s).ok())
        } else if field_id == kBcbpFieldIdSecondNonConsecutiveBaggageTagLicensePlateNumbers {
            bcbp.second_non_consecutive_baggage_tag_license_plate_numbers().and_then(|s| ffi::CString::new(s).ok())
        } else {
            None
        }
    };

    field_value
        .map(ffi::CString::into_raw)
        .unwrap_or(ptr::null_mut())
}

/// Identifies a field within the security data section of a boarding pass.
pub type BcbpSecurityFieldId = c_int;

/// Always `null`.
#[allow(non_upper_case_globals)]
pub const kBcbpSecurityFieldIdUnknown: c_int = 0;
/// The name of the passenger, required, 20 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpSecurityFieldIdTypeOfSecurityData: c_int = 1;
/// Electronic ticket indicator, required, 1 byte.
#[allow(non_upper_case_globals)]
pub const kBcbpSecurityFieldIdSecurityData: c_int = 2;

/// Returns a copy of the specified security data field.
///
/// # Note
///
/// If the `Bcbp` object provided is null, this will return a null pointer.
/// If an optional field is not specified, this will return a null pointer.
/// Even if a field is specified within a boarding pass, it may be empty (all space)
/// or contain invalid data.
///
/// # Safety
///
/// Make sure you destroy the result with [`BcbpDestroyString()`] once you are
/// done with it.
///
/// [`BcbpDestroyString()`]: fn.BcbpDestroyString.html
/// 
/// # Issues
/// _: Handle failure to coerce an &str into a CString differently, consider a panic.
/// _: Return a specific error in the event an invalid field is provided.
#[no_mangle]
pub unsafe extern "C" fn BcbpCopySecurityField(bcbp_ptr: *mut Bcbp, field_id: BcbpSecurityFieldId) -> *mut c_char {
    if bcbp_ptr.is_null() {
        return ptr::null_mut();
    }

    let bcbp = &*bcbp_ptr;

    // Extract the specified field from the boarding pass root.
    let field_value: Option<ffi::CString> = {
        if field_id == kBcbpSecurityFieldIdTypeOfSecurityData {
            bcbp.security_data().type_of_security_data().and_then(|c| ffi::CString::new(vec![c as u8]).ok())
        } else if field_id == kBcbpSecurityFieldIdSecurityData {
            bcbp.security_data().security_data().and_then(|s| ffi::CString::new(s).ok())
        } else {
            None
        }
    };

    field_value
        .map(ffi::CString::into_raw)
        .unwrap_or(ptr::null_mut())
}

/// Identifies a field within the security data section of a boarding pass.
pub type BcbpFlightLegFieldId = c_int;

/// Always `null`.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdUnknown: c_int = 0;
/// Operating carrier PNR code, required, 6 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdOperatingCarrierPNRCode: c_int = 1;
/// From city airport code, required, 4 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdFromCityAirportCode: c_int = 2;
/// To city airport code, required, 4 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdToCityAirportCode: c_int = 3;
/// Operating carrier designator, required, 3 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdOperatingCarrierDesignator: c_int = 4;
/// Flight number, required, 5 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdFlightNumber: c_int = 5;
/// Date of flight, required, 3 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdDateOfFlight: c_int = 6;
/// Compartment code, required, 1 byte.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdCompartmentCode: c_int = 7;
/// Seat number, required, 4 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdSeatNumber: c_int = 8;
/// Check-in Sequence Number, required, 5 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdCheckInSequenceNumber: c_int = 9;
/// Passenger status, required, 1 byte.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdPassengerStatus: c_int = 10;
/// Airline numeric code, optional, 3 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdAirlineNumericCode: c_int = 11;
/// Document form serial number, optional, 10 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdDocumentFormSerialNumber: c_int = 12;
/// Selectee Indicator, optional, 1 byte.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdSelecteeIndicator: c_int = 13;
/// International Document Verification, optional, 1 byte.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdInternationalDocumentVerification: c_int = 14;
/// Marketing Carrier Designator, optional, 3 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdMarketingCarrierDesignator: c_int = 15;
/// Frequent Flyer Airline Designator, optional, 3 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdFrequentFlyerAirlineDesignator: c_int = 16;
/// Frequent Flyer Number, optional, 16 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdFrequentFlyerNumber: c_int = 17;
/// ID/AD Indicator, optional, 1 byte.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdIdAdIndicator: c_int = 18;
/// Free Baggage Allowance, optional, 3 bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdFreeBaggageAllowance: c_int = 19;
/// Fast Track, optional, 1 byte.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdFastTrack: c_int = 19;
/// Airline Individual Use, optional, n bytes.
#[allow(non_upper_case_globals)]
pub const kBcbpFlightLegFieldIdAirlineIndividualUse: c_int = 20;

/// Returns a copy of the specified flight leg data field.
///
/// # Note
///
/// If the `Bcbp` object provided is null, a null pointer will be returned.
/// If an optional field is not specified, a null pointer will be returned.
/// If the flight leg is invalid, a null pointer will be returned.
/// Even if a field is specified within a boarding pass, it may be empty (all space)
/// or contain invalid data.
///
/// # Safety
///
/// Make sure you destroy the result with [`BcbpDestroyString()`] once you are
/// done with it.
///
/// [`BcbpDestroyString()`]: fn.BcbpDestroyString.html
/// 
/// # Issues
/// _: Handle failure to coerce an &str into a CString differently, consider a panic.
/// _: Return a specific error in the event an invalid field is provided.
/// _: Return a specific error in the event an invalid leg is provided.
#[no_mangle]
pub unsafe extern "C" fn BcbpCopyFlightLegField(bcbp_ptr: *mut Bcbp, leg: c_int, field_id: BcbpFlightLegFieldId) -> *mut c_char {
    if bcbp_ptr.is_null() {
        return ptr::null_mut();
    }

    let bcbp = &*bcbp_ptr;

    if leg < 0 || (leg as usize) >= bcbp.legs().len() {
        return ptr::null_mut();
    }

    let flight_leg = &bcbp.legs()[leg as usize];

    // Extract the specified field from the boarding pass root.
    let field_value: Option<ffi::CString> = {
        if field_id == kBcbpFlightLegFieldIdOperatingCarrierPNRCode {
            ffi::CString::new(flight_leg.operating_carrier_pnr_code()).ok()
        } else if field_id == kBcbpFlightLegFieldIdFromCityAirportCode {
            ffi::CString::new(flight_leg.from_city_airport_code()).ok()
        } else if field_id == kBcbpFlightLegFieldIdToCityAirportCode {
            ffi::CString::new(flight_leg.to_city_airport_code()).ok()
        } else if field_id == kBcbpFlightLegFieldIdOperatingCarrierDesignator {
            ffi::CString::new(flight_leg.operating_carrier_designator()).ok()
        } else if field_id == kBcbpFlightLegFieldIdFlightNumber {
            ffi::CString::new(flight_leg.flight_number()).ok()
        } else if field_id == kBcbpFlightLegFieldIdDateOfFlight {
            ffi::CString::new(flight_leg.date_of_flight()).ok()
        } else if field_id == kBcbpFlightLegFieldIdCompartmentCode {
            ffi::CString::new(vec![flight_leg.compartment_code() as u8]).ok()
        } else if field_id == kBcbpFlightLegFieldIdSeatNumber {
            ffi::CString::new(flight_leg.seat_number()).ok()
        } else if field_id == kBcbpFlightLegFieldIdCheckInSequenceNumber {
            ffi::CString::new(flight_leg.check_in_sequence_number()).ok()
        } else if field_id == kBcbpFlightLegFieldIdPassengerStatus {
            ffi::CString::new(vec![flight_leg.passenger_status() as u8]).ok()
        } else if field_id == kBcbpFlightLegFieldIdAirlineNumericCode {
            flight_leg.airline_numeric_code().and_then(|s| ffi::CString::new(s).ok())
        } else if field_id == kBcbpFlightLegFieldIdDocumentFormSerialNumber {
            flight_leg.document_form_serial_number().and_then(|s| ffi::CString::new(s).ok())
        } else if field_id == kBcbpFlightLegFieldIdSelecteeIndicator {
            flight_leg.selectee_indicator().and_then(|c| ffi::CString::new(vec![c as u8]).ok())
        } else if field_id == kBcbpFlightLegFieldIdInternationalDocumentVerification {
            flight_leg.international_document_verification().and_then(|c| ffi::CString::new(vec![c as u8]).ok())
        } else if field_id == kBcbpFlightLegFieldIdMarketingCarrierDesignator {
            flight_leg.marketing_carrier_designator().and_then(|s| ffi::CString::new(s).ok())
        } else if field_id == kBcbpFlightLegFieldIdFrequentFlyerAirlineDesignator {
            flight_leg.frequent_flyer_airline_designator().and_then(|s| ffi::CString::new(s).ok())
        } else if field_id == kBcbpFlightLegFieldIdFrequentFlyerNumber {
            flight_leg.frequent_flyer_number().and_then(|s| ffi::CString::new(s).ok())
        } else if field_id == kBcbpFlightLegFieldIdIdAdIndicator {
            flight_leg.id_ad_indicator().and_then(|c| ffi::CString::new(vec![c as u8]).ok())
        } else if field_id == kBcbpFlightLegFieldIdFreeBaggageAllowance {
            flight_leg.free_baggage_allowance().and_then(|s| ffi::CString::new(s).ok())
        } else if field_id == kBcbpFlightLegFieldIdFastTrack {
            flight_leg.fast_track().and_then(|c| ffi::CString::new(vec![c as u8]).ok())
        } else if field_id == kBcbpFlightLegFieldIdAirlineIndividualUse {
            flight_leg.airline_individual_use().and_then(|s| ffi::CString::new(s).ok())
        } else {
            None
        }
    };

    field_value
        .map(ffi::CString::into_raw)
        .unwrap_or(ptr::null_mut())
}
