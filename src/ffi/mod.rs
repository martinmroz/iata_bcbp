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
#[no_mangle]
pub static kBcbpFieldUnknown: BcbpFieldId = 0;
/// The name of the passenger, required, 20 bytes.
#[no_mangle]
pub static kBcbpFieldPassengerName: BcbpFieldId = 1;
/// Electronic ticket indicator, required, 1 byte.
#[no_mangle]
pub static kBcbpFieldElectronicTicketIndicator: BcbpFieldId = 2;
/// Passenger description, optional, 1 byte.
#[no_mangle]
pub static kBcbpFieldPassengerDescription: BcbpFieldId = 3;
/// Source of check-in, optional, 1 byte.
#[no_mangle]
pub static kBcbpFieldSourceOfCheckIn: BcbpFieldId = 4;
/// Source of boarding pass issuance, optional, 1 byte.
#[no_mangle]
pub static kBcbpFieldSourceOfBoardingPassIssuance: BcbpFieldId = 5;
/// Date of issue of boarding pass, optional, 4 bytes.
#[no_mangle]
pub static kBcbpFieldDateOfIssueOfBoardingPass: BcbpFieldId = 6;
/// Document type, optional, 1 byte.
#[no_mangle]
pub static kBcbpFieldDocumentType: BcbpFieldId = 7;
/// Airline designator of boarding pass issuer, optional, 3 bytes.
#[no_mangle]
pub static kBcbpFieldAirlineDesignatorOfBoardingPassIssuer: BcbpFieldId = 8;
/// Baggage tag license plate numbers, optional, 13 bytes.
#[no_mangle]
pub static kBcbpFieldBaggageTagLicensePlateNumbers: BcbpFieldId = 9;
/// First non-consecutive baggage tag license plate numbers, optional, 13 bytes.
#[no_mangle]
pub static kBcbpFieldFirstNonConsecutiveBaggageTagLicensePlateNumbers: BcbpFieldId = 10;
/// Second non-consecutive baggage tag license plate numbers, optional, 13 bytes.
#[no_mangle]
pub static kBcbpFieldSecondNonConsecutiveBaggageTagLicensePlateNumbers: BcbpFieldId = 11;

/// Copies the field.
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
#[no_mangle]
pub unsafe extern "C" fn BcbpCopyField(bcbp_ptr: *mut Bcbp, field_id: BcbpFieldId) -> *mut c_char {
    if bcbp_ptr.is_null() {
        return ptr::null_mut();
    }

    let bcbp = &*bcbp_ptr;

    ptr::null_mut()
}
