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
